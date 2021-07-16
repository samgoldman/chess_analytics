extern crate chess_analytics;
use chess_analytics::run;

#[test]
fn simple_count_10_compressed() -> Result<(), Box<dyn std::error::Error>> {
    let res = run(vec![
        "chess_analytics",
        "--glob",
        "tests/data/10_games_000000.bin.bz2",
        "--workflow",
        "tests/workflows/simple_count.json",
    ]
    .iter()
    .map(|x| x.to_string()));
    assert_eq!(res, "Bin	gameCount.sum\n\t10.0000\t\n");

    Ok(())
}

#[test]
fn simple_count_10_uncompressed() -> Result<(), Box<dyn std::error::Error>> {
    let res = run(vec![
        "chess_analytics",
        "--glob",
        "tests/data/10_games_000000.bin",
        "--workflow",
        "tests/workflows/simple_count.json",
    ]
    .iter()
    .map(|x| x.to_string()));

    assert_eq!(res, "Bin	gameCount.sum\n\t10.0000\t\n");
    let res = run(vec![
        "chess_analytics",
        "--glob",
        "tests/data/10_games_000000_bin",
        "--workflow",
        "tests/workflows/simple_count.json",
    ]
    .iter()
    .map(|x| x.to_string()));
    assert_eq!(res, "Bin	gameCount.sum\n\t10.0000\t\n");

    Ok(())
}

#[test]
fn simple_count_10000_compressed() -> Result<(), Box<dyn std::error::Error>> {
    let res = run(vec![
        "chess_analytics",
        "--glob",
        "tests/data/2013-01_000000.bin.bz2",
        "--workflow",
        "tests/workflows/simple_count.json",
    ]
    .iter()
    .map(|x| x.to_string()));
    assert_eq!(res, "Bin	gameCount.sum\n\t10000.0000\t\n");

    Ok(())
}
