#![allow(unused)]
#![allow(non_snake_case)]
use bevy::prelude::*;

//assets
const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32,f32) = (144., 75.);

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
fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    //camera 
    commands.spawn(Camera2dBundle::default());

    //add rect
    commands.spawn(SpriteBundle {
       texture: asset_server.load(PLAYER_SPRITE),
       
        ..default()
    });
}