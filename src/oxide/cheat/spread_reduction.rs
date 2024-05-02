use std::{ffi::CString, mem::transmute};

use crate::{
    call_original, interface, log, o,
    oxide::hook::cl_send_move::{self, ClSendMove},
    sdk::{
        bf_read::BfRead,
        entity::player::Player,
        net_channel::{NetMessage, NetMessageTypeClient},
        user_cmd::{ButtonFlags, UserCmd},
    },
    vmt_call,
};

use super::Cheat;

#[derive(Debug, Clone)]
pub struct SpreadReduction {
    pub playerperf_send_time: Option<f32>,
    pub time_delta: Option<f32>,
    pub cl_send_move_delay: Option<f32>,
    pub next_update: f32,
    pub updating_usercmd_delay: bool,
}
const PLAYERPERF_COOLDOWN: f32 = 10.0;
const PLAYERPERF_BIAS: f32 = 0.0008544922;

impl SpreadReduction {
    pub fn name() -> &'static str {
        "SpreadReduction"
    }
    pub fn init() -> SpreadReduction {
        SpreadReduction {
            playerperf_send_time: None,
            time_delta: None,
            cl_send_move_delay: None,
            next_update: 0.0,
            updating_usercmd_delay: false,
        }
    }
    pub fn should_sync_delta(&self, message: &NetMessage) -> bool {
        let time = (o!().util.plat_float_time)() as f32;
        matches!(message.get_type(), NetMessageTypeClient::Move)
            && self.playerperf_send_time.is_none()
            && self.next_update < time
    }
    pub fn send_user_msg_post(&mut self) {
        let time = (o!().util.plat_float_time)() as f32;
        self.next_update = time + PLAYERPERF_COOLDOWN;

        let playerperf_cmd = CString::new("playerperf").unwrap();
        vmt_call!(
            interface!(base_engine),
            send_cmd_unrestricted,
            playerperf_cmd.as_ptr()
        );

        //vmt_call!(channel, transmit, false);
        self.playerperf_send_time = Some((o!().util.plat_float_time)() as f32);
    }

    pub fn dispatch_user_message(&mut self, msg_type: u32, buffer: &mut BfRead) -> bool {
        if msg_type != 5 {
            return false;
        }
        let Some(send_time) = &self.playerperf_send_time else {return false};

        let destiantion = buffer.read_byte().unwrap();
        if destiantion != 2 {
            return false;
        }

        let message = buffer.read_string(200).unwrap();
        let mut server_time_records = Vec::new();
        for record in message.split("\n") {
            let data = record.split(" ").collect::<Vec<_>>();
            if !(data.len() == 7 || data.len() == 3) {
                continue;
            }
            let Ok(float) = data[0].parse::<f32>() else {return false};

            server_time_records.push(float);
        }
        let Some(&server_time) = server_time_records.first() else {return false};

        let mantissa_step =
            2 ^ ((unsafe { transmute::<_, i32>(server_time * 1000.0) } >> 23) & 0xff) - 127 - 23;
        if let Some(time_delta) = self.time_delta {
            log!(
                "resyncing server_delta\tpredicted server time: {}\terror: {}\tmantissa_step {}",
                send_time - time_delta,
                send_time + time_delta - server_time,
                mantissa_step
            );
        } else {
            log!("resyncing server_delta\tmantissa_step {}", mantissa_step);
        }
        self.time_delta = Some(server_time - send_time - PLAYERPERF_BIAS);

        self.playerperf_send_time = None;
        return true;
    }
    pub fn create_move(&mut self, cmd: &UserCmd) {
        let p_local = Player::get_local().unwrap();
        let weapon = vmt_call!(p_local.as_ent(), get_weapon);
        if !cmd.buttons.get(ButtonFlags::InAttack)
            || *weapon.get_next_primary_attack() >= o!().global_vars.curtime
        {
            return;
        }
        self.updating_usercmd_delay = true;
        let Some(delta) = self.time_delta else {return};
        let Some(cl_create_move_duration) = self.cl_send_move_delay else {return};
        let time = (o!().util.plat_float_time)() as f32;
        let predicted_time = time + delta + cl_create_move_duration;
        log!(
            "predicted seed: {}\tnum: {}\ttime: {}\tdelta: {}\tcl_create_move_duration: {}",
            unsafe { transmute::<_, i32>((predicted_time) * 1000.0) & 0xFF },
            cmd.command_number,
            predicted_time,
            delta,
            cl_create_move_duration
        );
    }
    pub fn write_user_cmd(&mut self, to: &UserCmd) {}
    pub fn cl_send_move(&mut self) {
        if !self.updating_usercmd_delay {
            call_original!(cl_send_move::NAME, ClSendMove);
            return;
        }
        let before = (o!().util.plat_float_time)() as f32;
        call_original!(cl_send_move::NAME, ClSendMove);
        let delay = (o!().util.plat_float_time)() as f32 - before;
        //log!("calculated usercmd delay: {}", delay);
        self.cl_send_move_delay = Some(delay);
        self.updating_usercmd_delay = false
    }
}
impl Cheat for SpreadReduction {}
