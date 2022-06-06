use bevy::input::mouse::{MouseWheel, MouseButtonInput};
use bevy::prelude::*;

use clap::Parser;
use std::iter::Iterator;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

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

trait TimingFunction {
    fn value_at(&self, t:f64) -> f64;
}

#[derive(Default)]
struct Transition<T:TimingFunction> {
    duration: f64,
    delay: f64,
    // easing
    timing: T,
}
struct Easing(f64, f64, f64, f64);

enum AnimationDirection {
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
}
struct Animation<T:TimingFunction> {
    start: f64,
    duration: f64,
    iteration_count: u32,
    direction: AnimationDirection,
    timing: T,
}

impl TimingFunction for Easing {
    fn value_at(&self, t:f64) -> f64 {
        let Easing(x1, x2, y1, y2) = *self;
        bezier_easing_function(x1, x2, y1, y2, t)
    }
}

#[derive(Component)]
struct AnimatableProperty {
    current_value: f64,
    target_value: f64,
    transition: Option<Transition<Easing>>,
}

impl AnimatableProperty {
    fn new(value:f64) -> AnimatableProperty {
        AnimatableProperty { current_value: value, target_value: value, transition: None }
    }
    fn to(mut self, value:f64) -> Self {
        self.target_value = value;
        match(self.transition) {
            None => (),
            Some(Transition { duration, delay, ref timing }) => {
                println!("mmmmmm");
            },
        }
        self
    }
    fn transition(mut self, t:Transition<Easing>) -> Self {
        self.transition = Some(t);
        self
    }
}
fn main() {
    let args = Args::parse();

    use glob::glob;

    let filenames = glob(format!("{}/**/*.jp*g", args.path).as_str())
        .expect("Failed to read glob pattern")
        .filter_map(|entry| entry.ok())
        .collect();

    // let animations:Vec<Arc<Mutex<Transition>>> = Vec::new();
    App::new()
        .insert_resource(Filenames { names: filenames })
        // .insert_resource(animations)
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        // .add_system(update_animations)
        // .add_system(update_props)
        .add_system(cursor_position)
        .add_system(mouse_button_events)
        // .add_system(scroll_events)
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

fn cursor_position(
    windows: Res<Windows>,
) {
    // Games typically only have one window (the primary window).
    // For multi-window applications, you need to use a specific window ID here.
    let window = windows.get_primary().unwrap();

    if let Some(_position) = window.cursor_position() {
        // cursor is inside the window, position given
        // println!("{}", _position);
        // _position.x, _position.y;
    } else {
        // cursor is not inside the window
    }
}


fn mouse_button_events(
    mut mousebtn_evr: EventReader<MouseButtonInput>,
) {
    use bevy::input::ElementState;

    for ev in mousebtn_evr.iter() {
        match ev.state {
            ElementState::Pressed => {
                println!("Mouse button press: {:?}", ev.button);
            }
            ElementState::Released => {
                println!("Mouse button release: {:?}", ev.button);
            }
        }
    }
}

// fn update_animations(animations: ResMut<Vec<Arc<Mutex<Transition>>>>, time: Res<Time>) {
    
//     for animation_mutex in animations.iter() {
//         let mut animation = animation_mutex.lock().unwrap();
//         let periods: f64 =
//                 (animation.start - time.seconds_since_startup()) / animation.duration;
//             let t = periods - periods.floor();
//         animation.value = bezier_easing_function(animation.x1, animation.y1, animation.x2, animation.y2, t);
//     }
// }

// fn update_props(mut props: Query<(&mut AnimatableProperty)>) {
//     for mut aprop in props.iter_mut() {
//         let value;
//         match &aprop.transition {
//             Some(transition)
//             => {
                
//                 value = interpolate(
//                 aprop.start_value,
//                 aprop.end_value,
//                 transition.value())
//             },
//             None => {
//                 value = aprop.end_value;
//                 aprop.start_value = aprop.end_value;
//             },
//         }
//         aprop.current_value = value;
//     }
// }

fn picture_grid(
    mut q: Query<(&mut Transform, &AnimatableProperty, &Photo)>,
    mut photo_grids: Query<(&mut PhotoGrid)>,
    time: Res<Time>,
) {
    for grid in photo_grids.iter_mut() {
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
                current_value,
                ..
            },
            _,
        ) in q.iter_mut()
        {
            transform.translation.x = x + *current_value as f32;
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
            .insert(
                AnimatableProperty::new(0.0)
                .transition(Transition {
                    duration: 3.0,
                    delay: 0.0,
                    timing: Easing(0.33, 1.0, 0.68, 1.0),
                })
            )
            .insert(Photo);
    })
}
