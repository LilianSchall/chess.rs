
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
                let numNorth: i8 = height;
                let numSouth: i8 = 7 - height;
                let numWest: i8 = width;
                let numEast: i8 = 7 - width;

                let squareIndex: usize = height as usize * 8 + width as usize;

                data[squareIndex] = [
                    numNorth,
                    numSouth,
                    numWest,
                    numEast,
                    min (numNorth as isize, numWest as isize) as i8,
                    min (numSouth as isize, numEast as isize) as i8,
                    min (numNorth as isize, numEast as isize) as i8,
                    min (numSouth as isize, numWest as isize) as i8
                ];
            }
        }
        data
    }
}


