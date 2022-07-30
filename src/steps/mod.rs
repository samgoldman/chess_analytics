mod bins;
mod filters;
mod io_steps;
mod maps;
mod misc_steps;
mod parsers;
mod reducers;

use crate::workflow_step::BoxedStep;

use bins::*;
use filters::*;
use io_steps::*;
use maps::*;
use misc_steps::*;
use parsers::*;
use reducers::*;

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
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
        "ExportGames" => ExportGames::try_new(params),
        "ParsePgnStep" => ParsePgnStep::try_new(params),
        _ => Err(format!("Step with name '{}' not found", name)),
    }
}
