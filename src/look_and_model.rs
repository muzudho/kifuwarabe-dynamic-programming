//! Display and data structure.  
//! 表示と、データ構造です。  
use crate::{
    Engine, GameResult, Piece, Position, ResultChannel, SearchDirection, SearchInfo, WayValue,
    BOARD_LEN, FILE_LEN, SQUARES_NUM,
};
use std::fmt;

impl Engine {
    /// Display the title.  
    /// タイトルを表示します。  
    pub fn title(&self) -> &str {
        "Kifuwarabe's connect-four
きふわらべのコネクト・フォー

Command:
コマンド:
`do d`      - Mark the d file.
                手番のプレイヤーが、 7 列目に印を付けます。
`go`        - The computer shows the next move.
                コンピューターが次の1手を示します。
`info-off`  - no info output.
                info出力なし。
`info-on`   - There is info output.(Default)
                info出力あり(既定)。
`learn`     - Learning.
                学習。
`pos`       - Position display.
                局面表示。
`position xfen 7/7/7/7/7/7 O moves d c`
            - Starting position and moves.
                初期局面と棋譜を入力。
`uh`        - Step of learning.
                学習のステップ。
`undo`      - 1 back.
                1手戻します。
`uxi`       - Returns 'uxiok connect-four {protocol-version}'. It is a version of the protocol, not software.
                'uxiok connect-four {protocol-version}' を返します。ソフトではなくプロトコルのバージョンです。
`xfen`      - The current xfen string display.
                現局面のxfen文字列表示。

Let's input from `pos`.
`pos` から入力してみましょう。
"
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::Piece::*;
        match self {
            Nought => write!(f, "O"),
            Cross => write!(f, "X"),
        }
    }
}

impl fmt::Display for GameResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::GameResult::*;
        match self {
            Win => write!(f, "win"),
            Draw => write!(f, "draw"),
            Lose => write!(f, "lose"),
        }
    }
}

impl fmt::Display for WayValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::WayValue::*;
        match self {
            Win => write!(f, "win"),
            PossiblyWin => write!(f, "possibly_win"),
            Draw => write!(f, "draw"),
            PossiblyDraw => write!(f, "possibly_draw"),
            PossiblyLose => write!(f, "possibly_lose"),
            Lose => write!(f, "lose"),
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Position {
            starting_turn: Piece::Nought,
            starting_board: [None; BOARD_LEN],
            starting_pieces_num: 0,
            turn: Piece::Nought,
            board: [None; BOARD_LEN],
            history: [' '; SQUARES_NUM],
            pieces_num: 0,
            info_enabled: true,
        }
    }
}
impl Position {
    /// Write on the pv.
    /// 読み筋に書きます。
    pub fn pv_json(&self) -> String {
        let mut text = String::new();

        for file in self.history.iter() {
            if *file == ' ' {
                break;
            } else if text.is_empty() {
                text.push_str(&format!("\"{}\"", file).to_string());
            } else {
                text.push_str(&format!(",\"{}\"", file).to_string());
            }
        }

        text
    }

    /// Display of square.  
    /// マスの表示。  
    fn cell(&self, index: usize) -> String {
        if let Some(piece) = self.board[index] {
            format!(" {} ", piece)
        } else {
            "   ".to_string()
        }
    }
    /// Display of position.  
    /// 局面の表示。  
    pub fn pos(&self) -> String {
        let s = &mut format!(
            "[Next {} piece(s) | Go {}]

",
            self.pieces_num + 1,
            self.turn
        );
        s.push_str(&format!(
            "  +---+---+---+---+---+---+---+ Please select a file. Example `do d`
6 |{0}|{1}|{2}|{3}|{4}|{5}|{6}| 列を選んでください。例 `do d`
  +---+---+---+---+---+---+---+
5 |{7}|{8}|{9}|{10}|{11}|{12}|{13}|    a b c d e f g
  +---+---+---+---+---+---+---+
4 |{14}|{15}|{16}|{17}|{18}|{19}|{20}|
  +---+---+---+---+---+---+---+
3 |{21}|{22}|{23}|{24}|{25}|{26}|{27}|
  +---+---+---+---+---+---+---+
2 |{28}|{29}|{30}|{31}|{32}|{33}|{34}|
  +---+---+---+---+---+---+---+
1 |{35}|{36}|{37}|{38}|{39}|{40}|{41}|
  +---+---+---+---+---+---+---+
    a   b   c   d   e   f   g",
            self.cell(0),
            self.cell(1),
            self.cell(2),
            self.cell(3),
            self.cell(4),
            self.cell(5),
            self.cell(6),
            self.cell(7),
            self.cell(8),
            self.cell(9),
            self.cell(10),
            self.cell(11),
            self.cell(12),
            self.cell(13),
            self.cell(14),
            self.cell(15),
            self.cell(16),
            self.cell(17),
            self.cell(18),
            self.cell(19),
            self.cell(20),
            self.cell(21),
            self.cell(22),
            self.cell(23),
            self.cell(24),
            self.cell(25),
            self.cell(26),
            self.cell(27),
            self.cell(28),
            self.cell(29),
            self.cell(30),
            self.cell(31),
            self.cell(32),
            self.cell(33),
            self.cell(34),
            self.cell(35),
            self.cell(36),
            self.cell(37),
            self.cell(38),
            self.cell(39),
            self.cell(40),
            self.cell(41)
        ));
        s.to_string()
    }

    /// Display results.  
    /// 結果の表示。  
    pub fn result(result: GameResult, winner: Option<Piece>) -> Option<String> {
        use crate::GameResult::*;
        match result {
            // ぜったい None が返ってこない仕様のときは .unwrap() でヌル・チェックを飛ばせだぜ☆（＾～＾）
            Win => Some(format!("win {}", winner.unwrap()).to_string()),
            Draw => Some(format!("draw").to_string()),
            Lose => None,
        }
    }
}

impl Default for SearchInfo {
    fn default() -> Self {
        SearchInfo {
            result_channel: ResultChannel::Win,
            weight_tensor: [
                [0, 0, 0, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
            ],
            nps: 0,
            nodes: 0,
            pv_json: "".to_string(),
            search_direction: SearchDirection::Forward,
            chosen_file: None,
            leaf: false,
            pieces_num: None,
            way_value: None,
            turn: Piece::Nought,
            comment: None,
        }
    }
}
impl SearchInfo {
    pub fn get_total_weight(&self) -> u16 {
        let mut sum: u16 = 0;
        for i in 0..FILE_LEN {
            sum += self.weight_tensor[i][0] as u16
                + self.weight_tensor[i][1] as u16
                + self.weight_tensor[i][2] as u16
                + self.weight_tensor[i][3] as u16;
        }
        sum
    }

    /// Information during a forward/backward search.  
    /// 前向き/後ろ向き 探索中の情報。  
    pub fn to_string(&self) -> String {
        format!(
            "info json {{ \"nps\":{: >6}, \"nodes\":{: >6}{}{}{}{}{}, \"channel1\":{:?}, \"choose\":\"{}\", \"total\":{}, \"a\":{}, \"b\":{}, \"c\":{}, \"d\":{}, \"e\":{}, \"f\":{}, \"g\":{}, \"pv\":[{}] }}",
            self.nps,
            self.nodes,
            if let Some(file) = self.chosen_file {
                use crate::SearchDirection::*;
                match self.search_direction {
                    Forward => format!(", \"push\":\"{}\"", file),
                    Backward => format!(", \"pop\" :\"{}\"", file),
                }
            } else {
                "            ".to_string()
            },
            if let Some(pieces_num) = self.pieces_num {
                format!(", \"pieces\":{}", pieces_num)
            } else {
                "            ".to_string()
            },
            if self.leaf {
                ", \"leaf\": true"
            } else {
                "              "
            },
            if let Some(way_value) = self.way_value {
                // length of possibly_lose is 13.
                format!(", \"value\":\"{:13}\"", way_value)
            } else {
                "                         ".to_string()
            },
            if let Some(comment) = &self.comment {
                format!(", \"{}\":\"{}\"", self.turn, comment).to_string()
            } else {
                format!(", \"{}\":\"\"", self.turn).to_string()
            },
            self.result_channel,
            if let Some(file) = self.chosen_file {
                file
            }else{
                ' '
            },
            self.get_total_weight(),
            self.weight_tensor[0][0] as u16 + self.weight_tensor[0][1] as u16 + self.weight_tensor[0][2] as u16 + self.weight_tensor[0][3] as u16,
            self.weight_tensor[1][0] as u16 + self.weight_tensor[1][1] as u16 + self.weight_tensor[1][2] as u16 + self.weight_tensor[1][3] as u16,
            self.weight_tensor[2][0] as u16 + self.weight_tensor[2][1] as u16 + self.weight_tensor[2][2] as u16 + self.weight_tensor[2][3] as u16,
            self.weight_tensor[3][0] as u16 + self.weight_tensor[3][1] as u16 + self.weight_tensor[3][2] as u16 + self.weight_tensor[3][3] as u16,
            self.weight_tensor[4][0] as u16 + self.weight_tensor[4][1] as u16 + self.weight_tensor[4][2] as u16 + self.weight_tensor[4][3] as u16,
            self.weight_tensor[5][0] as u16 + self.weight_tensor[5][1] as u16 + self.weight_tensor[5][2] as u16 + self.weight_tensor[5][3] as u16,
            self.weight_tensor[6][0] as u16 + self.weight_tensor[6][1] as u16 + self.weight_tensor[6][2] as u16 + self.weight_tensor[6][3] as u16,
            self.pv_json,
        )
        .to_string()
    }
}
