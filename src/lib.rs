extern crate sdl2;

pub mod event;
pub mod functions;
pub mod game_window;
pub mod grid;
pub mod missing;
pub mod panel;
pub mod ref_element;
pub mod scroll_view;
pub mod text_box;
pub mod ui_rect;
pub mod user_control;

use std::{thread, time};

use game_window::GameWindow;
use sdl2::pixels::Color;
use sdl2::render::{BlendMode, Canvas};
use sdl2::video::{Window, WindowBuilder};

pub fn run<
    Game: GameWindow,
    Func: Fn(&mut Canvas<Window>) -> Result<Game, String>,
    Func2: Fn(&mut WindowBuilder) -> &mut WindowBuilder,
>(
    title: &str,
    fps: f32,
    width: u32,
    height: u32,
    window: Func2,
    func: Func,
) -> Result<(), String> {
    let fps = time::Duration::from_secs_f32(1. / fps);

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // the window is the representation of a window in your operating system,
    // however you can only manipulate properties of that window, like its size, whether it's
    // fullscreen, ... but you cannot change its content without using a Canvas or using the
    // `surface()` method.
    let mut windowb = video_subsystem.window(title, width, height);

    // the canvas allows us to both manipulate the property of the window and to change its content
    // via hardware or software rendering. See CanvasBuilder for more info.
    let mut canvas = window(&mut windowb)
        .build()
        .map_err(|e| e.to_string())?
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    println!("Using SDL_Renderer \"{}\"", canvas.info().name);
    canvas.set_blend_mode(BlendMode::Blend);
    canvas.set_draw_color(Color::BLACK);
    // clears the canvas with the color we set in `set_draw_color`.
    canvas.clear();
    // However the canvas has not been updated to the window yet, everything has been processed to
    // an internal buffer, but if we want our buffer to be displayed on the window, we need to call
    // `present`. We need to call this everytime we want to render a new frame on the window.
    canvas.present();

    let mut game = func(&mut canvas)?;
    game.init(&mut canvas)?;

    let mut update_time = time::Instant::now();
    let mut event_pump = sdl_context.event_pump()?;
    while game.running() {
        let now = time::Instant::now();
        let (width, height) = canvas.window().size();
        let (width, height) = (width as f32, height as f32);

        game.init_frame(&mut canvas, width, height)?;

        for event in event_pump.poll_iter() {
            game.event(&mut canvas, event.into())?;
        }

        let now_update = time::Instant::now();
        let elapsed = now_update - update_time;
        update_time = now_update;
        game.update(&mut canvas, elapsed)?;
        game.draw(&mut canvas)?;
        canvas.present();

        let elapsed = time::Instant::now() - now;
        if elapsed < fps {
            thread::sleep(fps - elapsed);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{grid::grid_test::test_grid_click, panel::panel_test::test_panel_click};

    #[test]
    pub fn tests() {
        let sdl = sdl2::init();
        assert!(sdl.is_ok());
        let video = sdl.expect("Checked").video();
        assert!(video.is_ok());
        let window = video.expect("Checked").window("title", 50, 50).build();
        assert!(window.is_ok());
        let window = window.expect("Checked");
        let mut canvas = window.into_canvas().build();
        assert!(canvas.is_ok());
        let canvas = canvas.as_mut().expect("Checked");

        test_grid_click(canvas);
        test_panel_click(canvas);
    }
}
