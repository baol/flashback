use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(picture_fly)
        .run();
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

fn picture_fly(time: Res<Time>, mut q: Query<(&mut Transform, &mut Direction)>) {
    for (mut transform, mut direction) in q.iter_mut() {
        match *direction {
            Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
            Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
        }

        if transform.translation.y > 200. {
            *direction = Direction::Down;
        } else if transform.translation.y < -200. {
            *direction = Direction::Up;
        }
    }
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image = asset_server.load("textures/pine-watt-3_Xwxya43hE-unsplash.jpeg");
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            texture: image,
            transform: Transform::from_scale(Vec3::splat(0.3)),
            ..default()
        })
        .insert(Direction::Up);
}
