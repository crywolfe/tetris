extern crate sdl2;
extern crate rand;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::image::{LoadTexture, INIT_PNG, INIT_JPG};
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::TextureCreator;
use sdl2::render::Texture;
use sdl2::video::{Window, WindowContext};
use std::fs::File;
use std::io::{self, Write, Read};
use std::time::Duration;
use std::thread::sleep;
use std::time::SystemTime;

const TEXTURE_SIZE: u32 = 32;

#[derive(Clone, Copy)]
enum TextureColor {
    Green,
    Blue
}

type Piece = Vec<Vec<u8>>;

type States = Vec<Piece>;
struct Tetrimino {
    states: States,
    x: isize,
    y: usize,
    current_state: u8,
}

struct Tetris {
    game_map: Vec<Vec<u8>>,
    current_level: u32,
    score: u32,
    nb_lines: u32,
    current_piece: Option<Tetrimino>
}

trait TetriminoGenerator {
    fn new() -> Tetrimino;
    
}

struct TetriminoI;
struct TetriminoJ;
struct TetriminoL;
struct TetriminoO;
struct TetriminoS;
struct TetriminoT;
struct TetriminoZ;

impl TetriminoGenerator for TetriminoI {    
    fn new() -> Tetrimino {
        Tetrimino { states: vec![vec![vec![1,1,1,1],
                                      vec![0,0,0,0],
                                      vec![0,0,0,0],
                                      vec![0,0,0,0]],
                                 vec![vec![0,1,0,0],
                                      vec![0,1,0,0],
                                      vec![0,1,0,0],
                                      vec![0,1,0,0]]],
                    x: 4, 
                    y: 0, 
                    current_state: 0 }
    }
}

impl TetriminoGenerator for TetriminoJ {    
    fn new() -> Tetrimino {
        Tetrimino { states: vec![vec![vec![2,2,2,0],
                                      vec![2,0,0,0],
                                      vec![0,0,0,0],
                                      vec![0,0,0,0]],
                                 vec![vec![2,2,0,0],
                                      vec![0,2,0,0],
                                      vec![0,2,0,0],
                                      vec![0,0,0,0]],
                                 vec![vec![2,0,0,0],
                                      vec![2,0,0,0],
                                      vec![2,2,0,0],
                                      vec![0,0,0,0]]],
                    x: 4, 
                    y: 0, 
                    current_state: 0 }
    }
}

impl TetriminoGenerator for TetriminoL {
    fn new() -> Tetrimino {
        Tetrimino { states: vec![vec![vec![3,3,3,0],
                                      vec![0,0,3,0],
                                      vec![0,0,0,0],
                                      vec![0,0,0,0]],
                                 vec![vec![0,3,0,0],
                                      vec![0,3,0,0],
                                      vec![3,3,0,0],
                                      vec![0,0,0,0]],
                                 vec![vec![3,0,0,0],
                                      vec![3,3,3,0],
                                      vec![0,0,0,0],
                                      vec![0,0,0,0]],
                                 vec![vec![3,3,0,0],
                                      vec![3,0,0,0],
                                      vec![3,0,0,0],
                                      vec![0,0,0,0]],                                     
                                      ],
                    x: 4, 
                    y: 0, 
                    current_state: 0 }
    }
}

impl TetriminoGenerator for TetriminoO {    
    fn new() -> Tetrimino {
        Tetrimino { states: vec![vec![vec![4,4,0,0],
                                      vec![4,4,0,0],
                                      vec![0,0,0,0],
                                      vec![0,0,0,0]],
                                 ],
                    x: 5, 
                    y: 0, 
                    current_state: 0 }
    }
}

impl TetriminoGenerator for TetriminoS {    
    fn new() -> Tetrimino {
        Tetrimino { states: vec![vec![vec![0,5,5,0],
                                      vec![5,5,0,0],
                                      vec![0,0,0,0],
                                      vec![0,0,0,0]],
                                 vec![vec![0,5,0,0],
                                      vec![0,5,5,0],
                                      vec![0,0,5,0],
                                      vec![0,0,0,0]],
                                 ],
                    x: 4, 
                    y: 0, 
                    current_state: 0 }
    }
}

impl TetriminoGenerator for TetriminoZ {    
    fn new() -> Tetrimino {
        Tetrimino { states: vec![vec![vec![6,6,0,0],
                                      vec![0,6,6,0],
                                      vec![0,0,0,0],
                                      vec![0,0,0,0]],
                                 vec![vec![0,0,6,0],
                                      vec![0,6,6,0],
                                      vec![0,6,0,0],
                                      vec![0,0,0,0]],
                                 ],
                    x: 4, 
                    y: 0, 
                    current_state: 0 }
    }
}

impl TetriminoGenerator for TetriminoT {    
    fn new() -> Tetrimino {
        Tetrimino { states: vec![vec![vec![7,7,7,0],
                                      vec![0,7,0,0],
                                      vec![0,0,0,0],
                                      vec![0,0,0,0]],
                                 vec![vec![0,7,0,0],
                                      vec![7,7,0,0],
                                      vec![0,7,0,0],
                                      vec![0,0,0,0]],
                                 vec![vec![0,7,0,0],
                                      vec![7,7,7,0],
                                      vec![0,0,0,0],
                                      vec![0,0,0,0]],
                                 vec![vec![0,7,0,0],
                                      vec![0,7,7,0],
                                      vec![0,7,0,0],
                                      vec![0,0,0,0]],
                                 ],
                    x: 4, 
                    y: 0, 
                    current_state: 0 }
    }
}

impl Tetrimino {
    fn rotate(&mut self, game_map: &[Vec<u8>]) {
        let mut tmp_state = self.current_state + 1;
        if tmp_state as usize >= self.states.len() {
            tmp_state = 0;
        }
        
        self.current_state += 1;
        if self.current_state as usize >= self.states.len() {
            self.current_state = 0;
        }
    }

    fn test_position(&self, game_map: &[Vec<u8>], tmp_state: usize, x: isize, y: usize) -> bool {
        for decal_y in 0..4 {
            for decal_x in 0..4 {
                let x = x + decal_x;
                if self.states[tmp_state][decal_y][decal_x as usize] != 0 &&
                    (y + decal_y >= game_map.len() ||
                        x < 0 ||
                        x as usize >= game_map[y+decal_y].len() ||
                        game_map[y+decal_y][x as usize] != 0) {
                            return false;
                        }

            }
        }
        return true;
    }

}

fn read_from_file(file_name: &str) -> io::Result<String> {
    let mut f = File::open(file_name)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}

fn write_into_file(content: &str, file_name: &str) -> io::Result<()> {
    let mut f = File::create(file_name)?;
    f.write_all(content.as_bytes())
}

fn slice_to_string(slice: &[u32]) -> String {
    slice.iter().map(|highscore| highscore.to_string()).collect::<Vec<String>>().join(" ")
}

fn save_highscores_and_lines(highscores: &[u32], number_of_lines: &[u32]) -> bool {

    let s_highscores = slice_to_string(highscores);
    let s_number_of_lines = slice_to_string(number_of_lines);
    write_into_file(&format!("{}\n{}\n", s_highscores, s_number_of_lines), "scores.txt").is_ok()

}

fn line_to_slice(line: &str) -> Vec<u32> {
    line.split(" ").filter_map(|nb|nb.parse::<u32>().ok()).collect()
}

fn load_highscores_and_lines() -> Option<(Vec<u32>, Vec<u32>)> {
    if let Ok(content) = read_from_file("scores.txt") {
        let mut lines = content.splitn(2, "\n").map(|line| line_to_slice(line)).collect::<Vec<_>>();
        if lines.len() == 2 {
            let (number_lines, highscores) = (lines.pop().unwrap(), lines.pop().unwrap());
            Some((highscores, number_lines))
        } else {
            None
        }
    } else {
        None
    }
}

fn create_new_tetrimino() -> Tetrimino {
    static mut PREV: u8 = 7;
    let mut rand_nb = rand::random::<u8>() % 7;
    if unsafe { PREV } == rand_nb {
        rand_nb = rand::random::<u8>() % 7;
    }
    unsafe { PREV = rand_nb; }
    match rand_nb {
        0 => TetriminoI::new(),
        1 => TetriminoJ::new(),
        2 => TetriminoL::new(),
        3 => TetriminoO::new(),
        4 => TetriminoS::new(),
        5 => TetriminoZ::new(),
        6 => TetriminoT::new(),
        _ => unreachable!(),
    }
}

fn change_position(&mut self, game_map: &[Vec<u8>], new_x: isize, new_y: usize) -> bool {

    if self.test_position(game_map, self.current_state as usize, new_x, new_y) == true {
        self.x = new_x as isize;
        self.y = new_y;
        true
    } else {
        false
    }
}

fn test_current_position(&self, game_map: &[Vec<u8>]) -> bool {
    self.test_position(game_map, self.current_state as usize, self.x, self.y)
}

pub fn main() {
    let sdl_context = sdl2::init().expect("SDL initialization failed");
    let video_subsystem = sdl_context.video().expect("Couldn't get SLD video subsystem");

    sdl2::image::init(INIT_PNG | INIT_JPG).expect("Couldn't initialize image context");

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

    let green_square = create_texture_rect(&mut canvas, &texture_creator, TextureColor::Green, TEXTURE_SIZE).expect("Failed to create a texture");
    let blue_square = create_texture_rect(&mut canvas, &texture_creator, TextureColor::Blue, TEXTURE_SIZE).expect("Failed to create a texture");

    let timer = SystemTime::now();

    // get the event handler
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
        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.clear();

        // rectangle switch
        let display_green = match timer.elapsed() {
            Ok(elapsed) => elapsed.as_secs() % 2 == 0,
            Err(_) => {
                true
            }
        };
        let square_texture = if display_green {
            &green_square
        } else {
            &blue_square
        };
        canvas.copy(square_texture, None, Rect::new(0,0, TEXTURE_SIZE, TEXTURE_SIZE)).expect("Couldn't copy texture into window");
        // update window's display
        canvas.present();

        sleep(Duration::new(0, 1_000_000u32 /60));
    }
}

fn create_texture_rect<'a>(
    canvas: &mut Canvas<Window>, 
    texture_creator: &'a TextureCreator<WindowContext>,
    color: TextureColor, size: u32) -> Option<Texture<'a>> {
        if let Ok(mut square_texture) = texture_creator.create_texture_target(None, size, size) {
            canvas.with_texture_canvas(&mut square_texture, |texture| {
                match color {
                    TextureColor::Green => texture.set_draw_color(Color::RGB(0, 255,0)),
                    TextureColor::Blue => texture.set_draw_color(Color::RGB(0, 0, 255)),
                }
                texture.clear();
            }).expect("Failed to color a texture");
            Some(square_texture)
        } else {
            None
        }

    }
