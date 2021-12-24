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
pub type StepFactory = Box<dyn Fn(Option<serde_yaml::Value>) -> Result<BoxedStep, String>>;
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

    pub fn to_path_buf(&self) -> Option<PathBuf> {
        match self {
            SharedData::SharedPathBuf(v) => Some(v.clone()),
            _ => None,
        }
    }
}

impl std::fmt::Display for SharedData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SharedData::SharedU64(val) => write!(f, "{}", val),
            SharedData::SharedUSize(val) => write!(f, "{}", val),
            SharedData::SharedPathBuf(val) => write!(f, "{:?}", val),
            SharedData::SharedFileData(val) => write!(f, "{:?}", val),
            SharedData::SharedBool(val) => write!(f, "{}", val),
            SharedData::SharedGame(val) => write!(f, "{:?}", val),
            SharedData::SharedVec(val) => write!(f, "{:?}", val),
        }
    }
}


#[derive(Clone, Debug)]
pub struct StepDescription {
    pub step_type: String,
    pub parameters: std::option::Option<serde_yaml::Value>
}

impl StepDescription {
    pub fn to_step(&self) -> Result<BoxedStep, String> {
        get_step_by_name_and_params(
            self.step_type.to_string(),
            self.parameters.clone()
        )
    }
}

#[automock]
pub trait Step: fmt::Debug + Send + Sync {
    fn process(&mut self, data: StepGeneric) -> Result<(), String>;
}
