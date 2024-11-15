use std::{marker::PhantomData, time::Duration};

use sdl2::{
    mouse::MouseButton,
    rect::{FPoint, FRect},
    render::{BlendMode, Canvas},
    video::Window,
};

use crate::{event::Event, functions::FnColor, ui_element::grid::GridChildren, missing::rect::as_rect};

pub struct ScrollView<Parent, Child: GridChildren<Parent>> {
    parent: PhantomData<Parent>,
    surface: FRect,
    child: Child,
    child_size: (f32, f32),
    child_surface: FRect,
    scroll_color: FnColor<Parent, ScrollView<Parent, Child>>,
    v_scroll: f32,
    h_scroll: f32,
    v_selected: bool,
    h_selected: bool,
}

impl<Parent, Child: GridChildren<Parent>> ScrollView<Parent, Child> {
    pub fn new(
        child: Child,
        child_width: f32,
        child_height: f32,
        scroll_color: FnColor<Parent, ScrollView<Parent, Child>>,
    ) -> Self {
        Self {
            parent: PhantomData,
            surface: FRect::new(0., 0., 0., 0.),
            child,
            child_size: (child_width, child_height),
            child_surface: FRect::new(0., 0., 0., 0.),
            scroll_color,
            h_scroll: 0.,
            v_scroll: 0.,
            h_selected: false,
            v_selected: false,
        }
    }

    fn offset_event(&self, x: f32, y: f32) -> (f32, f32) {
        (
            if self.surface.x() > x {
                x - self.surface.x()
            } else if self.surface.x() + self.surface.width() < x {
                self.child_size.0 + x - self.surface.x() - self.surface.width()
            } else {
                (x - self.surface.x()) / self.surface.width() * self.child_surface.width()
                    + self.child_surface.x()
            },
            if self.surface.y() > y {
                y - self.surface.y()
            } else if self.surface.y() + self.surface.height() < y {
                self.child_size.1 + y - self.surface.y() - self.surface.height()
            } else {
                (y - self.surface.y()) / self.surface.height() * self.child_surface.height()
                    + self.child_surface.y()
            },
        )
    }

    fn h_scroll(&self) -> FRect {
        let w = (2. * self.surface.width() - self.child_size.0).max(30.);
        FRect::new(
            self.surface.x() + self.h_scroll * (self.surface.width() - w),
            self.surface.y() + self.surface.height() - 30.,
            w,
            30.,
        )
    }

    fn v_scroll(&self) -> FRect {
        let h = (2. * self.surface.height() - self.child_size.1).max(30.);
        FRect::new(
            self.surface.x() + self.surface.width() - 30.,
            self.surface.y() + self.v_scroll * (self.surface.height() - h),
            30.,
            h,
        )
    }
}

impl<Parent, Child: GridChildren<Parent>> GridChildren<Parent> for ScrollView<Parent, Child> {
    fn grid_init(
        &mut self,
        canvas: &mut Canvas<Window>,
        parent: &mut Parent,
    ) -> Result<(), String> {
        self.child.grid_init(canvas, parent)
    }

    fn grid_init_frame(
        &mut self,
        canvas: &mut Canvas<Window>,
        surface: sdl2::rect::FRect,
        parent: &mut Parent,
    ) -> Result<(), String> {
        self.surface = surface;
        self.child_surface
            .set_width(self.surface.width().min(self.child_size.0));
        self.child_surface
            .set_height(self.surface.height().min(self.child_size.1));
        self.child_surface
            .set_x(self.h_scroll * (self.child_size.0 - self.surface.width()));
        self.child_surface
            .set_y(self.v_scroll * (self.child_size.1 - self.surface.height()));

        self.child.grid_init_frame(
            canvas,
            FRect::new(0., 0., self.child_size.0, self.child_size.1),
            parent,
        )
    }

    #[allow(clippy::too_many_lines)]
    fn grid_event(
        &mut self,
        canvas: &mut Canvas<Window>,
        event: crate::event::Event,
        parent: &mut Parent,
    ) -> Result<(), String> {
        if self.child_size.0 > self.surface.width() {
            let h_scroll = self.h_scroll();
            match event {
                Event::MouseMotion { mousestate, x, .. } => {
                    if mousestate.left() && self.h_selected {
                        self.h_scroll = ((x - self.surface.x() - h_scroll.width() / 2.)
                            / (self.surface.width() - h_scroll.width()))
                        .clamp(0., 1.);
                        return Ok(());
                    }
                }
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    x,
                    y,
                    ..
                } => {
                    self.h_selected = h_scroll.contains_point(FPoint::new(x, y));
                    if self.h_selected {
                        return Ok(());
                    }
                }
                Event::MouseButtonUp { .. } => self.h_selected = false,
                _ => {}
            }
        }
        if self.child_size.1 > self.surface.height() {
            let v_scroll = self.v_scroll();
            match event {
                Event::MouseMotion { mousestate, y, .. } => {
                    if mousestate.left() && self.v_selected {
                        self.v_scroll = ((y - self.surface.y() - v_scroll.height() / 2.)
                            / (self.surface.height() - v_scroll.height()))
                        .clamp(0., 1.);
                        return Ok(());
                    }
                }
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    x,
                    y,
                    ..
                } => {
                    self.v_selected = v_scroll.contains_point(FPoint::new(x, y));
                    if self.v_selected {
                        return Ok(());
                    }
                }
                Event::MouseButtonUp { .. } => self.v_selected = false,
                _ => {}
            }
        }
        let event = match event {
            Event::MouseMotion {
                which,
                mousestate,
                x,
                y,
                moved_x,
                moved_y,
            } => {
                let (x, y) = self.offset_event(x, y);
                Event::MouseMotion {
                    which,
                    mousestate,
                    x,
                    y,
                    moved_x,
                    moved_y,
                }
            }
            Event::MouseButtonDown {
                which,
                mouse_btn,
                clicks,
                x,
                y,
            } => {
                let (x, y) = self.offset_event(x, y);
                Event::MouseButtonDown {
                    which,
                    mouse_btn,
                    clicks,
                    x,
                    y,
                }
            }
            Event::MouseButtonUp {
                which,
                mouse_btn,
                clicks,
                x,
                y,
            } => {
                let (x, y) = self.offset_event(x, y);
                Event::MouseButtonUp {
                    which,
                    mouse_btn,
                    clicks,
                    x,
                    y,
                }
            }
            Event::MouseWheel {
                which,
                scroll_x,
                scroll_y,
                direction,
                mouse_x,
                mouse_y,
            } => {
                if self.child_size.0 > self.surface.width() {
                    self.h_scroll = (self.h_scroll - scroll_x * 0.1).clamp(0., 1.);
                }
                if self.child_size.1 > self.surface.height() {
                    self.v_scroll = (self.v_scroll - scroll_y * 0.1).clamp(0., 1.);
                }

                let (mouse_x, mouse_y) = self.offset_event(mouse_x, mouse_y);
                Event::MouseWheel {
                    which,
                    scroll_x,
                    scroll_y,
                    direction,
                    mouse_x,
                    mouse_y,
                }
            }
            event => event,
        };
        self.child.grid_event(canvas, event, parent)
    }

    fn grid_update(
        &mut self,
        canvas: &mut Canvas<Window>,
        elapsed: Duration,
        parent: &mut Parent,
    ) -> Result<(), String> {
        self.child.grid_update(canvas, elapsed, parent)
    }

    fn grid_draw(&self, canvas: &mut Canvas<Window>, parent: &Parent) -> Result<(), String> {
        let tc = canvas.texture_creator();
        let mut sub = tc
            .create_texture_target(None, self.child_size.0 as u32, self.child_size.1 as u32)
            .map_err(|e| e.to_string())?;
        sub.set_blend_mode(BlendMode::Blend);
        let mut success = Ok(());
        canvas
            .with_texture_canvas(&mut sub, |sub| success = self.child.grid_draw(sub, parent))
            .map_err(|e| e.to_string())?;
        success?;
        canvas.copy_f(&sub, Some(as_rect(self.child_surface)), self.surface)?;
        let color = self.scroll_color.as_ref()(parent, self);
        canvas.set_draw_color(color);
        if self.child_size.0 > self.surface.width() {
            canvas.fill_frect(self.h_scroll())?;
        }
        if self.child_size.1 > self.surface.height() {
            canvas.fill_frect(self.v_scroll())?;
        }
        Ok(())
    }
}
