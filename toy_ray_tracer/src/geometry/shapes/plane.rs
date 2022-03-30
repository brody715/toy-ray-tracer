use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(JsonSchema, Serialize, Deserialize, Debug, PartialEq)]
pub enum Plane {
    YZ,
    ZX,
    XY,
}
