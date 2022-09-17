use rand::prelude::*;
use proconio::input;
use proconio::marker::Usize1;
use itertools::Itertools;

const T: usize = 100;
const N: usize = 20;

#[derive(Clone, Debug)]
pub struct Input {
    pub target: Vec<Vec<usize>>,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", T)?;
        writeln!(f, "{}", self.target.iter().map(|row|
            row.iter().map(|&i| i + 1).join(" ")).join("\n"))
    }
}

pub fn parse_input(f: &str) -> Input {
    let f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
        t: usize,
        target: [[Usize1; 3]; t],
    }
    Input { target, }
}

#[derive(Clone, Debug)]
pub struct Output {
    pub command: Vec<bool>,
}

pub fn parse_output(f: &str) -> Result<Output, String> {
    let ans = f.split('\n')
        .filter(|&x| !x.trim().is_empty()).collect_vec();

    if ans.len() != T {
        return Err("output length must be T".to_string());
    }

    let mut command = Vec::new();
    for command_char in ans {
        match command_char {
            "A" => { command.push(true); },
            "B" => { command.push(false); },
            _ => { return Err("Output must consist \"A\" or \"B\"".to_string()); }
        }
    }

    Ok(Output { command, })
}

pub fn compute_score(input: &Input, output: &Output) -> (isize, String) {
    let mut x = vec![0; N];
    let mut res = 0;
    for t in 0..T {
        input.target[t].iter().for_each(|&i|
            if output.command[t] { x[i] += 1 } else { x[i] -= 1; });
        res += x.iter().filter(|&&x| x == 0).count() as isize;
    }
    (res, "".to_string())
}

pub fn gen(seed: u64) -> Input {
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed);
    let mut target = Vec::new();
    let source = (0..N).collect_vec();
    for _ in 0..T {
        let indices = source.choose_multiple(&mut rng, 3)
            .cloned().sorted().collect_vec();
        target.push(indices);
    }
    Input { target, }
}
