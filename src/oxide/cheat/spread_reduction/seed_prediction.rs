use std::{
    ffi::CString,
    fmt::{self, Display},
    mem::transmute,
};

use crate::{
    error::{OxideError, OxideResult},
    interface, o,
    sdk::{
        bf_read::BfRead,
        net_channel::{LatencyFlow, NetMessage, NetMessageTypeClient},
    },
    vmt_call,
};

use super::SpreadReduction;

const PLAYERPERF_COOLDOWN: f32 = 1.0;
const PLAYERPERF_RESYNC_COOLDOWN: f32 = 10.0;
pub const SERVER_SPOOF_EXPONENT: i32 = 15;
const MIN_MANTISA: f32 = 1.0 / 8388608.0;

#[derive(Debug)]
pub enum State {
    SYNCING {
        delta: f32,
        next_playerperf: f32,
        precision: i32,
    },
    IMPOSSIBLE {
        precision: i32,
    },
    SYNCED {
        last_seed: Option<i32>,
        delta: f32,
        next_playerperf: f32,
        precision: i32,
    },
    UNSYNCED,
}

impl Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let time = (o!().util.plat_float_time)() as f32;
        match self {
            State::SYNCING {
                next_playerperf,
                precision,
                ..
            } => write!(
                f,
                "SYNCING\nseed quality: {}\nnext sync: {:.2}",
                precision,
                next_playerperf - time
            ),
            State::IMPOSSIBLE { precision } => write!(
                f,
                "IMPOSSIBLE:\nserver too young to sync\nquality: {}",
                precision
            ),
            State::SYNCED {
                next_playerperf,
                precision,
                ..
            } => write!(
                f,
                "SYNCED\nseed quality: {}\nnext sync: {:.2}",
                precision,
                next_playerperf - time
            ),
            State::UNSYNCED => write!(f, "UNSYNCED"),
        }
    }
}

impl State {}

impl SpreadReduction {
    pub fn should_sync_delta(&self, message: &NetMessage) -> bool {
        if !matches!(message.get_type(), NetMessageTypeClient::Move)
            || self.playerperf_send_data.is_some()
            || !self.should_run()
        {
            return false;
        }

        let time = (o!().util.plat_float_time)() as f32;

        match &self.state {
            State::SYNCING {
                next_playerperf, ..
            } => *next_playerperf < time,
            State::SYNCED {
                next_playerperf, ..
            } => *next_playerperf < time,
            State::UNSYNCED => true,
            State::IMPOSSIBLE { .. } => false,
        }
    }
    pub fn send_user_msg_post(&mut self) {
        let playerperf_cmd = CString::new("playerperf").unwrap();
        vmt_call!(
            interface!(base_engine),
            send_cmd_unrestricted,
            playerperf_cmd.as_ptr()
        );
        let latency = vmt_call!(
            interface!(base_engine).get_net_channel().unwrap(),
            get_latency,
            LatencyFlow::OUTGOING
        );
        self.playerperf_send_data = Some(((o!().util.plat_float_time)() as f32, latency));
    }

    pub fn dispatch_user_message(&mut self, msg_type: u32, buffer: &mut BfRead) -> bool {
        if msg_type != 5 {
            return false;
        }

        let destiantion = buffer.read_byte().unwrap();
        if destiantion != 2 {
            return false;
        }
        if self.playerperf_send_data.is_none() {
            return false;
        };

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
        //INFO: temporary for testing
        //let server_time = server_time + 2f32.powi(SERVER_SPOOF_EXPONENT);
        let send_data = &self.playerperf_send_data.unwrap();
        self.state = self.calculate_state(server_time, *send_data);
        self.playerperf_send_data = None;

        return true;
    }
    pub fn get_precision(&mut self, time: f32) -> i32 {
        (2f32.powi(((unsafe { transmute::<f32, i32>(time) } >> 23) & 0xff) - 127) * MIN_MANTISA)
            .log2() as i32
    }
    pub fn calculate_state(&mut self, server_time: f32, send_data: (f32, f32)) -> State {
        let precision = self.get_precision(server_time * 1000f32);
        if precision < 1 {
            return State::IMPOSSIBLE { precision };
        }

        let server_time = server_time;
        let now = (o!().util.plat_float_time)() as f32;

        match self.state {
            State::SYNCING { delta, .. } => {
                let guess = (send_data.0 + send_data.1) + delta;
                if guess * 1000f32 == server_time * 1000f32 {
                    return State::SYNCED {
                        last_seed: None,
                        delta,
                        next_playerperf: now + PLAYERPERF_RESYNC_COOLDOWN * precision.pow(2) as f32,
                        precision,
                    };
                }
                State::SYNCING {
                    delta: delta - (guess - server_time) / 2.0,
                    next_playerperf: now + PLAYERPERF_COOLDOWN,
                    precision,
                }
            }
            State::SYNCED {
                last_seed, delta, ..
            } => {
                let guess = (send_data.0 + send_data.1) + delta;
                if guess * 1000f32 == server_time * 1000f32 {
                    return State::SYNCED {
                        last_seed,
                        delta,
                        next_playerperf: now + PLAYERPERF_RESYNC_COOLDOWN * precision as f32,
                        precision,
                    };
                }
                State::SYNCING {
                    delta: delta - (guess - server_time) / 2.0,
                    next_playerperf: now + PLAYERPERF_COOLDOWN,
                    precision,
                }
            }
            State::UNSYNCED => State::SYNCING {
                delta: server_time - send_data.0,
                next_playerperf: now + PLAYERPERF_COOLDOWN,
                precision,
            },
            State::IMPOSSIBLE { .. } => unreachable!(),
        }
    }
    pub fn get_server_time(&self, client_time: f32) -> OxideResult<f32> {
        let State::SYNCED { delta, .. } = self.state else {return Err(OxideError::new("seed not synced"))};
        let latency = vmt_call!(
            interface!(base_engine).get_net_channel().unwrap(),
            get_latency,
            LatencyFlow::OUTGOING
        );

        Ok(client_time + delta + latency)
    }
    pub fn calculate_seed(&self, time: f32) -> i32 {
        (unsafe { transmute::<_, i32>((time) * 1000f32) } & 0xFF)
    }
}
