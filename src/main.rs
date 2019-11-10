extern crate console;
extern crate failure;
extern crate structopt;

use console::{style, Term};
use failure::Error;
use std::collections::VecDeque;
use std::io::{self, BufRead, BufReader};
use std::process::{Command, Stdio};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    #[structopt(
        short = "n",
        long = "number",
        default_value = "10",
        help = "Number of lines to display"
    )]
    number: usize,

    #[structopt(help = "Command to execute. Use -- to pass flags.")]
    command: Vec<String>,
}

fn handle_iterator<T>(buffer_size: usize, collection: T) -> Result<(), Error>
where
    T: Iterator<Item = Result<String, std::io::Error>>,
{
    let term = Term::stdout();

    let mut queue = VecDeque::with_capacity(buffer_size);
    for _ in 0..buffer_size {
        queue.push_back("".to_string())
    }

    let mut past_first_iteration = false;

    for mut item in collection.filter_map(Result::ok) {
        queue.pop_front();
        item.truncate((term.size().1) as usize);
        queue.push_back(item);
        if past_first_iteration {
            term.clear_last_lines(buffer_size)?;
        }

        for line in queue.iter() {
            term.write_line(line.as_str())?;
        }

        past_first_iteration = true;
    }
    Ok(())
}

fn main() {
    let args: Opt = Opt::from_args();
    match run(args.command, args.number) {
        Ok(exit_code) => std::process::exit(exit_code),
        Err(e) => {
            eprintln!("{}", style("Error:").red());
            for cause in e.iter_chain() {
                eprintln!("{}", style(cause).red());
            }
            std::process::exit(1);
        }
    }
}

fn run(command: Vec<String>, count: usize) -> Result<i32, Error> {
    if command.is_empty() {
        let stdin = io::stdin();
        let iterator = stdin.lock().lines();
        handle_iterator(count, iterator)?;
        Ok(0)
    } else {
        let mut child = Command::new(&command[0])
            .args(&command[1..])
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?;
        if let Some(ref mut stdout) = child.stdout {
            let reader = BufReader::new(stdout);
            handle_iterator(count, reader.lines())?;
        }

        let status = child.wait()?;
        Ok(status.code().unwrap_or(0))
    }
}
