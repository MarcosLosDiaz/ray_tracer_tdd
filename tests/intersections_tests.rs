use approx::assert_relative_eq;
use cucumber::{World, given, when, then};
use std::collections::HashMap;
use std::rc::Rc;
use nalgebra::Vector3;
use ray_tracer_tdd::ray::Ray;
use ray_tracer_tdd::sphere::Sphere;

#[derive(Debug, Default, World)]
pub struct IntersectionsWorld {
    pub rays: HashMap<String, Ray>,
    pub spheres: HashMap<String, Rc<Sphere>>,
    // Store intersection data as (t, sphere_name) to avoid lifetime issues
    pub intersections: HashMap<String, (f64, String)>,
    // Store lists as (ray_name, sphere_name) pairs
    pub lists: HashMap<String, Vec<(f64, String)>>,
}

// Using default sphere, do nothing (sphere is represented by its name)
#[given(expr = "{word} ← sphere\\(\\)")]
fn given_sphere(world: &mut IntersectionsWorld, name: String) {
    world.spheres.insert(name.clone(), Rc::new(Sphere::new(&name)));
}

#[given(expr = "{word} ← intersection\\({float}, {word}\\)")]
fn given_intersection(world: &mut IntersectionsWorld, name: String, t: f64, object: String) {
    world.intersections.insert(name, (t, object));
}

#[given(regex = r"^(\w+) ← intersections\((.+)\)$")]
#[when(regex = r"^(\w+) ← intersections\((.+)\)$")]
fn step_aggregate(world: &mut IntersectionsWorld, xs_name: String, list_str: String) {
    let mut xs_vec = Vec::new();
    for item in list_str.split(',') {
        let item_trimmed = item.trim();
        if !item_trimmed.is_empty() {
            let val = world.intersections.get(item_trimmed)
                .unwrap_or_else(|| panic!("intersection {} not found", item_trimmed))
                .clone();
            xs_vec.push(val);
        }
    }
    world.lists.insert(xs_name, xs_vec);
}

#[when(expr = "{word} ← intersection\\({float}, {word}\\)")]
fn when_make_intersection(world: &mut IntersectionsWorld, name: String, t: f64, object: String) {
    world.intersections.insert(name, (t, object));
}

#[when(expr = "{word} ← intersect\\({word}, {word}\\)")]
fn when_intersect(world: &mut IntersectionsWorld, xs_name: String, s_name: String, r_name: String) {
    let ray = *world.rays.get(&r_name).expect("ray not found");
    let sphere = world.spheres.get(&s_name).expect("sphere not found").clone();
    let xs_vec = match ray.intersect_sphere(sphere) {
        None => vec![],
        Some(xs) => xs.all().iter().map(|i| (i.t, s_name.clone())).collect(),
    };
    world.lists.insert(xs_name, xs_vec);
}

#[when(expr = "{word} ← hit\\({word}\\)")]
fn when_hit(world: &mut IntersectionsWorld, i_name: String, xs_name: String) {
    // reconstruct Intersections from the stored list of (t, sphere_name)
    let list = world.lists.get(&xs_name).expect("xs not found");
    let mut xs = ray_tracer_tdd::intersection::Intersections::new();
    for (t, s_name) in list.iter() {
        let s = world.spheres.get(s_name).expect("sphere not found").clone();
        let inter = ray_tracer_tdd::intersection::Intersection::new(*t, s);
        xs.add(inter);
    }
    if let Some(hit) = xs.hit() {
        let obj_name = hit.object.name.clone();
        world.intersections.insert(i_name, (hit.t, obj_name));
    }
}

#[given(expr = "{word} ← ray\\(point\\({float}, {float}, {float}\\), vector\\({float}, {float}, {float}\\)\\)")]
fn given_ray_inline(world: &mut IntersectionsWorld, name: String,
                    px: f64, py: f64, pz: f64,
                    vx: f64, vy: f64, vz: f64) {
    let origin = Vector3::new(px, py, pz);
    let direction = Vector3::new(vx, vy, vz);
    world.rays.insert(name, Ray::new(origin, direction));
}

#[then(regex = r"^([a-zA-Z0-9_]+)\.t = ([-+]?\d+(?:\.\d+)?)$")]
fn then_intersection_t(world: &mut IntersectionsWorld, name: String, expected: f64) {
    let (t, _obj) = world.intersections.get(&name).expect("intersection not found");
    assert_relative_eq!(*t, expected);
}

#[then(regex = r"^([a-zA-Z0-9_]+)\.object = ([a-zA-Z0-9_]+)$")]
fn then_intersection_object(world: &mut IntersectionsWorld, name: String, expected_obj: String) {
    let (_t, obj) = world.intersections.get(&name).expect("intersection not found");
    assert_eq!(obj, &expected_obj);
}

#[then(expr = "{word}.count = {int}")]
fn then_list_count(world: &mut IntersectionsWorld, name: String, expected: usize) {
    let list = world.lists.get(&name).expect("list not found");
    assert_eq!(list.len(), expected);
}

#[then(expr = "{word}[{int}].t = {float}")]
fn then_list_index_t(world: &mut IntersectionsWorld, name: String, index: usize, expected: f64) {
    let list = world.lists.get(&name).expect("list not found");
    let item = list.get(index).expect("index out of bounds");
    assert_relative_eq!(item.0, expected);
}

#[then(expr = "{word}[{int}].object = {word}")]
fn then_list_index_object(world: &mut IntersectionsWorld, name: String, index: usize, expected: String) {
    let list = world.lists.get(&name).expect("list not found");
    let item = list.get(index).expect("index out of bounds");
    assert_eq!(&item.1, &expected);
}

#[then(expr = "{word} is nothing")]
fn then_is_nothing(world: &mut IntersectionsWorld, name: String) {
    assert!(world.intersections.get(&name).is_none(), "expected {} to be nothing", name);
}

#[then(regex = r"^([a-zA-Z0-9_]+) = ([a-zA-Z0-9_]+)$")]
fn then_intersections_equal(world: &mut IntersectionsWorld, a: String, b: String) {
    let a_val = world.intersections.get(&a).expect("left not found");
    let b_val = world.intersections.get(&b).expect("right not found");
    assert_relative_eq!(a_val.0, b_val.0);
    assert_eq!(a_val.1, b_val.1);
}

fn main() {
    futures::executor::block_on(IntersectionsWorld::run(
        "tests/features/intersections.feature"
    ));
}
