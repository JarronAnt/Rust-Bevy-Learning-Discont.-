#![allow(unused)]
#![allow(non_snake_case)]
use bevy::prelude::*;


fn main() {
    //this stuff creates the app, builds a window and calls our setup
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .add_plugins(DefaultPlugins.set(WindowPlugin{
        window: WindowDescriptor {
            width: 598.0,
            height: 676.0,
            title: "learn bevy".to_string(),
            ..default()
        },
        ..default()
    }))
    .add_startup_system(setup_system)
    .run();
}

//this function handles inital setup of things such as camera 
fn setup_system(mut commands: Commands) {
    //camera 
    commands.spawn(Camera2dBundle::default());

    //add rect
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25,0.25,0.75),
            custom_size: Some(Vec2::new(150.0,150.0)),
            ..default()
        },
        ..default()
    });
}