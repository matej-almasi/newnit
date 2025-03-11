use clap::Parser;
use newnit_convert::{Unit, convert, parse_unit};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(allow_negative_numbers = true)]
    amount: f64,

    from: Unit,

    to: Unit,

    #[arg(short, long, default_value_t = 4)]
    precision: usize,
}

fn main() {
    let cli = Cli::parse();

    let amount = cli.amount;
    let from = parse_unit(&cli.from);
    let to = parse_unit(&cli.to);
    let precision = cli.precision;

    let result = convert(amount, from, to).unwrap_or_else(|e| panic!("{e}"));

    println!(
        "{amount} {:?} is {result:.2$} {:?} ",
        cli.from, cli.to, precision
    );
}
