#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PlayerClass {
    Undefined = 0,
    Scout,
    Sniper,
    Soldier,
    Demoman,
    Medic,
    Hwguy,
    Pyro,
    Spy,
    Engineer,
    Civilian,
    Random,
    Observer,
}
