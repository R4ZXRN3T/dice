use clap::{Parser};
use rand::Rng;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
	#[arg(short = 'm', long = "min", default_value_t = 1)]
	minimum: usize,

	#[arg(short = 'M', long = "max", default_value_t = 6)]
	maximum: usize,

	#[arg(short, long, default_value_t = 1)]
	rolls: usize,

	#[arg(short = 'c', long = "conclusion", default_value_t = false)]
	show_conclusion: bool,

	#[arg(short = 's', long = "silent", default_value_t = false)]
	silent: bool,
}

fn main() {
	let args = Args::parse();
	let mut rng = rand::rng();
	let mut values: Vec<usize> = vec![0; args.maximum];
	let start_time = std::time::Instant::now();
	for _i in 0..args.rolls {
		let rnd = rng.random_range(args.minimum..args.maximum + 1);
		values[rnd - 1] += 1;
		if !args.silent {
			println!("{}", rnd);
		}
	}
	let duration = start_time.elapsed();

	if args.show_conclusion {
		println!("\nSummary:\n");
		println!("Total time: {}s\n", duration.as_secs_f64());
		for i in args.minimum - 1..args.maximum {
			println!(
				"{}:\t{}\t{:.4}%",
				i + 1,
				values[i],
				(values[i] as f64 / args.rolls as f64) * 100f64
			);
		}
	}
}
