use std::mem::transmute;

use crate::{
    call_original, cfn, get_cheat, o,
    oxide::cheat::spread_reduction::SpreadReduction,
    sdk::user_cmd::{ButtonFlags, UserCmd},
    spread_prediction_log,
};

pub const NAME: &str = "AddToTail";

pub type AddToTail = cfn!(i32,*const u8, i32,&mut UserCmd);

pub extern "C" fn hook(ctx: *const u8,  something: i32, cmd : &mut UserCmd) -> i32 {
    if cmd.buttons.get(ButtonFlags::InAttack) {
        let time = (o!().util.plat_float_time)() as f32 + 2f32.powi(13);
        cmd.seed = unsafe { transmute::<_, i32>(time * 1000f32) } & 0xFF;
        let error = get_cheat!(SpreadReduction).last_predicted_time - time;
        spread_prediction_log!(
            "server seed:    {}\tnum: {}\ttime: {}\terror: {}",
            cmd.seed,
            cmd.command_number,
            time,
            error
        );
    }
    call_original!(NAME, AddToTail, ctx, something,cmd)
}
