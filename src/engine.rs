//! ThinkingEngine.  
//! 思考エンジン。  

use crate::{
    command_line_seek::CommandLineSeek,
    computer_player::{Evaluation, Learning, Search},
    log::LogExt,
    Engine, Position,
};
use casual_logger::Log;

impl Default for Engine {
    fn default() -> Self {
        Engine {
            pos: Position::default(),
            evaluation: Evaluation::default(),
            bestmove: None,
            undone: false,
            game_result: None,
        }
    }
}
impl Engine {
    /// Enter the command line.  
    /// コマンドラインを与えてください。  
    ///
    /// # Arguments
    ///
    /// * `line` - Command line.  
    ///             コマンドライン。  
    ///
    /// # Returns
    ///
    /// If this response quit, exit the your application.  
    /// Quitならアプリケーションを終了してください。  
    pub fn enter(&mut self, line: &str) -> Option<Response> {
        // p is the acronym for parser.
        // p は parser の頭文字。
        let mut p = CommandLineSeek::new(&line);

        // It is in alphabetical order because it is easy to find.
        // 探しやすいからアルファベット順です。
        if p.starts_with("do") {
            p.go_next_to("do ");
            if let Some(rest) = p.rest() {
                self.game_result = self.pos.do_(rest);
            } else {
                self.game_result = None;
            }
        } else if p.starts_with("uh") {
            // uh...
            // うーん……。
            let mut learning = Learning::default();
            learning.uh(self);
        } else if p.starts_with("go") {
            let mut search = Search::default();
            search.start_pieces_num = self.pos.pieces_num;
            let bestmove = search.go(&mut self.pos, &self.evaluation);
            Log::print_info(&format!(
                "info string pred_result={:?} nps={}",
                bestmove.pred_result,
                search.nps()
            ));

            Log::print_notice(&format!(
                "bestmove {}",
                if let Some(file) = bestmove.file {
                    file.to_string()
                } else {
                    "resign".to_string()
                }
            ));

            self.bestmove = Some(bestmove);
        } else if p.starts_with("info-off") {
            self.pos.info_enabled = false;
        } else if p.starts_with("info-on") {
            self.pos.info_enabled = true;
        } else if p.starts_with("learn") {
            let mut learning = Learning::default();
            learning.learn(self);
        } else if p.starts_with("position") {
            p.go_next_to("position ");
            if let Some(rest) = p.rest() {
                if let Some(pos_val) = Position::from_xfen(rest) {
                    self.pos = pos_val;
                }
            }
        } else if p.starts_with("pos") {
            Log::print_notice(&self.pos.pos());
        } else if p.starts_with("quit") {
            return Some(Response::Quit);
        } else if p.starts_with("undo") {
            self.undone = self.pos.undo();
        } else if p.starts_with("uxi") {
            Log::print_notice("uxiok connect-four v20200824.0.0");
        } else if p.starts_with("xfen") {
            Log::print_notice(&format!("{}", self.pos.to_xfen()));
        } else {
            Log::print_debug(&format!("Debug   | Invalid command=|{:?}|", p));
        }

        None
    }
}

/// Engine response.
/// エンジンの応答。
pub enum Response {
    /// Quit.
    /// 終了。
    Quit,
}
