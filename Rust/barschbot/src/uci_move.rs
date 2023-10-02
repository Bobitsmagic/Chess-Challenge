use crate::constants;

pub struct UCIMove {
    value: u16,
}

const NO_FLAG: u16 =                 0b0000;
const EN_PASSANT_CAPTURE_FLAG: u16 = 0b0001;
const CASTLE_FLAG: u16 =             0b0010;
const PAWN_TWO_UP_FLAG: u16 =        0b0011;
const PROMOTE_TO_QUEEN_FLAG: u16 =   0b0100;
const PROMOTE_TO_KNIGHT_FLAG: u16 =  0b0101;
const PROMOTE_TO_ROOK_FLAG: u16 =    0b0110;
const PROMOTE_TO_BISHOP_FLAG: u16 =  0b0111;

const START_SQUARE_MASK: u16 =  0b0000000000111111;
const TARGET_SQUARE_MASK: u16 = 0b0000111111000000;
//const FLAG_MASK: u16 =          0b1111000000000000;

pub const NULL_MOVE: UCIMove = UCIMove { value: 0 };

impl UCIMove {
    pub fn new(start_square: u16, target_square: u16, flag: u16) -> Self {
        return UCIMove { value: start_square | (target_square << 6) | (flag << 12)};
    }

    pub fn get_start_square(&self) -> u16 {
        return self.value & START_SQUARE_MASK;
    }
    pub fn get_target_square(&self) -> u16 {
        return (self.value & TARGET_SQUARE_MASK) >> 6;
    }

    pub fn get_flag(&self) -> u16 {
        return self.value >> 12;
    }

    pub fn is_promotion(&self) -> bool {
        return self.get_flag() >= PROMOTE_TO_QUEEN_FLAG
    }

    pub fn is_en_passant(&self) -> bool {
        return self.get_flag() == EN_PASSANT_CAPTURE_FLAG
    }

    pub fn promotion_piece_type(&self) -> u8 {
        return match self.get_flag() {
            PROMOTE_TO_ROOK_FLAG => constants::ROOK,
            PROMOTE_TO_KNIGHT_FLAG => constants::KNIGHT,
            PROMOTE_TO_BISHOP_FLAG => constants::BISHOP,
            PROMOTE_TO_QUEEN_FLAG => constants::QUEEN,
            _ => constants::EMPTY
        }
    }
}