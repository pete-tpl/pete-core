pub mod container;
pub mod static_node;

use crate::engine::RenderResult;
use crate::error::Error;
use crate::parameter::ParameterStore;

pub trait Node {
    fn add_child(&mut self, child: Box<dyn Node>);
    fn build(&mut self, template: &String, offset: usize) -> RenderResult;
    fn render(&self, parameters: &ParameterStore) -> Result<String, Error>;
}

pub struct BaseNode {
    children: Vec<Box<dyn Node>>,
    end_offset: usize,
    start_offset: usize,
}

impl BaseNode {
    fn new() -> BaseNode {
        BaseNode {
            children: Vec::new(),
            end_offset: 0,
            start_offset: 0
        }
    }
}

pub type NodeCreator = fn(template: &String, offset: usize) -> Option<Box<dyn Node>>;