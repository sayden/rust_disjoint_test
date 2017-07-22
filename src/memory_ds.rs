use element::*;
use std::collections::HashMap;
use union_joiner::UnionJoiner;
use std::clone::Clone;
use std::fmt::Debug;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct MemoryDS<T>
where
    T: Clone + Debug + Serialize + DeserializeOwned,
{
    pub hash: RefCell<HashMap<String, Element<T>>>,
}


impl<T> UnionJoiner<T> for MemoryDS<T>
where
    T: Clone + Debug + Serialize + DeserializeOwned,
{
    fn insert_element(&self, e: Element<T>) -> Result<(), String> {
        let id = e.get_id().clone();
        self.hash.borrow_mut().insert(id.clone(), e);
        Ok(())
    }

    fn get_element(&self, id: &str) -> Option<Element<T>>
    where
        T: Clone + Debug + Serialize + DeserializeOwned,
    {
        let a = self.hash.borrow_mut();
        let val: Option<Element<T>> = a.get(id).cloned();
        val
    }
}
