extern crate redis;

use element::*;
use union_joiner::UnionJoiner;
use std::clone::Clone;
use std::fmt::Debug;

use std::result::Result;

use self::redis::Connection;
use self::redis::RedisError;
use self::redis::Commands;

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
}

impl<T> UnionJoiner<T> for RedisUnion
where
    T: Clone + Debug,
{
    fn insert_element(&self, e: Element<T>) -> Result<bool, String> {
        // println!("Inserting: {:?}", e);

        match self.conn {
            Some(ref connection) => {
                redis::pipe()
                    .atomic()
                    .cmd("HSET")
                    .arg(e.get_id())
                    .arg("parent")
                    .arg(e.get_parent())
                    .ignore()
                    .cmd("HSET")
                    .arg(e.get_id())
                    .arg("rank")
                    .arg(e.get_rank())
                    .query::<Value>(connection)
                    .map_err(|err| format!("Error trying to insert element: {}", err))
                    .map(|_| true)
            }
            None => Err("No connection available".to_string()),
        }
    }

    fn get_element(&self, id: &str) -> Option<Element<T>> {
        // println!("Getting element");
        match self.conn {
            Some(ref connection) => {
                match connection.hgetall::<&str, Vec<Value>>(id) {
                    Ok(values) => {

                        if values.len() <= 1 {
                            return None;
                        }

                        // println!("ID: {}, got {:?}", id, values);

                        let element: Element<T> = Element {
                            id: id.to_string(),
                            parent: redis::from_redis_value(&values[1])
                                .or::<String>(Ok(id.to_owned()))
                                .unwrap(),

                            rank: values
                                .get(3)
                                .and_then(|value_found| {
                                    redis::from_redis_value(value_found).or::<usize>(Ok(0)).ok()
                                })
                                .unwrap(),
                            meta: None,
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
