use std::{marker::PhantomData, time::Duration};

use crate::{
    event::Event,
    functions::{FnColor, FnState, StateEnum},
    missing::{
        clipboard::{get_clipboard_text, set_clipboard_text},
        ui_string::UIString,
    },
    refs::{MutRef, Ref},
    user_control::UserControl, zero,
};
use anyhow::{anyhow, Result};
use sdl2::{
    keyboard::Keycode,
    mouse::MouseButton,
    rect::{FPoint, FRect},
    render::Canvas,
    ttf::Font,
    video::Window,
};

///Let the user enter text inside this element.
pub struct TextBox<Parent: 'static, State: 'static> {
    parent: PhantomData<Parent>,
    statel: PhantomData<State>,
    selected: Option<(usize, Option<usize>)>,
    font: &'static Font<'static, 'static>,
    surface: FRect,
    text: UIString,
    shift: bool,
    ctrl: bool,
    state: FnState<Self, Parent, State>,
    select_box_color: FnColor<Self, Parent, State>,
    select_line_color: FnColor<Self, Parent, State>,
    front_color: FnColor<Self, Parent, State>,
    back_color: FnColor<Self, Parent, State>,
}
impl<Parent: 'static, State: 'static> TextBox<Parent, State> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        font: &'static Font<'static, 'static>,
        text: UIString,
        state: FnState<Self, Parent, State>,
        select_box_color: FnColor<Self, Parent, State>,
        select_line_color: FnColor<Self, Parent, State>,
        front_color: FnColor<Self, Parent, State>,
        back_color: FnColor<Self, Parent, State>,
    ) -> Self {
        Self {
            parent: PhantomData,
            statel: PhantomData,
            selected: None,
            font,
            surface: zero(),
            text,
            shift: false,
            ctrl: false,
            state,
            select_box_color,
            select_line_color,
            front_color,
            back_color,
        }
    }

    pub const fn text(&self) -> &UIString {
        &self.text
    }

    pub fn text_mut(&mut self) -> &mut UIString {
        &mut self.text
    }

    fn select(&mut self, index: usize, to_index: Option<usize>) {
        self.selected = Some((index, to_index));
    }

    pub fn unselect(&mut self) {
        self.selected = None;
    }

    fn index_to_position(&self, index: usize) -> f32 {
        if index == 0 {
            return 0.;
        }
        self.font
            .size_of(&self.text.as_str()[..index])
            .expect("font error")
            .0 as f32
            / self.font.size_of(self.text.as_str()).expect("font error").0 as f32
    }

    fn position_to_index(&self, mut pos: f32) -> usize {
        if self.text.is_empty() {
            0
        } else {
            let scale = self.surface.width()
                / self.font.size_of(self.text.as_ref()).expect("font error").0 as f32;
            pos *= self.surface.width();
            for (i, c) in self.text.as_str().chars().enumerate() {
                let w = self.font.size_of_char(c).expect("font error").0 as f32 * scale;
                if w > pos {
                    if w / 2. > pos {
                        return i;
                    } else {
                        return i + 1;
                    }
                }
                pos -= w;
            }
            self.text.len()
        }
    }

    fn delete_selection(&mut self, index: &mut usize, to_index: usize) -> Result<()> {
        if *index < to_index {
            if self.text.drain(*index, to_index - *index)?.is_some() {
                self.select(*index, None);
            }
        } else if self.text.drain(to_index, *index - to_index)?.is_some() {
            self.select(to_index, None);
            *index = to_index
        }
        Ok(())
    }

    fn insert(
        &mut self,
        to_index: Option<usize>,
        index: &mut usize,
        mut text: String,
    ) -> Result<()> {
        if let Some(to_index) = to_index {
            self.delete_selection(index, to_index)?;
        }
        if self.shift {
            text = text.to_uppercase();
        } else {
            text = text.to_lowercase();
        }
        let tlen = self.text.insert_str(*index, text.as_str())?;
        self.select(*index + tlen, None);
        Ok(())
    }
}
impl<Parent: 'static, State: 'static> UserControl<Parent, State> for TextBox<Parent, State> {
    fn surface(this: Ref<Self>, _: Ref<Parent>, _: Ref<State>) -> FRect {
        this.surface
    }

    #[allow(clippy::too_many_lines)]
    fn event(
        mut this: MutRef<Self>,
        _: &Canvas<Window>,
        event: Event,
        parent: MutRef<Parent>,
        state: MutRef<State>,
    ) -> Result<()> {
        match event {
            Event::ElementMove { x, y } => {
                this.surface.set_x(x);
                this.surface.set_y(y);
                return Ok(());
            }
            Event::ElementResize { width, height } => {
                this.surface.set_width(width);
                this.surface.set_height(height);
                return Ok(());
            }
            _ => {}
        }
        if (this.state)(this.into(), parent.into(), state.into()) != StateEnum::Enable {
            return Ok(());
        }
        match (event.hover(this.surface), event) {
            (
                true,
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    x,
                    ..
                },
            ) => {
                if this.shift && this.selected.is_some() {
                    let (index1, _) = this.selected.ok_or(anyhow!("Checked"))?;
                    let index2 =
                        this.position_to_index((x - this.surface.x()) / this.surface.width());
                    this.select(index1, Some(index2));
                } else {
                    let index =
                        this.position_to_index((x - this.surface.x()) / this.surface.width());
                    this.select(index, None);
                }
            }
            (false, Event::MouseButtonDown { .. }) => this.unselect(),
            (true, Event::MouseMotion { mousestate, x, .. }) if mousestate.left() => {
                if let Some((index1, _)) = this.selected {
                    let index2 =
                        this.position_to_index((x - this.surface.x()) / this.surface.width());
                    this.select(index1, Some(index2));
                }
            }
            (
                _,
                Event::KeyDown {
                    keycode: Some(Keycode::LShift),
                    scancode: Some(_),
                    ..
                },
            )
            | (
                _,
                Event::KeyDown {
                    keycode: Some(Keycode::RShift),
                    scancode: Some(_),
                    ..
                },
            ) => {
                this.shift = true;
            }
            (
                _,
                Event::KeyUp {
                    keycode: Some(Keycode::LShift),
                    scancode: Some(_),
                    ..
                },
            )
            | (
                _,
                Event::KeyUp {
                    keycode: Some(Keycode::RShift),
                    scancode: Some(_),
                    ..
                },
            ) => {
                this.shift = false;
            }
            (
                _,
                Event::KeyDown {
                    keycode: Some(Keycode::LCtrl),
                    scancode: Some(_),
                    ..
                },
            )
            | (
                _,
                Event::KeyDown {
                    keycode: Some(Keycode::RCtrl),
                    scancode: Some(_),
                    ..
                },
            ) => {
                this.ctrl = true;
            }
            (
                _,
                Event::KeyUp {
                    keycode: Some(Keycode::LCtrl),
                    scancode: Some(_),
                    ..
                },
            )
            | (
                _,
                Event::KeyUp {
                    keycode: Some(Keycode::RCtrl),
                    scancode: Some(_),
                    ..
                },
            ) => {
                this.ctrl = false;
            }
            (
                _,
                Event::KeyDown {
                    keycode: Some(keycode),
                    scancode: Some(scancode),
                    ..
                },
            ) => {
                if let Some((mut index, to_index)) = this.selected {
                    match keycode {
                        Keycode::Backspace => {
                            if let Some(to_index) = to_index {
                                this.delete_selection(&mut index, to_index)?;
                            } else if index > 0 && this.text.remove(index - 1)?.is_some() {
                                this.select(index - 1, None);
                            }
                        }
                        Keycode::Delete => {
                            if let Some(to_index) = to_index {
                                this.delete_selection(&mut index, to_index)?;
                            } else if index < this.text.len() && this.text.remove(index)?.is_some()
                            {
                                this.select(index, None);
                            }
                        }
                        Keycode::Left => {
                            if let Some(to_index) = to_index {
                                if this.shift {
                                    if to_index > 0 {
                                        if index == to_index - 1 {
                                            this.select(index, None);
                                        } else {
                                            this.select(index, Some(to_index - 1));
                                        }
                                    }
                                } else {
                                    this.select(index.min(to_index), None);
                                }
                            } else if index == 0 {
                            } else if this.shift {
                                this.select(index, Some(index - 1));
                            } else {
                                this.select(index - 1, None);
                            }
                        }
                        Keycode::Right => {
                            if let Some(to_index) = to_index {
                                if this.shift {
                                    if to_index < this.text.len() {
                                        if index == to_index + 1 {
                                            this.select(index, None);
                                        } else {
                                            this.select(index, Some(to_index + 1));
                                        }
                                    }
                                } else {
                                    this.select(index.max(to_index), None);
                                }
                            } else if index == this.text.len() {
                            } else if this.shift {
                                this.select(index, Some(index + 1));
                            } else {
                                this.select(index + 1, None);
                            }
                        }
                        Keycode::Space => {
                            this.insert(to_index, &mut index, " ".to_owned())?;
                        }
                        Keycode::KP_0 => {
                            this.insert(to_index, &mut index, "0".to_owned())?;
                        }
                        Keycode::KP_1 => {
                            this.insert(to_index, &mut index, "1".to_owned())?;
                        }
                        Keycode::KP_2 => {
                            this.insert(to_index, &mut index, "2".to_owned())?;
                        }
                        Keycode::KP_3 => {
                            this.insert(to_index, &mut index, "3".to_owned())?;
                        }
                        Keycode::KP_4 => {
                            this.insert(to_index, &mut index, "4".to_owned())?;
                        }
                        Keycode::KP_5 => {
                            this.insert(to_index, &mut index, "5".to_owned())?;
                        }
                        Keycode::KP_6 => {
                            this.insert(to_index, &mut index, "6".to_owned())?;
                        }
                        Keycode::KP_7 => {
                            this.insert(to_index, &mut index, "7".to_owned())?;
                        }
                        Keycode::KP_8 => {
                            this.insert(to_index, &mut index, "8".to_owned())?;
                        }
                        Keycode::KP_9 => {
                            this.insert(to_index, &mut index, "9".to_owned())?;
                        }
                        Keycode::V if this.ctrl => {
                            this.insert(
                                to_index,
                                &mut index,
                                get_clipboard_text().unwrap_or(Ok(String::new()))?,
                            )?;
                        }
                        Keycode::C if this.ctrl => {
                            if let Some(to_index) = to_index {
                                if index != to_index {
                                    set_clipboard_text(
                                        &this.text.as_str()
                                            [index.min(to_index)..index.max(to_index)],
                                    )?;
                                }
                            }
                        }
                        Keycode::X if this.ctrl => {
                            if let Some(to_index) = to_index {
                                if index != to_index {
                                    set_clipboard_text(
                                        &this.text.as_str()
                                            [index.min(to_index)..index.max(to_index)],
                                    )?;
                                    this.delete_selection(&mut index, to_index)?;
                                }
                            }
                        }
                        Keycode::A if this.ctrl => {
                            if this.selected.is_some() {
                                let len = this.text.len();
                                this.select(0, Some(len));
                            }
                        }
                        _ if this.ctrl => {}
                        _ => {
                            this.insert(to_index, &mut index, scancode.to_string())?;
                        }
                    }
                }
            }
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
        let front_color = (this.front_color)(this, parent, state);
        canvas.set_draw_color(front_color);
        canvas.draw_frect(this.surface).map_err(|e| anyhow!(e))?;
        if !this.text.is_empty() {
            this.text.draw(canvas, None, this.surface, front_color)?;
        }
        if let Some((index, to_index)) = this.selected {
            if let Some(to_index) = to_index {
                canvas.set_draw_color((this.select_box_color)(this, parent, state));
                let pos1 = this.surface.width() * this.index_to_position(index) + this.surface.x();
                let pos2 =
                    this.surface.width() * this.index_to_position(to_index) + this.surface.x();
                canvas
                    .fill_frect(FRect::new(
                        pos1.min(pos2),
                        this.surface.y(),
                        pos1.max(pos2) - pos1.min(pos2),
                        this.surface.height(),
                    ))
                    .map_err(|e| anyhow!(e))?;
            } else {
                canvas.set_draw_color((this.select_line_color)(this, parent, state));
                canvas
                    .draw_fline(
                        FPoint::new(
                            this.surface.width() * this.index_to_position(index) + this.surface.x(),
                            this.surface.y(),
                        ),
                        FPoint::new(
                            this.surface.width() * this.index_to_position(index) + this.surface.x(),
                            this.surface.y() + this.surface.height(),
                        ),
                    )
                    .map_err(|e| anyhow!(e))?;
            }
        }
        Ok(())
    }
}
