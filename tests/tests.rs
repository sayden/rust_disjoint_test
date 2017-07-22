extern crate redis;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

pub mod union_joiner;
pub mod element;
pub mod memory_ds;
pub mod redis_union;

use rustlations::union_joiner::UnionJoinerImpl;
use rustlations::redis_union::RedisUnion;
use rustlations::element::*;
use rustlations::memory_ds::MemoryDS;

use std::cell::RefCell;
use std::collections::HashMap;


use redis::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Caracteristics {
    pub name: String,
    pub breed: String,
}

#[test]
fn test_redis() {

    let mut redis_union = Box::new(RedisUnion {
        address: "redis://localhost:6379".to_string(),
        conn: None,
    });

    match redis_union.open_connection() {
        Err(err) => {
            panic!("{:?}", err);
        }
        _ => (),
    };

    let mem: UnionJoinerImpl<Caracteristics> = UnionJoinerImpl { strategy: redis_union };

    // let mario = "mario";
    let tesla = "tesla";
    let tyrion = "tyrion";
    // let ula = "ula";
    if false {
        mem.insert_element(new_element(
            "mario",
            Some(Caracteristics {
                name: "mario".to_string(),
                breed: "human".to_string(),
            }),
        )).and_then(|_| {
                mem.insert_element(new_element(
                    "tesla",
                    Some(Caracteristics {
                        name: "tesla".to_string(),
                        breed: "german sheperd".to_string(),
                    }),
                ))
            })
            .and_then(|_| {
                mem.insert_element(new_element(
                    "tyrion",
                    Some(Caracteristics {
                        name: "tyrion".to_string(),
                        breed: "sheep".to_string(),
                    }),
                ))
            })
            .and_then(|_| {
                mem.insert_element(new_element(
                    "ula",
                    Some(Caracteristics {
                        name: "ula".to_string(),
                        breed: "human".to_string(),
                    }),
                ))
            })
            .unwrap();
    }

    mem.union_join(tesla, tyrion);
}

#[test]
fn test_find_a_recently_inserted_element() {
    let mut redis_union = Box::new(RedisUnion {
        address: "redis://localhost:6379".to_string(),
        conn: None,
    });

    match redis_union.open_connection() {
        Err(err) => {
            panic!("{:?}", err);
        }
        _ => (),
    };

    let mem: UnionJoinerImpl<Caracteristics> = UnionJoinerImpl { strategy: redis_union };


    let testing0 = new_element(
        "testing0",
        Some(Caracteristics {
            name: "testing0".to_string(),
            breed: "testing0_meta".to_string(),
        }),
    );

    mem.insert_element(testing0).unwrap();
    let result = mem.find("testing0").unwrap();

    assert_eq!(result.get_id().to_string(), "testing0".to_string());
    assert_eq!(
        result.get_meta().clone().unwrap().name,
        "testing0".to_string()
    );
    assert_eq!(
        result.get_meta().clone().unwrap().breed,
        "testing0_meta".to_string()
    );

    clean()
}

#[test]
fn test_join_two_elements() {
    let mut redis_union = Box::new(RedisUnion {
        address: "redis://localhost:6379".to_string(),
        conn: None,
    });

    match redis_union.open_connection() {
        Err(err) => {
            panic!("{:?}", err);
        }
        _ => (),
    };

    let mem: UnionJoinerImpl<Caracteristics> = UnionJoinerImpl { strategy: redis_union };

    let testing = new_element("testing2", None);
    let testing3 = new_element("testing3", None);

    println!("{}", mem.insert_element(testing).unwrap());
    println!("{}", mem.insert_element(testing3).unwrap());

    mem.union_join("testing2", "testing3");

    let testing_res = mem.find("testing2").unwrap();
    let testing_res3 = mem.find("testing3").unwrap();

    assert_eq!(testing_res.get_parent(), "testing3");
    assert_eq!(testing_res3.get_parent(), "testing3");
    assert_eq!(testing_res.get_rank(), 1);
    assert_eq!(testing_res3.get_rank(), 1);

    let testing_res_2 = mem.get_element("testing2").unwrap();
    assert_eq!(testing_res_2.get_rank(), 0);
    assert_eq!(testing_res_2.get_parent(), "testing3");

    let testing_res_3 = mem.get_element("testing3").unwrap();
    assert_eq!(testing_res_3.get_rank(), 1);
    assert_eq!(testing_res_3.get_parent(), "testing3");

    clean()
}


#[test]
fn test_join_various_elements_that_didnt_exist_before() {
    let mut redis_union = Box::new(RedisUnion {
        address: "redis://localhost:6379".to_string(),
        conn: None,
    });

    match redis_union.open_connection() {
        Err(err) => {
            panic!("{:?}", err);
        }
        _ => (),
    };

    let mem: UnionJoinerImpl<Caracteristics> = UnionJoinerImpl { strategy: redis_union };

    mem.union_join("testing4", "testing5");
    mem.union_join("testing6", "testing7");
    mem.union_join("testing7", "testing5");

    let res4 = mem.get_element("testing4").unwrap();
    let res5 = mem.get_element("testing5").unwrap();
    let res6 = mem.get_element("testing6").unwrap();
    let res7 = mem.get_element("testing7").unwrap();

    assert_eq!(res4.get_rank(), 0);
    assert_eq!(res5.get_rank(), 2);
    assert_eq!(res6.get_rank(), 0);
    assert_eq!(res7.get_rank(), 1);
    assert_eq!(res6.get_parent(), "testing7");


    let find_result = mem.find("testing6").unwrap();
    assert_eq!(find_result.get_parent(), "testing5");

    clean();
}

#[test]
fn test_find_a_recently_inserted_element_in_memory() {
    let memory_strategy = MemoryDS { hash: RefCell::new(HashMap::new()) };

    let mem: UnionJoinerImpl<Caracteristics> =
        UnionJoinerImpl { strategy: Box::new(memory_strategy) };


    let testing0 = new_element(
        "testing0",
        Some(Caracteristics {
            name: "testing0".to_string(),
            breed: "testing0_meta".to_string(),
        }),
    );

    mem.insert_element(testing0).unwrap();
    let result = mem.find("testing0").unwrap();

    assert_eq!(result.get_id().to_string(), "testing0".to_string());
    assert_eq!(
        result.get_meta().clone().unwrap().name,
        "testing0".to_string()
    );
    assert_eq!(
        result.get_meta().clone().unwrap().breed,
        "testing0_meta".to_string()
    );
}

fn clean() {
    println!(
        "Delete: {:?}",
        redis::Client::open("redis://localhost:6379")
            .and_then(|client| client.get_connection())
            .and_then(|connection| {
                redis::pipe()
                    .atomic()
                    .cmd("DEL")
                    .arg("testing")
                    .arg("testing0")
                    .arg("testing2")
                    .arg("testing3")
                    .arg("testing4")
                    .arg("testing5")
                    .arg("testing6")
                    .arg("testing7")
                    .arg("mario")
                    .arg("tesla")
                    .arg("tyrion")
                    .arg("ula")
                    .query::<Value>(&connection)
            })
    );
}
