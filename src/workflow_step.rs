use crate::game_wrapper::GameWrapper;
use crate::steps::get_step_by_name_and_params;
use mockall::predicate::*;
use mockall::*;
use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

pub type BoxedStep = Box<dyn Step>;
pub type StepFactory = Box<dyn Fn(Vec<String>) -> Result<BoxedStep, String>>;
pub type StepGeneric = Arc<Mutex<HashMap<String, SharedData>>>;

#[derive(Clone, Debug)]
pub enum SharedData {
    SharedU64(u64),
    SharedUSize(usize),
    SharedPathBuf(PathBuf),
    SharedFileData(Vec<u8>),
    SharedBool(bool),
    SharedGame(GameWrapper),
    SharedVec(Vec<SharedData>),
}

#[derive(Clone, Debug)]
pub struct StepDescription {
    pub step_type: String,
    pub parameters: Vec<String>,
}

impl StepDescription {
    pub fn to_step(&self) -> Result<BoxedStep, String> {
        get_step_by_name_and_params(
            self.step_type.to_string(),
            self.parameters.iter().map(|s| s.to_string()).collect(),
        )
    }
}

#[automock]
pub trait Step: fmt::Debug + Send + Sync {
    fn process(&mut self, data: StepGeneric) -> Result<(), String>;
}
