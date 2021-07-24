extern crate chess_analytics;
use chess_analytics::run;

use std::env;

#[cfg(not(tarpaulin_include))]
fn main() -> Result<(), String> {
    run(env::args())?;

    Ok(())
}
