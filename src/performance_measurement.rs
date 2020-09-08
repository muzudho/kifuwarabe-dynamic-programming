//! Measure performance.  
//! 性能を測ります。  
use crate::computer_player::Search;

/// Search.  
/// 探索部。  
impl Search {
    /// Seconds.  
    /// 秒。  
    pub fn sec(&self) -> u64 {
        self.stopwatch.elapsed().as_secs()
    }

    /// Node per second.  
    /// １秒当たりの状態ノード数。  
    pub fn nps(&self) -> u64 {
        let sec = self.sec();
        if 0 < sec {
            self.nodes as u64 / sec
        } else {
            // I searched everything in less than a second. There really should be more.
            // 1秒未満で全部探索してしまった。 本当は もっと多いはず。
            self.nodes as u64
        }
    }
}
