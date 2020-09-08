//! The thinking department of a computer.  
//! See 'Search' struct in 'look_and_model' for details.  
//! コンピューターの思考部です。  
//! 詳しくは 'look_and_model' の 'Search' 構造体 を見てください。  
use crate::computer_player::Bestmove;
use crate::log::LogExt;
use crate::{
    computer_player::{Evaluation, Search, WayValue},
    Position, ResultChannel, SearchDirection, SearchInfo, SQUARES_NUM,
};
use casual_logger::{Level, Log};
use rand::Rng;
use std::time::Instant;

impl Default for Bestmove {
    fn default() -> Self {
        Bestmove {
            file: None,
            pred_result: WayValue::Lose,
        }
    }
}

/// Search.  
/// 探索部。  
impl Default for Search {
    fn default() -> Self {
        Search {
            start_pieces_num: 0,
            nodes: 0,
            stopwatch: Instant::now(),
        }
    }
}

/// Search.  
/// 探索部。  
impl Search {
    /// This is the place to put the stone.  
    /// 石を置く場所です。  
    ///
    /// # Arguments
    ///
    /// * `pos` - Position.  
    ///             局面。  
    ///
    /// # Returns
    ///
    /// * `Option<u8>` - Address of square.  
    ///                     マスの番地。  
    /// * `GameResult` - Evaluation.  
    ///                     評価値。  
    pub fn go(&mut self, pos: &mut Position, evaluation: &Evaluation) -> Bestmove {
        let bestmove_to_win = self.node(pos, evaluation, &ResultChannel::Win, 0);
        match bestmove_to_win.pred_result {
            WayValue::Win => {
                return bestmove_to_win;
            }
            _ => {}
        }

        let bestmove_to_draw = self.node(pos, evaluation, &ResultChannel::Draw, 0);
        match bestmove_to_draw.pred_result {
            WayValue::Draw => {
                return bestmove_to_draw;
            }
            _ => {}
        }

        let number = rand::thread_rng().gen_range(0, 2);
        if number == 0 {
            bestmove_to_win
        } else {
            bestmove_to_draw
        }
    }

    /// The state node of the search tree. Commonly called search.  
    /// 検索ツリーの状態ノード。一般に 'search' と呼ばれます。  
    ///
    /// * `pos` - Position.  
    ///             局面。  
    ///
    /// # Returns
    ///
    /// * `Option<u8>` - Address of square.  
    ///                     マスの番地。  
    /// * `GameResult` - Evaluation.  
    ///                     評価値。  
    fn node(
        &mut self,
        pos: &mut Position,
        evaluation: &Evaluation,
        result_channel: &ResultChannel,
        depth: usize,
    ) -> Bestmove {
        let mut bestmove = Bestmove::default();

        // Select one at random.
        // ランダムに１つ選びます。
        if let (Some(file), mut search_info) = self.choose_file(pos, evaluation, result_channel) {
            self.node_exit(
                pos,
                evaluation,
                result_channel,
                file,
                &mut search_info,
                &mut bestmove,
                depth,
            );
        }

        // End of turn.
        // 手番の終わり。
        bestmove
    }

    pub fn node_exit(
        &mut self,
        pos: &mut Position,
        evaluation: &Evaluation,
        result_channel: &ResultChannel,
        file: char,
        search_info: &mut SearchInfo,
        bestmove: &mut Bestmove,
        depth: usize,
    ) {
        // I only look at the empty square.
        // 空きマスだけを見ます。
        if !pos.is_file_fill(file) {
            let mut info_backwarding = None;
            let (forward_cut_off, info_leaf_child) =
                self.node_exit_to_child_side(pos, file, search_info, depth);

            if let None = forward_cut_off {
                // If you move forward, it's your opponent's turn.
                // 前向きに探索したら、次は対戦相手の番です。
                let opponent_bestmove = self.node(pos, evaluation, result_channel, depth + 1);
                // I'm back.
                // 戻ってきました。
                info_backwarding = Some(opponent_bestmove.pred_result);
            }
            self.node_enter_from_child_side(
                pos,
                file,
                bestmove,
                forward_cut_off,
                info_leaf_child,
                info_backwarding,
                search_info,
            );
        }
    }

    fn node_exit_to_child_side(
        &mut self,
        pos: &mut Position,
        file: char,
        search_info: &mut SearchInfo,
        depth: usize,
    ) -> (Option<ForwardCutOff>, bool) {
        let mut info_leaf = false;
        // Let's put a stone for now.
        // とりあえず石を置きましょう。
        pos.do_move(file);
        self.nodes += 1;

        // Find out why you are not doing a forward search.
        // If not, I will search.
        // 前向き検索を行わない理由を調べてください。
        // 無ければ探索します。
        let forward_cut_off = if pos.is_opponent_win() {
            if depth == 0 {
                // The opponent wins.
                // 対戦相手の勝ち。

                if Log::enabled(Level::Info) && pos.info_enabled {
                    search_info.way_value = Some(WayValue::Win);
                    search_info.comment = Some("Resign.".to_string());
                }
                Some(ForwardCutOff::OpponentWin)
            } else {
                // The opponent possibly wins.
                // 対戦相手の多分勝ち。

                if Log::enabled(Level::Info) && pos.info_enabled {
                    search_info.way_value = Some(WayValue::PossiblyWin);
                    search_info.comment = Some("Possibly resign.".to_string());
                }
                Some(ForwardCutOff::OpponentPossiblyWin)
            }
        } else if SQUARES_NUM <= pos.pieces_num {
            if depth == 0 {
                // Draw if there is no place to put.
                // 置く場所が無ければ引き分け。
                if Log::enabled(Level::Info) && pos.info_enabled {
                    info_leaf = true;
                    search_info.way_value = Some(WayValue::Draw);
                    search_info.comment = Some("It is ok.".to_string());
                }
                Some(ForwardCutOff::Draw)
            } else {
                // Possibly draw if there is no place to put.
                // 置く場所が無ければ多分引き分け。
                if Log::enabled(Level::Info) && pos.info_enabled {
                    info_leaf = true;
                    search_info.way_value = Some(WayValue::PossiblyDraw);
                    search_info.comment = Some("It is ok.".to_string());
                }
                Some(ForwardCutOff::PossiblyDraw)
            }
        } else {
            if Log::enabled(Level::Info) && pos.info_enabled {
                search_info.comment = Some("Search.".to_string());
            }
            None
        };

        // (1) Outputs information for forward search.
        // (一) 前向き探索の情報を出力します。
        if pos.info_enabled {
            search_info.nps = self.nps();
            search_info.nodes = self.nodes;
            search_info.pv_json = pos.pv_json();
            search_info.search_direction = SearchDirection::Forward;
            search_info.leaf = info_leaf;
            search_info.pieces_num = None;
            search_info.turn = pos.turn;
            Log::print_info(&search_info.to_string());
        }

        return (forward_cut_off, info_leaf);
    }

    fn node_enter_from_child_side(
        &mut self,
        pos: &mut Position,
        file: char,
        bestmove: &mut Bestmove,
        forward_cut_off: Option<ForwardCutOff>,
        info_leaf: bool,
        info_backwarding: Option<WayValue>,
        search_info: &mut SearchInfo,
    ) {
        let mut backward_cut_off = None;
        // (2) Remove the placed stone.
        // (二) 置いた石は取り除きます。
        pos.undo_move();

        if let Some(opponent_way_value) = info_backwarding {
            match opponent_way_value {
                WayValue::Lose => {
                    // I beat the opponent.
                    // 相手を負かしました。

                    // The search ends.
                    // 探索を終了します。
                    backward_cut_off = Some(BackwardCutOff::YouWin);
                }
                WayValue::PossiblyLose => {
                    // Possibly, I beat the opponent.
                    // 多分、相手を負かしました。

                    // I will continue.
                    // まだ続けます。
                }
                WayValue::Draw => {
                    // If neither is wrong, draw.
                    // お互いがミスしなければ引き分け。

                    match bestmove.pred_result {
                        WayValue::Lose => {
                            // If it gets better, change it to this. Generally called 'Update alpha evaluation'.
                            // 良くなるならこの手に変えます。一般的には 'α評価値の更新' と呼びます。
                            bestmove.file = Some(file);
                            bestmove.pred_result = WayValue::Draw;
                        }
                        _ => {}
                    }
                    // I will continue.
                    // まだ続けます。
                }
                WayValue::PossiblyDraw => {
                    // Possibly, draw.
                    // 多分、引き分け。

                    // I will continue.
                    // まだ続けます。
                }
                WayValue::PossiblyWin | WayValue::Win => {
                    // Don't choose to lose.
                    // 自分が負ける手は選びません。

                    // I will continue.
                    // まだ続けます。
                }
            }
        }

        // (3) Outputs backward search information.
        // (三) 後ろ向き探索の情報を出力します。
        if Log::enabled(Level::Info) && pos.info_enabled {
            if let Some(opponent_way_value) = info_backwarding {
                match opponent_way_value {
                    WayValue::Lose => {
                        // I beat the opponent.
                        // 相手を負かしました。
                        search_info.way_value = Some(WayValue::Win);
                        search_info.comment = Some("Hooray!".to_string());
                    }
                    WayValue::PossiblyLose => {
                        // Possibly, I beat the opponent.
                        // 多分、相手を負かしました。
                        search_info.way_value = Some(WayValue::PossiblyWin);
                        search_info.comment = Some("Yeah!".to_string());
                    }
                    WayValue::Draw => {
                        // If neither is wrong, draw.
                        // お互いがミスしなければ引き分け。
                        search_info.way_value = Some(WayValue::Draw);
                        search_info.comment = Some("Fmmm.".to_string());
                    }
                    WayValue::PossiblyDraw => {
                        // If neither is wrong, possibly draw.
                        // お互いがミスしなければ多分引き分け。
                        search_info.way_value = Some(WayValue::PossiblyDraw);
                        search_info.comment = Some("Fmmm.".to_string());
                    }
                    WayValue::PossiblyWin => {
                        // Don't choose to possibly lose.
                        // 自分が多分負ける手は選びません。
                        search_info.way_value = Some(WayValue::PossiblyLose);
                        search_info.comment = Some("Oh!".to_string());
                    }
                    WayValue::Win => {
                        // Don't choose to lose.
                        // 自分が負ける手は選びません。
                        search_info.way_value = Some(WayValue::Lose);
                        search_info.comment = Some("Damn!".to_string());
                    }
                }
            }
            search_info.nps = self.nps();
            search_info.nodes = self.nodes;
            search_info.pv_json = pos.pv_json();
            search_info.search_direction = SearchDirection::Backward;
            search_info.leaf = info_leaf;
            search_info.pieces_num = Some(pos.pieces_num);
            search_info.turn = pos.turn;
            Log::print_info(&search_info.to_string());
        }

        // (4) Depending on the condition, the sibling node search is skipped.
        // (四) 条件によっては、兄弟ノードの検索がスキップされます。
        if let Some(forward_cut_off) = forward_cut_off {
            match forward_cut_off {
                ForwardCutOff::OpponentWin => {
                    bestmove.file = Some(file);
                    bestmove.pred_result = WayValue::Win;
                    return;
                }
                ForwardCutOff::OpponentPossiblyWin => {
                    bestmove.file = Some(file);
                    bestmove.pred_result = WayValue::PossiblyWin;
                    return;
                }
                ForwardCutOff::Draw => {
                    bestmove.file = Some(file);
                    bestmove.pred_result = WayValue::Draw;
                    return;
                }
                ForwardCutOff::PossiblyDraw => {
                    bestmove.file = Some(file);
                    bestmove.pred_result = WayValue::PossiblyDraw;
                    return;
                }
            }
        } else if let Some(backward_cut_off) = backward_cut_off {
            match backward_cut_off {
                BackwardCutOff::YouWin => {
                    bestmove.file = Some(file);
                    bestmove.pred_result = WayValue::Win;
                    return;
                }
            }
        }

        return;
    }

    /// Select one file at random.
    /// TODO 重みを付けて、ランダムに列を１つ選びます。
    fn choose_file(
        &mut self,
        pos: &Position,
        evaluation: &Evaluation,
        result_channel: &ResultChannel,
    ) -> (Option<char>, SearchInfo) {
        let tensor = evaluation.ways_weight(pos, result_channel);
        let mut search_info = SearchInfo::default();
        search_info.result_channel = *result_channel;
        search_info.weight_tensor = tensor;
        let w_a: u16 = {
            let file = 0;
            tensor[file][0] as u16
                + tensor[file][1] as u16
                + tensor[file][2] as u16
                + tensor[file][3] as u16
        };
        let w_b: u16 = {
            let file = 1;
            tensor[file][0] as u16
                + tensor[file][1] as u16
                + tensor[file][2] as u16
                + tensor[file][3] as u16
        };
        let w_c: u16 = {
            let file = 2;
            tensor[file][0] as u16
                + tensor[file][1] as u16
                + tensor[file][2] as u16
                + tensor[file][3] as u16
        };
        let w_d: u16 = {
            let file = 3;
            tensor[file][0] as u16
                + tensor[file][1] as u16
                + tensor[file][2] as u16
                + tensor[file][3] as u16
        };
        let w_e: u16 = {
            let file = 4;
            tensor[file][0] as u16
                + tensor[file][1] as u16
                + tensor[file][2] as u16
                + tensor[file][3] as u16
        };
        let w_f: u16 = {
            let file = 5;
            tensor[file][0] as u16
                + tensor[file][1] as u16
                + tensor[file][2] as u16
                + tensor[file][3] as u16
        };
        let w_g: u16 = {
            let file = 6;
            tensor[file][0] as u16
                + tensor[file][1] as u16
                + tensor[file][2] as u16
                + tensor[file][3] as u16
        };
        // Upper bound.
        let a_up = w_a;
        let b_up = a_up + w_b;
        let c_up = b_up + w_c;
        let d_up = c_up + w_d;
        let e_up = d_up + w_e;
        let f_up = e_up + w_f;
        let total = f_up + w_g;
        if total == 0 {
            if pos.info_enabled {
                search_info.chosen_file = None;
            }
            return (None, search_info);
        }

        let number = rand::thread_rng().gen_range(0, total);
        let file = if number < a_up {
            'a'
        } else if number < b_up {
            'b'
        } else if number < c_up {
            'c'
        } else if number < d_up {
            'd'
        } else if number < e_up {
            'e'
        } else if number < f_up {
            'f'
        } else {
            'g'
        };

        if pos.info_enabled {
            search_info.chosen_file = Some(file);
        }

        (Some(file), search_info)
    }
}

/// The reason for ending the forward search.  
/// 前向き探索を終了した理由。  
#[derive(Clone, Copy)]
enum ForwardCutOff {
    /// End with a opponent win.  
    /// 相手の勝ちにつき、終了。  
    OpponentWin,
    /// End with a opponent possibly win.  
    /// 相手の多分勝ちにつき、終了。  
    OpponentPossiblyWin,
    /// End with a draw.  
    /// 引き分けにつき、終了。  
    Draw,
    /// End with a possibly draw.  
    /// 多分引き分けにつき、終了。  
    PossiblyDraw,
}

/// The reason for ending the backward search.  
/// 後ろ向き探索を終了した理由。  
enum BackwardCutOff {
    /// End with a you win.  
    /// あなたの勝ちにつき、終了。  
    YouWin,
}
