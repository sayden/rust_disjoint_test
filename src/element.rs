use std::fmt::Debug;

pub trait Identificable {
    fn get_id(&self) -> &String;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Element<T: Clone + Debug> {
    pub id: String,
    pub parent: String,
    pub rank: usize,
    pub meta: Option<T>,
}


impl<T: Clone + Debug> Element<T> {
    pub fn set_parent_with_string(&mut self, id: String) {
        self.parent = id
    }

    pub fn get_rank(&self) -> usize {
        self.rank
    }

    pub fn set_rank(&mut self, new_rank: usize) {
        self.rank = new_rank
    }

    pub fn get_parent(&self) -> &String {
        &self.parent
    }
}

impl<T: Clone + Debug> Identificable for Element<T> {
    fn get_id(&self) -> &String {
        &self.id
    }
}
