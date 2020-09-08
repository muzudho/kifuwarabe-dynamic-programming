//! Let's proceed with development while testing.  
//! テストしながら開発を進めましょう。  

use crate::command_line_seek::CommandLineSeek;
use crate::computer_player::Evaluation;
use crate::log::LogExt;
use crate::{
  computer_player::Search, GameResult, Piece, Position, ResultChannel, SearchDirection, SearchInfo,
  WayValue,
};
use casual_logger::Log;
use std::{thread, time};
// use std;

/// It is a unit test. I am writing it here because it is a hassle.
/// Check it against the explanation in README.md.
/// 単体テストです。めんどくさいのでここに書いています。
/// README.mdの解説と照らし合わせてみてください。
pub fn test() {
  // Step 1.
  Log::debugln("Hello, world!!");
  Log::print_debug("こんにちわ、世界！！");
  // こんにちわ、世界！！

  // Step 2 is this.

  // Step 3.
  Log::print_debug(&format!("Nought=|{}|", Piece::Nought));
  // Nought=|O|
  Log::print_debug(&format!("Cross =|{}|", Piece::Cross));
  // Cross =|X|
  Log::print_debug(&format!("Win   =|{}|", GameResult::Win));
  // Win   =|win|
  Log::print_debug(&format!("Draw  =|{}|", GameResult::Draw));
  // Draw  =|draw|
  Log::print_debug(&format!("Lose  =|{}|", GameResult::Lose));
  // Lose  =|lose|

  let mut pos = Position::default();
  Log::print_debug(&pos.pos());
  /*
  [Next 1 piece(s) | Go O]

    +---+---+---+---+---+---+---+ Please select a file. Example `do d`
  6 |   |   |   |   |   |   |   | 列を選んでください。例 `do d`
    +---+---+---+---+---+---+---+
  5 |   |   |   |   |   |   |   |    a b c d e f g
    +---+---+---+---+---+---+---+
  4 |   |   |   |   |   |   |   |
    +---+---+---+---+---+---+---+
  3 |   |   |   |   |   |   |   |
    +---+---+---+---+---+---+---+
  2 |   |   |   |   |   |   |   |
    +---+---+---+---+---+---+---+
  1 |   |   |   |   |   |   |   |
    +---+---+---+---+---+---+---+
      a   b   c   d   e   f   g
       */
  // If not None is returned, .unwrap() skips the None check.
  // ぜったい None が返ってこないときは .unwrap() で None チェックを飛ばします。
  Log::print_debug(&Position::result(GameResult::Win, Some(Piece::Nought)).unwrap());
  // win O

  let mut search = Search::default();
  search.start_pieces_num = pos.pieces_num;
  Log::print_debug(&format!("pv=|{}|", pos.pv_json()));
  // pv=||
  // 適当な内容を入れて、入れ物として、入れた中身を見せてくれるか、チェックしろだぜ☆（＾～＾）
  let mut search_info = SearchInfo::default();
  search_info.result_channel = ResultChannel::Win;
  search_info.weight_tensor = [
    [1, 1, 1, 1],
    [1, 1, 1, 1],
    [1, 1, 1, 1],
    [1, 1, 1, 1],
    [1, 1, 1, 1],
    [1, 1, 1, 1],
    [1, 1, 1, 1],
  ];
  search_info.nps = 123;
  search_info.nodes = search.nodes;
  search_info.pv_json = pos.pv_json();
  search_info.search_direction = SearchDirection::Forward;
  search_info.chosen_file = Some('d');
  search_info.leaf = false;
  search_info.pieces_num = None;
  search_info.turn = Piece::Nought;
  search_info.comment = Some("Hello!".to_string());
  Log::print_debug(&search_info.to_string());
  // info json { "nps":   123, "nodes":     0, "push":"d"                                           , "O":"Hello!", "pv":[] }
  search_info = SearchInfo::default();
  search_info.result_channel = ResultChannel::Win;
  search_info.weight_tensor = [
    [1, 1, 1, 1],
    [1, 1, 1, 1],
    [1, 1, 1, 1],
    [1, 1, 1, 1],
    [1, 1, 1, 1],
    [1, 1, 1, 1],
    [1, 1, 1, 1],
  ];
  search_info.nps = 456;
  search_info.nodes = search.nodes;
  search_info.pv_json = pos.pv_json();
  search_info.search_direction = SearchDirection::Forward;
  search_info.chosen_file = Some('d');
  search_info.leaf = true;
  search_info.pieces_num = None;
  search_info.way_value = Some(WayValue::Win);
  search_info.turn = Piece::Cross;
  search_info.comment = Some("Hello!".to_string());
  Log::print_debug(&search_info.to_string());
  // info json { "nps":   456, "nodes":     0, "push":"d"            , "leaf": true, "result":"win" , "X":"Hello!", "pv":[] }
  search_info = SearchInfo::default();
  search_info.result_channel = ResultChannel::Win;
  search_info.weight_tensor = [
    [1, 1, 1, 1],
    [1, 1, 1, 1],
    [1, 1, 1, 1],
    [1, 1, 1, 1],
    [1, 1, 1, 1],
    [1, 1, 1, 1],
    [1, 1, 1, 1],
  ];
  search_info.nps = 789;
  search_info.nodes = search.nodes;
  search_info.pv_json = pos.pv_json();
  search_info.search_direction = SearchDirection::Backward;
  search_info.chosen_file = Some('d');
  search_info.leaf = false;
  search_info.pieces_num = Some(pos.pieces_num);
  search_info.way_value = Some(WayValue::Win);
  search_info.turn = Piece::Nought;
  search_info.comment = Some("Hello!".to_string());
  Log::print_debug(&search_info.to_string());
  // info json { "nps":   789, "nodes":     0, "pop" :"d", "pieces":0              , "result":"win" , "O":"Hello!", "pv":[] }

  // Step 4.
  pos.do_move('d');
  Log::print_debug(&pos.pos());
  /*
  [Next 2 piece(s) | Go X]

    +---+---+---+---+---+---+---+ Please select a file. Example `do d`
  6 |   |   |   |   |   |   |   | 列を選んでください。例 `do d`
    +---+---+---+---+---+---+---+
  5 |   |   |   |   |   |   |   |    a b c d e f g
    +---+---+---+---+---+---+---+
  4 |   |   |   |   |   |   |   |
    +---+---+---+---+---+---+---+
  3 |   |   |   |   |   |   |   |
    +---+---+---+---+---+---+---+
  2 |   |   |   |   |   |   |   |
    +---+---+---+---+---+---+---+
  1 |   |   |   | O |   |   |   |
    +---+---+---+---+---+---+---+
      a   b   c   d   e   f   g
  */
  pos.undo_move();
  Log::print_debug(&pos.pos());
  /*
  [Next 1 piece(s) | Go O]

    +---+---+---+---+---+---+---+ Please select a file. Example `do d`
  6 |   |   |   |   |   |   |   | 列を選んでください。例 `do d`
    +---+---+---+---+---+---+---+
  5 |   |   |   |   |   |   |   |    a b c d e f g
    +---+---+---+---+---+---+---+
  4 |   |   |   |   |   |   |   |
    +---+---+---+---+---+---+---+
  3 |   |   |   |   |   |   |   |
    +---+---+---+---+---+---+---+
  2 |   |   |   |   |   |   |   |
    +---+---+---+---+---+---+---+
  1 |   |   |   |   |   |   |   |
    +---+---+---+---+---+---+---+
      a   b   c   d   e   f   g
  */
  Log::print_debug(&format!("opponent=|{}|", pos.opponent()));
  // opponent=|X|

  // Step 5.
  let mut p = CommandLineSeek::new("Go to the Moon!");
  Log::print_debug(&format!("Go to   =|{}|", p.starts_with("Go to")));
  // Go to   =|true|
  Log::print_debug(&format!("Goto    =|{}|", p.starts_with("Goto")));
  // Goto    =|false|
  Log::print_debug(&format!("p.starts=|{}|", p.current()));
  // p.starts=|0|
  Log::print_debug(&format!(
    "p.rest  =|{}|",
    if let Some(rest) = p.rest() { rest } else { "" }
  ));
  // p.rest  =|Go to the Moon!|
  p.go_next_to("Go to");
  Log::print_debug(&format!("p.starts=|{}|", p.current()));
  // p.starts=|5|
  Log::print_debug(&format!(
    "p.rest  =|{}|",
    if let Some(rest) = p.rest() { rest } else { "" }
  ));
  // p.rest  =| the Moon!|

  // Step 6.
  Log::print_debug(&format!("xfen=|{}|", pos.to_xfen()));
  // xfen=|xfen 7/7/7/7/7/7 O|
  pos.do_("d");
  Log::print_debug(&pos.pos());
  /*
  [Next 2 piece(s) | Go X]

    +---+---+---+---+---+---+---+ Please select a file. Example `do d`
  6 |   |   |   |   |   |   |   | 列を選んでください。例 `do d`
    +---+---+---+---+---+---+---+
  5 |   |   |   |   |   |   |   |    a b c d e f g
    +---+---+---+---+---+---+---+
  4 |   |   |   |   |   |   |   |
    +---+---+---+---+---+---+---+
  3 |   |   |   |   |   |   |   |
    +---+---+---+---+---+---+---+
  2 |   |   |   |   |   |   |   |
    +---+---+---+---+---+---+---+
  1 |   |   |   | O |   |   |   |
    +---+---+---+---+---+---+---+
      a   b   c   d   e   f   g
  */
  let xfen = "xfen XOXOXOX/OXOXOXO/XOXOXOX/OXOXOXO/XOXOXOX/OXOXOXO O";
  pos = if let Some(pos) = Position::from_xfen(xfen) {
    pos
  } else {
    panic!(Log::print_fatal(&format!("Invalid xfen=|{}|", xfen)))
  };
  Log::print_debug(&pos.pos());
  /*
  [Next 43 piece(s) | Go O]

    +---+---+---+---+---+---+---+ Please select a file. Example `do d`
  6 | X | O | X | O | X | O | X | 列を選んでください。例 `do d`
    +---+---+---+---+---+---+---+
  5 | O | X | O | X | O | X | O |    a b c d e f g
    +---+---+---+---+---+---+---+
  4 | X | O | X | O | X | O | X |
    +---+---+---+---+---+---+---+
  3 | O | X | O | X | O | X | O |
    +---+---+---+---+---+---+---+
  2 | X | O | X | O | X | O | X |
    +---+---+---+---+---+---+---+
  1 | O | X | O | X | O | X | O |
    +---+---+---+---+---+---+---+
      a   b   c   d   e   f   g
  */
  let xfen = "xfen 7/7/7/7/5X1/1O1O1X1 O";
  pos = if let Some(pos) = Position::from_xfen(xfen) {
    pos
  } else {
    panic!(Log::print_fatal(&format!("Invalid xfen=|{}|", xfen)))
  };
  Log::print_debug(&pos.pos());
  /*
  [Next 5 piece(s) | Go O]

    +---+---+---+---+---+---+---+ Please select a file. Example `do d`
  6 |   |   |   |   |   |   |   | 列を選んでください。例 `do d`
    +---+---+---+---+---+---+---+
  5 |   |   |   |   |   |   |   |    a b c d e f g
    +---+---+---+---+---+---+---+
  4 |   |   |   |   |   |   |   |
    +---+---+---+---+---+---+---+
  3 |   |   |   |   |   |   |   |
    +---+---+---+---+---+---+---+
  2 |   |   |   |   |   | X |   |
    +---+---+---+---+---+---+---+
  1 |   | O |   | O |   | X |   |
    +---+---+---+---+---+---+---+
      a   b   c   d   e   f   g
  */
  let xfen = "xfen 7/7/7/7/7/7 O moves d f b f d f b f";
  pos = if let Some(pos) = Position::from_xfen(xfen) {
    pos
  } else {
    panic!(Log::print_fatal(&format!("Invalid xfen=|{}|", xfen)))
  };
  Log::print_debug(&pos.pos());
  /*
  [Next 9 piece(s) | Go O]

    +---+---+---+---+---+---+---+ Please select a file. Example `do d`
  6 |   |   |   |   |   |   |   | 列を選んでください。例 `do d`
    +---+---+---+---+---+---+---+
  5 |   |   |   |   |   |   |   |    a b c d e f g
    +---+---+---+---+---+---+---+
  4 |   |   |   |   |   | X |   |
    +---+---+---+---+---+---+---+
  3 |   |   |   |   |   | X |   |
    +---+---+---+---+---+---+---+
  2 |   | O |   | O |   | X |   |
    +---+---+---+---+---+---+---+
  1 |   | O |   | O |   | X |   |
    +---+---+---+---+---+---+---+
      a   b   c   d   e   f   g
  */
  pos.undo();
  Log::print_debug(&pos.pos());
  /*
  [Next 8 piece(s) | Go X]

    +---+---+---+---+---+---+---+ Please select a file. Example `do d`
  6 |   |   |   |   |   |   |   | 列を選んでください。例 `do d`
    +---+---+---+---+---+---+---+
  5 |   |   |   |   |   |   |   |    a b c d e f g
    +---+---+---+---+---+---+---+
  4 |   |   |   |   |   |   |   |
    +---+---+---+---+---+---+---+
  3 |   |   |   |   |   | X |   |
    +---+---+---+---+---+---+---+
  2 |   | O |   | O |   | X |   |
    +---+---+---+---+---+---+---+
  1 |   | O |   | O |   | X |   |
    +---+---+---+---+---+---+---+
      a   b   c   d   e   f   g
  */

  // Step 7.
  // Step 8.
  // Step 9.
  let xfen = "xfen 7/7/5X1/5X1/1O1O1X1/1O1O1X1 O";
  pos = if let Some(pos) = Position::from_xfen(xfen) {
    pos
  } else {
    panic!(Log::print_fatal(&format!("Invalid xfen=|{}|", xfen)))
  };
  Log::print_debug(&pos.pos());
  Log::print_debug(&format!("win=|{}|", pos.is_opponent_win()));
  /*
  [Next 9 piece(s) | Go O]

    +---+---+---+---+---+---+---+ Please select a file. Example `do d`
  6 |   |   |   |   |   |   |   | 列を選んでください。例 `do d`
    +---+---+---+---+---+---+---+
  5 |   |   |   |   |   |   |   |    a b c d e f g
    +---+---+---+---+---+---+---+
  4 |   |   |   |   |   | X |   |
    +---+---+---+---+---+---+---+
  3 |   |   |   |   |   | X |   |
    +---+---+---+---+---+---+---+
  2 |   | O |   | O |   | X |   |
    +---+---+---+---+---+---+---+
  1 |   | O |   | O |   | X |   |
    +---+---+---+---+---+---+---+
      a   b   c   d   e   f   g
  win=|true|
  */
  let xfen = "xfen XOXOXOX/OXOXOXO/XOXOXOX/XOXOXOX/OXOXOXO/OXOXOXO O";
  pos = if let Some(pos) = Position::from_xfen(xfen) {
    pos
  } else {
    panic!(Log::print_fatal(&format!("Invalid xfen=|{}|", xfen)))
  };
  Log::print_debug(&pos.pos());
  Log::print_debug(&format!("draw=|{}|", pos.is_draw()));
  /*
  [Next 43 piece(s) | Go O]

    +---+---+---+---+---+---+---+ Please select a file. Example `do d`
  6 | X | O | X | O | X | O | X | 列を選んでください。例 `do d`
    +---+---+---+---+---+---+---+
  5 | O | X | O | X | O | X | O |    a b c d e f g
    +---+---+---+---+---+---+---+
  4 | X | O | X | O | X | O | X |
    +---+---+---+---+---+---+---+
  3 | X | O | X | O | X | O | X |
    +---+---+---+---+---+---+---+
  2 | O | X | O | X | O | X | O |
    +---+---+---+---+---+---+---+
  1 | O | X | O | X | O | X | O |
    +---+---+---+---+---+---+---+
      a   b   c   d   e   f   g
  draw=|true|
  */

  // Step 10.
  // Since we have not searched, both nodes and nps will be 0.
  // 探索してないので、 nodes も nps も 0 になります。
  thread::sleep(time::Duration::from_secs(1));
  Log::print_debug(&format!("nodes={}", search.nodes));
  // nodes=0
  Log::print_debug(&format!("sec  ={}", search.sec()));
  // sec  =1
  Log::print_debug(&format!("nps  ={}", search.nps()));
  // nps  =0

  /*
  // Step 11.
  let xfen = "xfen 3/3/3 o moves 1 5 2 3 7 4";
  pos = if let Some(pos) = Position::from_xfen(xfen) {
      pos
  } else {
      panic!(Log::print_fatal(&format!("Invalid xfen=|{}|", xfen)))
  };
  let mut search = Search::new(pos.pieces_num);
  let (sq, result) = search.go(&mut pos);
  // info string "nps":......, "nodes":......, "pv":[O,X,O,X,O,X,O,X,O]
  // info json { "nps":     1, "nodes":     1, "pv":[6                ], "push":"6",               "pieces":7,                  "turn":"X", "comment":"Search." }
  // info json { "nps":     2, "nodes":     2, "pv":[6,8              ], "push":"8",               "pieces":8,                  "turn":"O", "comment":"Search." }
  // info json { "nps":     3, "nodes":     3, "pv":[6,8,9            ], "push":"9", "leaf": true, "pieces":9, "result":"draw", "turn":"X", "comment":"It is ok." }
  // info json { "nps":     3, "nodes":     3, "pv":[6,8              ], "pop" :"9",               "pieces":8, "result":"draw", "turn":"O" }
  // info json { "nps":     3, "nodes":     3, "pv":[6                ], "pop" :"8",               "pieces":7, "result":"draw", "turn":"X", "comment":"Fmmm." }
  // info json { "nps":     4, "nodes":     4, "pv":[6,9              ], "push":"9",               "pieces":8,                  "turn":"O", "comment":"Search." }
  // info json { "nps":     5, "nodes":     5, "pv":[6,9,8            ], "push":"8", "leaf": true, "pieces":9, "result":"draw", "turn":"X", "comment":"It is ok." }
  // info json { "nps":     5, "nodes":     5, "pv":[6,9              ], "pop" :"8",               "pieces":8, "result":"draw", "turn":"O" }
  // info json { "nps":     5, "nodes":     5, "pv":[6                ], "pop" :"9",               "pieces":7, "result":"draw", "turn":"X", "comment":"Fmmm." }
  // info json { "nps":     5, "nodes":     5, "pv":[                 ], "pop" :"6",               "pieces":6, "result":"draw", "turn":"O", "comment":"Fmmm." }
  // info json { "nps":     6, "nodes":     6, "pv":[8                ], "push":"8",               "pieces":7,                  "turn":"X", "comment":"Search." }
  // info json { "nps":     7, "nodes":     7, "pv":[8,6              ], "push":"6", "leaf": true, "pieces":8, "result":"win" , "turn":"O", "comment":"Resign." }
  // info json { "nps":     7, "nodes":     7, "pv":[8                ], "pop" :"6",               "pieces":7, "result":"win" , "turn":"X" }
  // info json { "nps":     7, "nodes":     7, "pv":[                 ], "pop" :"8",               "pieces":6, "result":"lose", "turn":"O", "comment":"Damn!" }
  // info json { "nps":     8, "nodes":     8, "pv":[9                ], "push":"9",               "pieces":7,                  "turn":"X", "comment":"Search." }
  // info json { "nps":     9, "nodes":     9, "pv":[9,6              ], "push":"6", "leaf": true, "pieces":8, "result":"win" , "turn":"O", "comment":"Resign." }
  // info json { "nps":     9, "nodes":     9, "pv":[9                ], "pop" :"6",               "pieces":7, "result":"win" , "turn":"X" }
  // info json { "nps":     9, "nodes":     9, "pv":[                 ], "pop" :"9",               "pieces":6, "result":"lose", "turn":"O", "comment":"Damn!" }
  Log::print_debug(&format!("result=|{}|", result));
  // result=|draw|
  Log::print_debug(&format!(
      "bestmove=|{}|",
      if let Some(sq) = sq {
          format!("{}", sq).to_string()
      } else {
          "resign".to_string()
      }
  ));
  // bestmove=|6|

  // End.
  test_win_lose_judgement();
  */

  // Step.12
  let mut evaluation = Evaluation::default();
  evaluation.save("test-evaluation.csv");
  evaluation.load("test-evaluation.csv");

  // Wait for logging to complete.
  // ロギングが完了するまで待ちます。
  Log::flush();
}
