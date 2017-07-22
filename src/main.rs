#![feature(plugin)]
#![plugin(rocket_codegen)]

use rocket::State;
extern crate serde;
extern crate serde_json;
extern crate rocket;

#[macro_use]
extern crate serde_derive;

pub mod union_joiner;
pub mod element;
pub mod memory_ds;
pub mod redis_union;

use union_joiner::UnionJoinerImpl;
use redis_union::RedisUnion;
use element::*;



#[post("/<id>", format = "application/json", data = "<input>")]
fn insert(state: State<UnionJoinerImpl<String>>, id: String, input: Option<String>) -> String {
    match state
        .insert_element(new_element(id.as_str(), input))
        .and_then(|_| Ok("Ok".to_string())) {
        Ok(msg) => msg,
        Err(msg) => msg,
    }
}

#[get("/<id>")]
fn get_element(id: String) -> String {
    format!("Get element: {}", id)
}

#[put("/<id>/son_of/<parent>")]
fn union(id: String, parent: String) -> String {
    format!("Union: {}, {}", id, parent)
}

fn main() {
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

    let mem: UnionJoinerImpl<String> = UnionJoinerImpl { strategy: redis_union };

    rocket::ignite()
        .manage(mem )
        .mount("/v1/element", routes![get_element, insert])
        .mount("/v1/union", routes![union])
        // .mount("/v1", routes![union])
        .launch();
}
