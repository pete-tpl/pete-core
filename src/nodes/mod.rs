pub mod container;
pub mod comment;
pub mod static_node;

use crate::engine::{NodeBuildResult, RenderResult};
use crate::parameter::ParameterStore;

pub trait Node {
    fn add_child(&mut self, child: Box<dyn Node>);
    fn build(&mut self, template: &String, offset: usize) -> NodeBuildResult;
    fn render(&self, parameters: &ParameterStore) -> RenderResult;
}

pub struct BaseNode {
    children: Vec<Box<dyn Node>>,
    end_offset: usize,
    has_nolinebreak_beginning: bool,
    has_nolinebreak_end: bool,
    start_offset: usize,
}

impl BaseNode {
    fn new() -> BaseNode {
        BaseNode {
            children: Vec::new(),
            end_offset: 0,
            has_nolinebreak_beginning: false,
            has_nolinebreak_end: false,
            start_offset: 0
        }
    }
}

pub type NodeCreator = fn(template: &String) -> Option<Box<dyn Node>>;