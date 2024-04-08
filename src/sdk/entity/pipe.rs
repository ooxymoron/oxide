use derivative::Derivative;

#[derive(Debug,Clone, Copy)]
pub enum PipeType{
	REGULAR = 0,
	RemoteDetonate,
	RemoteDetonatePractice,
	CANNONBALL,
}
impl PipeType {
    pub fn to_str(&self) -> &str{
        match self {
            PipeType::REGULAR => "pipe",
            PipeType::RemoteDetonate => "sticky",
            PipeType::RemoteDetonatePractice => "practice sticky",
            PipeType::CANNONBALL => "cannonball",
        }
    }
}

#[repr(C)]
#[derive(Derivative, Clone, Copy)]
#[derivative(Debug)]
pub struct Pipe {
    #[derivative(Debug = "ignore")]
    _pad6: [u8; 0x8DC],
    pub r#type: PipeType,
}

