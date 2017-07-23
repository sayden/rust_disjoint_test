use std::fmt::Debug;
use serde::{Serialize, Serializer, Deserialize};
use serde::de::{DeserializeOwned, Visitor, Deserializer};
use serde::de;
use serde::ser::SerializeStruct;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;

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

pub struct RawString String;

impl Serialize for Element<String> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Element", 4)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("parent", &self.parent)?;
        s.serialize_field("rank", &self.rank)?;
        s.serialize_field("meta", &self.meta)?;

        s.end()
    }
}

impl Display for Element<String> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self)
    }
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
