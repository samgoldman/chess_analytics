use crate::game::Game;
use crate::steps::get_step_by_name_and_params;
use mockall::automock;
use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

pub type BoxedStep = Box<dyn Step>;
pub type StepGeneric = Arc<Mutex<dyn StepGenericCore>>;

#[automock]
pub trait StepGenericCore: Send {
    fn insert(&mut self, k: &str, v: SharedData) -> Option<SharedData>;
    fn contains_key(&self, k: &str) -> bool;
    fn get(&self, k: &str) -> Option<SharedData>;
    fn remove(&mut self, k: &str) -> Option<SharedData>;
}

pub struct StepGenericCoreImpl {
    pub map: HashMap<String, SharedData>,
}

impl StepGenericCore for StepGenericCoreImpl {
    fn contains_key(&self, k: &str) -> bool {
        self.map.contains_key(k)
    }

    fn get(&self, k: &str) -> Option<SharedData> {
        self.map.get(k).map(|v| (*v).clone())
    }

    fn insert(&mut self, k: &str, v: SharedData) -> Option<SharedData> {
        self.map.insert(k.to_string(), v)
    }

    fn remove(&mut self, k: &str) -> Option<SharedData> {
        self.map.remove(k)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum SharedData {
    U64(u64),
    F64(f64),
    USize(usize),
    PathBuf(PathBuf),
    FileData(Vec<u8>),
    Bool(bool),
    Game(Game),
    BinnedValue((Box<SharedData>, Vec<SharedData>)),
    String(String),
    Vec(Vec<SharedData>),
    StepDescription(StepDescription),
    Map(HashMap<String, SharedData>),
}

impl SharedData {
    pub fn to_u64(&self) -> Option<u64> {
        match self {
            SharedData::U64(val) => Some(*val),
            _ => None,
        }
    }

    pub fn to_usize(&self) -> Option<usize> {
        match self {
            SharedData::USize(val) => Some(*val),
            _ => None,
        }
    }

    pub fn to_bool(&self) -> Option<bool> {
        match self {
            SharedData::Bool(val) => Some(*val),
            _ => None,
        }
    }

    pub fn to_vec(&self) -> Option<Vec<SharedData>> {
        match self {
            SharedData::Vec(v) => Some(v.clone()),
            _ => None,
        }
    }

    pub fn to_binned_value(&self) -> Option<&(Box<SharedData>, Vec<SharedData>)> {
        match self {
            SharedData::BinnedValue(v) => Some(v),
            _ => None,
        }
    }

    pub fn to_string(&self) -> Option<&String> {
        match self {
            SharedData::String(v) => Some(v),
            _ => None,
        }
    }

    pub fn to_step_description(&self) -> Option<&StepDescription> {
        match self {
            SharedData::StepDescription(v) => Some(v),
            _ => None,
        }
    }

    pub fn to_map(&self) -> Option<HashMap<String, SharedData>> {
        match self {
            SharedData::Map(v) => Some((*v).clone()),
            _ => None,
        }
    }

    pub fn to_path_buf(&self) -> Option<PathBuf> {
        match self {
            SharedData::PathBuf(v) => Some(v.clone()),
            _ => None,
        }
    }

    pub fn max(&self, rhs: &SharedData) -> SharedData {
        match self {
            SharedData::U64(s) => match rhs {
                SharedData::U64(r) => SharedData::U64(u64::max(*s, *r)),
                _ => panic!("Max: Cannot compare {:?} to {:?}", self, rhs),
            },
            SharedData::F64(s) => match rhs {
                SharedData::F64(r) => SharedData::F64(f64::max(*s, *r)),
                _ => panic!("Max: Cannot compare {:?} to {:?}", self, rhs),
            },
            SharedData::USize(s) => match rhs {
                SharedData::USize(r) => SharedData::USize(usize::max(*s, *r)),
                _ => panic!("Max: Cannot compare {:?} to {:?}", self, rhs),
            },
            _ => panic!("Max is not valid for {:?}", self),
        }
    }
}

impl std::fmt::Display for SharedData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SharedData::U64(val) => write!(f, "{}", val),
            SharedData::F64(val) => write!(f, "{}", val),
            SharedData::USize(val) => write!(f, "{}", val),
            SharedData::PathBuf(val) => write!(f, "{:?}", val),
            SharedData::FileData(val) => write!(f, "{:?}", val),
            SharedData::Bool(val) => write!(f, "{}", val),
            SharedData::Game(val) => write!(f, "{:?}", val),
            SharedData::BinnedValue(val) => write!(f, "{:?}", val),
            SharedData::String(val) => write!(f, "{}", val),
            SharedData::Vec(val) => write!(f, "{:?}", val),
            SharedData::StepDescription(val) => write!(f, "{:?}", val),
            SharedData::Map(val) => {
                writeln!(f, "Map: ").expect("Write fail");
                for (k, v) in val.iter() {
                    write!(f, "\t{:?}: {}\t", k, v).expect("Write fail");
                }
                Ok(())
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StepDescription {
    pub step_type: String,
    pub parameters: std::option::Option<serde_yaml::Value>,
}

impl StepDescription {
    pub fn to_step(&self) -> Result<BoxedStep, String> {
        get_step_by_name_and_params(&self.step_type, self.parameters.clone())
    }
}

#[automock]
pub trait Step: fmt::Debug + Send + Sync {
    fn process(&mut self, data: StepGeneric) -> Result<(), String>;
}
