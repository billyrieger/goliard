/*
 * This Source Code Form is subject to the terms of the Mozilla Public License,
 * v. 2.0. If a copy of the MPL was not distributed with this file, You can
 * obtain one at http://mozilla.org/MPL/2.0/.
 */

use crate::State;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

struct IsRunningView {
    is_running: Arc<AtomicBool>,
}

impl IsRunningView {
    pub fn new(is_running: Arc<AtomicBool>) -> Self {
        Self { is_running }
    }

    fn format(&self) -> String {
        if self.is_running.load(Ordering::SeqCst) {
            "running".to_owned()
        } else {
            "stopped".to_owned()
        }
    }
}

impl cursive::view::View for IsRunningView {
    fn draw(&self, printer: &cursive::Printer) {
        printer.print((0, 0), &self.format());
    }

    fn required_size(&mut self, _: cursive::vec::Vec2) -> cursive::vec::Vec2 {
        // (width, height)
        (self.format().len(), 1).into()
    }
}

struct GenerationView {
    life: Arc<Mutex<smeagol::Life>>,
}

impl GenerationView {
    pub fn new(life: Arc<Mutex<smeagol::Life>>) -> Self {
        Self { life }
    }

    fn format(&self) -> String {
        format!("gen: {}", self.life.lock().unwrap().generation())
    }
}

impl cursive::view::View for GenerationView {
    fn draw(&self, printer: &cursive::Printer) {
        printer.print((0, 0), &self.format());
    }

    fn required_size(&mut self, _: cursive::vec::Vec2) -> cursive::vec::Vec2 {
        // (width, height)
        (self.format().len(), 1).into()
    }
}

struct PopulationView {
    life: Arc<Mutex<smeagol::Life>>,
}

impl PopulationView {
    pub fn new(life: Arc<Mutex<smeagol::Life>>) -> Self {
        Self { life }
    }

    fn format(&self) -> String {
        format!("pop: {}", self.life.lock().unwrap().population())
    }
}

impl cursive::view::View for PopulationView {
    fn draw(&self, printer: &cursive::Printer) {
        printer.print((0, 0), &self.format());
    }

    fn required_size(&mut self, _: cursive::vec::Vec2) -> cursive::vec::Vec2 {
        // (width, height)
        (self.format().len(), 1).into()
    }
}

struct DelayView {
    delay: Arc<Mutex<u64>>,
}

impl DelayView {
    fn new(delay: Arc<Mutex<u64>>) -> Self {
        Self { delay }
    }

    fn format(&self) -> String {
        format!("delay: {} ms", self.delay.lock().unwrap())
    }
}

impl cursive::view::View for DelayView {
    fn draw(&self, printer: &cursive::Printer) {
        printer.print((0, 0), &self.format());
    }

    fn required_size(&mut self, _: cursive::vec::Vec2) -> cursive::vec::Vec2 {
        // (width, height)
        (self.format().len(), 1).into()
    }
}

struct StepView {
    life: Arc<Mutex<smeagol::Life>>,
}

impl StepView {
    fn new(life: Arc<Mutex<smeagol::Life>>) -> Self {
        Self { life }
    }

    fn format(&self) -> String {
        format!("step: {}", self.life.lock().unwrap().step_size())
    }
}

impl cursive::view::View for StepView {
    fn draw(&self, printer: &cursive::Printer) {
        printer.print((0, 0), &self.format());
    }

    fn required_size(&mut self, _: cursive::vec::Vec2) -> cursive::vec::Vec2 {
        // (width, height)
        (self.format().len(), 1).into()
    }
}

struct CenterView {
    center: Arc<Mutex<(i64, i64)>>,
}

impl CenterView {
    fn new(center: Arc<Mutex<(i64, i64)>>) -> Self {
        Self { center }
    }

    fn format(&self) -> String {
        let center = self.center.lock().unwrap();
        format!("center: ({}, {})", center.0, center.1)
    }
}

impl cursive::view::View for CenterView {
    fn draw(&self, printer: &cursive::Printer) {
        printer.print((0, 0), &self.format());
    }

    fn required_size(&mut self, _: cursive::vec::Vec2) -> cursive::vec::Vec2 {
        // (width, height)
        (self.format().len(), 1).into()
    }
}

struct ScaleView {
    scale: Arc<Mutex<u64>>,
}

impl ScaleView {
    fn new(scale: Arc<Mutex<u64>>) -> Self {
        Self { scale }
    }

    fn format(&self) -> String {
        format!("scale: {}:1", self.scale.lock().unwrap())
    }
}

impl cursive::view::View for ScaleView {
    fn draw(&self, printer: &cursive::Printer) {
        printer.print((0, 0), &self.format());
    }

    fn required_size(&mut self, _: cursive::vec::Vec2) -> cursive::vec::Vec2 {
        // (width, height)
        (self.format().len(), 1).into()
    }
}

pub struct LifeView {
    life: Arc<Mutex<smeagol::Life>>,
    center: Arc<Mutex<(i64, i64)>>,
    scale: Arc<Mutex<u64>>,
}

impl LifeView {
    fn new(
        life: Arc<Mutex<smeagol::Life>>,
        center: Arc<Mutex<(i64, i64)>>,
        scale: Arc<Mutex<u64>>,
    ) -> Self {
        Self {
            life,
            center,
            scale,
        }
    }
}

impl cursive::view::View for LifeView {
    #[allow(clippy::many_single_char_names)]
    fn draw(&self, printer: &cursive::Printer) {
        let width = printer.output_size.x as i64;
        let height = printer.output_size.y as i64;
        let front_color = cursive::theme::Color::Rgb(255, 255, 255);
        let back_color = cursive::theme::Color::Rgb(0, 0, 0);
        let life = self.life.lock().unwrap();
        let zoom_factor = *self.scale.lock().unwrap() as i64;
        let zoom_factor_minus_1 = zoom_factor - 1;
        let center = self.center.lock().unwrap();
        for x in 0..width {
            for y in 0..height {
                printer.with_color(
                    cursive::theme::ColorStyle::new(front_color, back_color),
                    |printer| {
                        printer.print((x as u32, y as u32), {
                            let x_offset = 2 * (x - (width / 2)) * zoom_factor + center.0;
                            let y_offset = 4 * (y - (height / 2)) * zoom_factor + center.1;
                            // +---+---+
                            // | a | b |
                            // +---+---+
                            // | c | d |
                            // +---+---+
                            // | e | f |
                            // +---+---+
                            // | g | h |
                            // +---+---+
                            let pos = smeagol::Position::new(x_offset, y_offset);
                            let a = if life.contains_alive_cells(smeagol::BoundingBox::new(
                                pos.offset(0, 0),
                                pos.offset(0, 0)
                                    .offset(zoom_factor_minus_1, zoom_factor_minus_1),
                            )) {
                                1
                            } else {
                                0
                            };
                            let b = if life.contains_alive_cells(smeagol::BoundingBox::new(
                                pos.offset(zoom_factor, 0),
                                pos.offset(zoom_factor, 0)
                                    .offset(zoom_factor_minus_1, zoom_factor_minus_1),
                            )) {
                                1
                            } else {
                                0
                            };
                            let c = if life.contains_alive_cells(smeagol::BoundingBox::new(
                                pos.offset(0, zoom_factor),
                                pos.offset(0, zoom_factor)
                                    .offset(zoom_factor_minus_1, zoom_factor_minus_1),
                            )) {
                                1
                            } else {
                                0
                            };
                            let d = if life.contains_alive_cells(smeagol::BoundingBox::new(
                                pos.offset(zoom_factor, zoom_factor),
                                pos.offset(zoom_factor, zoom_factor)
                                    .offset(zoom_factor_minus_1, zoom_factor_minus_1),
                            )) {
                                1
                            } else {
                                0
                            };
                            let e = if life.contains_alive_cells(smeagol::BoundingBox::new(
                                pos.offset(0, 2 * zoom_factor),
                                pos.offset(0, 2 * zoom_factor)
                                    .offset(zoom_factor_minus_1, zoom_factor_minus_1),
                            )) {
                                1
                            } else {
                                0
                            };
                            let f = if life.contains_alive_cells(smeagol::BoundingBox::new(
                                pos.offset(zoom_factor, 2 * zoom_factor),
                                pos.offset(zoom_factor, 2 * zoom_factor)
                                    .offset(zoom_factor_minus_1, zoom_factor_minus_1),
                            )) {
                                1
                            } else {
                                0
                            };
                            let g = if life.contains_alive_cells(smeagol::BoundingBox::new(
                                pos.offset(0, 3 * zoom_factor),
                                pos.offset(0, 3 * zoom_factor)
                                    .offset(zoom_factor_minus_1, zoom_factor_minus_1),
                            )) {
                                1
                            } else {
                                0
                            };
                            let h = if life.contains_alive_cells(smeagol::BoundingBox::new(
                                pos.offset(zoom_factor, 3 * zoom_factor),
                                pos.offset(zoom_factor, 3 * zoom_factor)
                                    .offset(zoom_factor_minus_1, zoom_factor_minus_1),
                            )) {
                                1
                            } else {
                                0
                            };
                            &braille::BRAILLE[a][b][c][d][e][f][g][h].to_string()
                        })
                    },
                );
            }
        }
    }
}

pub fn add_main_view(siv: &mut cursive::Cursive, state: &State) {
    let padding = ((1, 1), (0, 0));
    let mut stack = cursive::views::StackView::new();
    stack.add_fullscreen_layer(
        cursive::views::LinearLayout::vertical()
            .child(cursive::view::Boxable::full_screen(LifeView::new(
                state.life.clone(),
                state.center.clone(),
                state.scale.clone(),
            )))
            .child(
                cursive::views::LinearLayout::horizontal()
                    .child(cursive::views::PaddedView::new(
                        padding,
                        GenerationView::new(state.life.clone()),
                    ))
                    .child(cursive::views::TextView::new("|"))
                    .child(cursive::views::PaddedView::new(
                        padding,
                        PopulationView::new(state.life.clone()),
                    ))
                    .child(cursive::views::TextView::new("|"))
                    .child(cursive::views::PaddedView::new(
                        padding,
                        StepView::new(state.life.clone()),
                    ))
                    .child(cursive::views::TextView::new("|"))
                    .child(cursive::views::PaddedView::new(
                        padding,
                        DelayView::new(state.delay_millis.clone()),
                    ))
                    .child(cursive::views::TextView::new("|"))
                    .child(cursive::views::PaddedView::new(
                        padding,
                        IsRunningView::new(state.is_running.clone()),
                    ))
                    .child(cursive::view::Boxable::full_width(
                        cursive::views::DummyView,
                    ))
                    .child(cursive::views::PaddedView::new(
                        padding,
                        CenterView::new(state.center.clone()),
                    ))
                    .child(cursive::views::TextView::new("|"))
                    .child(cursive::views::PaddedView::new(
                        padding,
                        ScaleView::new(state.scale.clone()),
                    )),
            ),
    );
    let stack_with_id = cursive::views::IdView::new("stack", stack);
    siv.add_fullscreen_layer(stack_with_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cursive::view::View;

    #[test]
    fn life_view() {
        let theme = cursive::theme::Theme::default();
        let backend = cursive::backend::dummy::Backend::init();
        let printer = cursive::Printer::new((1, 1), &theme, &*backend);

        let empty_life_view = LifeView::new(
            Arc::new(Mutex::new(smeagol::Life::new())),
            Arc::new(Mutex::new((0, 0))),
            Arc::new(Mutex::new(1)),
        );
        empty_life_view.draw(&printer);

        let mut life = smeagol::Life::new();
        for x in 0..2 {
            for y in 0..4 {
                life.set_cell_alive(smeagol::Position::new(x, y));
            }
        }
        let partial_filled_view = LifeView::new(
            Arc::new(Mutex::new(life)),
            Arc::new(Mutex::new((0, 0))),
            Arc::new(Mutex::new(1)),
        );
        partial_filled_view.draw(&printer);
    }

    #[test]
    fn is_running_view() {
        let theme = cursive::theme::Theme::default();
        let backend = cursive::backend::dummy::Backend::init();
        let printer = cursive::Printer::new((1, 1), &theme, &*backend);

        let mut running = IsRunningView::new(Arc::new(AtomicBool::new(true)));
        let mut stopped = IsRunningView::new(Arc::new(AtomicBool::new(false)));

        assert_eq!(running.format(), "running".to_owned());
        assert_eq!(stopped.format(), "stopped".to_owned());

        running.draw(&printer);
        stopped.draw(&printer);

        assert_eq!(
            running.required_size((0, 0).into()),
            ("running".len(), 1).into()
        );
        assert_eq!(
            stopped.required_size((0, 0).into()),
            ("running".len(), 1).into()
        );
    }
}
