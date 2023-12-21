use glutin_window;
use graphics;
use opengl_graphics::{self, GlyphCache};
use piston;
use find_folder;

use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston_window::*;

use crate::chess_move::ChessMove;
use crate::colored_piece_type::ColoredPieceType;
use crate::game::Game;
use crate::square::Square;
use crate::zoberist_hash;

pub struct App {
    opengl: OpenGL,
    window: PistonWindow,
    textures: Vec<G2dTexture>,
    
}

const SIDE_LENGTH: u32 = 400;

impl App {
    pub fn new() -> Self {
        let opengl = OpenGL::V3_2;
        let mut window: PistonWindow =
            WindowSettings::new("Chess stuff", [SIDE_LENGTH, SIDE_LENGTH])
            .exit_on_esc(true)
            .graphics_api(opengl)
            .build()
            .unwrap();

        let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("textures").unwrap();

        const FILE_NAMES: [&str; 12] = [
            "wP.png", "bP.png", 
            "wN.png", "bN.png", 
            "wB.png", "bB.png", 
            "wR.png", "bR.png", 
            "wQ.png", "bQ.png", 
            "wK.png", "bK.png"];

        let mut textures = Vec::new();

        for i in 0..FILE_NAMES.len() {
            let image = assets.join(FILE_NAMES[i]);
            let texture: G2dTexture = Texture::from_path(
                &mut window.create_texture_context(),
                &image,
                Flip::None,
                &TextureSettings::new()
            ).unwrap();

            textures.push(texture);
        }


        
        
        return App { opengl, window, textures };
    }
    
    pub fn render_board(&mut self, type_field: &[ColoredPieceType; 64], lm: ChessMove) -> bool {
        use graphics::*;
        
        const LIGHT_SQUARE: [f32; 4] = [240.0 / 255.0, 217.0 / 255.0, 181.0 / 255.0, 1.0];
        const DARK_SQUARE: [f32; 4] = [181.0 / 255.0, 136.0 / 255.0, 99.0 / 255.0, 1.0];
        const LIGHT_MOVE_SQUARE: [f32; 4] = [205.0 / 255.0, 210.0 / 255.0, 106.0 / 255.0, 1.0];
        const DARK_MOVE_SQUARE: [f32; 4] = [170.0 / 255.0, 162.0 / 255.0, 58.0 / 255.0, 1.0];
        const FILE_NAMES: [&str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];
        
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("textures").unwrap();
        
        //let mut glyph_cache = GlyphCache::new("C:\\Users\\hmart\\Documents\\GitHub\\Chess-Challenge\\Rust\\barschbot\\textures\\FiraSans-Regular.ttf", (), TextureSettings::new()).unwrap();

        let side_length = self.window.size().width / 8.0;
        let square = rectangle::square(0.0, 0.0, side_length);
        if let Some(e) = self.window.next()  {

        self.window.draw_2d(&e, |c, g, _| {
            clear(DARK_SQUARE, g);
            
            for x in 0..8 {
                for y in 0..8 {
                    let transform = c
                        .transform
                        .trans(x as f64 * side_length, (7 - y)  as f64 * side_length);

                    if (x + y) % 2 == 1 {
                        rectangle(LIGHT_SQUARE, square, transform, g);
                    } 

                    if Square::from_u8(x + y * 8) == lm.start_square || 
                        Square::from_u8(x + y * 8) == lm.target_square {
                        if (x + y) % 2 == 1 {
                            rectangle(LIGHT_MOVE_SQUARE, square, transform, g);
                        }
                        else {
                            rectangle(DARK_MOVE_SQUARE, square, transform, g);
                        }
                    } 

                    let tp = type_field[(x + y * 8) as usize];
                    if tp != ColoredPieceType::None {
                            let texture = &self.textures[(tp as usize)];
                            image(texture, 
                                transform.scale(side_length / texture.get_width() as f64, side_length /texture.get_height() as f64), 
                                g);
                        }   
                    }
                }

                
                //for i in 0..8 {
                //    let transform = c.transform
                //    .trans(i as f64 * side_length, 7 as f64 * side_length);
                //    
                //    text::Text::new_color([1.0, 1.0, 1.0, 1.0], 20)
                //    .draw(
                //        FILE_NAMES[i as usize],
                //        &mut glyph_cache,
                //        &c.draw_state,
                //        transform,
                //        g,
                //    )
                //    .unwrap();
                //}
            });


            

            return true;    
        }

        return false;
        
    }

    pub fn read_move(&mut self) -> (Square, Square) {
        let mut start_square = Square::None;
        let mut target_square = Square::None;

        let mut location = [0.0_f64; 2];

        while let Some(event) = self.window.next() {

            if let Some(button) = event.press_args() {
                if button == Button::Mouse(MouseButton::Left) {
                    break;
                }
            };

            if let Some(pos) = event.mouse_cursor_args() {
                location = pos;
            }
        }

        

        let mut x = (location[0] / (SIDE_LENGTH as f64) * 8.0) as i32;
        let mut y = 7 - (location[1] / (SIDE_LENGTH as f64) * 8.0) as i32;
        start_square = Square::from_u8((x + y * 8) as u8);

        while let Some(event) = self.window.next() {

            if let Some(button) = event.release_args() {
                if button == Button::Mouse(MouseButton::Left) {
                    break;
                }
            };

            if let Some(pos) = event.mouse_cursor_args() {
                location = pos;
            }
        }

        let mut x = (location[0] / (SIDE_LENGTH as f64) * 8.0) as i32;
        let mut y = 7 - (location[1] / (SIDE_LENGTH as f64) * 8.0) as i32;
        target_square = Square::from_u8((x + y * 8) as u8);

        start_square.print();
        target_square.print();
        println!();
        return (start_square, target_square);
    }
}