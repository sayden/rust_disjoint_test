extern crate redis;

use element::*;
use union_joiner::UnionJoiner;
use std::clone::Clone;
use std::fmt::Debug;

use std::result::Result;

use self::redis::Connection;
use self::redis::RedisError;
use self::redis::Commands;

use serde::Serialize;
use serde::de::DeserializeOwned;

use serde_json;

use self::redis::Value;


pub struct RedisUnion {
    pub address: String,
    pub conn: Option<Connection>,
}

impl RedisUnion {
    pub fn open_connection(&mut self) -> Result<String, RedisError> {

        redis::Client::open(self.address.as_str())
            .and_then(|client| client.get_connection())
            .and_then(|connection| {
                self.conn = Some(connection);
                Ok("Ok".to_string())
            })

    }

    fn insert(
        &self,
        id: &String,
        parent: &String,
        rank: usize,
        meta: Option<String>,
    ) -> Result<(), String> {
        // println!("Inserting: {:?}", e);

        match (&self.conn, meta) {
            (&Some(ref connection), Some(ref meta_info)) => {
                redis::pipe()
                    .atomic()
                    .cmd("HSET")
                    .arg(id)
                    .arg("parent")
                    .arg(parent)
                    .ignore()
                    .cmd("HSET")
                    .arg(id)
                    .arg("rank")
                    .arg(rank)
                    .ignore()
                    .cmd("HSET")
                    .arg(id)
                    .arg("meta")
                    .arg(meta_info)
                    .query::<Value>(connection)
                    .map_err(|err| format!("Error trying to insert element: {}", err))
                    .map(|_| ())
            }
            (&Some(ref connection), None) => {
                redis::pipe()
                    .atomic()
                    .cmd("HSET")
                    .arg(id)
                    .arg("parent")
                    .arg(parent)
                    .ignore()
                    .cmd("HSET")
                    .arg(id)
                    .arg("rank")
                    .arg(rank)
                    .query::<Value>(connection)
                    .map_err(|err| format!("Error trying to insert element: {}", err))
                    .map(|_| ())
            }
            (&None, _) => Err("No connection available".to_string()),
        }
    }

    fn get(&self, id: &str) -> Option<Element<String>> {
        // println!("Getting element");
        match self.conn {
            Some(ref connection) => {
                match connection.hgetall::<&str, Vec<Value>>(id) {
                    Ok(values) => {

                        if values.len() <= 1 {
                            return None;
                        }

                        // println!("ID: {}, got {:?}", id, values);
                        let element: Element<String> = Element {
                            id: id.to_string(),
                            parent: redis::from_redis_value(&values[1])
                                .or::<String>(Ok(id.to_owned()))
                                .unwrap(),

                            rank: values
                                .get(3)
                                .map(|value_found: &Value| {
                                    redis::from_redis_value(value_found).unwrap_or(0)
                                })
                                .unwrap(),
                            meta: values.get(5).map(|value_found: &Value| {
                                println!("Element?: {:?}", value_found);
                                redis::from_redis_value(value_found).unwrap_or("".to_string())
                            }),
                        };

                        // println!("{:?}", &element);

                        Some(element)
                    }
                    Err(err) => {
                        println!("Error trying to get elements: {}", err);
                        None
                    }
                }
            }
            _ => {
                println!("Error trying to get connection");
                None
            }
        }
    }
}

impl<T> UnionJoiner<T> for RedisUnion
where
    T: Clone + Debug + Serialize + DeserializeOwned + ToString,
{
    fn insert_element(&self, e: Element<T>) -> Result<(), String> {
        let meta = e.get_meta().clone().and_then(
            |m: T| serde_json::to_string(&m).ok(),
        );

        self.insert(e.get_id(), e.get_parent(), e.get_rank(), meta)
    }

    fn get_element(&self, id: &str) -> Option<Element<T>> {
        let new_meta = match self.get(id) {
            Some(ele) => {
                match ele.get_meta() {
                    &Some(ref meta) => serde_json::from_str(meta.as_str()).ok(),
                    &None => None,
                }
            }
            None => None,
        };

        let result: Option<Element<T>> = match self.get(id) {
            Some(ele) => {
                Some(new_element_with_data(
                    ele.get_id(),
                    ele.get_parent(),
                    ele.get_rank(),
                    new_meta,
                ))
            }
            None => None,
        };

        result
    }
}
