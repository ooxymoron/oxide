pub type HFont = u32;

pub enum FontFlags {
    NONE         = 0x000,
    ITALIC       = 0x001,
    UNDERLINE    = 0x002,
    STRIKEOUT    = 0x004,
    SYMBOL       = 0x008,
    ANTIALIAS    = 0x010,
    GAUSSIANBLUR = 0x020,
    ROTARY       = 0x040,
    DROPSHADOW   = 0x080,
    ADDITIVE     = 0x100,
    OUTLINE      = 0x200,
    CUSTOM       = 0x400,
    BITMAP       = 0x800,
}
pub enum FontDrawType{
    Default,
    NonAdditiva,
    Additive,
}


#[derive(Debug)]
#[repr(C)]
pub struct Font {
    pub name: *const i8,
    pub tall: i32,
    pub weight: i32,
    pub flags: i32,
    pub id: HFont, 
}
