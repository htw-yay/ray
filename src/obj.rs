use crate::Intvl;
use crate::R;
use crate::V;

pub trait Object {
    fn hit(&self, r: &R, t_bound: Intvl) -> Option<HitRecord>;
    fn n(&self, p: V) -> V;
}

#[derive(Debug)]
pub struct Sphere {
    pub center: V,
    pub radius: f64,
}

impl Object for Sphere {
    fn hit(&self, r: &R, t_bound: Intvl) -> Option<HitRecord> {
        let qc = self.center - r.q;
        let a = r.d.norm_squared();
        let h = r.d.dot(qc);
        let c = qc.norm_squared() - self.radius * self.radius;

        let det = h * h - a * c;

        if det < 0.0 {
            return None;
        }
        let det_sqrt = det.sqrt();

        let mut t = (h - det_sqrt) / a;
        if !t_bound.surround(t) {
            t = (h + det_sqrt) / a;
            if !t_bound.surround(t) {
                return None;
            }
        }

        let p = r.at(t);
        let n = self.n(p);
        let hit_record = HitRecord { t, p, n };

        return Some(hit_record);
    }
    fn n(&self, p: V) -> V {
        (p - self.center) / self.radius
    }
}

#[derive(Debug, Default)]
pub struct HitRecord {
    pub t: f64,
    pub p: V,
    pub n: V,
}
