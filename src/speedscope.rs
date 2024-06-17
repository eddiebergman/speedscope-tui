// This module is essentially a copy out of py-spy's speedscope
// module. The real solution would be to depend on `py-spy` as
// a crate... but I wanna learn god damnit
// Just read a raw speedscope output file from `py-spy` and attempted
// to copy the structure
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::Path;

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ProfileType {
    sampled,
    evented,
}

// NOTE: This dervied PartialEq and Eq before...dunno why
// so removed until it's a problem
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ValueUnit {
    #[serde(rename = "bytes")]
    Bytes,
    #[serde(rename = "microseconds")]
    Microseconds,
    #[serde(rename = "milliseconds")]
    Milliseconds,
    #[serde(rename = "nanoseconds")]
    Nanoseconds,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "seconds")]
    Seconds,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileView {
    #[serde(rename = "type")]
    pub profile_type: ProfileType,

    pub name: String,

    pub unit: ValueUnit,

    #[serde(rename = "startValue")]
    pub start_value: f64,

    #[serde(rename = "endValue")]
    pub end_value: f64,

    pub samples: Vec<Vec<usize>>,
    pub weights: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Frame {
    pub name: String,
    pub file: Option<String>,
    pub line: Option<usize>,
    pub col: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Shared {
    pub frames: Vec<Frame>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub profiles: Vec<ProfileView>,
    pub shared: Shared,
}

#[derive(Debug, Clone)]
pub struct Stack {
    pub frames: Vec<Frame>,
}

impl Stack {
    pub fn len(&self) -> usize {
        self.frames.len()
    }
}

#[derive(Debug, Clone)]
pub struct Profile {
    pub profile_type: ProfileType,
    pub name: String,
    pub unit: ValueUnit,
    pub start_value: f64,
    pub end_value: f64,
    pub stacks: Vec<Stack>,
    pub weights: Vec<f64>,
}

impl Profile {
    pub fn len(&self) -> usize {
        self.stacks.len()
    }

    pub fn with_filter(&self, filter: &HashSet<String>) -> Profile {
        let stacks = self
            .stacks
            .iter()
            .map(|stack| {
                let frames: Vec<Frame> = stack
                    .frames
                    .iter()
                    .filter_map(|frame| {
                        if let Some(file) = &frame.file {
                            if filter.contains(file) {
                                return None;
                            }
                            return Some(frame);
                        }
                        None
                    })
                    .cloned()
                    .collect();
                Stack { frames }
            })
            .filter(|stack| !stack.frames.is_empty())
            .collect();

        Profile {
            profile_type: self.profile_type.clone(),
            name: self.name.clone(),
            unit: self.unit.clone(),
            start_value: self.start_value,
            end_value: self.end_value,
            stacks,
            // TODO: Currently sort of just ignoring these
            weights: self.weights.clone(),
        }
    }
}

#[derive(Debug)]
pub struct Data {
    pub profiles: HashMap<String, Profile>,
}

impl Data {
    pub fn from_reader<R>(buf: R) -> Result<Self>
    where
        R: crate::io::Read,
    {
        let file: File = serde_json::from_reader(buf).map_err(|e| anyhow::anyhow!(e))?;
        let frames = file.shared.frames;

        // Bit expensive on memory to clone all frames but meh for now, they're relatively small.
        // Only issue later might be that we need to cache information for files from these frames.
        let profiles: HashMap<String, Profile> = file
            .profiles
            .into_iter()
            .map(|profile_view| {

                let stacks: Vec<Stack> = profile_view
                    .samples
                    .into_iter()
                    .filter(|sample| !sample.is_empty())
                    .map(|sample| {
                        let stack_frames = sample
                            .into_iter()
                            .map(|index| frames[index].clone())
                            .collect();
                        Stack { frames: stack_frames }
                    })
                    .collect();

                let profile = Profile {
                    profile_type: profile_view.profile_type,
                    name: profile_view.name,
                    unit: profile_view.unit,
                    start_value: profile_view.start_value,
                    end_value: profile_view.end_value,
                    stacks,
                    weights: profile_view.weights,
                };

                (profile.name.clone(), profile)
            })
            .collect();
        Ok(Data { profiles })
    }

    pub fn from_path(path: &Path) -> Result<Self> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        Data::from_reader(reader)
    }
}
