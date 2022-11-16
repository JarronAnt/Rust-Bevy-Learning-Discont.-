#![allow(unused)]
use bevy::prelude::*;


fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.05,0.05,0.05)))
    .insert_resource(WindowDescriptor {
        title: "learning bevy".to_string(),
        width: 598.0,
        height: 676.0,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .run();
}
