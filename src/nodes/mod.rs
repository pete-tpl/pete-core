pub mod container;
pub mod comment;
pub mod tags;
pub mod expression;
pub mod static_node;

use crate::context::build_context::BuildContext;
use crate::context::render_context::RenderContext;
use crate::engine::{NodeBuildResult, RenderResult};

const EXPRESSION_START: &str = "{{";
const EXPRESSION_END: &str = "}}";
const COMMENT_START: &str = "{#";
const COMMENT_END: &str = "#}";
const TAG_START: &str = "{%";
const TAG_END: &str = "%}";

pub trait Node {
    fn add_child(&mut self, child: Box<dyn Node>);
    fn build(&mut self, context: &BuildContext) -> NodeBuildResult;
    fn is_continuation(&self, context: &BuildContext) -> bool;
    fn render(&self, context: &RenderContext) -> RenderResult;

    fn has_nolinebreak_end(&self) -> bool {
        self.get_base_node().has_nolinebreak_end
    }

    fn has_nolinebreak_beginning(&self) -> bool {
        self.get_base_node().has_nolinebreak_beginning
    }

    fn get_base_node(&self) -> &BaseNode;

    fn debug_name(&self) -> &str;
    fn debug_print(&self) -> String {
        return format!("[{} - {}] {}", self.get_base_node().start_offset, self.get_base_node().end_offset ,self.debug_name())
    }
    
    
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