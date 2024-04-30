use crate::cfn;

use super::{HasVmt, WithVmt};
pub type NetChannel = WithVmt<VMTNetChannel>;
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTNetChannel {
    _pad: [usize; 38],
    pub send_net_msg: cfn!(bool, &NetChannel, &NetMessage, bool, bool), //virtual bool	SendNetMsg(INetMessage &msg, bool bForceReliable = false, bool bVoice = false ) = 0;
}

pub type NetMessage = WithVmt<VMTNetMessage>;
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTNetMessage {
    _pad: [usize; 8],
    pub get_type: extern "C" fn() -> NetMessageTypeClient,
}
impl NetMessage {
    pub fn get_type(&self) -> NetMessageTypeClient {
        (self.get_vmt().get_type)()
    }
}
#[derive(Debug, Clone, Copy)]
pub enum NetMessageTypeServer {
    NOP = 0,
    Disconnect = 1,
    File = 2,
    Tick = 3,
    StringCmd = 4,
    SetConVar = 5,
    SignonState = 6,
    Print = 7,
    ServerInfo = 8,
    SendTable = 9,
    ClassInfo = 10,
    SetPause = 11,
    CreateStringTable = 12,
    UpdateStringTable = 13,
    VoiceInit = 14,
    VoiceData = 15,
    Sounds = 17,
    SetView = 18,
    FixAngle = 19,
    CrosshairAngle = 20,
    UserMessage = 23,
    EntityMessage = 24,
    GameEvent = 25,
    PacketEntities = 26,
    TempEntities = 27,
    Prefetch = 28,
    Menu = 29,
    GameEventList = 30,
    GetCvarValue = 31,
}
#[derive(Debug, Clone, Copy)]
pub enum NetMessageTypeClient {
    NOP = 0,
    Disconnect = 1,
    File = 2,
    Tick = 3,
    StringCmd = 4,
    SetConVar = 5,
    SignonState = 6,
    Print = 7,
    ClientInfo = 8,
    Move = 9,
    VoiceData = 10,
    BaselineAck = 11,
    ListenEvents = 12,
    RespondCvarValue = 13,
    FileCRCCheck = 14,
    CmdKeyValues = 16,
}
