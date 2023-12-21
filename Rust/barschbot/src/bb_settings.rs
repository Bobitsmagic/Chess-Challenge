use crate::evaluation::EvalAttributes;

pub struct BBSettings {
    pub max_depth: u8,
    pub max_quiescence_depth: u8,
    pub eval_factors: EvalFactors
}

pub const STANDARD_SETTINGS: BBSettings = BBSettings { max_depth: 4, max_quiescence_depth: 10, eval_factors: STANDARD_EVAL_FACTORS };

pub struct EvalFactors {
    //[Pawn, Knight, Bishop, Rook, Queen, King] 
    
    //The value of owning a piece of this type
    pub piece_value: [f32; 5],
    //The value of a possible safe move (no capture) a piece of this type can make
    pub safe_mobility: [f32; 6],
    //The value of a possible unsafe move (no capture) a piece of this type can make
    pub unsafe_mobility: [f32; 6],   

    //[TODO] safe mobillity ?

    //[TODO] Matrix (PT X PT) ?

    //Squares 
    //The value of owning a square (SEE) static exchange evaluation 
    pub square_control: f32,

    //Pawns
    pub pawn_push_value: [f32; 6],
    pub passed_pawn_value: f32,
    pub doubled_pawn_penalty: f32,
    pub isolated_pawn_penalty: f32,

    //[TODO] Matrix [passed?][doubled?][isolated?][rank] (64)

    //King
    pub king_attack: f32, //The value of squares controlled by a Queen or Knight move away from the King
}

pub const STANDARD_EVAL_FACTORS: EvalFactors = EvalFactors {
    piece_value: [1.0, 2.8, 3.2, 5.0, 9.0],
    safe_mobility: [0.0, 0.08, 0.07, 0.05, 0.02, 0.0],
    unsafe_mobility: [0.0, 0.03, 0.02, 0.01, 0.001, 0.0],
    
    square_control: 0.01,
    
    pawn_push_value: [0.0, 0.05, 0.07, 0.1, 0.15, 0.5],
    passed_pawn_value: 0.1,
    doubled_pawn_penalty: -0.15,
    isolated_pawn_penalty: -0.2,

    king_attack: 0.1,
};

pub const MATERIAL_EVAL_FACTORS: EvalFactors = EvalFactors {
    piece_value: [1.0, 2.8, 3.2, 5.0, 9.0],
    safe_mobility: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    unsafe_mobility: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    
    square_control: 0.0,
    
    pawn_push_value: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    passed_pawn_value: 0.0,
    doubled_pawn_penalty: -0.0,
    isolated_pawn_penalty: -0.0,

    king_attack: 0.0,
};

pub const RANDOM_MOVES: EvalFactors = EvalFactors {
    //piece_value: [1.0, 2.8, 3.2, 5.0, 9.0],
    piece_value: [0.0, 0.0, 0.0, 0.0, 0.0],
    safe_mobility: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    unsafe_mobility: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    
    square_control: 0.0,
    
    pawn_push_value: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    passed_pawn_value: 0.0,
    doubled_pawn_penalty: -0.0,
    isolated_pawn_penalty: -0.0,

    king_attack: 0.0,
};


impl EvalFactors {
    pub fn evaluate(&self, attributes: &EvalAttributes) -> f32 {
        
        let mut sum = 0.0;
        for i in 0..5 {
            sum += self.piece_value[i] * attributes.piece_dif[i] as f32;
        }
        for i in 0..6 {
            sum += self.safe_mobility[i] * attributes.safe_mobility_dif[i] as f32;
            sum += self.unsafe_mobility[i] * attributes.unsafe_mobility_dif[i] as f32;
        }

        sum += self.square_control * attributes.square_control_dif as f32;

        for i in 0..6 {
            sum += self.pawn_push_value[i] * attributes.pawn_push_dif[i] as f32;
        }

        sum += self.passed_pawn_value * attributes.passed_pawn_dif as f32;
        sum += self.doubled_pawn_penalty * attributes.doubled_pawn_dif as f32;
        sum += self.isolated_pawn_penalty * attributes.isolated_pawn_dif as f32;
        sum += self.king_attack * attributes.king_attack_dif as f32;

        return sum;
    }
}