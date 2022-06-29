extern crate chess_analytics;
use chess_analytics::run;
use std::fs;

#[test]
fn simple_count_10_games() -> Result<(), Box<dyn std::error::Error>> {
    let _res = std::fs::create_dir("tests/output/int_1");
    run(vec![
        "chess_analytics",
        "tests/workflows/1_simple_count_10_games.yaml",
    ]
    .iter()
    .map(|x| (*x).to_string()))?;

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
        "tests/workflows/2_simple_count_eval_and_clock_1_game.yaml",
    ]
    .iter()
    .map(|x| (*x).to_string()))?;

    let contents = fs::read_to_string("tests/output/int_2/tmp.txt")
        .expect("Something went wrong reading the file");

    assert_eq!(contents, "game_count: Map({\"\": U64(1)})\n");

    let _res = std::fs::remove_file("tests/output/int_2/tmp.txt");
    let _res = std::fs::remove_dir("tests/output/int_2");

    Ok(())
}

#[test]
fn count_mates_by_time_10_games() -> Result<(), Box<dyn std::error::Error>> {
    let _res = std::fs::create_dir("tests/output/int_3");
    run(vec![
        "chess_analytics",
        "tests/workflows/3_count_mates_by_time_10_games.yaml",
    ]
    .iter()
    .map(|x| (*x).to_string()))?;

    let contents = fs::read_to_string("tests/output/int_3/tmp.txt")
        .expect("Something went wrong reading the file");

    // Either option is valid
    // TODO: ideally create a step that can print specific bins in a pre-determined order
    let expected_1 = "game_count: Map({\"Blitz\": U64(1), \"Rapid\": U64(3)})\n";
    let expected_2 = "game_count: Map({\"Rapid\": U64(3), \"Blitz\": U64(1)})\n";

    assert!(contents == expected_1 || contents == expected_2);

    let _res = std::fs::remove_file("tests/output/int_3/tmp.txt");
    let _res = std::fs::remove_dir("tests/output/int_3");

    Ok(())
}

#[test]
fn player_elo_tc_bin_1() -> Result<(), Box<dyn std::error::Error>> {
    let _res = std::fs::create_dir("tests/output/int_4");
    run(vec![
        "chess_analytics",
        "tests/workflows/4_player_elo_tc_bin_1.yaml",
    ]
    .iter()
    .map(|x| (*x).to_string()))?;

    let contents = fs::read_to_string("tests/output/int_4/tmp.txt")
        .expect("Something went wrong reading the file");

    // Either option is valid
    // TODO: ideally create a step that can print specific bins in a pre-determined order
    let expected_1 = "game_count: Map({\"Blitz\": U64(1), \"Rapid\": U64(1)})\n";
    let expected_2 = "game_count: Map({\"Rapid\": U64(1), \"Blitz\": U64(1)})\n";

    assert!(contents == expected_1 || contents == expected_2);

    let _res = std::fs::remove_file("tests/output/int_4/tmp.txt");
    let _res = std::fs::remove_dir("tests/output/int_4");

    Ok(())
}
