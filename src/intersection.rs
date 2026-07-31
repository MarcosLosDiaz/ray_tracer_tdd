use std::rc::Rc;
use crate::sphere::Sphere;

#[derive(Debug, Clone)]
pub struct Intersection {
    pub t: f64,
    pub object: Rc<Sphere>,
}

impl Intersection {
    pub fn new(t: f64, object: Rc<Sphere>) -> Self {
        Self { t, object }
    }
}

#[derive(Debug, Clone)]
pub struct Intersections {
    pub negative_intersections: Vec<Intersection>,
    pub positive_intersections: Vec<Intersection>,
    pub positive_min: Option<Intersection>,
}

impl Intersections {
    pub fn new() -> Self {
        Self {
            negative_intersections: Vec::new(),
            positive_intersections: Vec::new(),
            positive_min: None,
        }
    }

    pub fn count(&self) -> usize {
        self.negative_intersections.len() + self.positive_intersections.len()
    }

    // Add an intersection: if positive, calculate mininum positive ("hit")
    pub fn add(&mut self, inter: Intersection) {
        if inter.t < 0.0 {
            self.negative_intersections.push(inter);
        } else {
            // store it for `all()` and update the cached minimum for `hit()`
            let is_smaller = match self.positive_min.as_ref() {
                Some(current_min) => inter.t < current_min.t,
                None => true,
            };

            if is_smaller {
                self.positive_min = Some(inter.clone());
            }
            self.positive_intersections.push(inter);
        }
    }

    // Return the smallest non-negative intersection (top of min-heap) or None
    pub fn hit(&self) -> Option<&Intersection> {
        self.positive_min.as_ref()
    }

    // Return combined ordered list: negatives (ascending) then positives (ascending)
    // This is inneficient, but its test only
    pub fn all(&self) -> Vec<Intersection> {
        let mut out = Vec::with_capacity(self.count());
        let mut negs = self.negative_intersections.clone();
        negs.sort_by(|a, b| a.t.total_cmp(&b.t));
        out.extend(negs.into_iter());
        let mut pos: Vec<Intersection> = self.positive_intersections.clone();
        pos.sort_by(|a, b| a.t.total_cmp(&b.t));
        out.extend(pos.into_iter());
        out
    }
}
