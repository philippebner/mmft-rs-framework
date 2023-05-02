use schemars::{JsonSchema};
use serde::{Deserialize, Serialize};
use super::{channel, primitives::{Point, Dimensions}};
use self::channel::Channel;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
/// A microfluidic channel network
pub struct Network {
    /// The set of nodes in the network
    pub nodes: Vec<Node>,

    /// The set of channels in the network
    pub channels: Vec<Channel>,

    /// The set of modules in the network
    pub modules: Vec<Module>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Copy, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
/// Microfluidic network node
pub struct Node {
    /// Unique id of the node
    pub id: NodeId,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
/// Microfluidic network module
pub struct Module {
    /// Unique id of the module
    pub id: usize,

    /// Position of the module
    pub position: Point,

    /// Size of the module
    pub size: Dimensions,

    /// Node ids that are part of the interface of this module
    pub nodes: Vec<NodeId>
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Copy, Clone, PartialEq)]
/// Identifier of a node
pub struct NodeId(pub usize);
