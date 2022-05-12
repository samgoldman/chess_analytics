extern crate chess_analytics;
use chess_analytics::run;
use std::fs;

#[test]
fn simple_count_10_games() -> Result<(), Box<dyn std::error::Error>> {
    let _res = std::fs::create_dir("tests/output/int_1");
    run(vec![
        "chess_analytics",
        "tests/workflows/simple_count_10_games.yaml",
    ]
    .iter()
    .map(|x| x.to_string()))?;

    let contents = fs::read_to_string("tests/output/int_1/tmp.txt")
    .expect("Something went wrong reading the file");

    assert_eq!(contents, "game_count: Map({\"\": U64(10)})\n");

    let _res = std::fs::remove_file("tests/output/int_1/tmp.txt");
    let _res = std::fs::remove_dir("tests/output/int_1");

    Ok(())
}

#[test]
fn simple_count_eval_and_clock_1_game() -> Result<(), Box<dyn std::error::Error>> {
    let _res = std::fs::create_dir("tests/output/int_2");
    run(vec![
        "chess_analytics",
        "tests/workflows/simple_count_eval_and_clock_1_game.yaml",
    ]
    .iter()
    .map(|x| x.to_string()))?;

    let contents = fs::read_to_string("tests/output/int_2/tmp.txt")
    .expect("Something went wrong reading the file");

    assert_eq!(contents, "game_count: Map({\"\": U64(1)})\n");

    let _res = std::fs::remove_file("tests/output/int_2/tmp.txt");
    let _res = std::fs::remove_dir("tests/output/int_2");

    Ok(())
}
