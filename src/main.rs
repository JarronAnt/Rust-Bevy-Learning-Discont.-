#![allow(unused)]
#![allow(non_snake_case)]
use bevy::prelude::*;
use player::PlayerPlugin;
use components::*;

mod player;
mod components;

//constants
const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32,f32) = (144., 75.);
const SPRITE_SCALE: f32 = 0.5;
const TIME_STEP: f32 = 1.0 / 60.0;
const BASE_SPEED: f32 = 500.0;


//this is a resource that holds the window size
//the # stuff is needed as of bevy 0.9
#[derive(Resource)]
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

#[derive(Resource)]
struct GameTexture {
    player: Handle<Image>,
}

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
    .add_plugin(PlayerPlugin)
    .add_startup_system(setup_system)
    .run();
}

//this function handles inital setup of things such as camera 
fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>, mut windows: ResMut<Windows>) {
    //camera 
    commands.spawn(Camera2dBundle::default());

    //get window size
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());

    //add window size to resources
    let win_size = WinSize {
        w: win_w,
        h: win_h,
    };
    commands.insert_resource(win_size);

    //add game tex to resources
    let game_tex = GameTexture {
        player: asset_server.load(PLAYER_SPRITE),
    };
    commands.insert_resource(game_tex);

    //position window 
    //window.set_position(MonitorSelection::Primary, IVec2::new(2780,4900));
}
