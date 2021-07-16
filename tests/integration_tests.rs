use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn simple_count_10_compressed() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("chess_analytics")?;

    cmd.arg("--glob")
        .arg("tests/data/10_games_000000.bin.bz2")
        .arg("--workflow")
        .arg("tests/workflows/simple_count.json");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Bin	gameCount.sum\n\t10.0000"));

    Ok(())
}
