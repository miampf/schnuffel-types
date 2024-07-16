use core::panic;
use std::collections::HashMap;

use extism::{typed_plugin, Manifest, Plugin, Wasm};
use extism_convert::{FromBytes, Msgpack, ToBytes};
use serde::{Deserialize, Serialize};

use crate::graph::{Graph, Node};

// The type information that a plugin must adhere to.
// TODO: let plugins return results once own error is written.
typed_plugin!(SchnuffelPlugin {
    default_config(Input<String>) -> Output<HashMap<String, String>>;
    exec_on_node(Input<Node>) -> Output<Graph>;
    exec_on_graph(Input<Graph>) -> Output<Graph>;
});

#[derive(Clone, Debug, ToBytes, FromBytes, Serialize, Deserialize)]
#[encoding(Msgpack)]
/// Input of a plugin defined function.
pub struct Input<T> {
    config: HashMap<String, String>,
    data: T,
}
#[derive(Clone, Debug, ToBytes, FromBytes, Serialize, Deserialize)]
#[encoding(Msgpack)]
/// Output of a plugin defined function.
pub struct Output<T> {
    data: T,
}
