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

#[derive(Component)]
struct Photo;

fn main() {
    let args = Args::parse();

    use glob::glob;

    let filenames = glob(format!("{}/**/*.jp*g", args.path).as_str())
        .expect("Failed to read glob pattern")
        .filter_map(|entry| entry.ok())
        .collect();

    App::new()
        .insert_resource(Filenames { names: filenames })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(picture_grid)
        .run();
}

fn picture_grid(mut q: Query<(&mut Transform, &Photo)>) {
    let startx = -500.0;
    let maxx = 500.0;
    let stepx = 170.0;
    let starty = 300.0;
    let stepy = -110.0;
    let mut x = startx;
    let mut y = starty;
    for (mut transform, _) in q.iter_mut() {
        transform.translation.x = x;
        transform.translation.y = y;
        x += stepx;
        if x > maxx {
            x = startx;
            y += stepy;
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
    images.for_each(|image: Handle<Image>| {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(160.0, 100.0)),
                    ..default()
                },
                texture: image,
                ..default()
            })
            .insert(Photo);
    })
}
