#![allow(unused)]
#![allow(non_snake_case)]
use bevy::prelude::*;

//assets
const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32,f32) = (144., 75.);
const SPRITE_SCALE: f32 = 0.5;


//this is a resource that holds the window size
//the # stuff is needed as of bevy 0.9
#[derive(Resource)]
pub struct WinSize {
    pub w: f32,
    pub h: f32,
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
    .add_startup_system(setup_system)
    .add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
    .run();
}

//this function handles inital setup of things such as camera 
fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>, mut windows: ResMut<Windows>) {
    //camera 
    commands.spawn(Camera2dBundle::default());

    //get window size
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());

    let win_size = WinSize {
        w: win_w,
        h: win_h,
    };
    commands.insert_resource(win_size);

    //position window 
    //window.set_position(MonitorSelection::Primary, IVec2::new(2780,4900));
}

fn player_spawn_system(mut commands: Commands, asset_server: Res<AssetServer>,  win_size: Res<WinSize>) {
    
    //add rect
    let bottom = -win_size.h/2.0;
    commands.spawn(SpriteBundle {
       texture: asset_server.load(PLAYER_SPRITE),
        transform: Transform {
            translation: Vec3::new(0.0, bottom+ PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5. , 10.),
            scale: Vec3::new(SPRITE_SCALE,SPRITE_SCALE,1.),
            ..default()
        },
        ..default()
    });
}