use crate::common::variable::VariableStore;

pub struct RenderContext {
    pub filename: String,
    // set inside node which is currently being rendered,
    // because renderer function is not aware of current node internals
    // example: {% if ... -%}CONTENT{%- else %}
    // ELSE block's flag goes here
    pub next_has_nolinebreak_beginning: bool,
    pub previous_has_nolinebreak_end: bool,
    pub previous_was_static: bool,
    pub offset: usize,
    pub parameters: VariableStore,
    pub template: String,
}

impl RenderContext {
    pub fn new() -> RenderContext {
        RenderContext {
            filename: String::new(),
            next_has_nolinebreak_beginning: false,
            previous_has_nolinebreak_end: false,
            previous_was_static: false,
            offset: 0,
            parameters: VariableStore::new(),
            template: String::new(),
        }
    }

    pub fn clone(&self) -> RenderContext {
        let mut cloned = RenderContext::new();
        cloned.filename = self.filename.clone();
        cloned.offset = self.offset;
        for (k, v) in self.parameters.iter() {
            cloned.parameters.insert(k.to_string(), v.clone());
        }
        cloned.template = self.template.clone();

        cloned
    }
}