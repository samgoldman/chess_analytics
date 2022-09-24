use criterion::{black_box, criterion_group, criterion_main, Criterion};

use chess_analytics::run;

fn criterion_benchmark(c: &mut Criterion) {
    let configs = vec![
        //"1_simple_count_10_games", // NOTE: Too much fluctuation
        "2_simple_count_eval_and_clock_1_game",
        "3_count_mates_by_time_10_games",
        "4_player_elo_tc_bin_1",
        "5_eval_available_filter",
        "6_parse_pgn",
        "7_perfect_checkmate_avg",
    ];

    for config in configs {
        c.bench_function(format!("run {}", config).as_str(), |b| {
            b.iter(|| {
                run(black_box(
                    vec![
                        "chess_analytics",
                        format!("benches/workflows/{}.yaml", config).as_str(),
                    ]
                    .iter(),
                ))
                .unwrap()
            })
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
