use std::{marker::PhantomData, time::Duration};

use crate::{
    event::Event,
    functions::{FnAction, FnColor, FnDraw, FnImage, FnState, FnText, StateEnum},
    missing::ui_string::UIString,
    refs::{MutRef, Ref},
    user_control::UserControl,
    zero,
};
use anyhow::{anyhow, Result};
use sdl2::{mouse::MouseButton, rect::FRect, render::Canvas, video::Window};

///Let you design a rectangle with the builder pattern.
pub struct UIRect<Parent: 'static, State: 'static> {
    parent: PhantomData<Parent>,
    statel: PhantomData<State>,
    ///It gets called when the mouse is hovering over the element and the left mouse button is down.
    action: Option<FnAction<Self, Parent, State>>,
    surface: FRect,
    text: Option<FnText<Self, Parent, State>>,
    state: FnState<Self, Parent, State>,
    back_color: FnColor<Self, Parent, State>,
    hover: bool,
    back_draw: Option<FnDraw<Self, Parent, State>>,
}
impl<Parent: 'static, State: 'static> UIRect<Parent, State> {
    pub fn new(
        state: FnState<Self, Parent, State>,
        back_color: FnColor<Self, Parent, State>,
    ) -> Self {
        Self {
            parent: PhantomData,
            statel: PhantomData,
            action: None,
            surface: zero(),
            text: None,
            state,
            back_color,
            hover: false,
            back_draw: None,
        }
    }

    pub const fn surface(&self) -> FRect {
        self.surface
    }

    pub fn state_mut(&mut self) -> &mut FnState<Self, Parent, State> {
        &mut self.state
    }

    pub fn action(mut self, action: FnAction<Self, Parent, State>) -> Self {
        self.action = Some(action);
        self
    }

    pub fn action_mut(&mut self) -> &mut Option<FnAction<Self, Parent, State>> {
        &mut self.action
    }

    pub fn text(mut self, text: FnText<Self, Parent, State>) -> Self {
        self.text = Some(text);
        self
    }

    pub fn text_mut(&mut self) -> &mut Option<FnText<Self, Parent, State>> {
        &mut self.text
    }

    pub fn image(mut self, image: FnImage<Self, Parent, State>) -> Self {
        self.back_draw = Some(Box::new(
            move |this, canvas: &mut Canvas<Window>, parent, state| {
                canvas
                    .copy_f(image(this, parent, state)?, None, this.surface)
                    .map_err(|e| anyhow!(e))
            },
        ));
        self
    }

    pub fn back_draw(mut self, back_draw: FnDraw<Self, Parent, State>) -> Self {
        self.back_draw = Some(back_draw);
        self
    }

    pub fn back_draw_mut(&mut self) -> &mut Option<FnDraw<Self, Parent, State>> {
        &mut self.back_draw
    }

    pub const fn hover(&self) -> bool {
        self.hover
    }

    pub fn get_text(
        this: Ref<Self>,
        parent: Ref<Parent>,
        state: Ref<State>,
    ) -> Result<Option<UIString>> {
        if let Some(text) = this.text.as_ref() {
            if let (Some(text), _) = text(this, parent, state)? {
                return Ok(Some(text));
            }
        }
        Ok(None)
    }
}
impl<Parent: 'static, State: 'static> UserControl<Parent, State> for UIRect<Parent, State> {
    fn surface(this: Ref<Self>, _: Ref<Parent>, _: Ref<State>) -> FRect {
        this.surface
    }

    fn event(
        mut this: MutRef<Self>,
        canvas: &Canvas<Window>,
        event: Event,
        parent: MutRef<Parent>,
        state: MutRef<State>,
    ) -> Result<()> {
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
        if (this.state)(this.into(), parent.into(), state.into()) != StateEnum::Enable {
            return Ok(());
        }
        match (event.hover(this.surface), event) {
            (
                true,
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    ..
                },
            ) => {
                let t = this;
                if let Some(action) = this.action.as_mut() {
                    (action)(t, parent, state, canvas)?;
                }
            }
            (true, Event::MouseMotion { .. }) => {
                this.hover = true;
            }
            (false, _) => this.hover = false,
            _ => {}
        }
        Ok(())
    }

    fn update(
        _: MutRef<Self>,
        _: &Canvas<Window>,
        _: Duration,
        _: MutRef<Parent>,
        _: MutRef<State>,
    ) -> Result<()> {
        Ok(())
    }

    fn draw(
        this: Ref<Self>,
        canvas: &mut Canvas<Window>,
        parent: Ref<Parent>,
        state: Ref<State>,
    ) -> Result<()> {
        if (this.state)(this, parent, state) == StateEnum::Hidden {
            return Ok(());
        }
        canvas.set_draw_color((this.back_color)(this, parent, state));
        canvas.fill_frect(this.surface).map_err(|e| anyhow!(e))?;
        if let Some(back_draw) = this.back_draw.as_ref() {
            back_draw(this, canvas, parent, state)?;
        }
        if let Some(text) = this.text.as_ref() {
            if let (Some(text), color) = text(this, parent, state)? {
                text.draw(canvas, None, this.surface, color)?;
            }
        }
        Ok(())
    }
}
