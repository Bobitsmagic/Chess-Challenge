use crate::board;

pub fn set_bit(bit_board: &mut u64, index: u8, value: bool) {
    debug_assert!(index < 64);

    if value {
        *bit_board |= (1u64 << index);
    }
    else { 
        *bit_board &= !(1u64 << index);
    }
}

pub fn get_bit(bit_board: u64, index: u8) -> bool {
    return (bit_board & (1 << index)) != 0
}

pub fn toggle_bit(bit_board: &mut u64, index: u8){
    *bit_board ^= 1u64 << index;
}