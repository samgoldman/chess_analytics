extern crate chess_analytics;
use chess_analytics::run;
use std::fs;

#[test]
fn simple_count_10_games() -> Result<(), Box<dyn std::error::Error>> {
    let _ = std::fs::create_dir("tests/output/int_1");
    run(vec![
        "chess_analytics",
        "tests/workflows/1_simple_count_10_games.yaml",
    ]
    .iter()
    .map(|x| (*x).to_string()))?;

    let contents = fs::read_to_string("tests/output/int_1/tmp.txt")
        .expect("Something went wrong reading the file");

    assert_eq!(contents, "game_count: \n\t\"\": 10\n\n");

    let _ = std::fs::remove_file("tests/output/int_1/tmp.txt");
    let _ = std::fs::remove_dir("tests/output/int_1");

    Ok(())
}

#[test]
fn simple_count_eval_and_clock_1_game() -> Result<(), Box<dyn std::error::Error>> {
    let _ = std::fs::create_dir("tests/output/int_2");
    run(vec![
        "chess_analytics",
        "tests/workflows/2_simple_count_eval_and_clock_1_game.yaml",
    ]
    .iter()
    .map(|x| (*x).to_string()))?;

    let contents = fs::read_to_string("tests/output/int_2/tmp.txt")
        .expect("Something went wrong reading the file");

    assert_eq!(contents, "game_count: \n\t\"\": 1\n\n");

    let _ = std::fs::remove_file("tests/output/int_2/tmp.txt");
    let _ = std::fs::remove_dir("tests/output/int_2");

    Ok(())
}

#[test]
fn count_mates_by_time_10_games() -> Result<(), Box<dyn std::error::Error>> {
    let _ = std::fs::create_dir("tests/output/int_3");
    run(vec![
        "chess_analytics",
        "tests/workflows/3_count_mates_by_time_10_games.yaml",
    ]
    .iter()
    .map(|x| (*x).to_string()))?;

    let contents = fs::read_to_string("tests/output/int_3/tmp.txt")
        .expect("Something went wrong reading the file");

    let expected = "game_count: \n\t\"Blitz\": 1\n\t\"Rapid\": 3\n\n";
    assert_eq!(contents, expected);

    let _ = std::fs::remove_file("tests/output/int_3/tmp.txt");
    let _ = std::fs::remove_dir("tests/output/int_3");

    Ok(())
}

#[test]
fn player_elo_tc_bin_1() -> Result<(), Box<dyn std::error::Error>> {
    let _ = std::fs::create_dir("tests/output/int_4");
    run(vec![
        "chess_analytics",
        "tests/workflows/4_player_elo_tc_bin_1.yaml",
    ]
    .iter()
    .map(|x| (*x).to_string()))?;

    let contents = fs::read_to_string("tests/output/int_4/tmp.txt")
        .expect("Something went wrong reading the file");

    let expected = "game_count: \n\t\"Blitz\": 1\n\t\"Rapid\": 1\n\n";
    assert_eq!(contents, expected);

    let _ = std::fs::remove_file("tests/output/int_4/tmp.txt");
    let _ = std::fs::remove_dir("tests/output/int_4");

    Ok(())
}

#[test]
fn count_games_with_eval_available() -> Result<(), Box<dyn std::error::Error>> {
    let _ = std::fs::create_dir("tests/output/int_5");
    run(vec![
        "chess_analytics",
        "tests/workflows/5_eval_available_filter.yaml",
    ]
    .iter()
    .map(|x| (*x).to_string()))?;

    let contents = fs::read_to_string("tests/output/int_5/tmp.txt")
        .expect("Something went wrong reading the file");

    assert_eq!(contents, "game_count: \n\t\"\": 1\n\n");

    let _ = std::fs::remove_file("tests/output/int_5/tmp.txt");
    let _ = std::fs::remove_dir("tests/output/int_5");

    Ok(())
}

#[test]
fn parse_pgn() -> Result<(), Box<dyn std::error::Error>> {
    let _ = std::fs::create_dir("tests/output/int_6");
    run(vec!["chess_analytics", "tests/workflows/6_parse_pgn.yaml"]
        .iter()
        .map(|x| (*x).to_string()))?;

    let contents = fs::read("tests/output/int_6/int_6_test_set_1.bin.bz2")
        .expect("Something went wrong reading the file");
    let expected =
        fs::read("tests/data/test_set_1.bin.bz2").expect("Something went wrong reading the file");

    assert_eq!(contents, expected);

    let _ = std::fs::remove_file("tests/output/int_6/int_6_test_set_1.bin.bz2");
    let _ = std::fs::remove_dir("tests/output/int_6");

    Ok(())
}

#[test]
fn avg_perfect_checkmate_unbinned() -> Result<(), Box<dyn std::error::Error>> {
    let _ = std::fs::create_dir("tests/output/int_7");
    run(vec![
        "chess_analytics",
        "tests/workflows/7_perfect_checkmate_avg.yaml",
    ]
    .iter()
    .map(|x| (*x).to_string()))?;

    let contents = fs::read_to_string("tests/output/int_7/tmp.txt")
        .expect("Something went wrong reading the file");

    assert_eq!(
        contents,
        "avg_perfect_checkmate: \n\t\"\": 1.6666666666666667\n\n"
    );

    let _ = std::fs::remove_file("tests/output/int_7/tmp.txt");
    let _ = std::fs::remove_dir("tests/output/int_7");

    Ok(())
}

#[test]
fn noop_workflow() -> Result<(), Box<dyn std::error::Error>> {
    let _ = std::fs::create_dir("tests/output/int_8");
    assert!(run(vec!["chess_analytics", "tests/workflows/8_noop.yaml",]
        .iter()
        .map(|x| (*x).to_string()))
    .is_ok());

    assert!(fs::read_to_string("tests/output/int_8/tmp.txt").is_err());

    let _ = std::fs::remove_file("tests/output/int_7/tmp.txt");

    Ok(())
}

#[test]
fn workflow_param_not_provided() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(
        run(vec!["chess_analytics",].iter().map(|x| (*x).to_string())),
        Err("First argument (configuration path) is required".to_string())
    );

    Ok(())
}

#[test]
fn non_existent_workflow() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(run(vec![
        "chess_analytics",
        "tests/workflows/does_not_exist.yaml",
    ]
    .iter()
    .map(|x| (*x).to_string())), Err("Could not open configuration file: Os { code: 2, kind: NotFound, message: \"No such file or directory\" }".to_string()));

    Ok(())
}
