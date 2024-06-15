use std::{borrow::BorrowMut, isize};

use sdl2_sys::*;

use crate::{
    d,
    draw::{
        colors::*,
        component::{crit_manipulation_window::CritManipulationWindow, player_list_window::PlayerListWindow, spectator_list::SpectatorListWindow, spread_reduction_window::SpreadReductionWindow},
        event::{Event, EventType},
        fonts::FontSize,
        frame::Frame,
    },
    error::OxideResult,
    interface, s,
    util::arcm::Arcm,
    vmt_call, NAME, VERSION,
};

use super::{
    aimbot_window::AimbotWindow, base::button::Button,
    crit_manipulation_info::CritManipulationInfoWindow, movement_window::MovementWindow,
    spread_reduction_info::SpreadReductionInfoWindow, visuals_window::VisualsWindow, Component,
    ComponentBase, Components,
};

const PADDING: isize = 10;
const BUTTON_HEIGHT: isize = FontSize::Small as isize + 2 * PADDING;
const TOP_OVERLAY_HEIGHT: isize = FontSize::Medium as isize + 2 * PADDING;
const LEFT_OVERLAY_WIDTH: isize = 200;

#[derive(Debug)]
pub struct Overlay {
    base: ComponentBase,
    pub visible: bool,
    pub components: Components,
    pub windows: Components,
    pub spectator_list: SpectatorListWindow,
    pub spread_reduction_info: SpreadReductionInfoWindow,
    pub crit_manipulation_info: CritManipulationInfoWindow,
}

impl Overlay {
    pub fn new() -> Overlay {
        let mut components = Components::new();
        let mut windows = Components::new();

        macro_rules! add_window {
            ($name: expr, $window: ident) => {{
                let show = Arcm::new(false);
                components.add(Button::new(
                    ComponentBase {
                        x: PADDING,
                        y: TOP_OVERLAY_HEIGHT
                            + PADDING * 2
                            + (PADDING + BUTTON_HEIGHT) * components.0.len() as isize,
                        w: LEFT_OVERLAY_WIDTH - PADDING * 2,
                        h: BUTTON_HEIGHT,
                    },
                    $name,
                    show.clone(),
                    FontSize::Medium,
                ));

                windows.add($window::new(show.clone()));
            }};
        }

        add_window!("AIMBOT", AimbotWindow);
        add_window!("SPREAD REDUCTION", SpreadReductionWindow);
        add_window!("CRIT MANIPULATION", CritManipulationWindow);
        add_window!("VISUALS", VisualsWindow);
        add_window!("MOVEMENT", MovementWindow);
        add_window!("PLAYER LIST", PlayerListWindow);

        let size = d!().window_size;
        Overlay {
            visible: false,
            components,
            windows,
            spectator_list: SpectatorListWindow::new(),
            spread_reduction_info: SpreadReductionInfoWindow::new(),
            crit_manipulation_info: CritManipulationInfoWindow::new(),
            base: ComponentBase {
                x: 0,
                y: 0,
                w: size.0,
                h: size.1,
            },
        }
    }
}

impl Default for Overlay {
    fn default() -> Self {
        Self::new()
    }
}

impl Overlay {
    fn draw_watermark(&mut self, frame: &mut Frame) {
        let text_size = frame
            .fonts
            .get_text_size(&NAME.to_uppercase(), FontSize::Medium);

        let pad = 5;
        let x = 30;
        let y = 30;
        let w = text_size.0 + 2 * pad;
        let h = (text_size.1 + text_size.2) + 2 * pad;

        frame.filled_rect(x, y, w + h, h, BACKGROUND, 200);
        frame.filled_rect(x, y, w + h, 1, FOREGROUND, 200);
        frame.logo(x, y + 1, h - 1, h - 1);
        frame.text(
            &NAME.to_uppercase(),
            x + w / 2 + h,
            y + h / 2,
            FontSize::Medium,
            true,
            true,
            FOREGROUND,
            230,
        );
    }
    fn update_cursor(&self) {
        if self.visible {
            vmt_call!(interface!(surface), unlock_cursor);
        } else {
            vmt_call!(interface!(surface), lock_cursor);
            // vmt_call!(interface!(input), activate_mouse);
            // vmt_call!(interface!(input), deactivate_mouse);
        }
        vmt_call!(interface!(surface), set_cursor_always_visible, self.visible);
        vmt_call!(interface!(surface), apply_changes);
    }
}

impl Component for Overlay {
    fn draw(&mut self, frame: &mut Frame) -> OxideResult<()> {
        let size = d!().window_size;

        self.spectator_list.draw_wrapper(frame, self.visible)?;
        self.spread_reduction_info
            .draw_wrapper(frame, self.visible)?;
        self.crit_manipulation_info
            .draw_wrapper(frame, self.visible)?;
        if !self.visible {
            self.draw_watermark(frame);
            return Ok(());
        }

        frame.filled_rect(
            LEFT_OVERLAY_WIDTH,
            0,
            size.0,
            TOP_OVERLAY_HEIGHT,
            BACKGROUND,
            200,
        );
        frame.filled_rect(0, 0, LEFT_OVERLAY_WIDTH, size.1, BACKGROUND, 200);

        frame.filled_rect(
            -1,
            -1,
            LEFT_OVERLAY_WIDTH,
            TOP_OVERLAY_HEIGHT,
            BACKGROUND,
            255,
        );
        frame.outlined_rect(
            -1,
            -1,
            LEFT_OVERLAY_WIDTH,
            TOP_OVERLAY_HEIGHT,
            FOREGROUND3,
            255,
        );
        frame.logo(0, 0, TOP_OVERLAY_HEIGHT - 2, TOP_OVERLAY_HEIGHT - 2);

        let version = format!("V{}", VERSION);
        let text_size = frame.fonts.get_text_size(&version, FontSize::Medium);
        frame.text(
            &version,
            size.0 - text_size.0 - PADDING,
            TOP_OVERLAY_HEIGHT / 2,
            FontSize::Medium,
            false,
            true,
            FOREGROUND,
            255,
        );
        frame.text(
            &NAME.to_uppercase(),
            LEFT_OVERLAY_WIDTH / 2,
            TOP_OVERLAY_HEIGHT / 2,
            FontSize::Large,
            true,
            true,
            FOREGROUND,
            255,
        );

        self.components.draw(frame)?;
        self.windows.draw(frame)?;
        Ok(())
    }

    fn handle_event(&mut self, event: &mut Event) {
        if matches!(
            event.r#type,
            EventType::KeyDown(SDL_Scancode::SDL_SCANCODE_INSERT)
        ) {
            self.visible = !self.visible;
            event.handled = true;

            if !self.visible {
                s!().save().unwrap();
            }
            self.update_cursor();
        }
        if !self.visible {
            return;
        }
        self.windows.handle_event(event);
        self.components.handle_event(event);
        self.spectator_list.handle_event(event);
        self.spread_reduction_info.handle_event(event);
        self.crit_manipulation_info.handle_event(event);
    }

    fn get_base(&mut self) -> &mut ComponentBase {
        self.base.borrow_mut()
    }
}
