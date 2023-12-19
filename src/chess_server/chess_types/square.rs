

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Square {
    board_index: u8
}

impl Square {

    #[inline(always)]
    pub unsafe fn from_coordinates_unchecked(row: i8, col: i8) -> Self {
        let board_index = (row * 8 + col) as u8;
        Square { board_index }
    }

    #[inline(always)]
    pub unsafe fn from_index_unchecked(index: i8) -> Self {
        Square { board_index: index as u8 }
    }

    pub fn from_coordinates(row: i8, col: i8) -> Option<Self> {
        if (0..8).contains(&row) && (0..8).contains(&col) 
        {
            unsafe {
                Some(Self::from_coordinates_unchecked(row, col))
            }
        } else {    
            None
        }
    }

    pub fn from_index(index: i8) -> Option<Self> {
        if (0..64).contains(&index) {
            unsafe {
                Some(Self::from_index_unchecked(index))
            }
        } else {
            None
        }
    }

    #[inline(always)]
    pub const fn get_coordinates(&self) -> (u8, u8) {
        (self.board_index / 8, self.board_index % 8)
    }

    #[inline(always)]
    pub const fn get_index(&self) -> u8 {
        self.board_index
    }

    unsafe fn add_unchecked(&self, d_row: i8, d_col: i8) -> Self {
        let (row, col) = self.get_coordinates();
        Self::from_coordinates_unchecked(row as i8 + d_row, col as i8 + d_col)
    }

    pub fn add(&self, d_row: i8, d_col: i8) -> Option<Self> {
        let (row, col) = self.get_coordinates();
        
        let new_row = (row as i16) + (d_row as i16);
        let new_col = (col as i16) + (d_col as i16);

        if (0..8).contains(&new_row) && (0..8).contains(&new_col) {
            unsafe {
                Some(Self::from_coordinates_unchecked(new_row as i8, new_col as i8))
            }
        } else{
            None
        }
    }

    pub fn from_chess_notation(coordinate: [char; 2]) -> Option<Square> {
        let col = match coordinate[0] {
            'a' | 'A' => 0,
            'b' | 'B' => 1,
            'c' | 'C' => 2,
            'd' | 'D' => 3,
            'e' | 'E' => 4,
            'f' | 'F' => 5,
            'g' | 'G' => 6,
            'h' | 'H' => 7,
            _ => return None
        };

        let row = match coordinate[1] {
            '1' => 7,
            '2' => 6,
            '3' => 5,
            '4' => 4,
            '5' => 3,
            '6' => 2,
            '7' => 1,
            '8' => 0,
            _ => return None
        };

        Square::from_coordinates(row, col)
    }

    pub fn to_str(&self) -> String {
        let (row, col) = self.get_coordinates();

        let char_row = match row {
            0 => '8',
            1 => '7',
            2 => '6',
            3 => '5',
            4 => '4',
            5 => '3',
            6 => '2',
            7 => '1',
            _ => unreachable!("Panic => Square {}, {}", row, col)
        };

        let char_col = match col {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => unreachable!("Panic => Square {}, {}", row, col)
        };
    
        char_col.to_string() + &char_row.to_string()    
    }

}