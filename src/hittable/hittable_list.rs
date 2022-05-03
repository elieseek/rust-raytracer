use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

use super::Object;

#[derive(Debug, Clone, Default)]
pub struct HittableList {
    pub objects: Vec<Object>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Object) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec = None;

        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(hit) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                rec = Some(hit);
            }
        }

        rec
    }
}
