use std::time::Duration;

use anyhow::{anyhow, Result};
use sdl2::{rect::FRect, render::Canvas, video::Window};

use crate::{
    event::Event,
    refs::{MutRef, Ref}, zero,
};

pub trait UserControl<Parent: 'static, State: 'static> {
    fn surface(this: Ref<Self>, parent: Ref<Parent>, state: Ref<State>) -> FRect;
    fn event(
        this: MutRef<Self>,
        canvas: &Canvas<Window>,
        event: Event,
        parent: MutRef<Parent>,
        state: MutRef<State>,
    ) -> Result<()>;
    fn update(
        this: MutRef<Self>,
        canvas: &Canvas<Window>,
        elapsed: Duration,
        parent: MutRef<Parent>,
        state: MutRef<State>,
    ) -> Result<()>;
    fn draw(
        this: Ref<Self>,
        canvas: &mut Canvas<Window>,
        parent: Ref<Parent>,
        state: Ref<State>,
    ) -> Result<()>;
}

impl<Parent: 'static, State: 'static> UserControl<Parent, State> for () {
    fn surface(_: Ref<Self>, _: Ref<Parent>, _: Ref<State>) -> FRect {
        zero()
    }

    fn event(
        _: MutRef<Self>,
        _: &Canvas<Window>,
        _: Event,
        _: MutRef<Parent>,
        _: MutRef<State>,
    ) -> Result<()> {
        Err(anyhow!("unit type used as a UserControl"))
    }

    fn update(
        _: MutRef<Self>,
        _: &Canvas<Window>,
        _: Duration,
        _: MutRef<Parent>,
        _: MutRef<State>,
    ) -> Result<()> {
        Err(anyhow!("unit type used as a UserControl"))
    }

    fn draw(_: Ref<Self>, _: &mut Canvas<Window>, _: Ref<Parent>, _: Ref<State>) -> Result<()> {
        Err(anyhow!("unit type used as a UserControl"))
    }
}

pub trait BWindow<State: 'static>: UserControl<(), State> {
    fn running(this: Ref<Self>, state: Ref<State>) -> bool;
}

pub trait EventWindow<State: 'static>: BWindow<State> {}

pub trait GameWindow<State: 'static>: BWindow<State> {
    fn time_scale(this: Ref<Self>, state: Ref<State>) -> f32;
    fn fps(this: Ref<Self>, state: Ref<State>) -> f32;
    fn fps_duration(this: Ref<Self>, state: Ref<State>) -> Duration {
        Duration::from_secs_f32(1. / Self::fps(this, state))
    }
}

impl<State: 'static, Other: EventWindow<State>> GameWindow<State> for Other {
    fn time_scale(_: Ref<Self>, _: Ref<State>) -> f32 {
        1.
    }

    fn fps(_: Ref<Self>, _: Ref<State>) -> f32 {
        20.
    }
}
