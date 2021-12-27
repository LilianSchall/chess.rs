use super::board::Board;
use super::piece::Piece;
use super::piece::PColor;

use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas, TextureCreator};
use sdl2::rect::Rect;
use sdl2::video::WindowContext;
use sdl2::mixer::Music;

use std::collections::HashMap;

pub struct Game<'a> {
    pub board: Board<'a>,
    
    pub current_player: PColor,
    
    pub piece_hold: Option<Piece>,
    pub x: i32,
    pub y: i32,
    
    pub last_move: Option<((i32,i32),(i32,i32))>,
    
    pub sounds: HashMap<String, Music<'a>>
}

impl Game<'_> {
    pub fn new<'a>(renderer: &'a TextureCreator<WindowContext>) -> Game<'a> {
        let mut board = Board::new(renderer);
        board.init();

        let mut sounds: HashMap<String, Music> = HashMap::new();
        sounds.insert(String::from("castling"), 
                      Music::from_file("sound/castling.mp3").unwrap());
        sounds.insert(String::from("check"),
                      Music::from_file("sound/check.mp3").unwrap());
        sounds.insert(String::from("move"),
                      Music::from_file("sound/placement.mp3").unwrap());
        sounds.insert(String::from("starting_game"),
                      Music::from_file("sound/starting_game.mp3").unwrap());
        sounds.insert(String::from("take"),
                      Music::from_file("sound/taking.mp3").unwrap());
        sounds.insert(String::from("game_over"),
                      Music::from_file("sound/game_over.mp3").unwrap());
        
        sounds.get("starting_game").unwrap().play(1);

        Game {
            board: board,
            current_player: PColor::WHITE,
            piece_hold: None,
            x: -1,
            y: -1,
            last_move: None,
            sounds: sounds
        }
    }

    pub fn select_piece(&mut self, x: i32, y: i32, width: u32, height: u32) {
        let i: usize = self.board.size * y as usize / height as usize;
        let j: usize = self.board.size * x as usize / width as usize;
        
        let selected = self.board.get(i,j);
        println!("found coordinate: ({},{})", j,i);
        match selected{
            None => {},
            Some(p) => {
                if p.color == self.current_player {
                    self.piece_hold = selected;
                    self.x = j as i32;
                    self.y = i as i32;
                    self.board.set(i,j,None);
                }
            }
        }
    }

    pub fn make_move(&mut self, x: i32, y: i32, width: u32, height: u32) {
        if self.piece_hold == None {
            return;
        }
        let i: usize = self.board.size * y as usize / height as usize;
        let j: usize = self.board.size * x as usize / width as usize;
        
        let selected = self.board.get(i,j);
        let mut move_made: bool = false;
        match selected {
            None => {
                self.board.set(i,j,self.piece_hold);
                move_made = self.x as usize != j ||  self.y as usize != i;
                if (move_made) {
                                self.sounds.get("move").unwrap().play(1);
                }
            },
            Some(p) => {
                if (p.color != self.current_player) {
                    self.board.set(i,j,self.piece_hold);
                    self.sounds.get("take").unwrap().play(1);
                    move_made = true;
                }
                else {
                    self.board.set(self.y as usize, self.x as usize, self.piece_hold);
                }
            }
        }
        if move_made {
            self.switch_player();
            self.last_move = Some(((self.x,self.y),(j as i32,i as i32)));
        }
        self.piece_hold = None;
        self.x = -1;
        self.y = -1;

    }

    pub fn draw(&self, canvas: &mut WindowCanvas, width: i32, height: i32,
                mouse_x: i32, mouse_y: i32) {
        self.board.draw_board(canvas, width, height);
        self.draw_last_move(canvas, width, height);
        self.board.draw_pieces(canvas, width, height);
        self.draw_hold(canvas, width, height,
                       mouse_x, mouse_y);
    }

    // --------------------------------------------------
    // --------------- PRIVATE FUNCTIONS ----------------
    // --------------------------------------------------
    
    fn switch_player(&mut self) {
        self.current_player = if (self.current_player == PColor::WHITE) 
            {PColor::BLACK} else {PColor::WHITE};

    }


    fn draw_hold(&self, canvas: &mut WindowCanvas, width: i32, height: i32,
                 mouse_x: i32, mouse_y: i32) {
        match self.piece_hold {
            None => {},
            Some(p) => {
                let case_height: i32 = height / self.board.size as i32;
                let case_width: i32 = width / self.board.size as i32;

                let rect = Rect::new(mouse_x - case_width / 2,
                                     mouse_y - case_height / 2,
                                     case_width as u32,
                                     case_height as u32);

                match p.color {
                            PColor::WHITE => {
                                canvas.copy(self.board.piece_textures.white_textures.get(&p.r#type).unwrap(),
                                    None, Some(rect));
                            },
                            PColor::BLACK => {
                                canvas.copy(self.board.piece_textures.black_textures.get(&p.r#type).unwrap(),
                                    None, Some(rect));

                            }
                }

            }
        }
    }

    fn draw_last_move(&self, canvas: &mut WindowCanvas, width: i32, height: i32) {
        match self.last_move {
            None => {},
            Some(((x1,y1),(x2,y2))) => {
                let case_height: i32 = height / self.board.size as i32;
                let case_width: i32 = width / self.board.size as i32;
                

                canvas.set_draw_color(Color::RGBA(0, 255, 0, 30));
                canvas.fill_rect(Rect::new(x1 * case_width,
                                           y1 * case_height,
                                           case_width as u32,
                                           case_height as u32));
                
                canvas.set_draw_color(Color::RGBA(255, 255, 0, 30));
                canvas.fill_rect(Rect::new(x2 * case_width,
                                           y2 * case_height,
                                           case_width as u32,
                                           case_height as u32));

            }
        }
    }
}
