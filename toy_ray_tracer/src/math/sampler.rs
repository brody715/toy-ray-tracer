use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::vec::Vec3;

#[derive(JsonSchema, Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SamplerType {
    Uniform { block_size: [i32; 2] },
    Random,
    // Random but with fixed sample points
    RandomFixed { block_size: [i32; 2] },
    BlueNoise { block_size: [i32; 2]},
}

pub trait Sampler {
    fn sample_direction(&self, _origin: &Vec3) -> Vec3;
}

pub type SamplerPtr = Box<dyn Sampler + Sync + Send>;

pub struct NopSampler {}

impl NopSampler {
    pub fn new() -> Self {
        Self {}
    }
}

impl Sampler for NopSampler {
    fn sample_direction(&self, _origin: &Vec3) -> Vec3 {
        unimplemented!()
    }
}
