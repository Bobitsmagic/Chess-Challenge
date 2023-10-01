#[derive(Copy, Clone)]
pub struct PieceList {
    piece_count: u8,
    occupied_squares: [u8; 16],
    map: [u8; 64]   
}

impl PieceList {
    pub fn new() -> Self {
        return PieceList { piece_count: 0, occupied_squares: [0 as u8; 16], map: [0 as u8; 64] }
    }

    pub fn count(&self) -> u8 {
        return self.piece_count;
    }

    pub fn add_at_square(&mut self, square: u8) {
        self.occupied_squares[self.piece_count as usize] = square;
        self.map[square as usize] = self.piece_count;
        self.piece_count += 1;
    }

    pub fn remove_at_square(&mut self, square: u8) {
        let piece_index = self.map[square as usize];
        self.occupied_squares[piece_index as usize] = self.occupied_squares[(self.piece_count - 1) as usize];
        self.map[self.occupied_squares[piece_index as usize] as usize] = piece_index;
        self.piece_count -= 1;
    }

    pub fn move_piece(&mut self, start_square: u8, target_square: u8) {
        let piece_index = self.map[start_square as usize];
        self.occupied_squares[piece_index as usize] = target_square;
        self.map[target_square as usize] = piece_index;
    }

    pub fn get_occupied_square(&self, index: u8) -> u8 {
        debug_assert!(index < self.piece_count);

        return self.occupied_squares[index as usize];
    }
}