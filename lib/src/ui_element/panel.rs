use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
    time::Duration,
};

use anyhow::Result;
use sdl2::{rect::FRect, render::Canvas, video::Window};

use crate::{
    event::Event,
    refs::{MutRef, Ref},
    state_manager::StateManager,
    user_control::UserControl,
    zero,
};

pub struct Panel<Parent: 'static, State: 'static, T: UserControl<Parent, State> + 'static> {
    state: PhantomData<State>,
    parent: PhantomData<Parent>,
    surface: FRect,
    subs: Vec<T>,
    #[allow(clippy::type_complexity)]
    pub state_manager: StateManager<(MutRef<Parent>, MutRef<Vec<T>>)>,
}

impl<Parent: 'static, State: 'static, T: UserControl<Parent, State>> Panel<Parent, State, T> {
    pub fn new(subs: Vec<T>) -> Self {
        Self {
            state: PhantomData,
            parent: PhantomData,
            surface: zero(),
            subs,
            state_manager: StateManager::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.subs.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.subs.iter_mut()
    }
}

impl<Parent: 'static, State: 'static, T: UserControl<Parent, State>> UserControl<Parent, State>
    for Panel<Parent, State, T>
{
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
        match event {
            Event::ElementMove { x, y } => {
                this.surface.set_x(x);
                this.surface.set_y(y);
            }
            Event::ElementResize { width, height } => {
                this.surface.set_width(width);
                this.surface.set_height(height);
            }
            _ => {}
        }
        for sub in this.subs.iter_mut() {
            UserControl::event(sub.into(), canvas, event.clone(), parent, state)?;
        }
        Ok(())
    }

    fn update(
        mut this: MutRef<Self>,
        canvas: &Canvas<Window>,
        elapsed: Duration,
        parent: MutRef<Parent>,
        state: MutRef<State>,
    ) -> Result<()> {
        for sub in this.subs.iter_mut() {
            UserControl::update(sub.into(), canvas, elapsed, parent, state)?;
        }
        let subs = (&mut this.subs).into();
        this.as_mut().state_manager.apply((parent, subs))?;
        Ok(())
    }

    fn draw(
        this: Ref<Self>,
        canvas: &mut Canvas<Window>,
        parent: Ref<Parent>,
        state: Ref<State>,
    ) -> Result<()> {
        for sub in this.subs.iter() {
            UserControl::draw(sub.into(), canvas, parent, state)?;
        }
        Ok(())
    }
}

impl<Parent: 'static, State: 'static, T: UserControl<Parent, State>> Index<usize>
    for Panel<Parent, State, T>
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.subs[index]
    }
}

impl<Parent: 'static, State: 'static, T: UserControl<Parent, State>> IndexMut<usize>
    for Panel<Parent, State, T>
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.subs[index]
    }
}

#[cfg(test)]
pub(crate) mod panel_test {
    use anyhow::Result;
    use sdl2::mouse::MouseButton;

    use crate::refs::{MutRef, Ref};

    use super::*;

    struct Button {
        surface: FRect,
    }

    impl UserControl<(), usize> for Button {
        fn surface(this: Ref<Self>, _: Ref<()>, _: Ref<usize>) -> FRect {
            this.surface
        }

        fn event(
            this: MutRef<Self>,
            _: &Canvas<Window>,
            event: Event,
            _: MutRef<()>,
            mut state: MutRef<usize>,
        ) -> Result<()> {
            if let Event::MouseButtonDown { .. } = event {
                if event.hover(this.surface) {
                    *state += 1;
                }
            }
            Ok(())
        }

        fn update(
            _: MutRef<Self>,
            _: &Canvas<Window>,
            _: Duration,
            _: MutRef<()>,
            _: MutRef<usize>,
        ) -> Result<()> {
            Ok(())
        }

        fn draw(_: Ref<Self>, _: &mut Canvas<Window>, _: Ref<()>, _: Ref<usize>) -> Result<()> {
            Ok(())
        }
    }

    #[allow(clippy::too_many_lines)]
    pub(crate) fn test_panel_click(canvas: &mut Canvas<Window>) {
        let mut counter = 0_usize;
        let counter = MutRef::new(&mut counter);
        let mut panel = Panel::new(vec![
            Button {
                surface: FRect::new(12., 12., 2., 2.),
            },
            Button {
                surface: FRect::new(12., 16., 2., 2.),
            },
            Button {
                surface: FRect::new(16., 12., 2., 2.),
            },
            Button {
                surface: FRect::new(16., 16., 2., 2.),
            },
            Button {
                surface: FRect::new(10., 30., 10., 10.),
            },
            Button {
                surface: FRect::new(30., 10., 10., 10.),
            },
            Button {
                surface: FRect::new(30., 30., 10., 10.),
            },
        ]);
        let panel = MutRef::new(&mut panel);
        let mut parent = ();
        let parent = MutRef::new(&mut parent);
        assert!(UserControl::event(
            panel,
            canvas,
            Event::ElementMove { x: 0., y: 0. },
            parent,
            counter
        )
        .is_ok());
        assert!(UserControl::event(
            panel,
            canvas,
            Event::ElementResize {
                width: 50.,
                height: 50.,
            },
            parent,
            counter,
        )
        .is_ok());
        assert_eq!(*counter, 0);
        click(panel, parent, counter, canvas, 0., 0.);
        assert_eq!(*counter, 0);
        click(panel, parent, counter, canvas, 11., 11.);
        assert_eq!(*counter, 0);
        click(panel, parent, counter, canvas, 13., 13.);
        assert_eq!(*counter, 1);
        click(panel, parent, counter, canvas, 15., 15.);
        assert_eq!(*counter, 1);
        click(panel, parent, counter, canvas, 17., 17.);
        assert_eq!(*counter, 2);
        click(panel, parent, counter, canvas, 21., 21.);
        assert_eq!(*counter, 2);
        click(panel, parent, counter, canvas, 31., 11.);
        assert_eq!(*counter, 3);
        click(panel, parent, counter, canvas, 11., 31.);
        assert_eq!(*counter, 4);
        click(panel, parent, counter, canvas, 31., 31.);
        assert_eq!(*counter, 5);
    }

    fn click(
        panel: MutRef<Panel<(), usize, Button>>,
        parent: MutRef<()>,
        state: MutRef<usize>,
        canvas: &Canvas<Window>,
        x: f32,
        y: f32,
    ) {
        assert!(UserControl::event(
            panel,
            canvas,
            Event::MouseButtonDown {
                which: 0,
                mouse_btn: MouseButton::Left,
                clicks: 1,
                x,
                y,
            },
            parent,
            state
        )
        .is_ok());
    }
}
