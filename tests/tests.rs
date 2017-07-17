extern crate rustlations;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use rustlations::union_joiner::UnionJoinerImpl;
use rustlations::redis_union::RedisUnion;
use rustlations::element::*;


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
    // mem.union_join(tyrion, mario);
    // mem.union_join(tyrion, ula);

    // println!("Mario? {:?}", mem.find(&mario).unwrap());
    // println!("Ula? {:?}", mem.find(&ula).unwrap());
    // println!("Tesla? {:?}", mem.find(&tesla).unwrap());
    // println!("Tyrion? {:?}", mem.find(&tyrion).unwrap());

    // mem.union_join(ula, mario);
    // mem.union_join("contador", "valverde");
    // mem.union_join("froome", "contador");
    // mem.union_join("froome", "tyrion");

    // mem.find("froome");
    // mem.find("contador");
    // mem.union_join("contador", "froome");
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


    let tesla = new_element(
        "tesla",
        Some(Caracteristics {
            name: "tesla".to_string(),
            breed: "german_sheperd".to_string(),
        }),
    );

    mem.insert_element(tesla).unwrap();
    let result = mem.find("tesla").unwrap();

    assert_eq!(result.get_id().to_string(), "tesla".to_string());
    assert_eq!(result.get_meta().clone().unwrap().name, "tesla".to_string());
    assert_eq!(
        result.get_meta().clone().unwrap().breed,
        "german_sheperd".to_string()
    );
}

// fn test_memory() {
// let memoryds = Box::new(MemoryDS {});

// let mem: UnionJoinerImpl<String> = UnionJoinerImpl { strategy: memoryds };

// let mario = "mario";
// let tesla = "tesla";
// let tyrion = "tyrion";
// let ula = "ula";

// mem.union_join(tesla, tyrion);
// mem.union_join(tyrion, mario);
// mem.union_join(tyrion, ula);

// println!("Mario? {:?}", mem.find(&mario).unwrap());
// println!("Ula? {:?}", mem.find(&ula).unwrap());
// println!("Tesla? {:?}", mem.find(&tesla).unwrap());
// println!("Tyrion? {:?}", mem.find(&tyrion).unwrap());

// println!("");

// for kv in rustlations::memory_ds::RELS.lock().unwrap().iter() {
//     println!(
//         "id: {}, parent: {}, rank: {}",
//         kv.0,
//         kv.1.get_parent(),
//         kv.1.get_rank()
//     );
// }

// println!("");

// println!("Mario? {:?}", mem.find(&mario).unwrap());

// println!("");

// mem.union_join(ula, mario);
// mem.union_join("contador", "valverde");
// mem.union_join("froome", "contador");
// mem.union_join("froome", "tyrion");

// for kv in rustlations::memory_ds::RELS.lock().unwrap().iter() {
//     println!("{},{},{}", kv.0, kv.1.get_parent(), kv.1.get_rank());
// }

// println!("");

// mem.find("froome");
// mem.find("contador");
// mem.union_join("contador", "froome");

// for kv in rustlations::memory_ds::RELS.lock().unwrap().iter() {
//     println!("{},{},{}", kv.0, kv.1.get_parent(), kv.1.get_rank());
// }
// }
