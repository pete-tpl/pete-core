pub struct BuildContext {
    pub offset: usize,
    pub template: String,
    pub template_remain: String,
}

impl BuildContext {
    pub fn new() -> BuildContext {
        BuildContext {
            offset: 0,
            template: String::new(),
            template_remain: String::new(),
        }
    }

    pub fn clone(&self) -> BuildContext {
        BuildContext {
            offset: self.offset,
            template: self.template.clone(),
            template_remain: self.template.clone(),
        }
    }
}