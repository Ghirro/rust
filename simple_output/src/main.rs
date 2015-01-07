extern crate serialize;
extern crate test;

use serialize::json::decode;
use serialize::json::encode;
use std::io::File;
use test::individuals::Player;

fn main() {
    let player_json =
        std::io::File::open(&Path::new("player.json")).read_to_string().unwrap();
    
    let player = serialize::json::decode::<Player>(player_json.as_slice()).unwrap();

    println!("{}", serialize::json::encode(&player));
}
