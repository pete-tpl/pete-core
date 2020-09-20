use crate::parameter::ParameterStore;

pub struct RenderContext {
    pub filename: String,
    pub offset: usize,
    pub parameters: ParameterStore,
    pub template: String,
}

impl RenderContext {
    pub fn new() -> RenderContext {
        RenderContext {
            filename: String::new(),
            offset: 0,
            parameters: ParameterStore::new(),
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