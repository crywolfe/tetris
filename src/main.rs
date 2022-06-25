extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use sdl2::render::Texture;
use std::time::Duration;
use std::thread::sleep;

pub fn main() {
    println!("Hello, world!");
    let sdl_context = sdl2::init().expect("SDL initialization failed");
    let video_subsystem = sdl_context.video().expect("Couldn't get SLD video subsystem");

    // Parameters are title, width, height
    let window = video_subsystem.window("Tetris", 800, 600)
    .position_centered()
    .build()
    .expect("Failed to create window");

    let mut canvas = window.into_canvas()
        .target_texture()
        .present_vsync()    
        .build()
        .expect("Couldn't get window's canvas");

    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    const TEXTURE_SIZE: u32 = 32;

    let mut square_texture: Texture = texture_creator.create_texture_target(None, TEXTURE_SIZE, TEXTURE_SIZE).expect("Failed to create a texture");

    canvas.with_texture_canvas(&mut square_texture, |texture| {
        // green
        texture.set_draw_color(Color::RGB(0,255,0));
        texture.clear();
    }).expect("Failed to color a texture");
    

    let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump");

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(255,0,0));
        canvas.clear();
        // copy texture into window
        canvas.copy(&square_texture, None, Rect::new(0,0,TEXTURE_SIZE, TEXTURE_SIZE)).expect("Couldn't copy texture into window");
        // update widnow's display
        canvas.present();
        sleep(Duration::new(0, 1_000_000u32 /60));
    }


}
