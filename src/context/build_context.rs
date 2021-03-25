use std::collections::LinkedList;

use crate::utils::collections::linked_list;

pub struct BuildContext {
    pub no_linebreaks_beginning: LinkedList<usize>,
    pub no_linebreaks_end: LinkedList<usize>,
    pub offset: usize,
    pub template: String,
    pub template_remain: String,
}

impl BuildContext {
    pub fn new() -> BuildContext {
        BuildContext {
            no_linebreaks_beginning: LinkedList::new(),
            no_linebreaks_end: LinkedList::new(),
            offset: 0,
            template: String::new(),
            template_remain: String::new(),
        }
    }

    pub fn clone(&self) -> BuildContext {
        BuildContext {
            no_linebreaks_beginning: linked_list::clone(&self.no_linebreaks_beginning),
            no_linebreaks_end: linked_list::clone(&self.no_linebreaks_end),
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