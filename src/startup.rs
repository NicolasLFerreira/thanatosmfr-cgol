use crate::orchestration::{SimulationOrchestration, SimulationParameters};
use crate::types::{SimulationFeed, SimulationPayload};
use crate::ui::app::App;
use crossbeam::atomic::AtomicCell;
use eframe::Renderer;
use std::sync::Arc;

pub struct StartupParameters {
    pub run_headless: bool,
    pub max_runs: u32,
}

/// Sets up the different parts of the program
pub fn startup(parameters: StartupParameters) {
    let feed = Arc::new(AtomicCell::new(Arc::new(SimulationPayload::default())));
    let so = SimulationOrchestration::new(Arc::clone(&feed));

    so.start(SimulationParameters {
        max_run_count: parameters.max_runs,
        blocking: false,
        uncapped: parameters.run_headless,
    });

    if !parameters.run_headless {
        start_ui(Arc::clone(&feed));
    }
}

fn start_ui(feed: SimulationFeed) {
    let native_options = eframe::NativeOptions {
        centered: true,
        renderer: Renderer::Wgpu,
        ..Default::default()
    };
    eframe::run_native(
        "Thanatos",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc, feed)))),
    )
    .unwrap();
}
