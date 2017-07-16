use element::*;
use std::collections::HashMap;
use union_joiner::UnionJoiner;
use std::clone::Clone;

use std::sync::Mutex;

lazy_static! {
    pub static ref RELS: Mutex<HashMap<String, Element<String>>> = {
        let m = Mutex::new(HashMap::new());
        m
    };
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemoryDS {}


impl UnionJoiner<String> for MemoryDS {
    fn insert_element(&self, e: Element<String>) -> Result<bool, String> {
        let id = e.get_id().clone();
        RELS.try_lock().unwrap().insert(id.clone(), e);
        Ok(true)
    }

    fn get_element(&self, id: &str) -> Option<Element<String>> {
        let a = RELS.try_lock().unwrap();
        let val: Option<Element<String>> = a.get(id).cloned();
        val
    }
}

impl MemoryDS {
    pub fn get_clone(&self) -> HashMap<String, Element<String>> {
        RELS.try_lock().unwrap().clone()
    }
}
