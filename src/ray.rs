use crate::colour::Colour;
use crate::point::Point;
use crate::world::{LightType, Sphere, World};
use crate::{CANVAS_HEIGHT, CANVAS_WIDTH};

const VIEWPORT_WIDTH: f64 = 1.;
const VIEWPORT_HEIGHT: f64 = 1.;
const DISTANCE_TO_VIEWPORT: f64 = 1.;
const EPSILON: f64 = 0.001;
pub const INF: f64 = 99999999999.;
pub fn canvas_to_viewport(canvas: Point) -> Point {
    Point(
        canvas.0 * VIEWPORT_WIDTH / CANVAS_WIDTH as f64,
        canvas.1 * VIEWPORT_HEIGHT / CANVAS_HEIGHT as f64,
        DISTANCE_TO_VIEWPORT,
    )
}

pub fn trace_ray(
    world: &World,
    origin: Point,
    viewport: Point,
    t_min: f64,
    t_max: f64,
    recursion: isize,
) -> Colour {
    let (closest_sphere, closest_point) =
        closest_intersection(&world.scene, origin, viewport, t_min, t_max);

    let p;
    let mut normal;
    let local_colour = match closest_point {
        None => return Colour::default(),
        Some(pt) => {
            p = origin + viewport * pt;
            let sphere = closest_sphere.unwrap();
            normal = p - sphere.center;
            normal = normal / length(normal);
            sphere.colour.clone() * compute_lighting(world, p, normal, -viewport, sphere.specular)
        }
    };

    let reflective = closest_sphere.unwrap().reflective;
    if (recursion <= 0) | (reflective <= 0.) {
        return local_colour;
    }

    let ray = reflect_ray(-viewport, normal);
    let reflected_colour = trace_ray(world, p, ray, EPSILON, INF, recursion - 1);
    local_colour * (1. - reflective) + (reflected_colour * reflective)
}
fn length(point: Point) -> f64 {
    Point::dot(point, point).sqrt()
}

fn reflect_ray(ray: Point, normal: Point) -> Point {
    2. * normal * Point::dot(ray, normal) - ray
}

fn compute_lighting(world: &World, point: Point, normal: Point, view: Point, spec: isize) -> f64 {
    let mut i = 0.0;

    for light in world.lights.iter() {
        let l: Point;
        let t_max: f64;
        match &light.light_type {
            LightType::Ambient => {
                i += light.intensity;
                continue;
            }
            LightType::Point(ref position) => {
                l = *position - point;
                t_max = 1.;
            }
            LightType::Directional(ref direction) => {
                l = *direction;
                t_max = INF;
            }
        }

        // Shadow check
        if let (Some(_), _) = closest_intersection(&world.scene, point, l, EPSILON, t_max) {
            continue;
        }

        // Diffuse
        let n_dot_l = Point::dot(normal, l);
        if n_dot_l > 0. {
            i += light.intensity * n_dot_l / (length(normal) * length(l));
        }

        // Specular
        if spec != -1 {
            let r = reflect_ray(l, normal);
            let r_dot_v = Point::dot(r, view);
            if r_dot_v > 0. {
                i += light.intensity * (r_dot_v / (length(r) * length(view))).powi(spec as i32);
            }
        }
    }
    i
}

fn intersect_ray_sphere(origin: Point, viewport: Point, sphere: &Sphere) -> Option<(f64, f64)> {
    let r = sphere.radius;
    let co = origin - sphere.center;

    let a = Point::dot(viewport, viewport);
    let b = 2_f64 * Point::dot(co, viewport);
    let c = Point::dot(co, co) - r * r;

    let discriminant = b * b - 4_f64 * a * c;

    if discriminant < 0. {
        None
    } else {
        let t1 = (-b + discriminant.sqrt()) / (2_f64 * a);
        let t2 = (-b - discriminant.sqrt()) / (2_f64 * a);
        Some((t1, t2))
    }
}

fn check_point<'a>(
    check_sphere: &'a Sphere,
    check_pt: f64,
    closest_sphere: Option<&'a Sphere>,
    closest_pt: Option<f64>,
) -> (Option<&'a Sphere>, Option<f64>) {
    let unchanged = (closest_sphere, closest_pt);
    let new = (Some(check_sphere), Some(check_pt));

    match closest_pt {
        Some(pt) => {
            if check_pt < pt {
                new
            } else {
                unchanged
            }
        }
        None => new,
    }
}

fn closest_intersection(
    things: &Vec<Sphere>,
    origin: Point,
    viewport: Point,
    t_min: f64,
    t_max: f64,
) -> (Option<&Sphere>, Option<f64>) {
    let mut closest_sphere = None;
    let mut closest_point = None;
    for sphere in things {
        if let Some((t1, t2)) = intersect_ray_sphere(origin, viewport, sphere) {
            if (t_min..t_max).contains(&t1) {
                (closest_sphere, closest_point) =
                    check_point(sphere, t1, closest_sphere, closest_point);
            }

            if (t_min..t_max).contains(&t2) {
                (closest_sphere, closest_point) =
                    check_point(sphere, t2, closest_sphere, closest_point);
            }
        }
    }
    (closest_sphere, closest_point)
}
