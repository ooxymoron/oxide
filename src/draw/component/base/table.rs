use std::{borrow::BorrowMut, usize};

use crate::draw::{
    colors::FOREGROUND2,
    component::{Component, ComponentBase},
};

#[derive(Debug)]
pub struct Table<const T: usize> {
    base: ComponentBase,
    data: Vec<[Box<dyn Component>; T]>,
    col_widths: [isize; T],
    row_heights: Vec<isize>,
}

impl<const T: usize> Table<T> {
    pub fn new(mut data: Vec<[Box<dyn Component>; T]>) -> Table<T> {
        let mut col_widths = [0; T];
        for row in &mut data {
            for i in 0..T {
                let col_w = row[i].get_base().w;
                col_widths[i] = col_widths[i].max(col_w);
            }
        }
        let w = col_widths.iter().fold(0, |acc, x| acc + x);

        let mut h = 0;
        let mut row_heights = Vec::new();
        for row in &mut data {
            let mut row_max_h = 0;
            for col_i in 0..T {
                let col_h = row[col_i].get_base().h;
                row_max_h = row_max_h.max(col_h);
            }
            row_heights.push(row_max_h);
            h += row_max_h;
        }
        Table::<T> {
            base: ComponentBase { x: 0, y: 0, w, h },
            data,
            col_widths,
            row_heights,
        }
    }
}

impl<const T: usize> Component for Table<T> {
    fn get_base(&mut self) -> &mut ComponentBase {
        self.base.borrow_mut()
    }
    fn draw(&mut self, frame: &mut crate::draw::frame::Frame) -> crate::error::OxideResult<()> {
        let ComponentBase { mut y, w, h,.. } = self.base;

        for (row_i, row) in self.data.iter_mut().enumerate() {
            let mut x = self.base.x;
            for (col_i, col) in row.iter_mut().enumerate() {
                col.get_base().x = x;
                col.get_base().y = y;
                col.draw(frame)?;
                x += self.col_widths[col_i];
            }
            y += self.row_heights[row_i];
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

        Ok(())
    }
}
