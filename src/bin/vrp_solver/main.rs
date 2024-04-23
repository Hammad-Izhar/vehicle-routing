mod graph;
mod vrp;

use std::{path::Path, time::Duration};

use clap::Parser;

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

    let problem = vrp::VehicleRoutingProblem::from_file(&args.input).unwrap();
    let solution = problem.solve(args.timeout.map(Duration::from_secs));
    solution.to_file(&output_file_name).unwrap();
    println!("{}", solution);
}
