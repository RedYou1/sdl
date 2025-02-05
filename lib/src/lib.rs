#![feature(ptr_as_ref_unchecked)]
extern crate sdl2;

pub mod event;
pub mod functions;
pub mod missing;
pub mod refs;
pub mod state_manager;
pub mod ui_element;
pub mod user_control;

use std::thread;
use std::time::{Duration, Instant};

use anyhow::{anyhow, Result};
use event::Event;
use refs::MutRef;
use sdl2::pixels::Color;
use sdl2::rect::FRect;
use sdl2::render::{BlendMode, Canvas};
use sdl2::sys::SDL_FRect;
use sdl2::video::{Window, WindowBuilder};
use user_control::{EventWindow, GameWindow};

pub fn zero() -> FRect {
    FRect::from(SDL_FRect {
        x: 0.,
        y: 0.,
        w: 0.,
        h: 0.,
    })
}

fn init(
    title: &str,
    width: u32,
    height: u32,
    window: impl FnOnce(&mut WindowBuilder) -> &mut WindowBuilder,
) -> Result<(sdl2::Sdl, Canvas<Window>)> {
    let sdl_context = sdl2::init().map_err(|e| anyhow!(e))?;
    let video_subsystem = sdl_context.video().map_err(|e| anyhow!(e))?;
    let mut windowb = video_subsystem.window(title, width, height);
    let mut canvas = window(&mut windowb)
        .build()
        .map_err(|e| anyhow!(e))?
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .map_err(|e| anyhow!(e))?;
    println!("Using SDL_Renderer \"{}\"", canvas.info().name);
    canvas.set_blend_mode(BlendMode::Blend);
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();
    Ok((sdl_context, canvas))
}

pub fn run_event<State: 'static, Game: EventWindow<State> + 'static>(
    title: &str,
    width: u32,
    height: u32,
    window: impl FnOnce(&mut WindowBuilder) -> &mut WindowBuilder,
    state_func: impl FnOnce(&mut Canvas<Window>) -> Result<State>,
    func: impl FnOnce(&mut Canvas<Window>, MutRef<State>) -> Result<Game>,
) -> Result<()> {
    let (sdl_context, mut canvas) = init(title, width, height, window)?;

    let (last_x, last_y) = canvas.window().position();
    let (mut last_x, mut last_y) = (last_x as f32, last_y as f32);
    let (last_width, last_height) = canvas.window().size();
    let (mut last_width, mut last_height) = (last_width as f32, last_height as f32);

    let mut parent = ();
    let parent = MutRef::new(&mut parent);
    let mut state = state_func(&mut canvas)?;
    let state = MutRef::new(&mut state);
    let mut game = func(&mut canvas, state)?;
    let game = MutRef::new(&mut game);
    Game::event(
        game,
        &canvas,
        Event::ElementMove {
            x: last_x,
            y: last_y,
        },
        parent,
        state,
    )?;
    Game::event(
        game,
        &canvas,
        Event::ElementResize {
            width: last_width,
            height: last_height,
        },
        parent,
        state,
    )?;

    let mut event_pump = sdl_context.event_pump().map_err(|e| anyhow!(e))?;
    loop {
        let mut a = false;
        loop {
            for event in event_pump.poll_iter() {
                Game::event(game, &canvas, event.into(), parent, state)?;
                a = true;
            }

            let (x, y) = canvas.window().position();
            let (x, y) = (x as f32, y as f32);
            if last_x != x || last_y != y {
                Game::event(game, &canvas, Event::ElementMove { x, y }, parent, state)?;
                last_x = x;
                last_y = y;
                a = true;
            }
            let (width, height) = canvas.window().size();
            let (width, height) = (width as f32, height as f32);
            if last_width != width || last_height != height {
                Game::event(
                    game,
                    &canvas,
                    Event::ElementResize { width, height },
                    parent,
                    state,
                )?;
                last_width = width;
                last_height = height;
                a = true;
            }
            if a {
                break;
            }
            thread::sleep(Duration::from_millis(100));
        }

        if !Game::running(game.into(), state.into()) {
            break;
        }

        Game::update(game, &canvas, Duration::ZERO, parent, state)?;
        Game::draw(game.into(), &mut canvas, parent.into(), state.into())?;
        canvas.present();
    }

    Ok(())
}

pub fn run_game<State: 'static, Game: GameWindow<State> + 'static>(
    title: &str,
    width: u32,
    height: u32,
    window: impl FnOnce(&mut WindowBuilder) -> &mut WindowBuilder,
    state_func: impl FnOnce(&mut Canvas<Window>) -> Result<State>,
    func: impl FnOnce(&mut Canvas<Window>, MutRef<State>) -> Result<Game>,
) -> Result<()> {
    let (sdl_context, mut canvas) = init(title, width, height, window)?;

    let (last_x, last_y) = canvas.window().position();
    let (mut last_x, mut last_y) = (last_x as f32, last_y as f32);
    let (last_width, last_height) = canvas.window().size();
    let (mut last_width, mut last_height) = (last_width as f32, last_height as f32);

    let mut parent = ();
    let parent = MutRef::new(&mut parent);
    let mut state = state_func(&mut canvas)?;
    let state = MutRef::new(&mut state);
    let mut game = func(&mut canvas, state)?;
    let game = MutRef::new(&mut game);
    Game::event(
        game,
        &canvas,
        Event::ElementMove {
            x: last_x,
            y: last_y,
        },
        parent,
        state,
    )?;
    Game::event(
        game,
        &canvas,
        Event::ElementResize {
            width: last_width,
            height: last_height,
        },
        parent,
        state,
    )?;

    let mut last_time = Instant::now();

    let mut event_pump = sdl_context.event_pump().map_err(|e| anyhow!(e))?;
    while Game::running(game.into(), state.into()) {
        let current_time = Instant::now();
        let elapsed = current_time - last_time;
        last_time = current_time;

        for event in event_pump.poll_iter() {
            Game::event(game, &canvas, event.into(), parent, state)?;
        }

        let (x, y) = canvas.window().position();
        let (x, y) = (x as f32, y as f32);
        if last_x != x || last_y != y {
            Game::event(game, &canvas, Event::ElementMove { x, y }, parent, state)?;
            last_x = x;
            last_y = y;
        }
        let (width, height) = canvas.window().size();
        let (width, height) = (width as f32, height as f32);
        if last_width != width || last_height != height {
            Game::event(
                game,
                &canvas,
                Event::ElementResize { width, height },
                parent,
                state,
            )?;
            last_width = width;
            last_height = height;
        }

        let ts = Game::time_scale(game.into(), state.into());
        Game::update(game, &canvas, elapsed.mul_f32(ts), parent, state)?;
        Game::draw(game.into(), &mut canvas, parent.into(), state.into())?;
        canvas.present();

        let elapsed = Instant::now() - current_time;
        if elapsed < Game::fps_duration(game.into(), state.into()) {
            thread::sleep(Game::fps_duration(game.into(), state.into()) - elapsed);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        refs::MutRef,
        ui_element::{grid::grid_test::test_grid_click, panel::panel_test::test_panel_click},
    };

    #[test]
    pub fn tests() {
        let mut a = ();
        let n = MutRef::new(&mut a);
        assert_eq!((&mut a) as *mut (), n.clone().as_mut() as *mut ());

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
