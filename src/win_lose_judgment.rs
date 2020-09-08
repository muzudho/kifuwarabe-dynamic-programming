//! Win/Lose judgment.  
//! 勝敗判定。  
use crate::{Piece, Position, BOARD_LEN, FILE_LEN, RANK_LEN};

/// A record of the game used to suspend or resume it.  
/// ゲームを中断したり、再開したりするときに使うゲームの記録です。  
impl Position {
    /// It will now be determined whether the player who already placed the stone, not the player who placed the stone, won.
    /// これから石を置くプレイヤーではなく、既に石を置いたプレイヤーが勝ったか判定します。  
    pub fn is_opponent_win(&self) -> bool {
        let opponent = self.opponent();

        // Files.
        //
        // X......
        // X......
        // X......
        // X......
        // .......
        // .......
        //
        // to
        //
        // .......
        // .......
        // ......X
        // ......X
        // ......X
        // ......X
        for reverse_rank in 0..3 {
            for file in 0..FILE_LEN {
                if self.is_vertical(opponent, file, reverse_rank) {
                    return true;
                }
            }
        }
        // Ranks.
        //
        // XXXX...
        // .......
        // .......
        // .......
        // .......
        // .......
        //
        // to
        //
        // .......
        // .......
        // .......
        // .......
        // .......
        // ...XXXX
        for reverse_rank in 0..RANK_LEN {
            for file in 0..4 {
                if self.is_horizontal(opponent, file, reverse_rank) {
                    return true;
                }
            }
        }
        // Baroque diagonal
        //
        // ...X...
        // ..X....
        // .X.....
        // X......
        // .......
        // .......
        //
        // to
        //
        // .......
        // .......
        // ......X
        // .....X.
        // ....X..
        // ...X...
        //
        // [(start, repeat), ...]
        for start_repeat in &[(3, 1), (4, 2), (5, 3), (6, 3), (13, 2), (20, 1)] {
            for i in 0..start_repeat.1 {
                if self.is_baroque_diagonal(opponent, start_repeat.0, i) {
                    return true;
                }
            }
        }
        // Sinister diagonal
        //
        // ...X...
        // ....X..
        // .....X.
        // ......X
        // .......
        // .......
        //
        // to
        //
        // .......
        // .......
        // X......
        // .X.....
        // ..X....
        // ...X...
        //
        // [(start, repeat), ...]
        for start_repeat in &[(14, 1), (7, 2), (0, 3), (1, 3), (2, 2), (3, 1)] {
            for i in 0..start_repeat.1 {
                if self.is_sinister_diagonal(opponent, start_repeat.0, i) {
                    return true;
                }
            }
        }

        false
    }

    /// If the player who has already placed the stone is not winning
    /// and the current player has no place to place the stone, it is a draw.
    /// 既に石を置いたプレイヤーが勝っていなくて、今のプレイヤーが石を置く場所がなければ引き分けです。
    pub fn is_draw(&self) -> bool {
        if self.is_opponent_win() {
            return false;
        }
        for sq in 1..BOARD_LEN {
            if let None = self.board[sq] {
                return false;
            }
        }

        true
    }

    /// # Arguments
    ///
    /// * `y` - Y-axis is down.  
    ///         Y軸は下向きです。  
    fn is_vertical(&self, opponent: Piece, x: usize, y: usize) -> bool {
        Some(opponent) == self.board[y * FILE_LEN + x]
            && Some(opponent) == self.board[(y + 1) * FILE_LEN + x]
            && Some(opponent) == self.board[(y + 2) * FILE_LEN + x]
            && Some(opponent) == self.board[(y + 3) * FILE_LEN + x]
    }

    /// # Arguments
    ///
    /// * `y` - Y-axis is down.  
    ///         Y軸は下向きです。  
    fn is_horizontal(&self, opponent: Piece, x: usize, y: usize) -> bool {
        Some(opponent) == self.board[y * FILE_LEN + x]
            && Some(opponent) == self.board[y * FILE_LEN + x + 1]
            && Some(opponent) == self.board[y * FILE_LEN + x + 2]
            && Some(opponent) == self.board[y * FILE_LEN + x + 3]
    }

    /// # Arguments
    ///
    /// * `y` - Y-axis is down.  
    ///         Y軸は下向きです。  
    fn is_baroque_diagonal(&self, opponent: Piece, sq: usize, i: usize) -> bool {
        Some(opponent) == self.board[sq + i * (FILE_LEN - 1)]
            && Some(opponent) == self.board[sq + (i + 1) * (FILE_LEN - 1)]
            && Some(opponent) == self.board[sq + (i + 2) * (FILE_LEN - 1)]
            && Some(opponent) == self.board[sq + (i + 3) * (FILE_LEN - 1)]
    }

    /// # Arguments
    ///
    /// * `y` - Y-axis is down.  
    ///         Y軸は下向きです。  
    fn is_sinister_diagonal(&self, opponent: Piece, sq: usize, i: usize) -> bool {
        Some(opponent) == self.board[sq + i * (FILE_LEN + 1)]
            && Some(opponent) == self.board[sq + (i + 1) * (FILE_LEN + 1)]
            && Some(opponent) == self.board[sq + (i + 2) * (FILE_LEN + 1)]
            && Some(opponent) == self.board[sq + (i + 3) * (FILE_LEN + 1)]
    }
}
