use std::collections::HashMap;

use extism_convert::{FromBytes, Msgpack, ToBytes};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, ToBytes, FromBytes, Serialize, Deserialize)]
#[encoding(Msgpack)]
/// Input of a plugin defined function.
pub struct Input<T> {
    pub config: HashMap<String, String>,
    pub data: T,
}
#[derive(Clone, Debug, ToBytes, FromBytes, Serialize, Deserialize)]
#[encoding(Msgpack)]
/// Output of a plugin defined function.
pub struct Output<T> {
    pub data: T,
}
