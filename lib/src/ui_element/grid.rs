use std::{collections::HashMap, fmt::Debug, hash::Hash, marker::PhantomData, time::Duration};

use anyhow::{anyhow, Result};
use sdl2::{rect::FRect, render::Canvas, video::Window};

use crate::{
    event::Event,
    refs::{MutRef, Ref},
    state_manager::StateManager,
    user_control::UserControl,
    zero,
};

#[derive(Debug)]
pub enum ColType {
    Px(f32),
    Ratio(f32),
}

impl ColType {
    pub fn scale_ration(&self, total_ratio: f32) -> Self {
        match self {
            Self::Px(f) => Self::Px(*f),
            Self::Ratio(f) => Self::Ratio(*f / total_ratio),
        }
    }

    pub fn to_px(&self, total_px: f32) -> f32 {
        match self {
            Self::Px(f) => *f,
            Self::Ratio(f) => *f * total_px,
        }
    }
}

#[derive(Debug)]
pub enum RowType {
    Px(f32),
    Ratio(f32),
}

impl RowType {
    pub fn scale_ration(&self, total_ratio: f32) -> Self {
        match self {
            Self::Px(f) => Self::Px(*f),
            Self::Ratio(f) => Self::Ratio(*f / total_ratio),
        }
    }

    pub fn to_px(&self, total_px: f32) -> f32 {
        match self {
            Self::Px(f) => *f,
            Self::Ratio(f) => *f * total_px,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

pub struct Grid<Parent: 'static, State: 'static, Child: UserControl<Parent, State> + 'static> {
    parent: PhantomData<Parent>,
    state: PhantomData<State>,
    elements: HashMap<Pos, Child>,
    static_x: f32,
    static_y: f32,
    cols: Vec<ColType>,
    rows: Vec<RowType>,
    surface: FRect,
    #[allow(clippy::type_complexity)]
    pub state_manager: StateManager<(
        MutRef<Parent>,
        MutRef<Vec<ColType>>,
        MutRef<Vec<RowType>>,
        MutRef<HashMap<Pos, Child>>,
    )>,
}

impl<Parent: 'static, State: 'static, Child: UserControl<Parent, State> + 'static>
    Grid<Parent, State, Child>
{
    pub fn new(cols: Vec<ColType>, rows: Vec<RowType>, elements: HashMap<Pos, Child>) -> Self {
        Self {
            parent: PhantomData,
            state: PhantomData,
            elements,
            static_x: 0.,
            static_y: 0.,
            cols,
            rows,
            surface: zero(),
            state_manager: StateManager::new(),
        }
    }

    pub fn clear(&mut self) {
        self.cols.clear();
        self.rows.clear();
        self.elements.clear();
    }

    pub fn rows(&self) -> &[RowType] {
        &self.rows
    }

    pub fn cols(&self) -> &[ColType] {
        &self.cols
    }

    pub fn get_element(&self, x: usize, y: usize) -> Option<&Child> {
        self.elements.get(&Pos { x, y })
    }

    pub fn get_element_mut(&mut self, x: usize, y: usize) -> Option<&mut Child> {
        self.elements.get_mut(&Pos { x, y })
    }

    fn reform(
        &'static mut self,
        canvas: &Canvas<Window>,
        parent: MutRef<Parent>,
        state: MutRef<State>,
    ) -> Result<()> {
        self.static_x = 0.;
        let mut dyn_x = 0.;
        for col in &self.cols {
            match col {
                ColType::Px(x) => self.static_x += *x,
                ColType::Ratio(x) => dyn_x += *x,
            }
        }
        for col in &mut self.cols {
            *col = col.scale_ration(dyn_x);
        }

        self.static_y = 0.;
        let mut dyn_y = 0.;
        for row in &self.rows {
            match row {
                RowType::Px(y) => self.static_y += *y,
                RowType::Ratio(y) => dyn_y += *y,
            }
        }
        for row in &mut self.rows {
            *row = row.scale_ration(dyn_y);
        }

        let remain_width = self.surface.width() - self.static_x;
        let remain_height = self.surface.height() - self.static_y;
        if remain_width < 0. || remain_height < 0. {
            return Err(anyhow!(
                "Not enough space: requested {}x{} in a grid of {}x{}",
                self.static_x,
                self.static_y,
                self.surface.width(),
                self.surface.height()
            ));
        }

        let mut p_y = self.surface.y();
        for (y, pos_y) in self.rows.iter().enumerate() {
            let mut p_x = self.surface.x();
            let height = pos_y.to_px(remain_height);
            for (x, pos_x) in self.cols.iter().enumerate() {
                let width = pos_x.to_px(remain_width);
                if let Some(element) = self.elements.get_mut(&Pos { x, y }) {
                    let surface = UserControl::surface(element.into(), parent.into(), state.into());
                    if surface.x() != p_x || surface.y() != p_y {
                        UserControl::event(
                            element.into(),
                            canvas,
                            Event::ElementMove { x: p_x, y: p_y },
                            parent,
                            state,
                        )?;
                    }
                    if surface.width() != width || surface.height() != height {
                        UserControl::event(
                            element.into(),
                            canvas,
                            Event::ElementResize { width, height },
                            parent,
                            state,
                        )?;
                    }
                }
                p_x += width;
            }
            p_y += height;
        }
        Ok(())
    }
}

impl<Parent: 'static, State: 'static, Child: UserControl<Parent, State> + 'static>
    UserControl<Parent, State> for Grid<Parent, State, Child>
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
                if x != this.surface.x() || y != this.surface.y() {
                    let dx = x - this.surface.x();
                    let dy = y - this.surface.y();
                    for element in this.elements.values_mut() {
                        let surface =
                            UserControl::surface(element.into(), parent.into(), state.into());
                        UserControl::event(
                            element.into(),
                            canvas,
                            Event::ElementMove {
                                x: surface.x() + dx,
                                y: surface.y() + dy,
                            },
                            parent,
                            state,
                        )?;
                    }
                    this.surface.set_x(x);
                    this.surface.set_y(y);
                }
            }
            Event::ElementResize { width, height } => {
                if width != this.surface.width() || height != this.surface.height() {
                    this.surface.set_width(width);
                    this.surface.set_height(height);
                    this.as_mut().reform(canvas, parent, state)?;
                }
            }
            _ => {
                for element in this.elements.values_mut() {
                    UserControl::event(element.into(), canvas, event.clone(), parent, state)?;
                }
            }
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
        for element in this.elements.values_mut() {
            UserControl::update(element.into(), canvas, elapsed, parent, state)?;
        }
        let cols = (&mut this.cols).into();
        let rows = (&mut this.rows).into();
        let elements = (&mut this.elements).into();
        if this
            .as_mut()
            .state_manager
            .apply((parent, cols, rows, elements))?
        {
            this.as_mut().reform(canvas, parent, state)?;
        }
        Ok(())
    }

    fn draw(
        this: Ref<Self>,
        canvas: &mut Canvas<Window>,
        parent: Ref<Parent>,
        state: Ref<State>,
    ) -> Result<()> {
        for element in this.elements.values() {
            UserControl::draw(element.into(), canvas, parent, state)?;
        }
        Ok(())
    }
}

impl<Parent: 'static, State: 'static, Child: UserControl<Parent, State> + 'static>
    Grid<Parent, State, Child>
{
    pub fn iter(&self) -> impl Iterator<Item = (&Pos, Ref<Child>)> {
        self.elements.iter().map(|(p, c)| (p, c.into()))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Pos, MutRef<Child>)> {
        self.elements.iter_mut().map(|(p, c)| (p, c.into()))
    }
}

#[macro_export]
/// Cols,..,..;</br>
/// Rows,..,..;</br>
/// Pos => Element,</br>
/// ...
macro_rules! simple_grid {
    ($($col:expr),*; $($row:expr),*; $($pos:expr => $child:expr),* $(,)?) => {
        Grid::new(
            vec![$($col),*],
            vec![$($row),*],
            HashMap::from([$(($pos, $child)),*])
        )
    };
}

#[cfg(test)]
pub(crate) mod grid_test {
    use red_sdl_macro::UserControl;
    use sdl2::mouse::MouseButton;

    use crate::refs::MutRef;

    use super::*;

    struct Button {
        surface: FRect,
    }

    impl UserControl<(), usize> for Button {
        fn surface(this: Ref<Self>, _: Ref<()>, _: Ref<usize>) -> FRect {
            this.surface
        }

        fn event(
            mut this: MutRef<Self>,
            _: &Canvas<Window>,
            event: Event,
            _: MutRef<()>,
            mut counter: MutRef<usize>,
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
                Event::MouseButtonDown { .. } if event.hover(this.surface) => {
                    *counter += 1;
                }
                _ => {}
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

    #[derive(UserControl)]
    #[state(usize)]
    enum TestGridClickChilds {
        Button(Button),
        Sub(Grid<(), usize, Button>),
    }

    #[allow(clippy::too_many_lines)]
    pub(crate) fn test_grid_click(canvas: &mut Canvas<Window>) {
        let mut counter = 0_usize;
        let counter = MutRef::new(&mut counter);
        let mut grid = simple_grid!(
            ColType::Px(10.),
            ColType::Ratio(1.),
            ColType::Px(10.),
            ColType::Ratio(1.),
            ColType::Px(10.);
            RowType::Px(10.),
            RowType::Ratio(1.),
            RowType::Px(10.),
            RowType::Ratio(1.),
            RowType::Px(10.);
            Pos { x: 1, y: 1 } => simple_grid!(
                    ColType::Px(2.),
                    ColType::Ratio(1.),
                    ColType::Px(2.),
                    ColType::Ratio(1.),
                    ColType::Px(2.);
                    RowType::Px(2.),
                    RowType::Ratio(1.),
                    RowType::Px(2.),
                    RowType::Ratio(1.),
                    RowType::Px(2.);
                    Pos { x: 1, y: 1 } => Button {
                            surface: zero(),
                        },
                    Pos { x: 3, y: 1 } => Button {
                            surface: zero(),
                        },
                    Pos { x: 1, y: 3 } => Button {
                            surface: zero(),
                        },
                    Pos { x: 3, y: 3 } => Button {
                            surface: zero(),
                        },
                ).into(),
            Pos { x: 3, y: 1 } => Button {
                    surface: zero(),
                }.into(),
            Pos { x: 1, y: 3 } => Button {
                    surface: zero(),
                }.into(),
            Pos { x: 3, y: 3 } => Button {
                    surface: zero(),
                }.into(),
        );
        let grid = MutRef::new(&mut grid);
        let mut parent = ();
        let parent = MutRef::new(&mut parent);
        UserControl::event(
            grid,
            canvas,
            Event::ElementMove { x: 0., y: 0. },
            parent,
            counter,
        )
        .expect("");
        UserControl::event(
            grid,
            canvas,
            Event::ElementResize {
                width: 50.,
                height: 50.,
            },
            parent,
            counter,
        )
        .expect("");
        assert_eq!(*counter, 0);
        click(grid, parent, counter, canvas, 0., 0.);
        assert_eq!(*counter, 0);
        click(grid, parent, counter, canvas, 11., 11.);
        assert_eq!(*counter, 0);
        click(grid, parent, counter, canvas, 13., 13.);
        assert_eq!(*counter, 1);
        click(grid, parent, counter, canvas, 15., 15.);
        assert_eq!(*counter, 1);
        click(grid, parent, counter, canvas, 17., 17.);
        assert_eq!(*counter, 2);
        click(grid, parent, counter, canvas, 21., 21.);
        assert_eq!(*counter, 2);
        click(grid, parent, counter, canvas, 31., 11.);
        assert_eq!(*counter, 3);
        click(grid, parent, counter, canvas, 11., 31.);
        assert_eq!(*counter, 4);
        click(grid, parent, counter, canvas, 31., 31.);
        assert_eq!(*counter, 5);
    }

    fn click(
        grid: MutRef<Grid<(), usize, TestGridClickChilds>>,
        parent: MutRef<()>,
        state: MutRef<usize>,
        canvas: &Canvas<Window>,
        x: f32,
        y: f32,
    ) {
        assert!(UserControl::event(
            grid,
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
