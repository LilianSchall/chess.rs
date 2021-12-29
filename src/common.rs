
pub mod Misc {
    pub fn islowercase(car: char) -> bool {
        let tmp = car.to_lowercase().last().unwrap() == car;
        
        tmp
    }

    pub fn to_digit (car: char) -> Option<u8> {
        match car {
            '0'..='9' => Some(car as u8 - '0' as u8),
            _ => None
        }
    }

    pub fn min (x: isize, y: isize) -> isize {
        (x + y - abs(x - y)) / 2
    }
    pub fn abs (x: isize) -> isize{
        if x < 0 {
            return -x;
        }
        x
    }
}

pub mod MoveData {
    use crate::common::Misc::min;
    
    //first 4 digits, move for column and line movements
    //last 4 digits, move for diagonals movements
    pub const DIRECTION_OFFSET: [i8; 8] = [-8,8, -1, 1, -9, 9, -7, 7];

    pub fn precomputed_move_data () -> [[i8;8]; 64] {
        let mut data: [[i8; 8]; 64] = [[0;8]; 64];

        for height in 0..8 {
            for width in 0 .. 8 {
                let num_north: i8 = height;
                let num_south: i8 = 7 - height;
                let num_west: i8 = width;
                let num_east: i8 = 7 - width;

                let square_index: usize = height as usize * 8 + width as usize;

                data[square_index] = [
                    num_north,
                    num_south,
                    num_west,
                    num_east,
                    min (num_north as isize, num_west as isize) as i8,
                    min (num_south as isize, num_east as isize) as i8,
                    min (num_north as isize, num_east as isize) as i8,
                    min (num_south as isize, num_west as isize) as i8
                ];
            }
        }
        data
    }
}

pub mod CanvasDisplay {
    use sdl2::rect::Rect;
    use sdl2::render::{WindowCanvas, Texture};

    pub fn canvas_fill(canvas: &mut WindowCanvas, rect: Rect) {
        match canvas.fill_rect(rect) {
            Ok(_) => {},
            Err(msg) => {println!("Error: {}", msg)}
        }
    }
    pub fn canvas_copy(canvas: &mut WindowCanvas, 
                       texture: &Texture,
                       rect1: Option<Rect>, rect2: Option<Rect>) {
        match canvas.copy(texture, rect1, rect2) {
            Ok(_) => {},
            Err(msg) => {println!("Error: {}", msg)}
        }

    }
}


