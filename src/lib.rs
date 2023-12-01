use std::{io::{Cursor, BufRead}, time::Duration, fmt::Display};
use cli::Args;
use include_dir::{include_dir, Dir};

mod cli;

pub static INPUTS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/inputs");

pub fn args(bin: &str) -> Args {
    let mut args = cli::args();
    args.day = day(bin).parse().unwrap();
    read_input(bin, &mut args);
    args
}

fn day(bin: &str) -> &str {
    bin.strip_prefix("d").expect("bin should start with d")
}

fn input_files(bin: &str) -> [String; 2] {
    [format!("{bin}.txt"), format!("{bin}_scratchpad.txt")]
}

fn read_input(bin: &str, args: &mut Args) {
    let name = &input_files(bin)[args.example as usize][..];
    let file = INPUTS.get_file(name).unwrap().contents_utf8().unwrap();

    let mut input: Vec<String> = Cursor::new(file).lines().filter_map(Result::ok).collect();
    if args.example {
        let test2 = input.pop().unwrap();
        let test1 = input.pop().unwrap();
        args.expected = Some([test1, test2]);
        args.input = input.join("\n").into();
    } else {
        args.input = file.into();
    }
}

#[inline]
fn part(args: &Args) -> usize {
    args.second as _
}

#[inline]
fn example_output<T: Display>(args: &Args, solution: T) {
    let expected = args
        .expected
        .as_ref()
        .and_then(|o| o.get(part(args)))
        .unwrap();
    println!("??? E {expected} == S {solution}");
}

#[inline]
pub fn result<T: Display>(solution: T, elapsed: Duration, args: &Args) {
    let day = args.day;
    let part = if args.second { 2 } else { 1 };
    if args.example {
        example_output(args, solution);
    } else {
        println!("[D{day:02}.{part}] solution: {solution}\n    solved in {elapsed:?}");
    }
}
