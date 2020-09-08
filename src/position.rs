//! Position. A record of the game used to suspend or resume it.  
//! 局面。 ゲームを中断したり、再開したりするときに使うゲームの記録です。  
use crate::log::LogExt;
use crate::{file_to_num, Piece, Position, BOARD_LEN, FILE_LEN};
use casual_logger::{Log, Table};

/// Position. A record of the game used to suspend or resume it.  
/// 局面。 ゲームを中断したり、再開したりするときに使うゲームの記録です。  
impl Position {
    /// Place the stone.  
    /// １手指します。  
    pub fn do_move(&mut self, file: char) {
        self.redo_move(file);
    }

    /// Place the stone.  
    /// Do not write to the pv.  
    /// １手指します。  
    /// 読み筋への書き込みを行いません。  
    pub fn redo_move(&mut self, file: char) {
        // I placed a stone.
        // 石を置いた。
        self.board[self.fallen_sq(file)] = Some(self.turn);
        // Write on the game record.
        // 棋譜に書きます。
        self.history[self.pieces_num] = file;
        // After writing on the game, count the stones you have placed.
        // 棋譜に書いたあと、置いた石を数えます。
        self.pieces_num += 1;
        // Change of turn.
        // 手番交代。
        self.turn = self.opponent();
    }

    /// 1 back.  
    /// 1手戻します。  
    ///
    /// # Return
    ///
    /// Undone.
    /// アンドゥした。
    pub fn undo_move(&mut self) -> bool {
        if self.history.len() < 1 || self.pieces_num < 1 {
            return false;
        }

        // Change of turn.
        // 手番交代。
        self.turn = self.opponent();
        // The number of stones points to the next element of the array,
        // so first reduce it and then extract the contents of the array.
        // 石の数は配列の次の要素を指しているので、先に戻してから、配列の中身を取り出してください。
        self.pieces_num -= 1;
        // Remove from the pv.
        // 読み筋から消します。
        let file = self.history[self.pieces_num];
        self.history[self.pieces_num] = ' ';
        // Turn off the stone.
        // 石を消します。
        if let Some(sq) = self.peak_sq_in_file(file) {
            self.board[sq] = None;
        }

        true
    }
    /// Opponent.
    /// 相手番。
    pub fn opponent(&self) -> Piece {
        use crate::position::Piece::*;
        match self.turn {
            Nought => Cross,
            Cross => Nought,
        }
    }

    pub fn fallen_sq_or_none(&self, file: char) -> Option<usize> {
        if self.is_file_fill(file) {
            None
        } else {
            Some(self.fallen_sq(file))
        }
    }

    /// It is the bottom of the specified row.  
    /// 指定した列の最下段の空升です。  
    pub fn fallen_sq(&self, file: char) -> usize {
        let mut sq = file_to_num(file) as usize;
        if let Some(_) = self.board[sq] {
            panic!(Log::print_fatal_t(
                "(Err.32) File is filled.",
                Table::default().char("file", file)
            ));
        }
        while sq + FILE_LEN < BOARD_LEN {
            if let None = self.board[sq + FILE_LEN] {
                sq += FILE_LEN;
            } else {
                break;
            }
        }
        sq
    }

    /// The square with the top piece of the specified row.  
    /// 指定した行の一番上のピースがあるマスです。  
    pub fn peak_sq_in_file(&mut self, file: char) -> Option<usize> {
        let mut sq = file_to_num(file) as usize;
        while sq < BOARD_LEN {
            if let None = self.board[sq] {
                sq += FILE_LEN;
            } else {
                return Some(sq);
            }
        }
        None
    }

    /// Is the file fill?  
    /// 列は埋まっていますか？  
    pub fn is_file_fill(&self, file: char) -> bool {
        let sq = file_to_num(file) as usize;
        if let Some(_) = self.board[sq] {
            true
        } else {
            false
        }
    }
}
