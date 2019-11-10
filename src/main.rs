extern crate console;
extern crate failure;
extern crate structopt;

use console::{style, Term};
use failure::Error;
use std::collections::VecDeque;
use std::io::{self, BufRead, BufReader};
use duct::cmd;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Tail, but dynamic.")]
struct Opt {
    #[structopt(
    short = "n",
    long = "number",
    default_value = "10",
    help = "Number of lines to display"
    )]
    number: usize,

    #[structopt(help = "Command to execute. To pass flags, add -- before: \"ptail -- cmd --flag\"")]
    command: Vec<String>,
}

fn handle_iterator<T>(buffer_size: usize, collection: T) -> Result<(), Error>
    where
        T: Iterator<Item=Result<String, std::io::Error>>,
{
    let term = Term::stdout();
    let is_terminal = term.is_term();

    let mut queue = VecDeque::with_capacity(buffer_size);

    let mut is_first_iteration = true;
    let mut last_line_count = queue.len();

    for mut item in collection.filter_map(Result::ok) {
        // If we are not a terminal, then just proxy the output
        if !is_terminal {
            println!("{}", item);
            continue;
        }

        if !is_first_iteration {
            term.clear_last_lines(last_line_count)?;
        }

        item.truncate((term.size().1) as usize);
        queue.push_back(item);

        while queue.len() > buffer_size {
            queue.pop_front();
        }

        for line in queue.iter() {
            term.write_line(line.as_str())?;
        }

        is_first_iteration = false;
        last_line_count = queue.len();
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
        let child = cmd(&command[0], &command[1..]);
        let stdout_reader = child.stderr_to_stdout().reader()?;
        let reader = BufReader::new(&stdout_reader);
        handle_iterator(count, reader.lines())?;
        let output = stdout_reader.try_wait()?;
        match output {
            Some(output) => Ok(output.status.code().unwrap_or(0)),
            None => Ok(0)
        }
    }
}
