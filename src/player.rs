use bevy::prelude::*;
use crate::{GameTexture, WinSize, PLAYER_SIZE, SPRITE_SCALE, BASE_SPEED, TIME_STEP};
use crate::components::Player;
use crate::components::Velocity;
use crate::components::Moveable;


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
            .add_system(player_keyboard_event_system)
            .add_system(player_fire_system);
    } 
}

fn player_spawn_system(mut commands: Commands, game_tex: Res<GameTexture>,  win_size: Res<WinSize>) {
    
    //add rect
    let bottom = -win_size.h/2.0;
    commands.spawn(SpriteBundle {
       texture: game_tex.player.clone(),
        transform: Transform {
            translation: Vec3::new(0.0, bottom+ PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5. , 10.),
            scale: Vec3::new(SPRITE_SCALE,SPRITE_SCALE,1.),
            ..default()
        },
        ..default()
    })
    .insert(Player)
    .insert(Moveable{auto_despawn: false})
    .insert(Velocity{x: 0.0, y: 0.0});
}

fn player_keyboard_event_system(kb: Res<Input<KeyCode>>, mut query: Query<&mut Velocity, With<Player>>) {
    
    if let Ok( mut velocity) = query.get_single_mut(){
        velocity.x = if kb.pressed(KeyCode::A) {
            -1.0
        } else if kb.pressed(KeyCode::D) {
            1.0
        } else {
            0.0
        };
    }
}

fn player_fire_system(mut commands: Commands, 
                    kb: Res<Input<KeyCode>>, 
                    game_tex: Res<GameTexture>,  
                    query: Query<&Transform, With<Player>> ) {

    if let Ok(player_tf) = query.get_single() {
        if kb.just_pressed(KeyCode::Space) {
            let (x,y) = (player_tf.translation.x, player_tf.translation.y);
            commands.spawn(SpriteBundle {
                texture: game_tex.player_laser.clone(),
                transform: Transform {
                    translation: Vec3::new(x,y,0.),
                    scale: Vec3::new(SPRITE_SCALE,SPRITE_SCALE,1.),
                    ..default()
                },
                ..default()
            })
            .insert(Moveable{auto_despawn: true})
            .insert(Velocity{x: 0.0, y: 1.0});
        }
    }

}