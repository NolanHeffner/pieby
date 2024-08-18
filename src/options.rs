
use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::RwLock};

/* use serde::{Deserialize, Serialize};
use serde_json::Value;   */

/* pub struct Options {
    pub hash: u32,
    pub threads: u8,
} */

#[derive(Clone, Copy)]
pub enum OptionValue {
    Integer(u64),
    Float(f32),
}

impl From<u64> for OptionValue {
    fn from(val: u64) -> Self {
        OptionValue::Integer(val)
    }
}

impl From<f32> for OptionValue {
    fn from(val: f32) -> Self {
        OptionValue::Float(val)
    }
}

#[derive(Clone, Copy)]
struct UCIOption {
    name: &'static str,
    value: OptionValue,
}

impl UCIOption {
    pub fn new<T: Into<OptionValue>>(name: &'static str, val: T) -> UCIOption {
        UCIOption {
            name: name,
            value: val.into(),
        }
    }

    pub fn get_name(&self) -> &'static str {
        self.name
    }

    pub fn get_value(&self) -> OptionValue {
        self.value
    }
}

type OptionsMap = HashMap::<&'static str, UCIOption>;
static UCI_OPTIONS : Lazy<RwLock<OptionsMap>> = Lazy::new(|| {RwLock::new(OptionsMap::new())});

pub fn get_option(name: &str) -> UCIOption {
    *UCI_OPTIONS.read().unwrap().get(&name).unwrap()
}

pub fn set_option(name: &'static str, value: OptionValue) {
    UCI_OPTIONS.write().unwrap().insert(name, UCIOption::new(name, value));
}

pub fn init_options() {
    let int_options : [(&str, u64); 2]  = [
        ("Hash", 16),
        ("Threads", 8),
        // Add more options as needed
    ];

    let flt_options : [(&str, f32); 0] = [];

    let mut map = UCI_OPTIONS.write().unwrap();

    for (name, value) in int_options.iter() {
        map.insert(
            name,
            UCIOption::new(name, *value)
        );
    }

    for (name, value) in flt_options.iter() {
        map.insert(
            name,
            UCIOption::new(name, *value)
        );
    }
}