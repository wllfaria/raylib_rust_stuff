use clap::{Parser, ValueEnum};

mod sierpinski_triangle;

#[derive(Debug, ValueEnum, Clone)]
enum Simulations {
    Sierpinski,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    simulation: Simulations,
}

fn main() {
    let args = Args::parse();
    match args.simulation {
        Simulations::Sierpinski => sierpinski_triangle::draw(),
    }
}
