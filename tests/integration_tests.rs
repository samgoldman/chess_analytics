// extern crate chess_analytics;
// use chess_analytics::run;

// // TODO fix tests

// #[test]
// fn simple_count_10_compressed() -> Result<(), Box<dyn std::error::Error>> {
//     let res = run(vec![
//         "chess_analytics",
//         "--glob",
//         "tests/data/10_games_000000.bin.bz2",
//         "--workflow",
//         "tests/workflows/simple_count.json",
//     ]
//     .iter()
//     .map(|x| x.to_string()));
//     assert_eq!(res, "Bin	gameCount.sum\n\t10.0000\t\n");

//     Ok(())
// }

// #[test]
// fn simple_count_10_uncompressed() -> Result<(), Box<dyn std::error::Error>> {
//     let res = run(vec![
//         "chess_analytics",
//         "--glob",
//         "tests/data/10_games_000000.bin",
//         "--workflow",
//         "tests/workflows/simple_count.json",
//     ]
//     .iter()
//     .map(|x| x.to_string()));

//     assert_eq!(res, "Bin	gameCount.sum\n\t10.0000\t\n");
//     let res = run(vec![
//         "chess_analytics",
//         "--glob",
//         "tests/data/10_games_000000_bin",
//         "--workflow",
//         "tests/workflows/simple_count.json",
//     ]
//     .iter()
//     .map(|x| x.to_string()));
//     assert_eq!(res, "Bin	gameCount.sum\n\t10.0000\t\n");

//     Ok(())
// }

// #[test]
// fn simple_count_10000_compressed() -> Result<(), Box<dyn std::error::Error>> {
//     let res = run(vec![
//         "chess_analytics",
//         "--glob",
//         "tests/data/2013-01_000000.bin.bz2",
//         "--workflow",
//         "tests/workflows/simple_count.json",
//     ]
//     .iter()
//     .map(|x| x.to_string()));
//     assert_eq!(res, "Bin	gameCount.sum\n\t10000.0000\t\n");

//     Ok(())
// }

// #[test]
// fn eco_count_no_filter() -> Result<(), Box<dyn std::error::Error>> {
//     let res = run(vec![
//         "chess_analytics",
//         "--glob",
//         "tests/data/10_games_000000.bin.bz2",
//         "--workflow",
//         "tests/workflows/basic_eco_no_filter.json",
//     ]
//     .iter()
//     .map(|x| x.to_string()));

//     assert_eq!(
//         res,
//         "Bin	gameCount.sum
// A	2.0000	
// B	2.0000	
// C	5.0000	
// D	1.0000	
// "
//     );

//     Ok(())
// }

// #[test]
// fn eco_count_filter_white_elo() -> Result<(), Box<dyn std::error::Error>> {
//     let res = run(vec![
//         "chess_analytics",
//         "--glob",
//         "tests/data/10_games_000000.bin",
//         "--workflow",
//         "tests/workflows/basic_eco_filter_white_elo.json",
//     ]
//     .iter()
//     .map(|x| x.to_string()));

//     assert_eq!(
//         res,
//         "Bin	gameCount.sum
// B	1.0000	
// "
//     );

//     Ok(())
// }

// #[test]
// fn final_fen_filter_10_games() -> Result<(), Box<dyn std::error::Error>> {
//     let res = run(vec![
//         "chess_analytics",
//         "--glob",
//         "tests/data/10_games_000000.bin",
//         "--workflow",
//         "tests/workflows/final_fen_10_games.json",
//     ]
//     .iter()
//     .map(|x| x.to_string()));

//     assert_eq!(
//         res,
//         "Bin	gameCount.avg
// https://lichess.org/a9tcp02g	1.0000	
// https://lichess.org/j1dkb5dw	1.0000	
// "
//     );

//     Ok(())
// }
