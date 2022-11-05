use crate::game::Game;
use crate::steps::get_step_by_name_and_params;
use itertools::Itertools;
use mockall::automock;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;

pub type BoxedStep = Box<dyn Step>;

#[automock]
pub trait StepData: Send {
    fn insert(&mut self, k: String, v: SharedData) -> Option<SharedData>;
    fn contains_key(&self, k: &str) -> bool;
    fn remove(&mut self, k: &str) -> Option<SharedData>;

    fn remove_vec(&mut self, k: &str) -> Option<Vec<SharedData>>;
    fn init_vec_if_unset(&mut self, k: &str);
    fn init_map_if_unset(&mut self, k: &str);
    fn clear_vec(&mut self, k: &str) -> Option<Vec<SharedData>>;
    fn try_push_to_vec(&mut self, k: &str, v: SharedData) -> Result<(), &'static str>;
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl StepData for HashMap<String, SharedData> {
    fn remove_vec(&mut self, k: &str) -> Option<Vec<SharedData>> {
        match self.remove(k) {
            Some(v) => v.into_vec(),
            None => None,
        }
    }

    fn insert(&mut self, k: String, v: SharedData) -> Option<SharedData> {
        self.insert(k, v)
    }

    fn contains_key(&self, k: &str) -> bool {
        self.contains_key(k)
    }

    fn remove(&mut self, k: &str) -> Option<SharedData> {
        self.remove(k)
    }

    fn init_vec_if_unset(&mut self, k: &str) {
        if !self.contains_key(k) {
            self.insert(k.to_string(), SharedData::Vec(vec![]));
        }
    }

    fn init_map_if_unset(&mut self, k: &str) {
        if !self.contains_key(k) {
            self.insert(k.to_string(), SharedData::Map(HashMap::new()));
        }
    }

    fn clear_vec(&mut self, k: &str) -> Option<Vec<SharedData>> {
        match self.entry(k.to_string()) {
            Entry::Occupied(entry) => {
                let old = entry.replace_entry(SharedData::Vec(vec![]));
                old.1.into_vec()
            }
            Entry::Vacant(_) => None,
        }
    }

    fn try_push_to_vec(&mut self, k: &str, v: SharedData) -> Result<(), &'static str> {
        match self.entry(k.to_string()) {
            Entry::Occupied(mut entry) => match entry.get_mut() {
                SharedData::Vec(vec) => {
                    vec.push(v);
                    Ok(())
                }
                _ => Err("Not a Vec"),
            },
            Entry::Vacant(_) => Err("Key not present"),
        }
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

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl SharedData {
    pub fn to_u64(&self) -> Option<u64> {
        match self {
            SharedData::U64(val) => Some(*val),
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

    pub fn into_vec(self) -> Option<Vec<SharedData>> {
        match self {
            SharedData::Vec(v) => Some(v),
            _ => None,
        }
    }

    #[cfg(test)]
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
            SharedData::Map(v) => Some(v.clone()),
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
                _ => panic!("Max: Cannot compare {self:?} to {rhs:?}"),
            },
            SharedData::F64(s) => match rhs {
                SharedData::F64(r) => SharedData::F64(f64::max(*s, *r)),
                _ => panic!("Max: Cannot compare {self:?} to {rhs:?}"),
            },
            SharedData::USize(s) => match rhs {
                SharedData::USize(r) => SharedData::USize(usize::max(*s, *r)),
                _ => panic!("Max: Cannot compare {self:?} to {rhs:?}"),
            },
            _ => panic!("Max is not valid for {self:?}"),
        }
    }
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl std::fmt::Display for SharedData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SharedData::U64(val) => write!(f, "{val}"),
            SharedData::F64(val) => write!(f, "{val}"),
            SharedData::USize(val) => write!(f, "{val}"),
            SharedData::PathBuf(val) => write!(f, "{val:?}"),
            SharedData::FileData(val) => write!(f, "{val:?}"),
            SharedData::Bool(val) => write!(f, "{val}"),
            SharedData::Game(val) => write!(f, "{val:?}"),
            SharedData::BinnedValue(val) => write!(f, "{val:?}"),
            SharedData::String(val) => write!(f, "{val}"),
            SharedData::Vec(val) => write!(f, "{val:?}"),
            SharedData::StepDescription(val) => write!(f, "{val:?}"),
            SharedData::Map(val) => {
                for k in val.keys().sorted() {
                    writeln!(f, "\t\"{k}\": {}", val.get(k).unwrap())?;
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

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl StepDescription {
    pub fn to_step(&self) -> Result<BoxedStep, String> {
        get_step_by_name_and_params(&self.step_type, self.parameters.clone())
    }
}

#[automock]
pub trait Step: fmt::Debug {
    fn process(&mut self, data: &mut HashMap<String, SharedData>) -> Result<bool, String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fmt_shared_data_map() {
        assert_eq!(
            format!("{}", SharedData::Map(HashMap::new())),
            String::new()
        );
        let mut map = HashMap::new();
        map.insert("key_string".to_string(), SharedData::U64(42));
        assert_eq!(
            format!("{}", SharedData::Map(map)),
            "\t\"key_string\": 42\n".to_string()
        );
    }
}
