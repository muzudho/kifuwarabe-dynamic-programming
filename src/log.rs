//! Extend the functionality of the log.  
//! ログの機能を拡張します。  
use casual_logger::{Level, Log, Table};

/// Extend the functionality of the log.  
/// ログの機能を拡張します。  
pub trait LogExt {
    /// Display 'debug' level messages and write to log.  
    /// デバッグ レベル メッセージを表示し、ログに書き込みます。  
    ///
    /// # Arguments
    ///
    /// * `s` - Message.  
    ///         メッセージ。  
    fn print_debug(s: &str);
    /// Display 'info' level messages and write to log.  
    /// 情報レベル メッセージを表示し、ログに書き込みます。  
    ///
    /// # Arguments
    ///
    /// * `s` - Message.  
    ///         メッセージ。  
    fn print_info(s: &str);
    /// Display 'notice' level messages and write to log.  
    /// 通知レベル メッセージを表示し、ログに書き込みます。  
    ///
    /// # Arguments
    ///
    /// * `s` - Message.  
    ///         メッセージ。  
    fn print_notice(s: &str);
    /// Display 'error' level messages and write to log.  
    /// エラー レベル メッセージを表示し、ログに書き込みます。  
    ///
    /// # Arguments
    ///
    /// * `s` - Message.  
    ///         メッセージ。  
    fn print_error(s: &str);
    /// Display 'fatal' level messages and write to log.  
    /// 致命的レベル メッセージを表示し、ログに書き込みます。  
    ///
    /// # Arguments
    ///
    /// * `s` - Message.  
    ///         メッセージ。  
    ///
    /// # Returns
    ///
    /// メッセージ。
    fn print_fatal(s: &str) -> String;

    /// Display 'fatal' level messages and write to log.  
    /// 致命的レベル メッセージを表示し、ログに書き込みます。  
    ///
    /// # Arguments
    ///
    /// * `s` - Message.  
    ///         メッセージ。  
    /// * `t` - Table.  
    ///         テーブル。  
    ///
    /// # Returns
    ///
    /// メッセージ。
    fn print_fatal_t(s: &str, t: &mut Table) -> String {
        // In the Computer Shogi USI protocol, "info string" is a display text.
        // コンピューター将棋の USIプロトコル で 'info string' というのがあって
        // 強制終了の直前に画面に出せるかもしれないから付けています。
        Log::fatal_t(&format!("info string panic! {}", s), t)
    }
}
impl LogExt for Log {
    /// Display 'debug' level messages and write to log.  
    /// デバッグ レベル メッセージを表示し、ログに書き込みます。  
    ///
    /// # Arguments
    ///
    /// * `s` - Message.  
    ///         メッセージ。  
    fn print_debug(s: &str) {
        if Log::enabled(Level::Debug) {
            println!("{}", s);
        }
        Log::debug(s);
    }

    /// Display 'info' level messages and write to log.  
    /// 情報レベル メッセージを表示し、ログに書き込みます。  
    ///
    /// # Arguments
    ///
    /// * `s` - Message.  
    ///         メッセージ。  
    fn print_info(s: &str) {
        if Log::enabled(Level::Info) {
            println!("{}", s);
        }
        Log::info(s);
    }

    /// Display 'notice' level messages and write to log.  
    /// 通知レベル メッセージを表示し、ログに書き込みます。  
    ///
    /// # Arguments
    ///
    /// * `s` - Message.  
    ///         メッセージ。  
    fn print_notice(s: &str) {
        if Log::enabled(Level::Notice) {
            println!("{}", s);
        }
        Log::notice(s);
    }

    /// Display 'error' level messages and write to log.  
    /// エラー レベル メッセージを表示し、ログに書き込みます。  
    ///
    /// # Arguments
    ///
    /// * `s` - Message.  
    ///         メッセージ。  
    fn print_error(s: &str) {
        if Log::enabled(Level::Notice) {
            println!("{}", s);
        }
        Log::error(s);
    }

    /// Display 'fatal' level messages and write to log.  
    /// 致命的レベル メッセージを表示し、ログに書き込みます。  
    ///
    /// # Arguments
    ///
    /// * `s` - Message.  
    ///         メッセージ。  
    ///
    /// # Returns
    ///
    /// メッセージ。
    fn print_fatal(s: &str) -> String {
        // In the Computer Shogi USI protocol, "info string" is a display text.
        // コンピューター将棋の USIプロトコル で 'info string' というのがあって
        // 強制終了の直前に画面に出せるかもしれないから付けています。
        Log::fatal(&format!("info string panic! {}", s))
    }

    /// Display 'fatal' level messages and write to log.  
    /// 致命的レベル メッセージを表示し、ログに書き込みます。  
    ///
    /// # Arguments
    ///
    /// * `s` - Message.  
    ///         メッセージ。  
    /// * `t` - Table.  
    ///         テーブル。  
    ///
    /// # Returns
    ///
    /// メッセージ。
    fn print_fatal_t(s: &str, t: &mut Table) -> String {
        // In the Computer Shogi USI protocol, "info string" is a display text.
        // コンピューター将棋の USIプロトコル で 'info string' というのがあって
        // 強制終了の直前に画面に出せるかもしれないから付けています。
        Log::fatal_t(&format!("info string panic! {}", s), t)
    }
}
