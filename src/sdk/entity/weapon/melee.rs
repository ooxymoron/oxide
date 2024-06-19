use derivative::Derivative;


#[repr(C)]
#[derive(Derivative, Clone)]
#[derivative(Debug)]
pub struct MeleeWeapon {
    #[derivative(Debug = "ignore")]
    _pad5: [u8; 0xc2c],
    pub smack_time: f32, /* 0xC2C */
    #[derivative(Debug = "ignore")]
    _pad6: [u8; 0x10],
    pub ready_to_backstab: bool, /*0xC40*/
}
