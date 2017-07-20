#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[post("/v1/<id>/<parent>")]
fn index(id: String, parent: String) -> String {
    format!("Index: {}, {}", id, parent)
}

#[get("/v1/<id>")]
fn get(id: String) -> String {
    format!("Get: {}", id)
}

#[put("/v1/<id>/son_of/<parent>")]
fn union(id: String, parent: String) -> String {
    format!("Union: {}, {}", id, parent)
}

fn main() {
    rocket::ignite()
        .mount("/v1/<id>/<parent>", routes![index])
        .launch();
    rocket::ignite().mount("/v1", routes![get]).launch();
    rocket::ignite()
        .mount("/v1<id>/son_of", routes![union])
        .launch();
}
