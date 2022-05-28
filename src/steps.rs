mod avg_reduce;
mod bz2_decompress_step;
mod checkmate_filter;
mod count_map;
mod eval_available_filter;
mod game_elo_bin;
mod glob_file_step;
mod init_bins_step;
mod max_reduce;
mod min_moves_filter;
mod noop_step;
mod parallel_step;
mod parse_bin_game_step;
mod perfect_checkmate_map;
mod player_elo_filter;
mod save_data_step;
mod sum_reduce;
mod time_control_bin;
mod ui_monitor_step;

use crate::workflow_step::*;

use itertools::izip;
use std::collections::HashMap;

use avg_reduce::AvgReduce;
use bz2_decompress_step::Bz2DecompressStep;
use checkmate_filter::CheckmateFilter;
use count_map::CountMap;
use eval_available_filter::EvalAvailableFilter;
use game_elo_bin::GameEloBin;
use glob_file_step::GlobFileStep;
use init_bins_step::InitBinStep;
use max_reduce::MaxReduce;
use min_moves_filter::MinMovesFilter;
use noop_step::NoopStep;
use parallel_step::ParallelStep;
use parse_bin_game_step::ParseBinGame;
use perfect_checkmate_map::PerfectCheckmateMap;
use player_elo_filter::PlayerEloFilter;
use save_data_step::SaveDataStep;
use sum_reduce::SumReduce;
use time_control_bin::TimeControlBin;
use ui_monitor_step::UiMonitorStep;

pub fn get_step_by_name_and_params(
    name: String,
    params: std::option::Option<serde_yaml::Value>,
) -> Result<BoxedStep, String> {
    let names = vec![
        "Bz2DecompressStep".to_string(),
        "InitBinStep".to_string(),
        "GameEloBin".to_string(),
        "TimeControlBin".to_string(),
        "AvgReduce".to_string(),
        "SumReduce".to_string(),
        "CountMap".to_string(),
        "MinMovesFilter".to_string(),
        "MaxReduce".to_string(),
        "SaveDataStep".to_string(),
        "ParallelStep".to_string(),
        "NoopStep".to_string(),
        "UiMonitorStep".to_string(),
        "PlayerEloFilter".to_string(),
        "PerfectCheckmateMap".to_string(),
        "CheckmateFilter".to_string(),
        "EvalAvailableFilter".to_string(),
        "ParseBinGame".to_string(),
        "GlobFileStep".to_string(),
    ];

    let funcs: Vec<StepFactory> = vec![
        Box::new(Bz2DecompressStep::try_new),
        Box::new(InitBinStep::try_new),
        Box::new(GameEloBin::try_new),
        Box::new(TimeControlBin::try_new),
        Box::new(AvgReduce::try_new),
        Box::new(SumReduce::try_new),
        Box::new(CountMap::try_new),
        Box::new(MinMovesFilter::try_new),
        Box::new(MaxReduce::try_new),
        Box::new(SaveDataStep::try_new),
        Box::new(ParallelStep::try_new),
        Box::new(NoopStep::try_new),
        Box::new(UiMonitorStep::try_new),
        Box::new(PlayerEloFilter::try_new),
        Box::new(PerfectCheckmateMap::try_new),
        Box::new(CheckmateFilter::try_new),
        Box::new(EvalAvailableFilter::try_new),
        Box::new(ParseBinGame::try_new),
        Box::new(GlobFileStep::try_new),
    ];

    let builders = izip!(names, funcs).collect::<HashMap<_, _>>();

    let result = builders.get(&name);

    match result {
        Some(step) => (step)(params),
        None => Err(format!("Step with name '{}' not found", name)),
    }
}
