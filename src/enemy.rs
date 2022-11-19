use bevy::prelude::*;
use rand::Rng;
use crate::{GameTexture, WinSize,SPRITE_SCALE, BASE_SPEED, TIME_STEP, ENEMY_SIZE};
use crate::components::Player;
use crate::components::Velocity;
use crate::components::Moveable;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, enemy_spawn_system);
    } 
}

fn enemy_spawn_system(mut commands: Commands, game_tex: Res<GameTexture>, win_size: Res<WinSize>) {

    //commute random spawn position
    let mut rng = rand::thread_rng();
    let w_span = win_size.w/2.0 - 100.;
    let h_span = win_size.h/2.0 - 100.;
    let x = rng.gen_range(-w_span..w_span);
    let y = rng.gen_range(-h_span..h_span);

    commands.spawn(SpriteBundle {
        texture: game_tex.enemy.clone(),
        transform: Transform {
            translation: Vec3::new(x, y, 10.),
            scale: Vec3::new(SPRITE_SCALE,SPRITE_SCALE,1.),
            ..default()
        },
        ..default()
    });
}