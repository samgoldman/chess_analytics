extern crate chess_analytics;
use chess_analytics::run;

use std::env;

fn main() {
    run(env::args())
}
