use std::{collections::HashMap, fmt::Debug, hash::Hash, time::Duration};

use sdl2::{rect::FRect, render::Canvas, video::Window};

use crate::{event::Event, user_control::UserControl};

pub trait GridChildren<T> {
    fn grid_init(&mut self, canvas: &mut Canvas<Window>, parent: &mut T) -> Result<(), String>;
    fn grid_init_frame(
        &mut self,
        canvas: &mut Canvas<Window>,
        surface: FRect,
        parent: &mut T,
    ) -> Result<(), String>;
    fn grid_event(
        &mut self,
        canvas: &mut Canvas<Window>,
        event: Event,
        parent: &mut T,
    ) -> Result<(), String>;
    fn grid_update(
        &mut self,
        canvas: &mut Canvas<Window>,
        elapsed: Duration,
        parent: &mut T,
    ) -> Result<(), String>;
    fn grid_draw(&self, canvas: &mut Canvas<Window>, parent: &T) -> Result<(), String>;
}

impl<T, K: UserControl> GridChildren<T> for K {
    fn grid_init(&mut self, canvas: &mut Canvas<Window>, _: &mut T) -> Result<(), String> {
        self.init(canvas)
    }

    fn grid_init_frame(
        &mut self,
        canvas: &mut Canvas<Window>,
        surface: FRect,
        _: &mut T,
    ) -> Result<(), String> {
        self.init_frame(canvas, surface)
    }

    fn grid_event(
        &mut self,
        canvas: &mut Canvas<Window>,
        event: Event,
        _: &mut T,
    ) -> Result<(), String> {
        self.event(canvas, event)
    }

    fn grid_update(
        &mut self,
        canvas: &mut Canvas<Window>,
        elapsed: Duration,
        _: &mut T,
    ) -> Result<(), String> {
        self.update(canvas, elapsed)
    }

    fn grid_draw(&self, canvas: &mut Canvas<Window>, _: &T) -> Result<(), String> {
        self.draw(canvas)
    }
}

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

pub struct GridElement<T> {
    surface: FRect,
    element: Box<dyn GridChildren<T>>,
}

pub struct Grid<T> {
    parent: *mut T,
    elements: HashMap<Pos, GridElement<T>>,
    static_x: f32,
    static_y: f32,
    cols: Vec<ColType>,
    rows: Vec<RowType>,
    last_width: f32,
    last_height: f32,
}

impl<T> Grid<T> {
    /// # Safety
    /// can't call any function because will fail.
    #[allow(invalid_value)]
    pub unsafe fn empty() -> Self {
        Self {
            parent: std::ptr::null_mut(),
            elements: HashMap::new(),
            static_x: 0.,
            static_y: 0.,
            cols: Vec::new(),
            rows: Vec::new(),
            last_width: 0.,
            last_height: 0.,
        }
    }

    pub fn new(
        parent: *mut T,
        cols: Vec<ColType>,
        rows: Vec<RowType>,
        elements: HashMap<Pos, Box<dyn GridChildren<T>>>,
    ) -> Self {
        let mut static_x = 0.;
        let mut dyn_x = 0.;
        for col in &cols {
            match col {
                ColType::Px(x) => static_x += *x,
                ColType::Ratio(x) => dyn_x += *x,
            }
        }
        let mut static_y = 0.;
        let mut dyn_y = 0.;
        for row in &rows {
            match row {
                RowType::Px(y) => static_y += *y,
                RowType::Ratio(y) => dyn_y += *y,
            }
        }

        let elements = elements
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    GridElement {
                        surface: FRect::new(0., 0., 0., 0.),
                        element: v,
                    },
                )
            })
            .collect();

        Self {
            parent,
            elements,
            static_x,
            static_y,
            cols: cols.into_iter().map(|c| c.scale_ration(dyn_x)).collect(),
            rows: rows.into_iter().map(|r| r.scale_ration(dyn_y)).collect(),
            last_width: 0.,
            last_height: 0.,
        }
    }
}

impl<T> UserControl for Grid<T> {
    fn init(&mut self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let parent = unsafe { self.parent.as_mut().ok_or("unwrap ptr init grid")? };
        for (_, GridElement { element, .. }) in self.elements.iter_mut() {
            element.grid_init(canvas, parent)?;
        }
        Ok(())
    }

    fn init_frame(&mut self, canvas: &mut Canvas<Window>, surface: FRect) -> Result<(), String> {
        let parent = unsafe { self.parent.as_mut().ok_or("unwrap ptr init frame grid")? };
        if self.last_width != surface.width() || self.last_height != surface.height() {
            let mut p_x = surface.x();
            let mut p_y = surface.y();
            let remain_width = surface.width() - self.static_x;
            let remain_height = surface.height() - self.static_y;
            if remain_width < 0. || remain_height < 0. {
                return Err(format!(
                    "Not enough space: requested {}x{} in a grid of {}x{}",
                    self.static_x,
                    self.static_y,
                    surface.width(),
                    surface.height()
                ));
            }

            for (y, pos_y) in self.rows.iter().enumerate() {
                let height = pos_y.to_px(remain_height);
                for (x, pos_x) in self.cols.iter().enumerate() {
                    let width = pos_x.to_px(remain_width);
                    if let Some(GridElement { surface, element }) =
                        self.elements.get_mut(&Pos { x, y })
                    {
                        *surface = FRect::new(p_x, p_y, width, height);
                        element.grid_init_frame(canvas, *surface, parent)?;
                    }
                    p_x += width;
                }
                p_x = surface.x();
                p_y += height;
            }
            self.last_width = surface.width();
            self.last_height = surface.height();
        } else {
            for GridElement { surface, element } in self.elements.values_mut() {
                element.grid_init_frame(canvas, *surface, parent)?;
            }
        }
        Ok(())
    }

    fn event(&mut self, canvas: &mut Canvas<Window>, event: Event) -> Result<(), String> {
        let parent = unsafe { self.parent.as_mut().ok_or("unwrap ptr event grid")? };
        for (_, GridElement { element, .. }) in self.elements.iter_mut() {
            element.grid_event(canvas, event.clone(), parent)?;
        }
        Ok(())
    }

    fn update(&mut self, canvas: &mut Canvas<Window>, elapsed: Duration) -> Result<(), String> {
        let parent = unsafe { self.parent.as_mut().ok_or("unwrap ptr update grid")? };
        for (_, GridElement { element, .. }) in self.elements.iter_mut() {
            element.grid_update(canvas, elapsed, parent)?;
        }
        Ok(())
    }

    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let parent = unsafe { self.parent.as_ref().ok_or("unwrap ptr draw grid")? };
        for (_, GridElement { element, .. }) in self.elements.iter() {
            element.grid_draw(canvas, parent)?;
        }
        Ok(())
    }
}

impl<T> Grid<T> {
    pub fn iter(&self) -> Vec<(&Pos, &dyn GridChildren<T>)> {
        self.elements
            .iter()
            .map(|(pos, GridElement { element, .. })| (pos, element.as_ref()))
            .collect()
    }

    // pub fn iter_mut(&mut self) -> Vec<(&Pos, &mut dyn UserControl)> {
    //     self.elements
    //         .iter_mut()
    //         .map(|(pos, GridElement { element, .. })| (pos, element.as_mut()))
    //         .collect()
    // }
}

#[macro_export]
macro_rules! simple_grid {
    ($self:ident, $type:ty, $($col:expr),*; $($row:expr),*; $($pos:expr => $child:expr),* $(,)?) => {
        Grid::new(
            $self,
            vec![$($col),*],
            vec![$($row),*],
            HashMap::from([$(($pos, Box::new($child) as Box<dyn GridChildren<$type>>)),*])
        )
    };
}

#[cfg(test)]
pub(crate) mod grid_test {
    use sdl2::mouse::MouseButton;

    use super::*;

    struct Button {
        surface: FRect,
    }

    impl GridChildren<usize> for Button {
        fn grid_init(&mut self, _: &mut Canvas<Window>, _: &mut usize) -> Result<(), String> {
            Ok(())
        }

        fn grid_init_frame(
            &mut self,
            _: &mut Canvas<Window>,
            surface: FRect,
            _: &mut usize,
        ) -> Result<(), String> {
            self.surface = surface;
            Ok(())
        }

        fn grid_event(
            &mut self,
            _: &mut Canvas<Window>,
            event: Event,
            counter: &mut usize,
        ) -> Result<(), String> {
            if let Event::MouseButtonDown { .. } = event {
                if event.hover(self.surface) {
                    *counter += 1;
                }
            }
            Ok(())
        }

        fn grid_update(
            &mut self,
            _: &mut Canvas<Window>,
            _: Duration,
            _: &mut usize,
        ) -> Result<(), String> {
            Ok(())
        }

        fn grid_draw(&self, _: &mut Canvas<Window>, _: &usize) -> Result<(), String> {
            Ok(())
        }
    }

    #[allow(clippy::too_many_lines)]
    pub(crate) fn test_grid_click(canvas: &mut Canvas<Window>) {
        let mut counter = 0;
        let c = &mut counter;
        let mut grid = simple_grid!(
            c,
            usize,
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
                    c,
                    usize,
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
                    Pos { x: 1, y: 1 } => Button {surface: FRect::new(0., 0., 0., 0.)},
                    Pos { x: 3, y: 1 } => Button {surface: FRect::new(0., 0., 0., 0.)},
                    Pos { x: 1, y: 3 } => Button {surface: FRect::new(0., 0., 0., 0.)},
                    Pos { x: 3, y: 3 } => Button {surface: FRect::new(0., 0., 0., 0.)}
                ),
            Pos { x: 3, y: 1 } => Button {surface: FRect::new(0., 0., 0., 0.)},
            Pos { x: 1, y: 3 } => Button {surface: FRect::new(0., 0., 0., 0.)},
            Pos { x: 3, y: 3 } => Button {surface: FRect::new(0., 0., 0., 0.)},
        );
        assert!(grid.init(canvas).is_ok());
        assert_eq!(
            grid.init_frame(canvas, FRect::new(0., 0., 50., 50.)),
            Ok(())
        );
        assert_eq!(counter, 0);
        click(&mut grid, canvas, 0., 0.);
        assert_eq!(counter, 0);
        click(&mut grid, canvas, 11., 11.);
        assert_eq!(counter, 0);
        click(&mut grid, canvas, 13., 13.);
        assert_eq!(counter, 1);
        click(&mut grid, canvas, 15., 15.);
        assert_eq!(counter, 1);
        click(&mut grid, canvas, 17., 17.);
        assert_eq!(counter, 2);
        click(&mut grid, canvas, 21., 21.);
        assert_eq!(counter, 2);
        click(&mut grid, canvas, 31., 11.);
        assert_eq!(counter, 3);
        click(&mut grid, canvas, 11., 31.);
        assert_eq!(counter, 4);
        click(&mut grid, canvas, 31., 31.);
        assert_eq!(counter, 5);
    }

    fn click(grid: &mut Grid<usize>, canvas: &mut Canvas<Window>, x: f32, y: f32) {
        assert!(grid
            .event(
                canvas,
                Event::MouseButtonDown {
                    which: 0,
                    mouse_btn: MouseButton::Left,
                    clicks: 1,
                    x,
                    y,
                }
            )
            .is_ok());
    }
}
