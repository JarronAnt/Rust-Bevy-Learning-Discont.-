use bevy::prelude::*;
use crate::{GameTexture, WinSize, PLAYER_SIZE, SPRITE_SCALE, BASE_SPEED, TIME_STEP};
use crate::components::Player;
use crate::components::Velocity;


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
            .add_system(player_movement_system)
            .add_system(player_keyboard_event_system);
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
    .insert(Velocity{x: 0.0, y: 0.0});
}

fn player_movement_system(mut query: Query<(&Velocity, &mut Transform), With<Player>>) {
    for (velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;
    } 

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

