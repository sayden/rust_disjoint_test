use std::fmt::Debug;
use serde::Serialize;
use serde::de::DeserializeOwned;

#[derive(Debug, Clone)]
pub struct Element<T>
where
    T: Clone + Debug + Serialize + DeserializeOwned,
{
    pub id: String,
    pub parent: String,
    pub rank: usize,
    pub meta: Option<T>,
}

impl<T> Element<T>
where
    T: Clone + Debug + Serialize + DeserializeOwned,
{
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

    pub fn set_meta(&mut self, meta: T) {
        self.meta = Some(meta)
    }

    pub fn get_meta(&self) -> &Option<T> {
        &self.meta
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }
}

pub fn new_element<T>(id: &str, meta: Option<T>) -> Element<T>
where
    T: Debug + Clone + Serialize + DeserializeOwned,
{
    Element {
        id: id.to_string(),
        meta: meta,
        parent: id.to_string(),
        rank: 0,
    }
}


pub fn new_element_with_data<T>(
    id: &String,
    parent: &String,
    rank: usize,
    meta: Option<T>,
) -> Element<T>
where
    T: Debug + Clone + Serialize + DeserializeOwned,
{
    Element {
        id: id.clone(),
        meta: meta,
        parent: parent.clone(),
        rank: rank,
    }
}
