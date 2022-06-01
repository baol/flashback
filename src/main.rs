use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

use clap::Parser;
use std::iter::Iterator;
use std::path::PathBuf;
use std::sync::Arc;

mod bezier;
use bezier::*;

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

#[derive(Component)]
struct PhotoGrid {
    grid_columns: u32,
    scroll_position: f32,
}

#[derive(Component)]
struct Animation {
    // timeline
    start: f64,
    duration: f64,

    // easing
    x1: f64,
    x2: f64,
    y1: f64,
    y2: f64,
}

#[derive(Component)]
struct AnimatableProperty {
    start_value: f64,
    end_value: f64,
    animation: Arc<Animation>,
}
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
        .add_system(scroll_events)
        .add_system(picture_grid)
        .run();
}

fn scroll_events(
    mut scroll_evr: EventReader<MouseWheel>,
    mut photo_grids: Query<(&mut PhotoGrid)>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    for mut q in photo_grids.iter_mut() {
        for ev in scroll_evr.iter() {
            match ev.unit {
                MouseScrollUnit::Line => {
                    q.scroll_position += ev.y;
                    println!("photogrid scroll: {}", q.scroll_position);
                }
                MouseScrollUnit::Pixel => {
                    q.scroll_position += ev.y;
                    println!("photogrid scroll: {}", q.scroll_position);
                }
            }
        }
    }
}

fn picture_grid(
    mut q: Query<(&mut Transform, &AnimatableProperty, &Photo)>,
    mut photo_grids: Query<(&mut PhotoGrid)>,
    time: Res<Time>,
) {
    for mut grid in photo_grids.iter_mut() {
        let startx = -500.0;
        let maxx = 500.0;
        let stepx = (maxx - startx) / (grid.grid_columns as f32);
        let starty = 300.0;
        let stepy = -110.0;
        let mut x = startx;
        let mut y = starty + grid.scroll_position;
        for (
            mut transform,
            AnimatableProperty {
                start_value,
                end_value,
                animation,
            },
            _,
        ) in q.iter_mut()
        {
            let periods: f64 =
                (animation.start - time.seconds_since_startup()) / animation.duration;
            let t = periods - periods.floor();

            transform.translation.x = x + interpolate(
                *start_value,
                *end_value,
                bezier_easing_function(animation.x1, animation.y1, animation.x2, animation.y2, t),
            ) as f32;
            transform.translation.y = y;
            x += stepx;
            if x > maxx {
                x = startx;
                y += stepy;
            }
        }
    }
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>, filenames: Res<Filenames>) {
    // FIXME: loading all images in advance: should track the visible
    // images and pre-load just the needed ones.

    // let placeholder: Handle<Image> = asset_server.load("textures/placeholder.png");

    let images = filenames
        .names
        .iter()
        .map(|name| asset_server.load(name.as_path()));

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn().insert(PhotoGrid {
        grid_columns: 3,
        scroll_position: 0.0,
    });

    // prop.to_value(10).animation(a);
    let a = Arc::new(Animation {
        duration: 3.0,
        start: 0.0,
        x1: 0.33,
        y1: 1.0,
        x2: 0.68,
        y2: 1.0,
    });
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
            .insert(AnimatableProperty {
                start_value: -10.0,
                end_value: 10.0,
                animation: a.clone(),
            })
            .insert(Photo);
    })
}
