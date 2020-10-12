use crate::parameter::Parameter;
use crate::context::render_context::RenderContext;

pub trait Node {
    fn render(&self, context: &RenderContext) -> Literal;
}

struct BaseNode {
    children: Vec<Box<dyn Node>>,
}

impl BaseNode {
    fn new() -> BaseNode {
        BaseNode {
            children: Vec::new(),
        }
    }
}

pub struct Literal {
    base_node: BaseNode,
    value: Parameter,
}

impl Literal {
    pub fn new(value: Parameter) -> Literal {
        Literal {
            base_node: BaseNode::new(),
            value: value,
        }
    }
}

impl Node for Literal {
    fn render(&self, _context: &RenderContext) -> Literal {
        Literal::new(self.value.clone())
    }
}