use crate::move_gen::MovesMap;
use crate::CACHE;

use super::bitboard::BitB64;
use super::PlayerColor;
use std::collections::HashMap;
pub struct CacheEntry {
    pub scores: HashMap<usize, i32>, // Scores computed for specific depth.
    pub legal_continuations: Option<MovesMap>,
    pub raw_attacked_squares: HashMap<usize, BitB64>, // Maps raw attacked squared by color (white = 0, black = 1)
}

// pub fn get_mut_or_create(key: &u64) -> &mut CacheEntry {
//     if !CACHE.contains_key(key) {
//         CACHE.insert(
//             *key,
//             CacheEntry {
//                 scores: HashMap::new(),
//                 raw_attacked_squares: HashMap::new(),
//                 legal_continuations: None,
//             },
//         );
//         CACHE.get_mut(key).unwrap()
//     }
// }
// pub struct PositionCache {
//     pub cache: HashMap<u64, CacheEntry>,
//     pub max_size: usize,
// }

// impl PositionCache {
//     pub fn new(max_size: usize) -> Self {
//         Self {
//             cache: HashMap::new(),
//             max_size,
//         }
//     }

//     fn get_or_create_entry(&mut self, key: u64) -> &mut CacheEntry {
//         self.cache.entry(key).or_insert_with(|| CacheEntry {
//             scores: HashMap::new(),
//             legal_continuations: None,
//             raw_attacked_squares: HashMap::new(),
//         })
//     }

//     pub fn add_score(&mut self, key: u64, score: i32, depth: u8) {
//         let entry = self.get_or_create_entry(key);
//         entry.scores.insert(depth as usize, score);
//     }

//     pub fn get_score(&self, key: u64, depth: u8) -> Option<i32> {
//         self.cache.get(&key).and_then(|entry| {
//             entry
//                 .scores
//                 .get(&(depth as usize))
//                 .and_then(|score| Some(*score))
//         })
//     }

//     pub fn add_legal_continuations(&mut self, key: u64, legal_continuations: &MovesMap) {
//         let entry = self.get_or_create_entry(key);
//         entry.legal_continuations = Some(legal_continuations.clone());
//     }

//     pub fn get_legal_continuations(&self, key: u64) -> Option<&MovesMap> {
//         self.cache
//             .get(&key)
//             .and_then(|entry| entry.legal_continuations.as_ref())
//     }

//     pub fn add_raw_attacked_squares(
//         &mut self,
//         key: u64,
//         color: PlayerColor,
//         raw_attacked_squares: BitB64,
//     ) {
//         let entry = self.get_or_create_entry(key);
//         entry
//             .raw_attacked_squares
//             .insert(color as usize, raw_attacked_squares);
//     }

//     pub fn get_raw_attacked_squares(&self, key: u64, color: PlayerColor) -> Option<&BitB64> {
//         self.cache.get(&key).and_then(|entry| {
//             entry
//                 .raw_attacked_squares
//                 .get(&(color as usize))
//                 .and_then(|raw_attacked_squares| Some(raw_attacked_squares))
//         })
//     }
// }

// pub struct PositionCacheManager {
//     pub singleton: PositionCache,
// }

// impl PositionCacheManager {
//     pub fn new(max_size: usize) -> Self {
//         Self {
//             singleton: PositionCache::new(max_size),
//         }
//     }

//     pub fn get_mut(&mut self) -> &mut PositionCache {
//         &mut self.singleton
//     }
// }
