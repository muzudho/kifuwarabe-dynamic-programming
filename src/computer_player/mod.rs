pub mod evaluation_file;
pub mod evaluation_model;
mod learn;
pub mod search;

use crate::WayValue;
use std::time::Instant;

/// Vertical, Horizontal Baroque diagonal, Sinister diagonal.  
/// 垂直、水平、右上がりナナメ、左上がりナナメ。  
pub const FEATURE_V_H_B_S_LEN: usize = 4;

/// Nought and cross.
/// 先後。
pub const NOUGHT_AND_CROSS_LEN: usize = 2;

/// Win and draw.
/// 勝ちと負け。
pub const WIN_AND_DRAW_LEN: usize = 2;

/// 3^4
pub const N3POW4: usize = 81;

/// 3^5
pub const N3POW5: usize = 243;

/// 3^6
pub const N3POW6: usize = 729;

/// 3^7
pub const N3POW7: usize = 2187;

/// Learning.  
/// 学習部。  
pub struct Learning {}

/// Next put.  
/// 次の一手。  
pub struct Bestmove {
    pub file: Option<char>,
    /// Prediction result.  
    /// 結果予測。  
    pub pred_result: WayValue,
}

/// Search.  
/// 探索部。  
pub struct Search {
    /// The number of stones on the board at the start of this search.  
    /// この探索の開始時に盤の上に有った石の数。  
    pub start_pieces_num: usize,
    /// Number of state nodes searched.  
    /// 探索した状態ノード数。  
    pub nodes: u32,
    /// Start the stopwatch when this structure is created.  
    /// この構造体を生成した時点からストップ・ウォッチを開始します。  
    pub stopwatch: Instant,
}

/// Evaluation.
/// 評価値。
pub struct Evaluation {
    // Win and draw value.
    // 勝ち評価値と、引き分け評価値。
    pub features_1_to_7: [[[[u8; NOUGHT_AND_CROSS_LEN]; WIN_AND_DRAW_LEN]; N3POW6]; 7],
    pub features_8_to_13: [[[[u8; NOUGHT_AND_CROSS_LEN]; WIN_AND_DRAW_LEN]; N3POW7]; 6],
    pub features_14_19_20_25: [[[[u8; NOUGHT_AND_CROSS_LEN]; WIN_AND_DRAW_LEN]; N3POW4]; 4],
    pub features_15_18_21_24: [[[[u8; NOUGHT_AND_CROSS_LEN]; WIN_AND_DRAW_LEN]; N3POW5]; 4],
    pub features_16_17_22_23: [[[[u8; NOUGHT_AND_CROSS_LEN]; WIN_AND_DRAW_LEN]; N3POW6]; 4],
}
