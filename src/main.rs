use bevy::prelude::*;

fn hello_world() {
    println!("Hello World");
}

fn main() {
    App::build()
        //registers the startup system
        .add_startup_system(add_people.system())
        //converts hello_world func into system type
        //add_system func adds system to apps schedule
        .add_system(hello_world.system())
        .run();
}

//
struct Person;

fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("John
Doe".to_string()));
}









// struct Position { x: f32, y: f32}

// fn print_position_system(query: Query<&Transform>) {
//     for transform in query.iter() {
//         println!("position: {:?}", transform.translation);
//     }
// }

// struct Entity(u64);