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

impl SharedData {
    pub fn to_u64(&self) -> Option<u64> {
        match self {
            SharedData::SharedU64(val) => Some(*val),
            _ => None,
        }
    }

    pub fn to_u64_mut(&mut self) -> Option<&mut u64> {
        match self {
            SharedData::SharedU64(val) => Some(val),
            _ => None,
        }
    }

    pub fn to_usize_mut(&mut self) -> Option<&mut usize> {
        match self {
            SharedData::SharedUSize(val) => Some(val),
            _ => None,
        }
    }

    pub fn to_bool(&self) -> Option<bool> {
        match self {
            SharedData::SharedBool(val) => Some(*val),
            _ => None,
        }
    }

    pub fn to_bool_mut(&mut self) -> Option<&mut bool> {
        match self {
            SharedData::SharedBool(val) => Some(val),
            _ => None,
        }
    }

    pub fn to_game_mut(&mut self) -> Option<&mut GameWrapper> {
        match self {
            SharedData::SharedGame(val) => Some(val),
            _ => None,
        }
    }

    pub fn to_vec(&self) -> Option<Vec<SharedData>> {
        match self {
            SharedData::SharedVec(v) => Some(v.to_vec()),
            _ => None,
        }
    }

    pub fn to_vec_mut(&mut self) -> Option<&mut Vec<SharedData>> {
        match self {
            SharedData::SharedVec(v) => Some(v),
            _ => None,
        }
    }
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
