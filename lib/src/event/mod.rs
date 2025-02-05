use sdl2::rect::{FPoint, FRect};

mod _enum;
mod from;

pub use _enum::Event;

impl Event {
    pub fn hover(&self, sub: FRect) -> bool {
        match self {
            &Self::MouseMotion { x, y, .. }
            | &Self::MouseButtonDown { x, y, .. }
            | &Self::MouseButtonUp { x, y, .. }
            | &Self::MouseWheel {
                mouse_x: x,
                mouse_y: y,
                ..
            } => sub.contains_point(FPoint::new(x, y)),
            _ => true,
        }
    }
}
