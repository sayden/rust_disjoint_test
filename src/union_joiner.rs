use element::*;
use std::fmt::Debug;
use serde::Serialize;
use serde::de::DeserializeOwned;

pub trait UnionJoiner<T>
where
    T: Clone + Debug + Serialize + DeserializeOwned,
{
    fn insert_element(&self, e: Element<T>) -> Result<bool, String>;
    fn get_element(&self, id: &str) -> Option<Element<T>>;
}

pub struct UnionJoinerImpl<T>
where
    T: Clone + Debug + Serialize + DeserializeOwned,
{
    pub strategy: Box<UnionJoiner<T>>,
}


impl<T> UnionJoinerImpl<T>
where
    T: Clone + Debug + Serialize + DeserializeOwned,
{
    pub fn find(&self, id: &str) -> Option<Element<T>> {
        let mut _id: String = id.to_string();

        // println!("Find 0!");

        let mut initial: Option<Element<T>> = None;

        loop {
            let new_element: Option<Element<T>> = self.strategy.get_element(&_id);
            match initial {
                None => initial = new_element.clone(),
                _ => (),
            }

            match new_element {
                Some(candidate) => {
                    let candidate_parent = candidate.get_parent().clone();
                    let candidate_id = candidate.get_id();

                    // Is this the parent node?
                    if *candidate_parent == *candidate_id {

                        // Is this the same than the searched element?
                        if candidate_id == initial.clone().unwrap().get_id() {
                            return initial;
                        }

                        //Point the initial node to this new candidate
                        match self.strategy.insert_element(Element {
                            id: id.to_owned(),
                            parent: candidate_parent.clone(),
                            rank: candidate.get_rank(),
                            meta: initial.unwrap().get_meta().clone(),
                        }) {
                            Err(err) => println!("Error trying to insert: {}", err),
                            _ => (),
                        }

                        // println!("Finding");

                        return Some(candidate.clone());
                    }

                    //Parent node is still unknown
                    _id = candidate_parent;
                }
                None => return None,
            }
        }
    }

    pub fn union_join(&self, from: &str, parent: &str) {
        let f = self.find(&from);
        let p = self.find(&parent);

        // println!("f: {:?}, p: {:?}", f, p);

        let (_f, must_insert_from, _p, must_insert_parent) = match (f, p) {
            (Some(e1), Some(e2)) => (e1, false, e2, false),
            (Some(e), None) => (e, false, new_element(parent, None), true),
            (None, Some(e)) => (new_element(from, None), true, e, false),
            (None, None) => (
                new_element(from, None),
                true,
                new_element(parent, None),
                true,
            ),
        };

        // println!("UnionJoin!");

        match (must_insert_from, must_insert_parent) {
            (true, false) => {
                match self.strategy.insert_element(_f.clone()) {
                    Err(err) => println!("{}", err),
                    _ => (),
                }
            }
            (false, true) => {
                match self.strategy.insert_element(_p.clone()) {
                    Err(err) => println!("{}", err),
                    _ => (),
                }
            }
            (true, true) => {
                match self.strategy.insert_element(_p.clone()) {
                    Err(err) => println!("{}", err),
                    _ => (),
                };
            }
            (_, _) => (),
        }

        match self.set_relation(_f, _p) {
            Err(err) => println!("{}", err),
            _ => (),
        };
    }


    pub fn set_relation(&self, from: Element<T>, parent: Element<T>) -> Result<bool, String> {
        // println!(
        // "Setting relationships: F:{}, P:{}",
        // from.get_id(),
        // parent.get_id()
        // );

        if from.get_id() == parent.get_id() {
            return Ok(true);
        }

        if from.get_rank() > parent.get_rank() {
            return self.change_parents_and_insert(parent, from);
        } else {
            return self.change_parents_and_insert(from, parent);
        }
    }

    fn change_parents_and_insert(
        &self,
        mut son: Element<T>,
        mut parent: Element<T>,
    ) -> Result<bool, String> {
        println!(
            "son: {},{} parent {},{}",
            son.get_id(),
            son.get_rank(),
            parent.get_id(),
            parent.get_rank()
        );

        let mut new_rank = parent.get_rank() + son.get_rank();
        if new_rank == 0 {
            new_rank = 1;
        }


        parent.set_rank(new_rank);
        son.set_parent_with_string(parent.get_id().clone());

        println!(
            "son: {},{} parent {},{}",
            son.get_id(),
            son.get_rank(),
            parent.get_id(),
            parent.get_rank()
        );

        match (
            self.strategy.insert_element(parent),
            self.strategy.insert_element(son),
        ) {
            (Ok(_), Ok(_)) => return Ok(true),
            (Err(e), _) => return Err(e),
            (_, Err(e)) => return Err(e),
        }
    }

    pub fn insert_element(&self, e: Element<T>) -> Result<bool, String> {
        self.strategy.insert_element(e)
    }
}
