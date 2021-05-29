use std::io::BufRead;

use poloto::prelude::*;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn main() {
    if let Err(err) = try_main() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn try_main() -> Result<()> {
    let mut unmarked = false;
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "-u" | "--unmarked" => unmarked = true,
            "-h" | "--help" => {
                println!("\
iwp
I Want Plot -- draws a plot reading data points from stdin.

USAGE:
    $ iwp <<EOF
    > P! 1 1
    > P! 2 4
    > this line is skipped
    > P! 3 9
    > EOF
    $ open plot.svg

OPTIONS:
  -u, --unmarked        read two columns of numbers, don't use P! marker
")
            }
            arg => return Err(format!("unexpected argument: {}", arg).into()),
        }
    }

    let data = collect_data(unmarked)?;
    plot(data)?;
    Ok(())
}

fn collect_data(unmarked: bool) -> Result<Vec<[f64; 2]>> {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();

    let mut res = Vec::new();
    for line in stdin.lines() {
        let line = line?;

        let data_line = if unmarked {
            line.as_str()
        } else {
            match line.strip_prefix("P! ") {
                Some(it) => it,
                None => continue,
            }
        };

        match pares_data_line(data_line) {
            Some(data_pont) => res.push(data_pont),
            None => {
                eprintln!("skipping: {:?}", line)
            }
        }
    }
    Ok(res)
}

fn pares_data_line(data_line: &str) -> Option<[f64; 2]> {
    let mut it = data_line
        .split_ascii_whitespace()
        .map(|it| it.parse::<f64>());
    let x = it.next()?.ok()?;
    let y = it.next()?.ok()?;
    if it.next().is_some() {
        return None;
    }
    Some([x, y])
}

fn plot(data: Vec<[f64; 2]>) -> Result<()> {
    let mut plotter = poloto::plot("", "x", "y");

    plotter.scatter("", data.twice_iter());

    let plot = plotter.render_to_string()?;
    std::fs::write("plot.svg", &plot)?;
    Ok(())
}
