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

use crate::workflow_step::BoxedStep;

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
    name: &str,
    params: std::option::Option<serde_yaml::Value>,
) -> Result<BoxedStep, String> {
    match name {
        "Bz2DecompressStep" => Bz2DecompressStep::try_new(params),
        "InitBinStep" => InitBinStep::try_new(params),
        "GameEloBin" => GameEloBin::try_new(params),
        "TimeControlBin" => TimeControlBin::try_new(params),
        "AvgReduce" => AvgReduce::try_new(params),
        "SumReduce" => SumReduce::try_new(params),
        "CountMap" => CountMap::try_new(params),
        "MinMovesFilter" => MinMovesFilter::try_new(params),
        "MaxReduce" => MaxReduce::try_new(params),
        "SaveDataStep" => SaveDataStep::try_new(params),
        "ParallelStep" => ParallelStep::try_new(params),
        "NoopStep" => Ok(NoopStep::boxed_new()),
        "UiMonitorStep" => UiMonitorStep::try_new(params),
        "PlayerEloFilter" => PlayerEloFilter::try_new(params),
        "PerfectCheckmateMap" => PerfectCheckmateMap::try_new(params),
        "CheckmateFilter" => CheckmateFilter::try_new(params),
        "EvalAvailableFilter" => EvalAvailableFilter::try_new(params),
        "ParseBinGame" => Ok(ParseBinGame::boxed_new()),
        "GlobFileStep" => GlobFileStep::try_new(params),
        _ => Err(format!("Step with name '{}' not found", name)),
    }
}
