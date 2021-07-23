extern crate chess_analytics;
use chess_analytics::run;

use std::env;

#[cfg(not(tarpaulin_include))]
fn main() {
    run(env::args());
}
