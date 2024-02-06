use std::cell::RefCell;
use std::collections::HashMap;

// TODO: Make it generic not just for &str, just anything that is hashable, then export it to lib
// TODO: Add translation back using a Vec
#[derive(Default)]
pub struct Dictionary<'a> {
    lookup: RefCell<HashMap<&'a str, usize>>,
    reverse_lookup: RefCell<Vec<&'a str>>,
}

// A dictionary useful for translating ridiculous keys to usize which can be used for graphs
impl<'a> Dictionary<'a> {
    pub fn new() -> Self {
        Dictionary {
            ..Default::default()
        }
    }

    pub fn id(&self, name: &'a str) -> usize {
        let lookup = self.lookup.borrow();
        if let Some(id) = lookup.get(name) {
            *id
        } else {
            let new_id = lookup.len();
            drop(lookup);

            let mut lookup = self.lookup.borrow_mut();
            lookup.insert(name, new_id);
            self.reverse_lookup.borrow_mut().push(name);
            new_id
        }
    }
    pub fn rev(&self, idx: usize) -> &str {
        self.reverse_lookup.borrow()[idx]
    }
}
