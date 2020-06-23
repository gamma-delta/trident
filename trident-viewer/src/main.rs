use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::input::keyboard::{self, KeyCode};
use ggez::{Context, ContextBuilder, GameResult};

use std::{collections::HashSet, env, fs, path::PathBuf};

use trident::*;

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;

fn main() {
    // Make a Context and an EventLoop.
    let (mut ctx, mut event_loop) = ContextBuilder::new("trident-viewer", "gamma-delta")
        .window_setup(WindowSetup {
            title: "Trident Viewer".to_string(),
            ..Default::default()
        })
        .window_mode(WindowMode {
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            resizable: true,
            ..Default::default()
        })
        .build()
        .unwrap();

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object
    // so it can load resources like images during setup.
    let mut my_game = State::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct State {
    /// Where the camera is centered on.
    camera: (f32, f32),
    /// The zoom factor. 1 = normal size. More = zoomed out
    zoom: f32,
    /// The path of the file we're viewing
    path: PathBuf,
    /// The mesh we're viewing, or the error that we're getting when we loaded it.
    viewing: Result<Mesh, String>,

    /// The keys pressed the previous frame
    prev_keys: HashSet<KeyCode>,
}

impl State {
    pub fn new(_ctx: &mut Context) -> State {
        let path = env::args().nth(1).unwrap().into();
        let viewing = load_mesh(&path);
        State {
            camera: (0.0, 0.0),
            zoom: 1.0,
            path,
            viewing,
            prev_keys: HashSet::new(),
        }
    }
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const PAN_SPEED: f32 = 15.0;
        const ZOOM_RATIO: f32 = 1.02;

        // Controls
        if keyboard::is_key_pressed(ctx, KeyCode::W) {
            self.camera.1 -= PAN_SPEED;
        } else if keyboard::is_key_pressed(ctx, KeyCode::S) {
            self.camera.1 += PAN_SPEED;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::A) {
            self.camera.0 -= PAN_SPEED;
        } else if keyboard::is_key_pressed(ctx, KeyCode::D) {
            self.camera.0 += PAN_SPEED;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Q) {
            self.zoom *= ZOOM_RATIO;
        } else if keyboard::is_key_pressed(ctx, KeyCode::Z) {
            self.zoom /= ZOOM_RATIO;
        }

        if keyboard::is_key_pressed(ctx, KeyCode::R) && !self.prev_keys.contains(&KeyCode::R) {
            self.viewing = load_mesh(&self.path);
        }

        self.prev_keys = keyboard::pressed_keys(ctx).clone();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        use graphics::{DrawMode, DrawParam, Text};

        graphics::clear(ctx, graphics::WHITE);

        match &self.viewing {
            Ok(mesh) => {
                let mut mb = graphics::MeshBuilder::new();
                for shape in mesh.shapes.iter() {
                    mb.polygon(
                        DrawMode::fill(),
                        &shape
                            .points
                            .iter()
                            .map(|&(x, y)| [x, y])
                            .collect::<Vec<_>>(),
                        shape.color.into(),
                    )?;
                }
                let mesh = mb.build(ctx)?;
                graphics::draw(
                    ctx,
                    &mesh,
                    DrawParam {
                        dest: [-self.camera.0, -self.camera.1].into(),
                        scale: [self.zoom, self.zoom].into(),
                        ..Default::default()
                    },
                )?;
            }
            Err(msg) => {
                let text = format!("An error occured:\n{:#?}", msg);
                let text = Text::new(text);
                graphics::draw(ctx, &text, ([1.0, 1.0], graphics::BLACK))?;
            }
        }

        graphics::present(ctx)
    }
}

/// Load a Mesh from a filepath
fn load_mesh(path: &PathBuf) -> Result<Mesh, String> {
    let data = fs::read_to_string(path).map_err(|err| err.to_string())?;
    println!("{}", &data);
    let res = trident::Mesh::parse(&data).map_err(|err| err.to_string());
    println!("{:?}", &res);
    res
}
