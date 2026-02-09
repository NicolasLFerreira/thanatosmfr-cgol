use clap::Parser;
use thanatos::startup::{StartupParameters, startup};

#[derive(Parser)]
struct Cli {
    // No GUI
    #[arg(long, default_value_t = false)]
    headless: bool,
    // Run cap
    #[arg(long, default_value_t = 1000)]
    max_runs: u32,
}

fn main() {
    let cli = Cli::parse();

    // build startup
    let startup_params = StartupParameters {
        run_headless: cli.headless,
        max_runs: cli.max_runs,
    };

    startup(startup_params);
}
