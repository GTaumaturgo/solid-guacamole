#[derive(Clone, Copy, Debug)]
pub enum ScoreType {
    Exact,
    LowerBound,
    UpperBound,
}

#[derive(Clone, Copy, Debug)]
pub struct CacheEntry {
    pub depth: u8,
    pub score: i32,
    pub score_type: ScoreType,
}
