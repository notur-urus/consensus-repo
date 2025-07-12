pub mod decay;
pub mod threshold;
pub mod vote;
pub mod consensus;

use std::time::{Duration, SystemTime};
use consensus::Consensus;
use decay::{ExpDecay, LinearDecay, StepDecay};
use threshold::LinearEsc;
use vote::Window;
use clap::Parser;

#[derive(Parser)]
#[command(about = "Minimal CLI for Time-Decay Consensus", version)]
struct Args {
    #[arg(short, long)]
    votes: String,
    #[arg(short, long, default_value = "exp")]
    decay: String,
}

fn main() {
    let args = Args::parse();

    let decay: Box<dyn decay::Decay> = match args.decay.as_str() {
        "linear" => Box::new(LinearDecay(Duration::from_secs(300))),
        "step" => Box::new(StepDecay(Duration::from_secs(60))),
        _ => Box::new(ExpDecay(Duration::from_secs(60))),
    };
    

    let decay: Box<dyn decay::Decay> = Box::new(ExpDecay(Duration::from_secs(60)));
    let esc: Box<dyn threshold::Escalator> = Box::new(LinearEsc { base: 0.51, slope: 0.0, cap: 1.0 });
    let window = Window { start: SystemTime::now(), duration: Duration::from_secs(300) };
    let mut c = Consensus::new(decay, esc, window);

    for v in args.votes.split(',') {
        c.cast(v.trim());
        std::thread::sleep(Duration::from_millis(20));
    }

    match c.result() {
        Some(v) => println!(" Consensus: {v}"),
        None => println!(" No consensus reached"),
    }
}

