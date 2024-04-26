use std::isize;

use sdl2_sys::*;

use crate::{
    draw::{
        colors::*,
        event::{Event, EventType},
        fonts::FontSize,
        frame::Frame,
    },
    error::OxideResult,
    interface,
    util::arcm::Arcm,
    vmt_call, NAME, VERSION,
};

use super::{
    aimbot_window::AimbotWindow, base::button::Button, movement_window::MovementWindow,
    visuals_window::VisualsWindow, Component, Components,
};

const LEFT_OVERLAY_WIDTH: isize = 300;
const TOP_OVERLAY_HEIGHT: isize = 50;
const PADDING: isize = 10;
const BUTTON_HEIGHT: isize = 50;

#[derive(Debug)]
pub struct Overlay {
    pub visible: bool,
    pub components: Components,
    pub windows: Components,
}

impl Overlay {
    pub fn new() -> Overlay {
        let mut components = Components::new();

        let show_aimbot_window = Arcm::new(false);
        let show_visuals_window = Arcm::new(false);
        let show_movement_window = Arcm::new(false);

        components.add(Button::new(
            "AIMBOT",
            PADDING,
            TOP_OVERLAY_HEIGHT + PADDING * 2,
            LEFT_OVERLAY_WIDTH - PADDING * 2,
            BUTTON_HEIGHT,
            show_aimbot_window.clone(),
            FontSize::Medium,
        ));

        components.add(Button::new(
            "VISUALS",
            PADDING,
            TOP_OVERLAY_HEIGHT + PADDING * 3 + BUTTON_HEIGHT,
            LEFT_OVERLAY_WIDTH - PADDING * 2,
            BUTTON_HEIGHT,
            show_visuals_window.clone(),
            FontSize::Medium,
        ));

        components.add(Button::new(
            "MOVEMENT",
            PADDING,
            TOP_OVERLAY_HEIGHT + PADDING * 4 + BUTTON_HEIGHT * 2,
            LEFT_OVERLAY_WIDTH - PADDING * 2,
            BUTTON_HEIGHT,
            show_movement_window.clone(),
            FontSize::Medium,
        ));

        let mut windows = Components::new();
        windows.add(AimbotWindow::new(show_aimbot_window.clone()));
        windows.add(VisualsWindow::new(show_visuals_window.clone()));
        windows.add(MovementWindow::new(show_movement_window.clone()));

        Overlay {
            visible: true,
            components,
            windows,
        }
    }
}

impl Overlay {
    fn draw_watermark(&mut self, frame: &mut Frame) {
        let text_size = frame
            .fonts
            .get_text_size(&NAME.to_uppercase(), FontSize::Small);

        let pad = 5;
        let x = 30;
        let y = 30;
        let w = text_size.0 + 2 * pad;
        let h = (text_size.1 + text_size.2) + 2 * pad;

        frame.filled_rect(x, y, w+h, h, BACKGROUND, 200);
        frame.filled_rect(x, y, w+h, 1, FOREGROUND, 200);
        frame.logo(x, y + 1, h - 1, h - 1);
        frame.text(
            &NAME.to_uppercase(),
            x + w / 2 + h,
            y + h / 2,
            FontSize::Small,
            true,
            FOREGROUND,
            230,
        );
    }
}

impl Component for Overlay {
    fn draw(&mut self, frame: &mut Frame, _: isize, _: isize) -> OxideResult<()> {
        let size = frame.window_size();
        if self.visible != vmt_call!(interface!(surface), id_cursor_visible) {
            vmt_call!(interface!(surface), set_cursor_always_visible, self.visible);
        }

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
            220,
        );
        frame.filled_rect(0, 0, LEFT_OVERLAY_WIDTH, size.1, BACKGROUND, 255);

        frame.outlined_rect(-1, -1, LEFT_OVERLAY_WIDTH, TOP_OVERLAY_HEIGHT, CURSOR, 255);
        frame.logo(0, 0, TOP_OVERLAY_HEIGHT - 2, TOP_OVERLAY_HEIGHT - 2);

        let version = format!("V{}", VERSION);
        let text_size = frame.fonts.get_text_size(&version, FontSize::Small);
        frame.text(
            &version,
            size.0 - text_size.0 - PADDING,
            TOP_OVERLAY_HEIGHT / 2,
            FontSize::Small,
            false,
            FOREGROUND,
            255,
        );
        frame.text(
            &NAME.to_uppercase(),
            LEFT_OVERLAY_WIDTH / 2,
            TOP_OVERLAY_HEIGHT / 2,
            FontSize::Large,
            true,
            FOREGROUND,
            255,
        );

        self.components.draw(frame, 0, 0)?;
        self.windows
            .draw(frame, LEFT_OVERLAY_WIDTH, TOP_OVERLAY_HEIGHT)?;
        Ok(())
    }

    fn handle_event(&mut self, event: &mut Event) {
        if matches!(
            event.r#type,
            EventType::KeyDown(SDL_Scancode::SDL_SCANCODE_INSERT)
        ) {
            self.visible = !self.visible;
            event.handled = true;

            if self.visible {
                vmt_call!(interface!(surface), unlock_cursor);
            } else {
                vmt_call!(interface!(surface), lock_cursor);
                // vmt_call!(interface!(input), activate_mouse);
                // vmt_call!(interface!(input), deactivate_mouse);
            }
        }
        if !self.visible {
            return;
        }
        self.windows.handle_event(event);
        self.components.handle_event(event);
    }
}
