#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1, 
    A2, B2, C2, D2, E2, F2, G2, H2, 
    A3, B3, C3, D3, E3, F3, G3, H3, 
    A4, B4, C4, D4, E4, F4, G4, H4, 
    A5, B5, C5, D5, E5, F5, G5, H5, 
    A6, B6, C6, D6, E6, F6, G6, H6, 
    A7, B7, C7, D7, E7, F7, G7, H7, 
    A8, B8, C8, D8, E8, F8, G8, H8, 
    None,
}


const ARRAY: [Square; 65] = [
    Square::A1,
    Square::B1,
    Square::C1,
    Square::D1,
    Square::E1,
    Square::F1,
    Square::G1,
    Square::H1,
    
    Square::A2,
    Square::B2,
    Square::C2,
    Square::D2,
    Square::E2,
    Square::F2,
    Square::G2,
    Square::H2,
    
    Square::A3,
    Square::B3,
    Square::C3,
    Square::D3,
    Square::E3,
    Square::F3,
    Square::G3,
    Square::H3,
    
    Square::A4,
    Square::B4,
    Square::C4,
    Square::D4,
    Square::E4,
    Square::F4,
    Square::G4,
    Square::H4,
    
    Square::A5,
    Square::B5,
    Square::C5,
    Square::D5,
    Square::E5,
    Square::F5,
    Square::G5,
    Square::H5,
    
    Square::A6,
    Square::B6,
    Square::C6,
    Square::D6,
    Square::E6,
    Square::F6,
    Square::G6,
    Square::H6,

    Square::A7,
    Square::B7,
    Square::C7,
    Square::D7,
    Square::E7,
    Square::F7,
    Square::G7,
    Square::H7,
    
    Square::A8,
    Square::B8,
    Square::C8,
    Square::D8,
    Square::E8,
    Square::F8,
    Square::G8,
    Square::H8,
    Square::None,
];

impl Square {
    pub fn from_str(str: &str) -> Square {
        return match str {
            "a1" => Square::A1,
            "b1" => Square::B1,
            "c1" => Square::C1,
            "d1" => Square::D1,
            "e1" => Square::E1,
            "f1" => Square::F1,
            "g1" => Square::G1,
            "h1" => Square::H1,
    
            "a2" => Square::A2,
            "b2" => Square::B2,
            "c2" => Square::C2,
            "d2" => Square::D2,
            "e2" => Square::E2,
            "f2" => Square::F2,
            "g2" => Square::G2,
            "h2" => Square::H2,
    
            "a3" => Square::A3,
            "b3" => Square::B3,
            "c3" => Square::C3,
            "d3" => Square::D3,
            "e3" => Square::E3,
            "f3" => Square::F3,
            "g3" => Square::G3,
            "h3" => Square::H3,
    
            "a4" => Square::A4,
            "b4" => Square::B4,
            "c4" => Square::C4,
            "d4" => Square::D4,
            "e4" => Square::E4,
            "f4" => Square::F4,
            "g4" => Square::G4,
            "h4" => Square::H4,
    
            "a5" => Square::A5,
            "b5" => Square::B5,
            "c5" => Square::C5,
            "d5" => Square::D5,
            "e5" => Square::E5,
            "f5" => Square::F5,
            "g5" => Square::G5,
            "h5" => Square::H5,
    
            "a6" => Square::A6,
            "b6" => Square::B6,
            "c6" => Square::C6,
            "d6" => Square::D6,
            "e6" => Square::E6,
            "f6" => Square::F6,
            "g6" => Square::G6,
            "h6" => Square::H6,
            
            "a7" => Square::A7,
            "b7" => Square::B7,
            "c7" => Square::C7,
            "d7" => Square::D7,
            "e7" => Square::E7,
            "f7" => Square::F7,
            "g7" => Square::G7,
            "h7" => Square::H7,
    
            "a8" => Square::A8,
            "b8" => Square::B8,
            "c8" => Square::C8,
            "d8" => Square::D8,
            "e8" => Square::E8,
            "f8" => Square::F8,
            "g8" => Square::G8,
            "h8" => Square::H8,
    
            _ => panic!("Tried to parse weird square")
        }
    }
    pub fn from_u8(index: u8) -> Square {
        return ARRAY[index as usize];
    }
    pub fn rank(&self) -> u8 {
        return (*self) as u8 / 8;
    }
    pub fn file(&self) -> u8 {
        return (*self) as u8 % 8;
    }

    pub fn is_orthogonal_to(&self, other: Square) -> bool {
        return self.file() == other.file() || 
            self.rank() == other.rank();
    }

    pub fn file_char(&self) -> char {
        const COLUMN_CHAR: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

        return COLUMN_CHAR[self.file() as usize];
    }

    pub const fn bit_board(&self) -> u64 {
        return 1_u64 << (*self) as u8;
    }

    pub fn print(&self) {
        print!("{}", self.to_string());
    }

    pub fn to_string(&self) -> String {
        const COLUMN_CHAR: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

        let index = (*self) as u8;
        let x = index % 8;
        let y = index / 8;

        return COLUMN_CHAR[x as usize].to_string() + &(y + 1).to_string();
    }
}


