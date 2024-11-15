use std::time::Duration;

use sdl2::{render::Canvas, video::Window};

use crate::{event::Event, ui_element::grid::GridChildren};

pub struct RefElement<'a, Parent> {
    element: &'a mut dyn GridChildren<Parent>,
}

impl<'a, Parent> RefElement<'a, Parent> {
    pub fn new(element: &'a mut dyn GridChildren<Parent>) -> Self {
        Self { element }
    }
}

impl<'a, Parent> GridChildren<Parent> for RefElement<'a, Parent> {
    fn grid_init(
        &mut self,
        canvas: &mut Canvas<Window>,
        parent: &mut Parent,
    ) -> Result<(), String> {
        self.element.grid_init(canvas, parent)
    }

    fn grid_init_frame(
        &mut self,
        canvas: &mut Canvas<Window>,
        surface: sdl2::rect::FRect,
        parent: &mut Parent,
    ) -> Result<(), String> {
        self.element.grid_init_frame(canvas, surface, parent)
    }

    fn grid_event(
        &mut self,
        canvas: &mut Canvas<Window>,
        event: Event,
        parent: &mut Parent,
    ) -> Result<(), String> {
        self.element.grid_event(canvas, event, parent)
    }

    fn grid_update(
        &mut self,
        canvas: &mut Canvas<Window>,
        elapsed: Duration,
        parent: &mut Parent,
    ) -> Result<(), String> {
        self.element.grid_update(canvas, elapsed, parent)
    }

    fn grid_draw(&self, canvas: &mut Canvas<Window>, parent: &Parent) -> Result<(), String> {
        self.element.grid_draw(canvas, parent)
    }
}
