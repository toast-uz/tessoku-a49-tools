use std::io::prelude::*;
use itertools::Itertools;
use tools::*;

fn main() {
    let args = std::env::args().collect_vec();
    if args.len() != 2 {
        eprintln!("Usage: {} seeds.txt", args[0]);
        return;
    }
    if !std::path::Path::new("in").exists() {
        std::fs::create_dir("in").unwrap();
    }
    let s = std::fs::read_to_string(&args[1]).unwrap_or_else(|_| {
            eprintln!("no such file: {}", args[1]);
            std::process::exit(1);
        });
    let seeds = match s.split('\n')
            .filter(|&x| !x.trim().is_empty()).map(|x|
            x.parse::<u64>()).collect::<Result<Vec<u64>, std::num::ParseIntError>>() {
        Ok(seeds) => seeds,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        },
    };
    for (id, &seed) in seeds.iter().enumerate() {
        let input = gen(seed);
        let mut w =
            std::io::BufWriter::new(std::fs::File::create(format!("in/{:04}.txt", id)).unwrap());
        write!(w, "{}", input).unwrap();
    }
}
