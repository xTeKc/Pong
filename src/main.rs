use bevy::prelude::*;

fn main() {
   App::build()
       .add_system(hello_world_system.system())
       .run();
}

fn hello_world_system() {
   println!("hello world");
}



struct MyPlugin;

impl Plugin for MyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // use `app` builder
    }
}

//Resources
// fn my_system(a: Res<MyResourceA>, mut b: ResMut<MyResourceB>) {
//    // do something with `a` and `b`
// }

// fn main() {
//    App::build()
//        // specific resource value
//        .insert_resource(MyResource::new())
//        // auto-init using `Default` or `FromWorld`
//        .init_resource::<MyAutoResource>()
//        .run();
// }


struct MyEvent;

fn sender(mut writer: EventWriter<MyEvent>) {
    writer.send(MyEvent);
}

fn receiver(mut reader: EventReader<MyEvent>) {
    for event in reader.iter() {
        // handle event
    }
}

fn main_event() {
    App::build()
        .add_event::<MyEvent>()
        // ...
        .run();
}


fn app_builder() {
    // labels for custom stages:
    static DEBUG: &str = "debug";
    static MY_START: &str = "my_start";

    App::build()
        // bevy
        .add_plugins(DefaultPlugins)
        // custom plugin
        .add_plugin(MyPlugin::default())
        // specific resource value
        .insert_resource(MyResource::new())
        // auto-init using `Default` or `FromWorld`
        .init_resource::<MyAutoResource>()
        // add a custom event type:
        .add_event::<MyEvent>()
        // run once at startup:
        .add_startup_system(setup.system())
        // run each frame (in `UPDATE` stage):
        .add_system(game_update.system())
        // add custom stage:
        .add_stage_before(CoreStage::Update, MY_START, SystemStage::parallel())
        // serial stage (parallel system execution disabled)
        .add_stage_after(CoreStage::Update, DEBUG, SystemStage::single_threaded())
        // run system in specific stage:
        .add_system_to_stage(DEBUG, debug_system.system())
        // enter the bevy runtime
        .run();

}