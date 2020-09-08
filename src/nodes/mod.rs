pub mod static_node;

use crate::engine::RenderResult;
use crate::error::Error;

pub trait Node {
    fn render(&self, template: &String, offset: usize) -> Result<RenderResult, Error>;
}

pub type NodeCreator = fn(template: &String, offset: usize) -> Option<Box<dyn Node>>;