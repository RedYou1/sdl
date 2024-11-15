use std::time::Duration;

use sdl2::{rect::FRect, render::Canvas, video::Window};

use crate::{event::Event, user_control::UserControl};

pub struct PanelChild {
    pub z_index: i8,
    pub element: Box<dyn UserControl>,
}

pub struct Panel {
    surface: FRect,
    subs: Vec<Box<dyn UserControl>>,
}

impl Panel {
    pub fn new(mut subs: Vec<PanelChild>) -> Self {
        subs.sort_by_key(|sub| sub.z_index);
        Self {
            surface: FRect::new(0., 0., 0., 0.),
            subs: subs.into_iter().map(|sub| sub.element).collect(),
        }
    }
}

impl UserControl for Panel {
    fn init(&mut self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        for sub in self.subs.iter_mut() {
            sub.init(canvas)?;
        }
        Ok(())
    }

    fn init_frame(&mut self, canvas: &mut Canvas<Window>, surface: FRect) -> Result<(), String> {
        self.surface = surface;
        for sub in self.subs.iter_mut() {
            sub.init_frame(canvas, surface)?;
        }
        Ok(())
    }

    fn event(&mut self, canvas: &mut Canvas<Window>, event: Event) -> Result<(), String> {
        for sub in self.subs.iter_mut() {
            sub.event(canvas, event.clone())?;
        }
        Ok(())
    }

    fn update(&mut self, canvas: &mut Canvas<Window>, elapsed: Duration) -> Result<(), String> {
        for sub in self.subs.iter_mut() {
            sub.update(canvas, elapsed)?;
        }
        Ok(())
    }

    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        for sub in self.subs.iter() {
            sub.draw(canvas)?;
        }
        Ok(())
    }
}

impl AsRef<[Box<dyn UserControl>]> for Panel {
    fn as_ref(&self) -> &[Box<dyn UserControl>] {
        &self.subs
    }
}

impl AsMut<[Box<dyn UserControl>]> for Panel {
    fn as_mut(&mut self) -> &mut [Box<dyn UserControl>] {
        &mut self.subs
    }
}

#[cfg(test)]
pub(crate) mod panel_test {
    use sdl2::mouse::MouseButton;

    use super::*;

    struct Button {
        counter: *mut usize,
        surface: FRect,
    }

    impl UserControl for Button {
        fn init(&mut self, _: &mut Canvas<Window>) -> Result<(), String> {
            Ok(())
        }

        fn init_frame(&mut self, _: &mut Canvas<Window>, _: FRect) -> Result<(), String> {
            Ok(())
        }

        fn event(&mut self, _: &mut Canvas<Window>, event: Event) -> Result<(), String> {
            if let Event::MouseButtonDown { .. } = event {
                if event.hover(self.surface) {
                    *(unsafe { self.counter.as_mut().ok_or("unwrap ptr panel test")? }) += 1;
                }
            }
            Ok(())
        }

        fn update(&mut self, _: &mut Canvas<Window>, _: Duration) -> Result<(), String> {
            Ok(())
        }

        fn draw(&self, _: &mut Canvas<Window>) -> Result<(), String> {
            Ok(())
        }
    }

    #[allow(clippy::too_many_lines)]
    pub(crate) fn test_panel_click(canvas: &mut Canvas<Window>) {
        let mut counter = 0;
        let c = &mut counter;
        let mut panel = Panel::new(vec![
            PanelChild {
                z_index: 0,
                element: Box::new(Button {
                    counter: c,
                    surface: FRect::new(12., 12., 2., 2.),
                }) as Box<dyn UserControl>,
            },
            PanelChild {
                z_index: 0,
                element: Box::new(Button {
                    counter: c,
                    surface: FRect::new(12., 16., 2., 2.),
                }) as Box<dyn UserControl>,
            },
            PanelChild {
                z_index: 0,
                element: Box::new(Button {
                    counter: c,
                    surface: FRect::new(16., 12., 2., 2.),
                }) as Box<dyn UserControl>,
            },
            PanelChild {
                z_index: 0,
                element: Box::new(Button {
                    counter: c,
                    surface: FRect::new(16., 16., 2., 2.),
                }) as Box<dyn UserControl>,
            },
            PanelChild {
                z_index: 0,
                element: Box::new(Button {
                    counter: c,
                    surface: FRect::new(10., 30., 10., 10.),
                }) as Box<dyn UserControl>,
            },
            PanelChild {
                z_index: 0,
                element: Box::new(Button {
                    counter: c,
                    surface: FRect::new(30., 10., 10., 10.),
                }) as Box<dyn UserControl>,
            },
            PanelChild {
                z_index: 0,
                element: Box::new(Button {
                    counter: c,
                    surface: FRect::new(30., 30., 10., 10.),
                }) as Box<dyn UserControl>,
            },
        ]);
        assert!(panel.init(canvas).is_ok());
        assert_eq!(
            panel.init_frame(canvas, FRect::new(0., 0., 50., 50.)),
            Ok(())
        );
        assert_eq!(counter, 0);
        click(&mut panel, canvas, 0., 0.);
        assert_eq!(counter, 0);
        click(&mut panel, canvas, 11., 11.);
        assert_eq!(counter, 0);
        click(&mut panel, canvas, 13., 13.);
        assert_eq!(counter, 1);
        click(&mut panel, canvas, 15., 15.);
        assert_eq!(counter, 1);
        click(&mut panel, canvas, 17., 17.);
        assert_eq!(counter, 2);
        click(&mut panel, canvas, 21., 21.);
        assert_eq!(counter, 2);
        click(&mut panel, canvas, 31., 11.);
        assert_eq!(counter, 3);
        click(&mut panel, canvas, 11., 31.);
        assert_eq!(counter, 4);
        click(&mut panel, canvas, 31., 31.);
        assert_eq!(counter, 5);
    }

    fn click(panel: &mut Panel, canvas: &mut Canvas<Window>, x: f32, y: f32) {
        assert!(panel
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
