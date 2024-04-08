
#[derive(Debug,Clone,Copy)]
pub struct Flags (u32);
impl Flags {
    pub fn get(&self, flag: Flag) -> bool {
        let flag = flag as u8;
        let shifted = 1 << flag;
        let Flags(b) = *self;
        b & shifted == shifted
    }
}

pub enum Flag {
    ONGROUND,
    DUCKING,
    WATERJUMP,
    ONTRAIN,
    INRAIN,
    FROZEN,
    ATCONTROLS,
    CLIENT,
    FAKECLIENT,
    INWATER,
    FLY,
    SWIM,
    CONVEYOR,
    NPC,
    GODMODE,
    NOTARGET,
    AIMTARGET,
    PARTIALGROUND,
    STATICPROP,
    GRAPHED,
    GRENADE,
    STEPMOVEMENT,
    DONTTOUCH,
    BASEVELOCITY,
    WORLDBRUSH,
    OBJECT,
    KILLME,
    ONFIRE,
    DISSOLVING,
    TRANSRAGDOLL,
    UnblockableByPlayer,
}
