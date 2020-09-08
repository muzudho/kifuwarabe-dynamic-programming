//! Converts a position into a string or restores a string into a position.  
//! 局面を文字列に変換したり、文字列を局面に復元します。  
use crate::log::LogExt;
use crate::{GameResult, Piece, Position, BOARD_LEN, FILE_LEN};
use casual_logger::Log;

/// A record of the game used to suspend or resume it.  
/// ゲームを中断したり、再開したりするときに使うゲームの記録です。  
impl Position {
    /// Converts the current position to xfen.  
    /// 現局面を xfen に変換します。  
    pub fn to_xfen(&self) -> String {
        let mut xfen = String::default();
        xfen.push_str("xfen ");

        // Starting board.
        // 開始盤面。
        let mut spaces = 0;
        for sq in 0..BOARD_LEN {
            if let Some(piece) = self.starting_board[sq] {
                if 0 < spaces {
                    xfen.push_str(&spaces.to_string());
                    spaces = 0;
                }
                xfen.push(match piece {
                    Piece::Nought => 'o',
                    Piece::Cross => 'x',
                });
            } else {
                spaces += 1;
            }

            if (sq + 1) % FILE_LEN == 0 && sq + 1 != BOARD_LEN {
                if 0 < spaces {
                    xfen.push_str(&spaces.to_string());
                    spaces = 0;
                }
                xfen.push('/');
            }
        }

        // Flush the remaining space.
        // 残っているスペースを flush します。
        if 0 < spaces {
            xfen.push_str(&spaces.to_string());
        }

        // Next stone at the start.
        // 開始局面で、次に置く石。
        xfen.push_str(&format!(" {}", self.starting_turn));

        // A game record.
        // 棋譜。
        if 0 < self.pieces_num - self.starting_pieces_num {
            xfen.push_str(" moves");
            for i in self.starting_pieces_num..self.pieces_num {
                xfen.push_str(&format!(" {}", self.history[i].to_string()));
            }
        }

        xfen.to_string()
    }

    /// Convert xfen to board.  
    /// xfen を盤に変換します。  
    pub fn from_xfen(xfen: &str) -> Option<Position> {
        if !xfen.starts_with("xfen ") {
            return None;
        }

        let mut pos = Position::default();
        let mut starts = 0usize;
        // Square. The upper left is 0.
        // マス。左上が0。
        let mut sq = 0;

        #[derive(Debug)]
        enum MachineState {
            /// Parse start.
            /// パース開始。
            Start,
            /// Analyzing the board on the initial stage.
            /// 初期局面の盤上を解析中。
            StartingBoard,
            /// My turn is being analyzed.
            /// 手番の解析中。
            Phase,
            /// Reading ` moves `.
            /// ` moves ` 読取中。
            MovesLabel,
            /// The game record is being analyzed.
            /// 棋譜の解析中。
            Moves,
        }
        let mut machine_state = MachineState::Start;
        // Read one character at a time.
        // １文字ずつ読取。
        for (i, ch) in xfen.chars().enumerate() {
            match machine_state {
                MachineState::Start => {
                    if i + 1 == "xfen ".len() {
                        // If you skip the top `xfen `, go to the next.
                        // 先頭の `xfen ` を読み飛ばしたら次へ。
                        machine_state = MachineState::StartingBoard;
                    }
                }
                MachineState::StartingBoard => match ch {
                    'X' => {
                        // It's not the order of the game, so I don't know the turn.
                        // 棋譜の順ではないので、手番は分かりません。
                        pos.starting_board[sq] = Some(Piece::Cross);
                        pos.pieces_num += 1;
                        sq += 1;
                    }
                    'O' => {
                        pos.starting_board[sq] = Some(Piece::Nought);
                        pos.pieces_num += 1;
                        sq += 1;
                    }
                    '1' => sq += 1,
                    '2' => sq += 2,
                    '3' => sq += 3,
                    '4' => sq += 4,
                    '5' => sq += 5,
                    '6' => sq += 6,
                    '7' => sq += 7,
                    '/' => {}
                    ' ' => {
                        // Explicitly clone.
                        // 明示的にクローンしてください。
                        pos.board = pos.starting_board.clone();
                        pos.starting_pieces_num = pos.pieces_num;
                        machine_state = MachineState::Phase;
                    }
                    _ => {
                        Log::error(&format!("(Err.138) xfen starting_board error: {}", ch));
                        return None;
                    }
                },
                MachineState::Phase => {
                    match ch {
                        'X' => {
                            pos.starting_turn = Piece::Cross;
                            pos.turn = Piece::Cross;
                        }
                        'O' => {
                            pos.starting_turn = Piece::Nought;
                            pos.turn = Piece::Nought;
                        }
                        _ => {
                            Log::error(&format!("(Err.153) xfen phase error: {}", ch));
                            return None;
                        }
                    }
                    // Temporary memory.
                    // 一時記憶。
                    starts = i;
                    machine_state = MachineState::MovesLabel;
                }
                MachineState::MovesLabel => {
                    if starts + " moves ".len() <= i {
                        machine_state = MachineState::Moves;
                    }
                }
                MachineState::Moves => {
                    if ch == ' ' {
                    } else {
                        pos.do_(&ch.to_string());
                    }
                }
            }
        }

        Some(pos)
    }

    /// Place the pieces. If you are programming yourself, legal move decisions can be postponed.  
    /// 駒を置きます。自分でプログラミングするなら、合法手は後回しで構いません。  
    ///
    /// # Arguments
    ///
    /// * `arg_str` - The rest of the command line. Here is the place to put the pieces. For example, `1` or `7`. (コマンドラインの残り。ここでは駒を置く場所。 `1` とか `7` など)
    ///
    /// # Return
    ///
    /// Game result.  
    /// ゲームの結果。  
    pub fn do_(&mut self, arg_str: &str) -> Option<GameResult> {
        let file: char = match arg_str.parse() {
            Ok(x) => x,
            Err(_x) => {
                Log::error(&format!(
                    "(Err.194) Please input 'do <file>'. args=|{}|",
                    arg_str
                ));
                return None;
            }
        };

        // Legal hand judgment. The destination row must not be full.
        // 合法手判定。 移動先の列が埋まっていてはいけません。
        match file {
            'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' => {}
            _ => {
                Log::error(&format!("(Err.204) Specify from a to g. File={}", file));
                return None;
            }
        }
        if self.is_file_fill(file) {
            Log::error(&format!(
                "(Err.211) Please put it in a place where there are no pieces. File={}",
                file
            ));
            return None;
        }

        self.redo_move(file);

        // Win/loss judgment. Let's implement this after creating Position::result and is_opponent_win().
        // 勝ち負け判定。 これは Position::result, is_opponent_win() を作ったあとで実装しましょう。
        if self.is_opponent_win() {
            if let Some(result) = Position::result(GameResult::Win, Some(self.opponent())) {
                Log::print_notice(&result);
            }
            return Some(GameResult::Win);
        } else if self.is_draw() {
            if let Some(result) = Position::result(GameResult::Draw, None) {
                Log::print_notice(&result);
            }
            return Some(GameResult::Draw);
        }

        return None;
    }

    /// 1 back.  
    /// １手戻します。  
    ///
    /// # Return
    ///
    /// Undone.
    /// アンドゥした。
    pub fn undo(&mut self) -> bool {
        self.undo_move()
    }
}
