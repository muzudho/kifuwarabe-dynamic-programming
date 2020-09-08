//! Let's proceed with development while testing.  
//! テストしながら開発を進めましょう。  

use crate::log::LogExt;
use casual_logger::Log;

/// It is a unit test. I am writing it here because it is a hassle.
/// Check it against the explanation in README.md.
/// 単体テストです。めんどくさいのでここに書いています。
/// README.mdの解説と照らし合わせてみてください。
pub fn test() {
  // Step 2.
  Log::debugln("こんにちわ、世界！！");
  Log::print_debug("Hello, world!!");
  // Hello, world!!
}
