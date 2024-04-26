use std::{
    fmt::{Debug, Display},
    mem::transmute,
};

#[derive(Debug, Clone, Copy)]
pub struct Flags(u32);
impl Flags {
    pub fn get(&self, flag: Flag) -> bool {
        let flag = flag as u8;
        let shifted = 1 << flag;
        let Flags(b) = *self;
        b & shifted == shifted
    }
}
impl Display for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let flags = (1..Flag::TRANSRAGDOLL as i8)
            .into_iter()
            .filter_map(|flag| {
                let flag: Flag = unsafe { transmute(flag) };
                if self.get(flag) {
                    return Some(format!("{:?}", flag));
                }
                None
            })
            .collect::<Vec<_>>();
        write!(f, "{}", flags.join(" | "))
    }
}

#[derive(Debug, Clone, Copy)]
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
