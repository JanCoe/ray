use crate::colour::Colour;
use crate::point::Point;
use crate::ray::{self, INF};
pub const CANVAS_WIDTH: u32 = 400;
pub const CANVAS_HEIGHT: u32 = 300;

pub struct World {
    pub scene: Vec<Sphere>,
    pub lights: Vec<Light>,
    pub camera: Camera,
}

impl World {
    pub fn new() -> Self {
        World {
            scene: Self::get_things(),
            lights: Self::get_lights(),
            camera: Camera::default(),
        }
    }

    pub fn update(&mut self) {
        self.camera.pitch = self.camera.pitch + 0.08;
        self.camera.roll = self.camera.roll + 0.08;
        self.scene[0].radius -= 0.01;
        let Point(_, y, _) = self.scene[1].center;
        if y < 199. {self.scene[1].center = self.scene[1].center + Point(0., 0.1, 0.1);}
        self.scene[1].center = self.scene[1].center + Point(0., 0.1, 0.1);
        self.lights[1].intensity -= 0.005;
        self.lights[0].intensity -= 0.005;
    }

    pub fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let pxl = i as isize;
            let width = CANVAS_WIDTH as isize;
            let height = CANVAS_HEIGHT as isize;
            let x = (pxl % width - width / 2) as i16;
            let y = (-pxl / width + height / 2) as i16;

            let canvas = Point(x as f64, y as f64, 0.);
            let viewport = ray::canvas_to_viewport(canvas)
                .yaw(self.camera.yaw)
                .pitch(self.camera.pitch)
                .roll(self.camera.roll);
            let (r, g, b, a) = ray::trace_ray(self, self.camera.position, viewport, 1., INF, 3).convert();
            let rgba = [r, g, b, a];
            pixel.copy_from_slice(&rgba);
        }
    }

    fn get_things() -> Vec<Sphere> {
        vec![
            Sphere {
                center: Point(0., -1., 3.),
                radius: 1.,
                colour: Colour {
                    r: 255,
                    ..default()
                },
                specular: 500,
                reflective: 0.2,
            },
            Sphere {
                center: Point(2., 0., 4.),
                radius: 1.,
                colour: Colour {
                    b: 255,
                    ..default()
                },
                specular: 500,
                reflective: 0.3,
            },
            Sphere {
                center: Point(-2., 0., 4.),
                radius: 1.,
                colour: Colour {
                    g: 255,
                    ..default()
                },
                specular: 10,
                reflective: 0.4,
            },
            Sphere {
                center: Point(0., -5001., 0.),
                radius: 5000.,
                colour: Colour {
                    r: 255,
                    g: 255,
                    ..default()
                },
                specular: 1000,
                reflective: 0.5,
            },
        ]
    }

    fn get_lights() -> Vec<Light> {
        vec![
            Light {
                light_type: LightType::Ambient,
                intensity: 0.2,
            },
            Light {
                light_type: LightType::Point(Point(2., 1., 0.)),
                intensity: 0.6,
            },
            Light {
                light_type: LightType::Directional(Point(1., 4., 4.)),
                intensity: 0.2,
            },
        ]
    }
}

pub struct Camera {
    pub position: Point,
    pub roll: f64,
    pub pitch: f64,
    pub yaw: f64,
}

impl Camera {
    pub fn default() -> Self {
        Self {
            position: Point(0., 0.5, 0.),
            roll: 0.,
            pitch: 0.,
            yaw: 0.,
        }
    }
}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub colour: Colour,
    pub specular: isize,
    pub reflective: f64,
}

pub enum LightType {
    Ambient,
    Point(Point),
    Directional(Point),
}
pub struct Light {
    pub light_type: LightType,
    pub intensity: f64,
}

fn default<T: Default>() -> T {
    Default::default()
}
