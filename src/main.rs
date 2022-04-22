use bevy::prelude::*;
use heron::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_startup_system(init)        
//        .add_startup_system(place_window)
        .add_system(update)
        .run();
}

fn init(
    mut commands: Commands,
) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(10.,10.)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Sphere { radius: 10.0 })
        .insert(PhysicMaterial {
            density: 0.2,
            friction: 0.0,
            ..Default::default()
        })
        .insert(Velocity::default())
        .insert(Acceleration::from_linear(Vec3::new(100.0,0.0,0.0)));
}

#[derive(Default)]
struct IsPaused(bool);

fn update(
    keys: Res<Input<KeyCode>>,
    mut physics_time: ResMut<PhysicsTime>,
    mut is_paused: Local<IsPaused>,
) {
    if keys.just_pressed(KeyCode::Space) {
        if is_paused.0 {
            physics_time.resume();
        } else {
            physics_time.pause();
        }
        is_paused.0 = !is_paused.0;
    } 
}

// fn place_window(    
//     mut windows: ResMut<Windows>,
// ) {
//     let w = windows.get_primary_mut().unwrap();
//     w.set_position(IVec2::new(1900,0));
// }
