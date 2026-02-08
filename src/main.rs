mod conway;
mod thanatos;
mod types;
mod ui;

use crate::types::cell_configuration::CellConfiguration;
use conway::simulation::*;
use eframe::Renderer;
use std::time::Instant;
use ui::app::App;

const CELL_SIZE_PX: f32 = 16.0;

fn main() {
    let native_options = eframe::NativeOptions {
        centered: true,
        renderer: Renderer::Wgpu,
        ..Default::default()
    };
    eframe::run_native(
        "Thanatos CGoL",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
    .unwrap();
}

fn logical_step(cconf: &CellConfiguration) -> CellConfiguration {
    let mut start = Instant::now();

    thanatos::tmfroc::run(&cconf);

    let elapsed = start.elapsed();
    println!("Thanatos: {:?}", elapsed);

    start = Instant::now();

    let new_cconf = simulation(cconf);

    let elapsed = start.elapsed();
    println!("Simulation: {:?}", elapsed);

    new_cconf
}
