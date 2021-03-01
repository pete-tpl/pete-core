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

    // Increments offset, removes the part of template_remain before offset
    pub fn apply_offset(&mut self, offset: usize) {
        self.template_remain = self.template_remain[offset+1..].to_string();
        self.offset += offset;
    }
}