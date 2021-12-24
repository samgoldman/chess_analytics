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
    U64(u64),
    USize(usize),
    PathBuf(PathBuf),
    FileData(Vec<u8>),
    Bool(bool),
    Game(GameWrapper),
    Vec(Vec<SharedData>),
}

impl SharedData {
    pub fn to_u64(&self) -> Option<u64> {
        match self {
            SharedData::U64(val) => Some(*val),
            _ => None,
        }
    }

    pub fn to_u64_mut(&mut self) -> Option<&mut u64> {
        match self {
            SharedData::U64(val) => Some(val),
            _ => None,
        }
    }

    pub fn to_usize_mut(&mut self) -> Option<&mut usize> {
        match self {
            SharedData::USize(val) => Some(val),
            _ => None,
        }
    }

    pub fn to_bool(&self) -> Option<bool> {
        match self {
            SharedData::Bool(val) => Some(*val),
            _ => None,
        }
    }

    pub fn to_bool_mut(&mut self) -> Option<&mut bool> {
        match self {
            SharedData::Bool(val) => Some(val),
            _ => None,
        }
    }

    pub fn to_game_mut(&mut self) -> Option<&mut GameWrapper> {
        match self {
            SharedData::Game(val) => Some(val),
            _ => None,
        }
    }

    pub fn to_vec(&self) -> Option<Vec<SharedData>> {
        match self {
            SharedData::Vec(v) => Some(v.to_vec()),
            _ => None,
        }
    }

    pub fn to_vec_mut(&mut self) -> Option<&mut Vec<SharedData>> {
        match self {
            SharedData::Vec(v) => Some(v),
            _ => None,
        }
    }

    pub fn to_path_buf(&self) -> Option<PathBuf> {
        match self {
            SharedData::PathBuf(v) => Some(v.clone()),
            _ => None,
        }
    }
}

impl std::fmt::Display for SharedData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SharedData::U64(val) => write!(f, "{}", val),
            SharedData::USize(val) => write!(f, "{}", val),
            SharedData::PathBuf(val) => write!(f, "{:?}", val),
            SharedData::FileData(val) => write!(f, "{:?}", val),
            SharedData::Bool(val) => write!(f, "{}", val),
            SharedData::Game(val) => write!(f, "{:?}", val),
            SharedData::Vec(val) => write!(f, "{:?}", val),
        }
    }
}

#[derive(Clone, Debug)]
pub struct StepDescription {
    pub step_type: String,
    pub parameters: std::option::Option<serde_yaml::Value>,
}

impl StepDescription {
    pub fn to_step(&self) -> Result<BoxedStep, String> {
        get_step_by_name_and_params(self.step_type.to_string(), self.parameters.clone())
    }
}

#[automock]
pub trait Step: fmt::Debug + Send + Sync {
    fn process(&mut self, data: StepGeneric) -> Result<(), String>;
}
