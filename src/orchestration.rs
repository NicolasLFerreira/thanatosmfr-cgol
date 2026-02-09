use crate::types::{
    CellConfiguration, CellCoord, MfracOutcome, MfracTerminationReason, SimulationFeed,
    SimulationPayload,
};
use crate::{conway, mfrac};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[derive(Default, Copy, Clone)]
pub struct SimulationParameters {
    pub blocking: bool,
    pub uncapped: bool,
    pub max_run_count: u32,
}

pub struct SimulationOrchestration {
    sim_feed: SimulationFeed,
}

impl SimulationOrchestration {
    pub fn new(sim_feed: SimulationFeed) -> Self {
        Self { sim_feed }
    }

    pub fn start(&self, params: SimulationParameters) {
        let handle = {
            let clone = Arc::clone(&self.sim_feed);
            thread::spawn(move || Self::state_machine(clone, params.max_run_count, params.uncapped))
        };

        if params.uncapped {
            handle.join().unwrap();
        }
    }

    fn state_machine(feed: SimulationFeed, mut max_iteration: u32, uncapped: bool) {
        let max_iteration = 20;

        for i in 0..max_iteration {
            let seed_cells = CellConfiguration::random_configuration(i as u64, 5, 5, 0.4);
            let outcome = Self::simulation_run(seed_cells, Arc::clone(&feed), uncapped);

            println!("Run {i} finished.");

            match outcome {
                MfracOutcome::Collision(hash) => {
                    println!("Collided with: {}", hash);
                }
                MfracOutcome::Termination(reason) => match reason {
                    MfracTerminationReason::LimitExceeded(limit) => {
                        println!("Limit exceeded: {limit}");
                    }
                    MfracTerminationReason::StaleLife => {}
                    MfracTerminationReason::Oscillator => {}
                },
            }
        }
    }

    fn simulation_run(
        seed_cells: Vec<CellCoord>,
        feed: SimulationFeed,
        uncapped: bool,
    ) -> MfracOutcome {
        let mut cconf = CellConfiguration::with_seed_configuration(seed_cells);
        for i in 0..10_000 {
            // Run Thanatos on current configuration
            let option = mfrac::run_pipeline(&cconf);
            if let Some(outcome) = option {
                return outcome;
            }

            // Step Conway
            let new_cconf = conway::step(&cconf);

            // Publish results
            cconf = new_cconf;
            feed.store(Arc::new(SimulationPayload::new(Some(cconf.clone()))));

            // Currently capping for UI and development
            if !uncapped {
                thread::sleep(Duration::from_millis(100));
            }
        }

        MfracOutcome::Termination(MfracTerminationReason::LimitExceeded(10_000))
    }
}
