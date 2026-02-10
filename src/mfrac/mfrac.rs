use crate::mfrac::canonical::compute_canonical;
use crate::mfrac::hash::canonical_hash;
use crate::persistence::Database;
use crate::types::{CellConfiguration, ConfigurationChainNode, MfracOutcome, MfracStatus};
use fxhash::FxBuildHasher;

/// Heart of Thanatos
pub struct Mfrac {
    db: Database,
    cache: Vec<(u128, Vec<u64>)>,
    prev_hash: u128,
}

impl Mfrac {
    pub fn init() -> Self {
        Self {
            db: Database::open(),
            cache: Default::default(),
            prev_hash: 0,
        }
    }

    pub fn run_pipeline(&mut self, configuration: &CellConfiguration) -> MfracStatus {
        let canonical = compute_canonical(configuration);
        let hash = canonical_hash(&canonical);

        // collision detection
        let collision = self.db.contains(hash);

        match collision {
            // YAY
            true => {
                self.collision_handler(hash);
                Some(MfracOutcome::Collision(hash))
            }
            // aw
            false => {
                
                self.cache.push((hash, canonical));
                None
            }
        }
    }

    pub fn collision_handler(&mut self, hash: u128) {}
}
