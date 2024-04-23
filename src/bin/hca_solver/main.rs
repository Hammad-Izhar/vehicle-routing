mod hca;

use std::time::Duration;

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
    let args = Args::parse();

    let problem = hca::HealthcareAnalyticsProblem::from_file(&args.input).unwrap();
    let solution = problem.solve(args.timeout.map(Duration::from_secs));
    println!("{}", solution);
}
