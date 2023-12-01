use std::time::Instant;
use y15::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

fn main() {
    let args = args(BIN);
    let now = Instant::now();

    let solution: i32 = if !args.second {
        todo!()
    } else {
        todo!()
    };

    result(solution, now.elapsed(), &args);
}
