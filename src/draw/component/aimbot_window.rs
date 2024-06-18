use crate::{
    draw::{
        component::base::linear_layout::{LinearLayout, LinearLayoutOrientation},
        event::Event,
        frame::Frame,
    },
    error::OxideResult,
    s,
    sdk::entity::hitbox::PlayerHitboxId,
    util::arcm::Arcm,
};

use super::{
    base::{
        checkbox::Checkbox, float_input::FloatInput, int_input::IntInput, key_input::KeyInput,
        select::Select, window::Window,
    },
    Component, ComponentBase,
};

#[derive(Debug)]
pub struct AimbotWindow {
    window: Window,
}

impl AimbotWindow {
    pub fn new(visible: Arcm<bool>) -> AimbotWindow {
        let mut window = Window::new("AIMBOT".to_owned(), Some(visible));

        let mut container = LinearLayout::new(LinearLayoutOrientation::VERTICAL, 16, 10);

        let mut main_settings = LinearLayout::new(LinearLayoutOrientation::VERTICAL, 8, 0);
        main_settings.add(Checkbox::new("enable", s!().aimbot.enabled.clone(), 0, 0));
        main_settings.add(Checkbox::new(
            "draw fov",
            s!().aimbot.draw_fov.clone(),
            0,
            0,
        ));
        main_settings.add(FloatInput::new(
            0,
            0,
            Some("aimbot fov".to_string()),
            s!().aimbot.fov.clone(),
            None,
        ));
        main_settings.add(Select::new(
            PlayerHitboxId::all(),
            Arcm::new((true, s!().aimbot.hitboxes.clone())),
            true,
            Some("hitboxes".to_string()),
        ));
        main_settings.add(FloatInput::new(
            0,
            0,
            Some("target persistance duration".to_string()),
            s!().aimbot.target_persistance_duration.clone(),
            None,
        ));
        main_settings.add(Checkbox::new(
            "always_on",
            s!().aimbot.always_on.clone(),
            0,
            0,
        ));
        main_settings.add(KeyInput::new(0, 0, "aimbot key", s!().aimbot.key.clone()));
        container.add(main_settings);

        let mut multipoint_settings = LinearLayout::new(LinearLayoutOrientation::VERTICAL, 8, 0);
        multipoint_settings.add(Checkbox::new(
            "multipoint",
            s!().aimbot.multipoint.clone(),
            0,
            0,
        ));
        multipoint_settings.add(FloatInput::new(
            0,
            0,
            Some("hitbox scale".to_string()),
            s!().aimbot.hitbox_scale.clone(),
            Some(|val| val <= 1.0 && val >= 0.0),
        ));
        container.add(multipoint_settings);

        let mut fire_settings = LinearLayout::new(LinearLayoutOrientation::VERTICAL, 8, 0);
        fire_settings.add(Checkbox::new(
            "autoshoot",
            s!().aimbot.autoshoot.clone(),
            0,
            0,
        ));
        fire_settings.add(Checkbox::new("silent", s!().aimbot.silent.clone(), 0, 0));
        fire_settings.add(Checkbox::new(
            "fire only when able",
            s!().aimbot.fire_only_when_able.clone(),
            0,
            0,
        ));
        container.add(fire_settings);

        let mut target_settings = LinearLayout::new(LinearLayoutOrientation::VERTICAL, 8, 0);
        target_settings.add(Checkbox::new(
            "target sentries",
            s!().aimbot.target_sentries.clone(),
            0,
            0,
        ));
        target_settings.add(Checkbox::new(
            "target invisible",
            s!().aimbot.target_invisible.clone(),
            0,
            0,
        ));
        target_settings.add(Checkbox::new(
            "target disguised",
            s!().aimbot.target_disguised.clone(),
            0,
            0,
        ));
        target_settings.add(Checkbox::new(
            "target stickies",
            s!().aimbot.target_stickies.clone(),
            0,
            0,
        ));
        container.add(target_settings);

        let mut misc_settings = LinearLayout::new(LinearLayoutOrientation::VERTICAL, 8, 0);
        misc_settings.add(Checkbox::new(
            "wait for charge",
            s!().aimbot.wait_for_charge.clone(),
            0,
            0,
        ));
        misc_settings.add(Checkbox::new(
            "baim if lethal",
            s!().aimbot.baim_if_lethal.clone(),
            0,
            0,
        ));
        misc_settings.add(Checkbox::new(
            "auto zoom",
            s!().aimbot.auto_scope.clone(),
            0,
            0,
        ));
        misc_settings.add(Checkbox::new(
            "auto unzoom",
            s!().aimbot.auto_unscope.clone(),
            0,
            0,
        ));
        misc_settings.add(Checkbox::new(
            "auto rev",
            s!().aimbot.auto_rev.clone(),
            0,
            0,
        ));
        container.add(misc_settings);

        window.add(container);

        AimbotWindow { window }
    }
}

impl Component for AimbotWindow {
    fn draw(&mut self, frame: &mut Frame) -> OxideResult<()> {
        self.window.draw(frame)
    }
    fn handle_event(&mut self, event: &mut Event) {
        self.window.handle_event(event);
    }
    fn get_draw_order(&self) -> super::DrawOrder {
        self.window.get_draw_order()
    }
    fn set_draw_order(&mut self, order: super::DrawOrder) {
        self.window.set_draw_order(order)
    }
    fn get_base(&mut self) -> &mut ComponentBase {
        self.window.get_base()
    }
}
