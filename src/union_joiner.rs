use element::*;
use std::fmt::Debug;

pub trait UnionJoiner<T: Clone + Debug> {
    fn insert_element(&self, e: Element<T>) -> Result<bool, String>;
    fn get_element(&self, id: &str) -> Option<Element<T>>;
}

pub struct UnionJoinerImpl<T: Clone + Debug> {
    pub strategy: Box<UnionJoiner<T>>,
}


impl<T> UnionJoinerImpl<T>
where
    T: Clone + Debug,
{
    pub fn find(&self, id: &str) -> Option<Element<T>> {
        let mut _id = id.to_string();

        // println!("Find 0!");

        loop {
            let new_id: Option<Element<T>>;
            {
                new_id = self.strategy.get_element(&_id)
            }

            match new_id {
                Some(candidate) => {
                    let candidate_parent = candidate.get_parent();
                    let candidate_id = candidate.get_id();

                    // This is the parent node
                    if *candidate_parent == *candidate_id {
                        //Point the initial node to this new candidate
                        match self.strategy.insert_element(Element {
                            id: id.to_owned(),
                            parent: candidate_parent.clone(),
                            rank: candidate.get_rank(),
                            meta: None,
                        }) {
                            Err(err) => println!("Error trying to insert: {}", err),
                            _ => (),
                        }

                        // println!("Finding");

                        return Some(candidate.clone());
                    }

                    //Parent node is still unknown
                    _id = candidate_parent.clone();
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
            (Some(e), None) => (
                e,
                false,
                Element {
                    id: parent.to_string(),
                    parent: parent.to_string(),
                    rank: 0,
                    meta: None,
                },
                true,
            ),

            (None, Some(e)) => (
                Element {
                    id: from.to_string(),
                    parent: from.to_string(),
                    rank: 0,
                    meta: None,
                },
                true,
                e,
                false,
            ),

            (None, None) => (
                Element {
                    id: from.to_string(),
                    parent: from.to_string(),
                    rank: 0,
                    meta: None,
                },
                true,
                Element {
                    id: parent.to_string(),
                    parent: parent.to_string(),
                    rank: 0,
                    meta: None,
                },
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
            (_, _) => {
                match self.strategy.insert_element(_p.clone()) {
                    Err(err) => println!("{}", err),
                    _ => (),
                };
            }
        }

        match self.set_relation(_f, _p) {
            Err(err) => println!("{}", err),
            _ => (),
        };
    }


    pub fn set_relation(&self, from: Element<T>, parent: Element<T>) -> Result<bool, String> {
        // println!(
        //     "Setting relationships: F:{}, P:{}",
        //     from.get_id(),
        //     parent.get_id()
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
        let mut new_rank = parent.get_rank() + son.get_rank();
        if new_rank == 0 {
            new_rank = 1;
        }

        parent.set_rank(new_rank);
        son.set_parent_with_string(parent.get_id().clone());

        match (
            self.strategy.insert_element(parent),
            self.strategy.insert_element(son),
        ) {
            (Ok(_), Ok(_)) => return Ok(true),
            (Err(e), _) => return Err(e),
            (_, Err(e)) => return Err(e),
        }
    }
}
