// Publish:
//
// (1) `cargo test`
// (2) `cargo run`
// (3) Open auto-generated log file. I check it.
// (4) Remove the log file.
// (5) Version up on Cargo.toml.
// (6) `cargo doc --open`
// (7) Comit to Git-hub.
// (8) `cargo publish --dry-run`
// (9) `cargo publish`

mod log;
mod test;

use crate::test::test;
use casual_logger::{Level, Log};

/// Evaluation file name.  
/// 評価値のファイル名。  
pub const EVALUATION_FILE_NAME: &'static str = "evaluation.csv";

fn main() {
    // Log file name.
    // ログ ファイル名。
    Log::set_file_name("kifuwarabe-dynamic-programming");
    // Log level.
    // ログ レベル。
    Log::set_level(Level::Debug);
    // Log file retention days.
    // ログ ファイル保持日数。
    Log::set_retention_days(2);
    // Remove old log files. This is determined by the date in the filename.
    // 古いログファイルを削除します。これは、ファイル名の日付によって決定されます。
    Log::remove_old_logs();

    // Test.
    // テスト。
    if Log::enabled(Level::Debug) {
        test();
    }

    // Wait for logging to complete.
    // ロギングが完了するまで待ちます。
    Log::flush();
}
