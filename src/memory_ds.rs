// use element::*;
// use std::collections::HashMap;
// use union_joiner::UnionJoiner;
// use std::clone::Clone;
// use std::fmt::Debug;
// use serde::{Deserialize, Serialize};
// use std::sync::Mutex;

// lazy_static! {
//     pub static ref RELS: Mutex<HashMap<String, Element<'static, T>>> = {
//         let m = Mutex::new(HashMap::new());
//         m
//     };
// }

// #[derive(Debug, Clone, PartialEq)]
// pub struct MemoryDS {}


// impl<'de, T> UnionJoiner<'de, T> for MemoryDS
// where
//     T: Clone + Debug + Serialize + Deserialize<'de>,
// {
//     fn insert_element(&self, e: Element<'de, T>) -> Result<bool, String> {
//         let id = e.get_id().clone();
//         RELS.try_lock().unwrap().insert(id.clone(), e);
//         Ok(true)
//     }

//     fn get_element(&self, id: &str) -> Option<Element<'de, T>>
//     where
//         T: Clone + Debug + Serialize + Deserialize<'de>,
//     {
//         let a = RELS.try_lock().unwrap();
//         let val: Option<Element<'de, String>> = a.get(id).cloned();
//         val
//     }
// }

// impl MemoryDS {
//     pub fn get_clone(&self) -> HashMap<String, Element<String>> {
//         RELS.try_lock().unwrap().clone()
//     }

//     fn cast<'de, T>(i: T) -> Option<String>
//     where
//         T: Serialize + Deserialize<'de> + Debug + Clone,
//     {

//     }
// }
