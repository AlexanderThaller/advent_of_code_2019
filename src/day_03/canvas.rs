use crate::day_03::{
    Position,
    Positions,
};
use ggez::{
    self,
    event::{
        self,
        KeyCode,
        KeyMods,
    },
    graphics::{
        self,
        Color,
        DrawMode,
        Mesh,
        MeshBuilder,
    },
    mint,
    nalgebra as na,
    Context,
    GameResult,
};

const FIELD_SIZE_X: f32 = 900.0;
const FIELD_SIZE_Y: f32 = 900.0;
const SCALE_FACTOR: f32 = 25.0;
const DRAW_START_POINT_X: f32 = FIELD_SIZE_X / 2.0;
const DRAW_START_POINT_Y: f32 = FIELD_SIZE_Y - 100.0;
const VIEW_STEP_FACTOR: f32 = 10.0;

pub const MAGENTA: Color = Color {
    r: 255.0,
    g: 0.0,
    b: 255.0,
    a: 1.0,
};

pub const BLUE: Color = Color {
    r: 0.0,
    g: 188.0,
    b: 255.0,
    a: 1.0,
};

pub const YELLOW: Color = Color {
    r: 255.0,
    g: 255.0,
    b: 0.0,
    a: 1.0,
};

pub const WHITE: Color = Color {
    r: 255.0,
    g: 255.0,
    b: 255.0,
    a: 1.0,
};

pub const GREEN: Color = Color {
    r: 0.0,
    g: 255.0,
    b: 0.0,
    a: 1.0,
};

pub const RED: Color = Color {
    r: 255.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};

pub const COLOR_BACKGROUND: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};

pub struct Canvas {
    pixels: Vec<MeshBuilder>,
    cached_pixels: Vec<Mesh>,
    zoom: f32,
    view_x: f32,
    view_y: f32,
}

impl Default for Canvas {
    fn default() -> Self {
        Self {
            pixels: Vec::new(),
            cached_pixels: Vec::new(),
            zoom: 1.0,
            view_x: 0.0,
            view_y: 0.0,
        }
    }
}

impl Canvas {
    pub fn run(self) {
        let window_setup = ggez::conf::WindowSetup {
            title: "Boxes!".to_owned(),
            samples: ggez::conf::NumSamples::Zero,
            vsync: true,
            icon: "".to_owned(),
            srgb: true,
        };

        let window_mode = ggez::conf::WindowMode {
            width: FIELD_SIZE_X + 20.0,
            height: FIELD_SIZE_Y + 20.0,
            maximized: false,
            fullscreen_type: ggez::conf::FullscreenType::Windowed,
            borderless: false,
            min_width: 0.0,
            max_width: 0.0,
            min_height: 0.0,
            max_height: 0.0,
            resizable: false,
        };

        let context_builder = ggez::ContextBuilder::new("game_boxes", "ggez")
            .window_setup(window_setup)
            .window_mode(window_mode);

        let (context, events_loop) = &mut context_builder.build().unwrap();

        let mut state = self;
        event::run(context, events_loop, &mut state).unwrap();
    }

    pub fn add_start_point(&mut self, start_point: &Position, color: Color) {
        let mut mesh = MeshBuilder::new();

        mesh.circle(
            DrawMode::fill(),
            na::Point2::new(
                start_point.x as f32 / SCALE_FACTOR,
                start_point.y as f32 / SCALE_FACTOR,
            ),
            5.0,
            5.0,
            color,
        );

        self.pixels.push(mesh);
    }

    pub fn add_intersections(&mut self, positions: &Positions, color: Color) {
        let mut mesh = MeshBuilder::new();

        let points: Vec<_> = positions.into();
        for (x, y) in points {
            mesh.circle(
                DrawMode::stroke(1.0),
                na::Point2::new(x / SCALE_FACTOR, y / SCALE_FACTOR),
                5.0,
                5.0,
                color,
            );
        }

        self.pixels.push(mesh);
    }

    pub fn add_closest_intersection(&mut self, intersection: &Position, color: Color) {
        let mut mesh = MeshBuilder::new();

        mesh.circle(
            DrawMode::stroke(1.0),
            na::Point2::new(
                intersection.x as f32 / SCALE_FACTOR,
                intersection.y as f32 / SCALE_FACTOR,
            ),
            5.0,
            5.0,
            color,
        );

        self.pixels.push(mesh);
    }

    pub fn add_positions(&mut self, positions: &Positions, color: Color) {
        let mut mesh = MeshBuilder::new();

        let points: Vec<_> = positions.into();
        let points: Vec<_> = points
            .into_iter()
            .map(|(x, y)| mint::Point2 {
                x: x / SCALE_FACTOR,
                y: y / SCALE_FACTOR,
            })
            .collect();

        mesh.line(&points, 1.0, color).unwrap();

        self.pixels.push(mesh);
    }
}

impl event::EventHandler for Canvas {
    fn update(&mut self, context: &mut Context) -> GameResult {
        if self.cached_pixels.is_empty() {
            self.cached_pixels = self
                .pixels
                .iter()
                .map(|builder| builder.build(context).unwrap())
                .collect();
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, COLOR_BACKGROUND);

        for mesh in &self.cached_pixels {
            graphics::draw(
                context,
                mesh,
                (na::Point2::new(DRAW_START_POINT_X, DRAW_START_POINT_Y),),
            )?;
        }

        graphics::present(context)?;

        Ok(())
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        match keycode {
            KeyCode::Up => {
                self.view_y -= VIEW_STEP_FACTOR;
                println!("View y is now {}", self.view_y);
            }

            KeyCode::Down => {
                self.view_y += VIEW_STEP_FACTOR;
                println!("View y is now {}", self.view_y);
            }

            KeyCode::Left => {
                self.view_x -= VIEW_STEP_FACTOR;
                println!("View x is now {}", self.view_x);
            }

            KeyCode::Right => {
                self.view_x += VIEW_STEP_FACTOR;
                println!("View x is now {}", self.view_x);
            }

            KeyCode::Add => {
                self.zoom -= 0.1;
                println!("Zoom is now {}", self.zoom);
            }

            KeyCode::Subtract => {
                self.zoom += 0.1;
                println!("Zoom is now {}", self.zoom);
            }

            KeyCode::R => {
                let default = Canvas::default();

                self.zoom = default.zoom;
                self.view_x = default.view_x;
                self.view_y = default.view_y;

                println!("Reset settings to default");
            }

            _ => {}
        }

        let (w, h) = graphics::size(ctx);

        let new_rect = graphics::Rect::new(
            self.view_x,
            self.view_y,
            w as f32 * self.zoom,
            h as f32 * self.zoom,
        );

        graphics::set_screen_coordinates(ctx, new_rect).unwrap();
    }
}
