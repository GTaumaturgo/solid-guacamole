use crate::move_gen::MovesMap;
use crate::CACHE;

use super::bitboard::BitB64;
use super::PlayerColor;
use std::collections::{hash_map::Entry, HashMap};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum CachedScoreType {
    Exact,
    LowerBound,
    UpperBound,
}

pub struct CachedScoreInfo {
    pub score: i32,
    pub depth: u8,
}
pub struct CacheEntry {
    pub scores: HashMap<CachedScoreType, CachedScoreInfo>, // Maps scores by type and then by depth.
    pub legal_continuations: Option<MovesMap>,
    pub raw_attacked_squares: HashMap<usize, BitB64>, // Maps raw attacked squared by color (white = 0, black = 1)
}

impl CacheEntry {
    /// Creates a new, empty CacheEntry.
    pub fn new() -> Self {
        Self {
            scores: HashMap::new(),
            legal_continuations: None,
            raw_attacked_squares: HashMap::new(),
        }
    }

    /// Gets a reference to a score, if it exists.
    ///
    /// This function immutably borrows `self` and returns an `Option`
    /// containing a reference to the `CachedScoreInfo`.
    pub fn get_score(&self, score_type: &CachedScoreType) -> Option<&CachedScoreInfo> {
        self.scores.get(score_type)
    }

    /// Updates or inserts a score.
    ///
    /// This is a "smart" update. It will only insert or update the score
    /// if the **new depth is greater than or equal to** the existing depth
    /// for that score type. This prevents overwriting a deep search
    /// result with a shallower one.
    pub fn try_update_score(&mut self, score_type: CachedScoreType, new_score: i32, new_depth: u8) {
        match self.scores.entry(score_type) {
            Entry::Occupied(mut entry) => {
                // Score type already exists, check depth
                if new_depth >= entry.get().depth {
                    // New score is from a deeper or equal search, update it
                    entry.insert(CachedScoreInfo {
                        score: new_score,
                        depth: new_depth,
                    });
                }
                // else: Existing score is from a deeper search, so we do nothing
            }
            Entry::Vacant(entry) => {
                // No score of this type exists, insert the new one
                entry.insert(CachedScoreInfo {
                    score: new_score,
                    depth: new_depth,
                });
            }
        }
    }

    /// A simple insert function that **always** overwrites the existing score.
    /// You might prefer this if you don't need the depth-checking logic.
    pub fn insert_score(
        &mut self,
        score_type: CachedScoreType,
        score: i32,
        depth: u8,
    ) -> Option<CachedScoreInfo> {
        self.scores
            .insert(score_type, CachedScoreInfo { score, depth })
    }
}

// --- Required for `or_default()` on DashMap ---

impl Default for CacheEntry {
    fn default() -> Self {
        Self::new()
    }
}
