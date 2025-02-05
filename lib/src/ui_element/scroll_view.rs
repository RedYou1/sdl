use std::{marker::PhantomData, time::Duration};

use anyhow::{anyhow, Result};
use sdl2::{
    mouse::MouseButton,
    rect::{FPoint, FRect},
    render::{BlendMode, Canvas},
    video::Window,
};

use crate::{
    event::Event,
    functions::FnColor,
    missing::rect::as_rect,
    refs::{MutRef, Ref},
    user_control::UserControl,
    zero,
};

///Let you have an unrestrained sized sub element inside your restrained sized Window/SubElement.
pub struct ScrollView<Parent: 'static, State: 'static, Child: UserControl<Parent, State> + 'static>
{
    parent: PhantomData<Parent>,
    surface: FRect,
    child: Child,
    child_size: (f32, f32),
    child_surface: FRect,
    scroll_color: FnColor<Self, Parent, State>,
    v_scroll: f32,
    h_scroll: f32,
    v_selected: bool,
    h_selected: bool,
}

impl<Parent: 'static, State: 'static, Child: UserControl<Parent, State> + 'static>
    ScrollView<Parent, State, Child>
{
    pub fn new(
        child: Child,
        child_width: f32,
        child_height: f32,
        scroll_color: FnColor<Self, Parent, State>,
    ) -> Self {
        Self {
            parent: PhantomData,
            surface: zero(),
            child,
            child_size: (child_width, child_height),
            child_surface: zero(),
            scroll_color,
            h_scroll: 0.,
            v_scroll: 0.,
            h_selected: false,
            v_selected: false,
        }
    }

    pub const fn child(&self) -> &Child {
        &self.child
    }

    pub fn child_mut(&mut self) -> &mut Child {
        &mut self.child
    }

    pub const fn child_size(&self) -> (f32, f32) {
        self.child_size
    }

    pub fn child_size_mut(&mut self) -> &mut (f32, f32) {
        &mut self.child_size
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

impl<Parent: 'static, State: 'static, Child: UserControl<Parent, State> + 'static>
    UserControl<Parent, State> for ScrollView<Parent, State, Child>
{
    fn surface(this: Ref<Self>, _: Ref<Parent>, _: Ref<State>) -> FRect {
        this.surface
    }

    #[allow(clippy::too_many_lines)]
    fn event(
        mut this: MutRef<Self>,
        canvas: &Canvas<Window>,
        event: Event,
        parent: MutRef<Parent>,
        state: MutRef<State>,
    ) -> Result<()> {
        //TODO send event mouse move when any current returning
        if let Event::ElementMove { x, y } = event {
            this.surface.set_x(x);
            this.surface.set_y(y);
            return Ok(());
        }
        if let Event::ElementResize { width, height } = event {
            this.surface.set_width(width);
            this.surface.set_height(height);
            return Ok(());
        }

        if this.child_size.0 > this.surface.width() {
            let h_scroll = this.h_scroll();
            match event {
                Event::MouseMotion { mousestate, x, .. } => {
                    if mousestate.left() && this.h_selected {
                        this.h_scroll = ((x - this.surface.x() - h_scroll.width() / 2.)
                            / (this.surface.width() - h_scroll.width()))
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
                    this.h_selected = h_scroll.contains_point(FPoint::new(x, y));
                    if this.h_selected {
                        return Ok(());
                    }
                }
                Event::MouseButtonUp { .. } => this.h_selected = false,
                _ => {}
            }
        }
        if this.child_size.1 > this.surface.height() {
            let v_scroll = this.v_scroll();
            match event {
                Event::MouseMotion { mousestate, y, .. } => {
                    if mousestate.left() && this.v_selected {
                        this.v_scroll = ((y - this.surface.y() - v_scroll.height() / 2.)
                            / (this.surface.height() - v_scroll.height()))
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
                    this.v_selected = v_scroll.contains_point(FPoint::new(x, y));
                    if this.v_selected {
                        return Ok(());
                    }
                }
                Event::MouseButtonUp { .. } => this.v_selected = false,
                _ => {}
            }
        }
        UserControl::event(
            MutRef::new(&mut this.child),
            canvas,
            match event {
                Event::MouseMotion {
                    which,
                    mousestate,
                    x,
                    y,
                    moved_x,
                    moved_y,
                } => {
                    let (x, y) = this.offset_event(x, y);
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
                    let (x, y) = this.offset_event(x, y);
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
                    let (x, y) = this.offset_event(x, y);
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
                    if this.child_size.0 > this.surface.width() {
                        this.h_scroll = (this.h_scroll - scroll_x * 0.1).clamp(0., 1.);
                    }
                    if this.child_size.1 > this.surface.height() {
                        this.v_scroll = (this.v_scroll - scroll_y * 0.1).clamp(0., 1.);
                    }

                    let (mouse_x, mouse_y) = this.offset_event(mouse_x, mouse_y);
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
            },
            parent,
            state,
        )
    }

    fn update(
        mut this: MutRef<Self>,
        canvas: &Canvas<Window>,
        elapsed: Duration,
        parent: MutRef<Parent>,
        state: MutRef<State>,
    ) -> Result<()> {
        let surface = Child::surface(Ref::new(&this.child), parent.into(), state.into());
        if 0. != surface.x() || 0. != surface.y() {
            Child::event(
                MutRef::new(&mut this.child),
                canvas,
                Event::ElementMove { x: 0., y: 0. },
                parent,
                state,
            )?;
        }
        if this.child_size.0 != surface.width() || this.child_size.1 != surface.height() {
            Child::event(
                MutRef::new(&mut this.child),
                canvas,
                Event::ElementResize {
                    width: this.child_size.0,
                    height: this.child_size.1,
                },
                parent,
                state,
            )?;
        }
        let (a, b) = this.child_size;
        let s = this.surface;
        let (h, v) = (this.h_scroll, this.v_scroll);
        this.child_surface.set_width(s.width().min(a));
        this.child_surface.set_height(s.height().min(b));
        this.child_surface.set_x(h * (a - s.width()));
        this.child_surface.set_y(v * (b - s.height()));
        Child::update(MutRef::new(&mut this.child), canvas, elapsed, parent, state)
    }

    fn draw(
        this: Ref<Self>,
        canvas: &mut Canvas<Window>,
        parent: Ref<Parent>,
        state: Ref<State>,
    ) -> Result<()> {
        let tc = canvas.texture_creator();
        let mut sub = tc
            .create_texture_target(None, this.child_size.0 as u32, this.child_size.1 as u32)
            .map_err(|e| anyhow!(e))?;
        sub.set_blend_mode(BlendMode::Blend);
        let mut success = Ok(());
        canvas
            .with_texture_canvas(&mut sub, |sub| {
                success = Child::draw(Ref::new(&this.child), sub, parent, state)
            })
            .map_err(|e| anyhow!(e))?;
        success?;
        canvas
            .copy_f(&sub, Some(as_rect(this.child_surface)), this.surface)
            .map_err(|e| anyhow!(e))?;
        let color = this.scroll_color.as_ref()(this, parent, state);
        canvas.set_draw_color(color);
        if this.child_size.0 > this.surface.width() {
            canvas.fill_frect(this.h_scroll()).map_err(|e| anyhow!(e))?;
        }
        if this.child_size.1 > this.surface.height() {
            canvas.fill_frect(this.v_scroll()).map_err(|e| anyhow!(e))?;
        }
        Ok(())
    }
}
