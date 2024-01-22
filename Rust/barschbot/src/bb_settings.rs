use crate::evaluation::EvalAttributes;

#[derive(Clone)]
pub struct BBSettings {
    pub max_depth: u8,
    pub max_quiescence_depth: u8,
    pub end_game_table: bool,
    pub max_extensions: u8,
    pub eval_factors: EvalFactors,
    pub min_search_time: u64
}

#[derive(Debug, Copy, Clone)]
pub enum FactorName {
    PieceValueP, PieceValueN, PieceValueB, PieceValueR, PieceValueQ,
    SafeMobilityP, SafeMobilityN, SafeMobilityB, SafeMobilityR, SafeMobilityQ, SafeMobilityK,
    UnsafeMobilityP, UnsafeMobilityN, UnsafeMobilityB, UnsafeMobilityR, UnsafeMobilityQ, UnsafeMobilityK,

    LateFactorRange,
    SquareControl,

    PawnRank2, PawnRank3, PawnRank4, PawnRank5, PawnRank6, PawnRank7,
    PassedPawn,
    DoubledPawn,
    IsolatedPawn,

    KnightOutpost,

    KingExposed,
    KingControl,
    SafeCheck,
    UnsafeCheck,
}

pub const ALL_NAMES: [FactorName; 33] = [
    FactorName::PieceValueP, FactorName::PieceValueN, FactorName::PieceValueB, FactorName::PieceValueR, FactorName::PieceValueQ,
    FactorName::SafeMobilityP, FactorName::SafeMobilityN, FactorName::SafeMobilityB, FactorName::SafeMobilityR, FactorName::SafeMobilityQ, FactorName::SafeMobilityK,
    FactorName::UnsafeMobilityP, FactorName::UnsafeMobilityN, FactorName::UnsafeMobilityB, FactorName::UnsafeMobilityR, FactorName::UnsafeMobilityQ, FactorName::UnsafeMobilityK,

    FactorName::LateFactorRange,
    FactorName::SquareControl,

    FactorName::PawnRank2, FactorName::PawnRank3, FactorName::PawnRank4, FactorName::PawnRank5, FactorName::PawnRank6, FactorName::PawnRank7,
    FactorName::PassedPawn,
    FactorName::DoubledPawn,
    FactorName::IsolatedPawn,

    FactorName::KnightOutpost,

    FactorName::KingExposed,
    FactorName::KingControl,
    FactorName::SafeCheck,
    FactorName::UnsafeCheck,
];

pub const STANDARD_SETTINGS: BBSettings = BBSettings { max_depth: 6, max_quiescence_depth: 3, end_game_table: true, max_extensions: 2, min_search_time: 3000, eval_factors: STANDARD_EVAL_FACTORS };
pub const BEST_SETTINGS: BBSettings = BBSettings { max_depth: 3, max_quiescence_depth: 3, end_game_table: true, max_extensions: 2, min_search_time: 3000, eval_factors: SF_TUNED_VALUES };

#[derive(Clone)]
pub struct EvalFactors {
    values: [f32; 33],
}

pub const STANDARD_EVAL_FACTORS: EvalFactors = EvalFactors {
    values: [
        //Piece value
        1.0, 2.8, 3.2, 5.0, 11.0,
        //Safe mobility 
        0.01, 0.0618192, 0.07, 0.053, 0.005, 0.106,
        //Unsafe Mobility
        -0.01, -0.06, -0.02, -0.03, -0.09, -0.07,

        //Late factor range
        0.01,
        //Square control
        0.0106,

        //Pawn push bonus
        -0.062, 0.05, 0.077, 0.1, 0.15, 0.5,
        //Passed pawn value
        0.204, 
        //Doubled pawn penalty
        -0.15, 
        //Isolated pawn penalty
        -0.15,

        //Knight outpost value
        0.062,

        //King exposed penalty
        -0.0066,
        //King control penalty
        -0.162,
        //Safe check value
        0.2,
        //Unsafe check value 
        0.086,
    ]
};

pub const AUTO_TUNED_VALUES: EvalFactors = EvalFactors {
    values: [
        //PieceValueP
        0.9810926,
        //PieceValueN
        2.594,
        //PieceValueB
        3.3177702,
        //PieceValueR
        5.34578,
        //PieceValueQ
        10.850797,
        //SafeMobilityP
        0.009179999,
        //SafeMobilityN
        0.0743492,
        //SafeMobilityB
        0.07696921,
        //SafeMobilityR
        0.04579276,
        //SafeMobilityQ
        0.0055,
        //SafeMobilityK
        0.055476826,
        //UnsafeMobilityP
        -0.057623997,
        //UnsafeMobilityN
        0.0706456,
        //UnsafeMobilityB
        -0.01832443,
        //UnsafeMobilityR
        0.054166567,
        //UnsafeMobilityQ
        -0.06561,
        //UnsafeMobilityK
        -0.07732016,
        //LateFactorRange
        0.01,
        //SquareControl
        0.013546616,
        //PawnRank2
        -0.059148,
        //PawnRank3
        0.045,
        //PawnRank4
        0.069299996,
        //PawnRank5
        0.11,
        //PawnRank6
        0.1815,
        //PawnRank7
        0.73205006,
        //PassedPawn
        0.237864,
        //DoubledPawn
        -0.102789,
        //IsolatedPawn
        -0.15300001,
        //KnightOutpost
        0.062,
        //KingExposed
        -0.00713592,
        //KingControl
        -0.1338444,
        //SafeCheck
        0.2634326,
        //UnsafeCheck
        0.07736905,
    ]
};

pub const SF_TUNED_VALUES: EvalFactors = EvalFactors {
    values: [
        //PieceValueP
        0.9810926,
        //PieceValueN
        2.594,
        //PieceValueB
        3.3177702,
        //PieceValueR
        5.34578,
        //PieceValueQ
        10.850797,
        //SafeMobilityP
        0.008721,
        //SafeMobilityN
        0.0743492,
        //SafeMobilityB
        0.07696921,
        //SafeMobilityR
        0.048082396,
        //SafeMobilityQ
        0.0055,
        //SafeMobilityK
        0.055476826,
        //UnsafeMobilityP
        0.0547428,
        //UnsafeMobilityN
        0.0706456,
        //UnsafeMobilityB
        0.01832443,
        //UnsafeMobilityR
        0.054166567,
        //UnsafeMobilityQ
        0.06561,
        //UnsafeMobilityK
        0.07732016,
        //LateFactorRange
        0.01,
        //SquareControl
        0.013546616,
        //PawnRank2
        -0.059148,
        //PawnRank3
        0.045,
        //PawnRank4
        0.069299996,
        //PawnRank5
        0.11,
        //PawnRank6
        0.1815,
        //PawnRank7
        0.73205006,
        //PassedPawn
        0.237864,
        //DoubledPawn
        -0.102789,
        //IsolatedPawn
        -0.15300001,
        //KnightOutpost
        0.062,
        //KingExposed
        -0.00713592,
        //KingControl
        -0.1338444,
        //SafeCheck
        0.2634326,
        //UnsafeCheck
        0.07736905,
    ]
};

pub const MAX_MATERIAL_SUM: i32 = 3 * 8 + 5 * 4 + 9 * 2;
impl EvalFactors {
    pub fn evaluate(&self, attributes: &EvalAttributes) -> f32 {
        let values = self.values;
        
        let mut sum = 0.0;
        const START_MAT_SUM: f32 = MAX_MATERIAL_SUM as f32;

        let late_factor = 1.0 + self.get_value(FactorName::LateFactorRange) - attributes.material_sum as f32 / START_MAT_SUM * self.get_value(FactorName::LateFactorRange);

        for i in 0..5 {
            sum += self.get_array(FactorName::PieceValueP, i) * attributes.piece_dif[i] as f32;
        }

        sum *= late_factor;

        sum += self.get_value(FactorName::SquareControl) * attributes.sq_control_dif as f32;

        for i in 0..6 {
            sum += self.get_array(FactorName::SafeMobilityP, i) * attributes.safe_mobility_dif[i] as f32;
            sum += self.get_array(FactorName::UnsafeMobilityP, i) * attributes.unsafe_mobility_dif[i] as f32;
        }

        for i in 0..6 {
            sum += self.get_array(FactorName::PawnRank2, i) * attributes.pawn_push_dif[i] as f32;
        }

        sum += self.get_value(FactorName::PassedPawn) * attributes.passed_pawn_dif as f32;
        sum += self.get_value(FactorName::DoubledPawn) * attributes.doubled_pawn_dif as f32;
        sum += self.get_value(FactorName::IsolatedPawn) * attributes.isolated_pawn_dif as f32;
        
        sum += self.get_value(FactorName::KnightOutpost) * attributes.knight_outpost_dif as f32;

        sum += self.get_value(FactorName::KingExposed) * attributes.king_qn_moves_dif as f32;
        sum += self.get_value(FactorName::KingControl) * attributes.king_control_dif as f32;
        sum += self.get_value(FactorName::SafeCheck) * attributes.safe_check_dif as f32;
        sum += self.get_value(FactorName::UnsafeCheck) * attributes.unsafe_check_dif as f32;

        return sum;
    }

    pub fn get_value(&self, index: FactorName) -> f32 {
        return self.values[index as usize];
    }

    pub fn set_value(&mut self, index: FactorName, value: f32) {
        self.values[index as usize] = value;
    }

    pub fn get_array(&self, index: FactorName, offset: usize) -> f32 {
        return self.values[index as usize + offset];
    }

    pub fn print_all(&self) {
        println!("Settings: ");
        for f in ALL_NAMES {
            println!("\t{:?} -> {}", f, self.get_value(f));
        }
    }
}