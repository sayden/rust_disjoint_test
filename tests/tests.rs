extern crate rustlations;

use rustlations::union_joiner::UnionJoinerImpl;
use rustlations::redis_union::RedisUnion;


use rustlations::memory_ds::MemoryDS;

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

    let mem: UnionJoinerImpl<String> = UnionJoinerImpl { strategy: redis_union };

    let mario = "mario";
    let tesla = "tesla";
    let tyrion = "tyrion";
    let ula = "ula";

    mem.union_join(tesla, tyrion);
    mem.union_join(tyrion, mario);
    mem.union_join(tyrion, ula);

    println!("Mario? {:?}", mem.find(&mario).unwrap());
    println!("Ula? {:?}", mem.find(&ula).unwrap());
    println!("Tesla? {:?}", mem.find(&tesla).unwrap());
    println!("Tyrion? {:?}", mem.find(&tyrion).unwrap());

    // mem.union_join(ula, mario);
    // mem.union_join("contador", "valverde");
    // mem.union_join("froome", "contador");
    // mem.union_join("froome", "tyrion");

    // mem.find("froome");
    // mem.find("contador");
    // mem.union_join("contador", "froome");
}

#[test]
fn test_memory() {
    let memoryds = Box::new(MemoryDS {});

    let mem: UnionJoinerImpl<String> = UnionJoinerImpl { strategy: memoryds };

    let mario = "mario";
    let tesla = "tesla";
    let tyrion = "tyrion";
    let ula = "ula";

    mem.union_join(tesla, tyrion);
    mem.union_join(tyrion, mario);
    mem.union_join(tyrion, ula);

    println!("Mario? {:?}", mem.find(&mario).unwrap());
    println!("Ula? {:?}", mem.find(&ula).unwrap());
    println!("Tesla? {:?}", mem.find(&tesla).unwrap());
    println!("Tyrion? {:?}", mem.find(&tyrion).unwrap());

    println!("");

    for kv in rustlations::memory_ds::RELS.lock().unwrap().iter() {
        println!(
            "id: {}, parent: {}, rank: {}",
            kv.0,
            kv.1.get_parent(),
            kv.1.get_rank()
        );
    }

    println!("");

    println!("Mario? {:?}", mem.find(&mario).unwrap());

    println!("");

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

}
