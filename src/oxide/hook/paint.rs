use crate::{
    define_hook, interface,
    oxide::hook::send_user_msg::SendUserMessageHook,
    sdk::{interfaces::engine_vgui::EngineVgui, net_channel, HasVmt},
    util::{
        debug::print_module_addres_offset,
        handles::{CLIENT, ENGINE},
    },
    vmt_call,
};

unsafe fn hook(engine_vgui: &EngineVgui, mode: isize, org: PaintHook::RawFn) {
    (org)(engine_vgui,mode);
    let net_channel = interface!(base_engine).get_net_channel();
    if let Some(net_channel) = net_channel {
        if o!()
            .hooks
            .ptr_hooks
            .get(&SendUserMessageHook::name())
            .is_none()
        {
            o!().hooks.ptr_hooks.insert(
                SendUserMessageHook::name(),
                Box::new(SendUserMessageHook::init(
                    &(net_channel.get_vmt().send_net_msg),
                )) as Box<dyn Hook>,
            );
        }
    }

    o!().paint.paint().unwrap();
}
define_hook!(
    PaintHook,
    "Paint",
    hook,
    (),
    (),
    engine_vgui,
    &EngineVgui,
    mode,
    isize
);
