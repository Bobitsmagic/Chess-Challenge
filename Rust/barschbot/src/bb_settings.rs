use crate::evaluation::EvalAttributes;

#[derive(Clone)]
pub struct BBSettings {
    pub max_depth: u8,
    pub max_quiescence_depth: u8,
    pub end_game_table: bool,
    pub eval_factors: EvalFactors
}

pub const STANDARD_SETTINGS: BBSettings = BBSettings { max_depth: 2, max_quiescence_depth: 2, end_game_table: true, eval_factors: STANDARD_EVAL_FACTORS };

#[derive(Clone)]
pub struct EvalFactors {
    //[Pawn, Knight, Bishop, Rook, Queen, King] 
    
    //The value of owning a piece of this type
    pub piece_value: [f32; 5],
    //The value of a possible safe move (no capture) a piece of this type can make
    pub safe_mobility: [f32; 6],
    //The value of a possible unsafe move (no capture) a piece of this type can make
    pub unsafe_mobility: [f32; 6],   

    pub late_factor_range: f32,

    //[TODO] Matrix (PT X PT) ?

    //Squares 
    //The value of owning a square (SEE) static exchange evaluation 
    pub square_control: f32,

    //Pawns
    pub pawn_push_value: [f32; 6],
    pub passed_pawn_value: f32,
    pub doubled_pawn_penalty: f32,
    pub isolated_pawn_penalty: f32,

    pub knight_outpost_value: f32,

    //[TODO] Matrix [passed?][doubled?][isolated?][rank] (64)

    //King
    pub king_exposed_penalty: f32,
    pub king_control_value: f32,          
    pub safe_check_value: f32,      
    pub unsafe_check_value: f32,
}

pub const STANDARD_EVAL_FACTORS: EvalFactors = EvalFactors {
    piece_value: [1.0, 2.8, 3.2, 5.0, 11.0],
    safe_mobility: [0.01, 0.08, 0.07, 0.05, 0.005, 0.0],
    unsafe_mobility: [0.001, 0.03, 0.02, 0.01, 0.001, 0.0],
    
    square_control: 0.01,
    late_factor_range: 0.01,
    
    pawn_push_value: [0.0, 0.05, 0.07, 0.1, 0.15, 0.5],
    passed_pawn_value: 0.2,
    doubled_pawn_penalty: -0.15,
    isolated_pawn_penalty: -0.15,

    knight_outpost_value: 0.0,

    king_exposed_penalty: -0.006,
    safe_check_value: 0.2,
    unsafe_check_value: 0.086,
    king_control_value: -0.162,
};

pub const MATERIAL_EVAL_FACTORS: EvalFactors = EvalFactors {
    piece_value: [1.0, 2.8, 3.2, 5.0, 9.0],
    safe_mobility: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    unsafe_mobility: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    
    square_control: 0.0,
    
    late_factor_range: 0.0,
    
    pawn_push_value: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    passed_pawn_value: 0.0,
    doubled_pawn_penalty: -0.0,
    isolated_pawn_penalty: -0.0,

    knight_outpost_value:  0.0,

    king_exposed_penalty: 0.0,
    safe_check_value: 0.0,
    unsafe_check_value: 0.0,
    king_control_value: 0.0,
};

pub const RANDOM_MOVES: EvalFactors = EvalFactors {
    //piece_value: [1.0, 2.8, 3.2, 5.0, 9.0],
    piece_value: [0.0, 0.0, 0.0, 0.0, 0.0],
    safe_mobility: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    unsafe_mobility: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    
    square_control: 0.0,

    late_factor_range: 0.0,
    
    pawn_push_value: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    passed_pawn_value: 0.0,
    doubled_pawn_penalty: -0.0,
    isolated_pawn_penalty: -0.0,

    knight_outpost_value:  0.0,

    king_exposed_penalty: 0.0,
    safe_check_value: 0.0,
    unsafe_check_value: 0.0,
    king_control_value: 0.0,
};

pub const MAX_MATERIAL_SUM: i32 = 3 * 8 + 5 * 4 + 9 * 2;
impl EvalFactors {
    pub fn evaluate(&self, attributes: &EvalAttributes) -> f32 {
        
        let mut sum = 0.0;
        const START_MAT_SUM: f32 = MAX_MATERIAL_SUM as f32;

        let late_factor = 1.0 + self.late_factor_range - attributes.material_sum as f32 / START_MAT_SUM * self.late_factor_range;

        for i in 0..5 {
            sum += self.piece_value[i] * attributes.piece_dif[i] as f32;
        }

        sum *= late_factor;

        sum += self.square_control * attributes.sq_control_dif as f32;

        for i in 0..6 {
            sum += self.safe_mobility[i] * attributes.safe_mobility_dif[i] as f32;
            sum += self.unsafe_mobility[i] * attributes.unsafe_mobility_dif[i] as f32;
        }

        for i in 0..6 {
            sum += self.pawn_push_value[i] * attributes.pawn_push_dif[i] as f32;
        }

        sum += self.passed_pawn_value * attributes.passed_pawn_dif as f32;
        sum += self.doubled_pawn_penalty * attributes.doubled_pawn_dif as f32;
        sum += self.isolated_pawn_penalty * attributes.isolated_pawn_dif as f32;
        
        sum += self.knight_outpost_value * attributes.knight_outpost_dif as f32;

        sum += self.king_exposed_penalty * attributes.king_qn_moves_dif as f32;
        sum += self.king_control_value * attributes.king_control_dif as f32;
        sum += self.safe_check_value * attributes.safe_check_dif as f32;
        sum += self.unsafe_check_value * attributes.unsafe_check_dif as f32;

        return sum;
    }
}