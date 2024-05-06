use crate::define_hook;

pub const NAME: &str = "SendPerfServer";

pub unsafe fn hook(
    this: *const u8,
    recipient: *const u8,
    max_records: u32,
    org: SendPerfServerHook::RawFn,
) {
    (org)(this, recipient, max_records);
}

define_hook!(
    SendPerfServerHook,
    "SendPerfServer",
    hook,
    (),
    (),
    this,
    *const u8,
    recepient,
    *const u8,
    max_recors,
    u32
);
