use std::collections::LinkedList;

/// Clones a linked list
/// 
/// # Examples
/// ```
/// use std::collections::LinkedList;
/// use pete_core::utils::collections::linked_list;
/// 
/// let mut src: LinkedList<usize> = LinkedList::new();
/// src.push_back(100);
/// src.push_back(300);
/// assert_eq!(src.pop_back().unwrap(), 300);
/// assert_eq!(src.pop_back().unwrap(), 100);
/// assert_eq!(src.pop_back().is_none(), true);
/// ```
pub fn clone(src: &LinkedList<usize>) -> LinkedList<usize> {
    let mut result: LinkedList<usize> = LinkedList::new();

    for item in src {
        result.push_back(*item);
    }

    result
}