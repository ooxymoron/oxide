use std::{borrow::BorrowMut, collections::HashMap, usize};

use crate::draw::{
    colors::FOREGROUND2,
    component::{Component, ComponentBase},
};

#[derive(Debug)]
pub struct Table<const T: usize> {
    base: ComponentBase,
    pub data: HashMap<i32, [Box<dyn Component>; T]>,
    headers: [Box<dyn Component>; T],
    col_widths: [isize; T],
    row_heights: Vec<isize>,
}

impl<const T: usize> Table<T> {
    pub fn new(
        headers: [Box<dyn Component>; T],
        data: HashMap<i32, [Box<dyn Component>; T]>,
    ) -> Table<T> {
        let mut table = Table::<T> {
            base: ComponentBase {
                x: 0,
                y: 0,
                w: 0,
                h: 0,
            },
            data,
            headers,
            col_widths: [0; T],
            row_heights: Vec::new(),
        };
        table.update_data();
        table
    }
    pub fn update_data(&mut self) {
        let mut col_widths = [0; T];
        for row in self.data.values_mut() {
            for i in 0..T {
                let col_w = row[i].get_base().w;
                col_widths[i] = col_widths[i].max(col_w);
            }
        }
        for i in 0..T {
            col_widths[i] = col_widths[i].max(self.headers[i].get_base().w);
        }
        let mut row_heights = Vec::new();
        let mut max_height = 0;
        for i in 0..T {
            max_height = max_height.max(self.headers[i].get_base().h);
        }
        row_heights.push(max_height);
        for row in self.data.values_mut() {
            let mut row_max_h = 0;
            for col_i in 0..T {
                let col_h = row[col_i].get_base().h;
                row_max_h = row_max_h.max(col_h);
            }
            row_heights.push(row_max_h);
        }

        let mut y = 0;
        let mut x = 0;
        for (col_i, col) in self.headers.iter_mut().enumerate() {
            col.get_base().x = x;
            col.get_base().y = y;
            col.get_base().h = row_heights[0];
            col.get_base().w = col_widths[col_i];
            x += col_widths[col_i];
        }
        for (row_i, (_, row)) in self.data.iter_mut().enumerate() {
            y += row_heights[row_i + 1];
            let mut x = 0;
            for (col_i, col) in row.iter_mut().enumerate() {
                col.get_base().x = x;
                col.get_base().y = y;
                col.get_base().h = row_heights[row_i + 1];
                col.get_base().w = col_widths[col_i];
                x += col_widths[col_i];
            }
        }
        self.base.w = col_widths.iter().fold(0, |acc, x| acc + x);
        self.base.h = row_heights.iter().fold(0, |acc, x| acc + x);
        self.col_widths = col_widths;
        self.row_heights = row_heights;
    }

    pub fn componesate_components(&mut self) {
        let ComponentBase { x, y, .. } = self.base;

        for col in &mut self.headers {
            col.get_base().x += x;
            col.get_base().y += y;
        }
        for row in self.data.values_mut() {
            for col in row {
                col.get_base().x += x;
                col.get_base().y += y;
            }
        }
    }
    pub fn uncomponesate_components(&mut self) {
        let ComponentBase { x, y, .. } = self.base;

        for col in &mut self.headers {
            col.get_base().x -= x;
            col.get_base().y -= y;
        }
        for row in self.data.values_mut() {
            for col in row {
                col.get_base().x -= x;
                col.get_base().y -= y;
            }
        }
    }
}

impl<const T: usize> Component for Table<T> {
    fn get_base(&mut self) -> &mut ComponentBase {
        self.base.borrow_mut()
    }
    fn draw(&mut self, frame: &mut crate::draw::frame::Frame) -> crate::error::OxideResult<()> {
        let ComponentBase { w, h, .. } = self.base;

        self.componesate_components();
        for col in &mut self.headers {
            col.draw(frame)?;
        }
        let mut draw_top = Vec::new();
        for row in self.data.values_mut() {
            for col in row {
                if matches!(col.get_draw_order(), crate::draw::component::DrawOrder::Top) {
                    draw_top.push(col);
                    continue;
                }
                col.draw(frame)?;
            }
        }

        let mut y = self.base.y;
        let x = self.base.x;
        for row in &self.row_heights {
            frame.line(x, y, x + w - 1, y, FOREGROUND2, 255);
            y += row;
        }

        let y = self.base.y;
        let mut x = self.base.x;
        for col in &self.col_widths {
            frame.line(x, y, x, y + h - 1, FOREGROUND2, 255);
            x += col;
        }
        for component in draw_top {
            component.draw(frame)?;
        }
        self.uncomponesate_components();

        Ok(())
    }
    fn handle_event(&mut self, event: &mut crate::draw::event::Event) {
        self.componesate_components();
        let mut handle_first = Vec::new();
        for row in self.data.values_mut() {
            for col in row {
                if matches!(col.get_draw_order(), crate::draw::component::DrawOrder::Top) {
                    handle_first.push(col);
                    continue;
                }
            }
        }
        for component in handle_first {
            component.handle_event(event);
        }
        for row in self.data.values_mut() {
            for col in row {
                if matches!(col.get_draw_order(), crate::draw::component::DrawOrder::Top) {
                    continue;
                }
                col.handle_event(event);
            }
        }

        self.uncomponesate_components();
    }
}
