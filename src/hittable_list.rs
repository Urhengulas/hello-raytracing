use crate::{hittable::Hittable, interval::Interval};

#[derive(Default)]
pub struct HittableList<'a> {
    objects: Vec<Box<dyn Hittable + 'a>>,
}

impl<'a> HittableList<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn _clear(&mut self) {
        self.objects.clear()
    }

    pub fn add<T>(&mut self, object: T)
    where
        T: Hittable + 'a,
    {
        self.objects.push(Box::new(object))
    }
}

impl Hittable for HittableList<'_> {
    fn hit(&self, r: &crate::ray::Ray, ray_t: &Interval) -> Option<crate::hittable::HitRecord> {
        let mut hit_anything = None;
        let mut ray_t = ray_t.clone();

        for o in &self.objects {
            if let Some(rec) = o.hit(r, &ray_t) {
                ray_t.max = rec.t;
                hit_anything = Some(rec);
            }
        }

        hit_anything
    }
}
