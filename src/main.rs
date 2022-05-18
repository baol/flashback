use bevy::prelude::*;
use clap::Parser;
use std::iter::Iterator;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The path to explore for pictures
    #[clap(short, long)]
    path: String,
}

#[derive(Default)]
struct Filenames {
    names: Vec<PathBuf>,
}

fn main() {
    let args = Args::parse();

    use glob::glob;

    let filenames = glob(format!("{}/**/*.jp?g", args.path).as_str())
        .expect("Failed to read glob pattern")
        .filter(|entry| entry.is_ok())
        .map(|entry| entry.unwrap())
        .collect();

    App::new()
        .insert_resource(Filenames { names: filenames })
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, filenames: Res<Filenames>) {
    // FIXME: loading all images in advance: should track the visible
    // images and pre-load just the needed ones.
    let images = filenames
        .names
        .iter()
        .map(|name| asset_server.load(name.as_path()));

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            // FIXME: taking just the first image for now
            texture: images.last().take().unwrap(),
            transform: Transform::from_scale(Vec3::splat(0.3)),
            ..default()
        })
        .insert(Direction::Up);
}
