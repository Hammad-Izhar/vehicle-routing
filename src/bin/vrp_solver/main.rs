use std::{path::Path, time::Duration};

use clap::Parser;
use log::info;
use vehicle_routing::ails::vrp::problem::VehicleRoutingProblem;

#[derive(Parser, Debug)]
struct Args {
    #[arg(index = 1, help = "Input Problem Instance")]
    input: String,

    #[arg(short, long, help = "Solver timeout (in seconds)")]
    timeout: Option<u64>,

    #[arg(short, long, help = "Output Solution File")]
    output: Option<String>,
}

fn main() {
    env_logger::init();

    let args = Args::parse();
    let output_file_name = format!(
        "output/vrp/{}",
        Path::new(&args.input)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
            + ".sol"
    );

    // time this code
    let now = std::time::Instant::now();
    let problem = VehicleRoutingProblem::from_file(&args.input).unwrap();
    info!("Problem loaded in {:?}", now.elapsed());
    let solution = problem.solve(args.timeout.map(Duration::from_secs));
    info!("Solution computed in {:?}", now.elapsed());
    solution.to_file(&output_file_name).unwrap();
    println!("{}", solution);
}
