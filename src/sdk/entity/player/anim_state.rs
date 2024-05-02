use derivative::Derivative;


#[repr(C)]
#[derive(Derivative, Clone, Copy)]
#[derivative(Debug)]
pub struct VMTAnimState {}

#[repr(C)]
#[derive(Derivative, Clone, Copy)]
#[derivative(Debug)]
pub struct AnimState {
    pub vmt: *mut VMTAnimState,
}
