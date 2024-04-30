use crate::{
    log, o,
    sdk::{
        bf_read::BfRead,
        net_channel::{NetMessage, NetMessageTypeClient},
        HasVmt,
    },
};

use super::Cheat;

#[derive(Debug)]
pub struct SpreadReduction {}

impl SpreadReduction {
    pub fn name() -> &'static str {
        "SpreadReduction"
    }
    pub fn init() -> SpreadReduction {
        SpreadReduction {}
    }

    pub fn send_user_msg_pre(&self, message: &NetMessage) -> bool {
        if !matches!(message.get_type(), NetMessageTypeClient::Move) {
            return false;
        }
        o!().client_state.send_string_cmd("playerperf\n");
        return true;
    }
    pub fn send_user_msg_post(&self, message: &NetMessage) {
        if !matches!(message.get_type(), NetMessageTypeClient::Move) {
            return;
        }
        o!().client_state.send_string_cmd("playerperf\n");
    }

    pub fn dispatch_user_message(&mut self, msg_type: u32, buffer: &mut BfRead) -> bool {
        if msg_type != 5 {
            return false;
        }

        let destiantion = buffer.read_byte().unwrap();
        dbg!(destiantion);
        if destiantion != 2 {
            return false;
        }

        let message = buffer.read_string(200).unwrap();
        log!("{}", message);
        //log!("{}", (o!().util.plat_float_time)());

        return true;
    }
}
impl Cheat for SpreadReduction {}
