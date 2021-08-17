use bevy::prelude::*;

fn hello_world() {
    println!("Hello World");
}

fn main() {
    App::build()
        //converts hello_world func into system type
        //add_system func adds system to apps schedule
        .add_system(hello_world.system())
        .run();
}

// struct Position { x: f32, y: f32}

// fn print_position_system(query: Query<&Transform>) {
//     for transform in query.iter() {
//         println!("position: {:?}", transform.translation);
//     }
// }

// struct Entity(u64);