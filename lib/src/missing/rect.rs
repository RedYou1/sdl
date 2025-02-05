use sdl2::rect::{FRect, Rect};

pub fn scale(surface: FRect, scale: FRect) -> FRect {
    FRect::new(
        scale.x() * surface.width() + surface.x(),
        scale.y() * surface.height() + surface.y(),
        scale.width() * surface.width(),
        scale.height() * surface.height(),
    )
}

pub fn as_rect(surface: FRect) -> Rect {
    Rect::new(
        surface.x() as i32,
        surface.y() as i32,
        surface.width() as u32,
        surface.height() as u32,
    )
}
