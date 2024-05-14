use std::f64::INFINITY;

use crate::{HitRecord, Intvl, Object, V};

#[derive(Debug, Clone, Copy)]
pub struct R {
    pub q: V,
    pub d: V,
}

impl R {
    pub fn connect(st: V, ed: V) -> R {
        R { q: st, d: ed - st }
    }
    pub fn at(&self, t: f64) -> V {
        self.q + t * self.d
    }
    pub fn color(&self, objects: &Vec<Box<dyn Object>>, remaining_depth: u32) -> V {
        if remaining_depth <= 0 {
            return V(0.0, 0.0, 0.0);
        }
        let mut t_closest = INFINITY;
        let mut hit_record = HitRecord::default();
        let mut is_hit = false;
        for object in objects {
            if let Some(hit_record_tmp) = object.hit(self, Intvl(0.001, t_closest)) {
                t_closest = hit_record_tmp.t;
                hit_record = hit_record_tmp;
                is_hit = true;
            }
        }
        if is_hit {
            return 0.5
                * R {
                    q: hit_record.p,
                    d: hit_record.n + V::random_unit(),
                }
                .color(objects, remaining_depth - 1);
        }
        let t = 0.5 * (self.d.unit().y() + 1.0);
        R::connect(V(1.0, 1.0, 1.0), V(0.5, 0.7, 1.0)).at(t)
    }
}
