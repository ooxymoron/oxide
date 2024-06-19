use crate::{
    draw::{
        component::base::{
            key_input::KeyInput,
            linear_layout::{LinearLayout, LinearLayoutOrientation},
        },
        event::Event,
        frame::Frame,
    },
    error::OxideResult,
    s,
    util::arcm::Arcm,
};

use super::{
    base::{checkbox::Checkbox, float_input::FloatInput, window::Window},
    Component,
};

#[derive(Debug)]
pub struct VisualsWindow {
    window: Window,
}

impl VisualsWindow {
    pub fn new(visible: Arcm<bool>) -> VisualsWindow {
        let mut window = Window::new("VISUALS".to_owned(), Some(visible));
        let mut container = LinearLayout::new(LinearLayoutOrientation::VERTICAL, 16, 10);

        let mut third_person_settings = LinearLayout::new(LinearLayoutOrientation::VERTICAL, 8, 0);
        third_person_settings.add(Checkbox::new(
            "third person",
            s!().visual.third_person.clone(),
            0,
            0,
        ));
        third_person_settings.add(KeyInput::new(
            0,
            0,
            "toggle key",
            s!().visual.tp_key.clone(),
        ));
        third_person_settings.add(KeyInput::new(
            0,
            0,
            "offset key",
            s!().visual.tp_offset_key.clone(),
        ));
        third_person_settings.add(FloatInput::new(
            0,
            0,
            Some("x offset".to_string()),
            s!().visual.tp_offset_x.clone(),
            None,
        ));
        third_person_settings.add(FloatInput::new(
            0,
            0,
            Some("y offset".to_string()),
            s!().visual.tp_offset_y.clone(),
            None,
        ));
        third_person_settings.add(FloatInput::new(
            0,
            0,
            Some("z offset".to_string()),
            s!().visual.tp_offset_z.clone(),
            None,
        ));
        container.add(third_person_settings);

        let mut fov_settings = LinearLayout::new(LinearLayoutOrientation::VERTICAL, 8, 0);
        fov_settings.add(FloatInput::new(0, 0, Some("fov".to_string()), s!().visual.fov.clone(), None));
        container.add(fov_settings);

        let mut esp_settings = LinearLayout::new(LinearLayoutOrientation::VERTICAL, 8, 0);
        esp_settings.add(Checkbox::new("esp", s!().visual.esp.clone(), 0, 0));
        esp_settings.add(Checkbox::new(
            "friendlies",
            s!().visual.esp_friendlies.clone(),
            0,
            0,
        ));
        esp_settings.add(Checkbox::new(
            "sentries",
            s!().visual.esp_sentreis.clone(),
            0,
            0,
        ));
        esp_settings.add(Checkbox::new(
            "buildings",
            s!().visual.esp_buildings.clone(),
            0,
            0,
        ));
        esp_settings.add(Checkbox::new(
            "projectiles",
            s!().visual.esp_projectiles.clone(),
            0,
            0,
        ));
        container.add(esp_settings);

        let mut hitbox_settings = LinearLayout::new(LinearLayoutOrientation::VERTICAL, 8, 0);
        hitbox_settings.add(Checkbox::new(
            "hitboxes",
            s!().visual.hitboxes.clone(),
            0,
            0,
        ));
        container.add(hitbox_settings);

        let mut remove_settings = LinearLayout::new(LinearLayoutOrientation::VERTICAL, 8, 0);
        remove_settings.add(Checkbox::new(
            "remove scope",
            s!().visual.remove_scope.clone(),
            0,
            0,
        ));
        remove_settings.add(Checkbox::new(
            "remove zoom",
            s!().visual.remove_zoom.clone(),
            0,
            0,
        ));
        remove_settings.add(Checkbox::new(
            "remove disguises",
            s!().visual.remove_disguises.clone(),
            0,
            0,
        ));
        container.add(remove_settings);

        let mut spectator_list = LinearLayout::new(LinearLayoutOrientation::VERTICAL, 8, 0);
        spectator_list.add(Checkbox::new(
            "spectator list",
            s!().visual.spectator_list.clone(),
            0,
            0,
        ));
        container.add(spectator_list);

        let mut pure_bypass = LinearLayout::new(LinearLayoutOrientation::VERTICAL, 8, 0);
        pure_bypass.add(Checkbox::new(
            "pure bypass",
            s!().visual.pure_bypass.clone(),
            0,
            0,
        ));
        container.add(pure_bypass);

        let mut tracer_settings = LinearLayout::new(LinearLayoutOrientation::VERTICAL, 8, 0);
        tracer_settings.add(Checkbox::new("tracers", s!().visual.tracers.clone(), 0, 0));
        tracer_settings.add(Checkbox::new("impacts", s!().visual.impacts.clone(), 0, 0));
        tracer_settings.add(Checkbox::new("explosives", s!().visual.explosives.clone(), 0, 0));
        container.add(tracer_settings);
        window.add(container);
        VisualsWindow { window }
    }
}

impl Component for VisualsWindow {
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
    fn get_base(&mut self) -> &mut super::ComponentBase {
        self.window.get_base()
    }
}
