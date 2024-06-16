use std::borrow::BorrowMut;

use crate::draw::component::{Component, ComponentBase, Components, DrawOrder};

#[derive(Debug, Clone, Copy)]
pub enum LinearLayoutOrientation {
    HORIZONTAL,
    VERTICAL,
}

#[derive(Debug)]
pub struct LinearLayout {
    base: ComponentBase,
    components: Components,
    orientation: LinearLayoutOrientation,
    inner_pad: isize,
    outer_pad: isize,
    pub center: bool,
}

impl LinearLayout {
    pub fn new(
        orientation: LinearLayoutOrientation,
        inner_pad: isize,
        outer_pad: isize,
    ) -> LinearLayout {
        LinearLayout {
            base: ComponentBase {
                x: 0,
                y: 0,
                w: 0,
                h: 0,
            },
            components: Components::new(),
            orientation,
            inner_pad,
            outer_pad,
            center: false,
        }
    }
    pub fn clear(&mut self) {
        self.components.0.clear();
        let old_base = self.get_base();
        self.base = ComponentBase {
            x: old_base.x,
            y: old_base.y,
            w: 0,
            h: 0,
        };
    }
    pub fn add(&mut self, mut component: impl Component + 'static) {
        let component_base = component.get_base();

        match self.orientation {
            LinearLayoutOrientation::HORIZONTAL => {
                component_base.x = if let Some(last) = self.components.0.last_mut() {
                    last.get_base().x + last.get_base().w + self.inner_pad
                } else {
                    self.outer_pad
                };

                component_base.y += self.outer_pad;
            }
            LinearLayoutOrientation::VERTICAL => {
                component_base.y = if let Some(last) = self.components.0.last_mut() {
                    last.get_base().y + last.get_base().h + self.inner_pad
                } else {
                    self.outer_pad
                };

                component_base.x = self.outer_pad
            }
        }

        self.get_base().w = self
            .get_base()
            .w
            .max(component_base.x + component_base.w + self.outer_pad);
        self.get_base().h = self
            .get_base()
            .h
            .max(component_base.y + component_base.h + self.outer_pad);

        component_base.x += self.base.x;
        component_base.y += self.base.y;

        self.components.add(component);
    }
    pub fn compensate_components(&mut self) {
        let base = self.base.clone();
        for compnent in self.components.0.iter_mut() {
            compnent.get_base().x += base.x;
            compnent.get_base().y += base.y;
        }
    }
    pub fn uncompensate_component(&mut self) {
        let base = self.base.clone();
        for compnent in self.components.0.iter_mut() {
            compnent.get_base().x -= base.x;
            compnent.get_base().y -= base.y;
        }
    }
}

impl Component for LinearLayout {
    fn get_base(&mut self) -> &mut ComponentBase {
        self.base.borrow_mut()
    }
    fn draw(&mut self, frame: &mut crate::draw::frame::Frame) -> crate::error::OxideResult<()> {
        if self.center {
            for component in self.components.0.iter_mut() {
                component.get_base().x = (self.base.w - component.get_base().w) / 2
            }
        }
        self.compensate_components();
        self.components.draw(frame)?;
        self.uncompensate_component();
        Ok(())
    }
    fn handle_event(&mut self, event: &mut crate::draw::event::Event) {
        self.compensate_components();
        self.components.handle_event(event);
        self.uncompensate_component();
    }
    fn get_draw_order(&self) -> DrawOrder {
        self.components.0.iter().fold(
            DrawOrder::Value(0),
            |acc, x| if matches!(x.get_draw_order(), DrawOrder::Top) { x.get_draw_order() } else { acc },
        )
    }
}
