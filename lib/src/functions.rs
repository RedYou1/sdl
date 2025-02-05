use anyhow::Result;
use sdl2::{
    pixels::Color,
    render::{Canvas, Texture},
    video::Window,
};

use crate::{
    missing::ui_string::UIString,
    refs::{MutRef, Ref},
};

pub type FnAction<Element, Parent, State> =
    Box<dyn FnMut(MutRef<Element>, MutRef<Parent>, MutRef<State>, &Canvas<Window>) -> Result<()>>;
pub type FnText<Element, Parent, State> =
    Box<dyn Fn(Ref<Element>, Ref<Parent>, Ref<State>) -> Result<(Option<UIString>, Color)>>;
#[derive(Debug, PartialEq, Eq)]
pub enum StateEnum {
    Hidden,
    Showing,
    Enable,
}
pub type FnState<Element, Parent, State> =
    Box<dyn Fn(Ref<Element>, Ref<Parent>, Ref<State>) -> StateEnum>;
pub type FnColor<Element, Parent, State> =
    Box<dyn Fn(Ref<Element>, Ref<Parent>, Ref<State>) -> Color>;
pub type FnImage<Element, Parent, State> =
    Box<dyn Fn(Ref<Element>, Ref<Parent>, Ref<State>) -> Result<&'static Texture<'static>>>;
pub type FnDraw<Element, Parent, State> = Box<
    dyn Fn(
        Ref<Element>,
        &mut Canvas<Window>,
        Ref<Parent>,
        Ref<State>,
    ) -> Result<()>,
>;
