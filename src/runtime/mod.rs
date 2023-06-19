use crate::{bitb, chess::bitboard::BitB64};
use std::collections::HashMap;

pub struct RuntimeInfo {
    pub bitboard_to_id: HashMap<BitB64, usize>,
}

pub struct EngineRuntime {
    pub info: RuntimeInfo,
}

impl EngineRuntime {
    pub fn new() -> EngineRuntime {
        let mut bitboard_to_id = HashMap::new();
        for i in 0..64 {
            bitboard_to_id.insert(bitb!(i), i);
        }
        println!("sucessfully initialized runtime");
        EngineRuntime {
            info: RuntimeInfo {
                bitboard_to_id: bitboard_to_id,
            },
        }
    }
}
