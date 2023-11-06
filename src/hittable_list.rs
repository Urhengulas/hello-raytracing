use crate::hittable::Hittable;

#[derive(Default)]
struct HittableList<'a> {
    objects: Vec<Box<dyn Hittable + 'a>>,
}

impl<'a> HittableList<'a> {
    fn new() -> Self {
        Self::default()
    }

    fn clear(&mut self) {
        self.objects.clear()
    }

    fn add<T>(&mut self, object: T)
    where
        T: Hittable + 'a,
    {
        self.objects.push(Box::new(object))
    }
}

impl Hittable for HittableList<'_> {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_tmin: f64,
        ray_tmax: f64,
    ) -> Option<crate::hittable::HitRecord> {
        let mut hit_anything = None;
        let mut closest_so_far = ray_tmax;

        for o in &self.objects {
            if let Some(rec) = o.hit(r, ray_tmin, closest_so_far) {
                closest_so_far = rec.t;
                hit_anything = Some(rec);
            }
        }

        hit_anything
    }
}
