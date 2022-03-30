use core::fmt;

use visitor::EnterContext;

use crate::{
    aabb::AABB,
    geometry::visitor,
    hittable::{HitRecord, Hittable},
    material::MaterialPtr,
    vec::{vec3, Vec3},
};

use super::Plane;

pub struct Cylinder {
    // center bottom
    center0: Vec3,
    // center up
    center1: Vec3,
    // radius
    radius: f32,
    // axis aligned plane
    #[allow(dead_code)]
    plane: Plane,
    material: MaterialPtr,
}

impl fmt::Debug for Cylinder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cylinder")
            .field("center0", &self.center0)
            .field("center1", &self.center1)
            .field("radius", &self.radius)
            .field("plane", &self.plane)
            .finish()
    }
}

#[allow(dead_code)]
fn error_axis_aligned_cylinder(c0: Vec3, c1: Vec3, r: f32) -> ! {
    panic!(
        "we only support axis aligned cylinder, but got c0={:?} c1={:?} r={}",
        c0, c1, r
    );
}

impl Cylinder {
    pub fn new(c0: Vec3, c1: Vec3, r: f32, material: MaterialPtr) -> Self {
        let ko = c0.iter().zip(c1.iter()).position(|(l, r)| l != r);

        let k = ko.unwrap_or(3);

        if ko.is_none() || k > 2 {
            error_axis_aligned_cylinder(c0, c1, r);
        }

        // check other two
        let a = (k + 1) % 3;
        let b = (k + 2) % 3;

        if (c0[a] != c1[a]) || (c0[b] != c0[b]) {
            error_axis_aligned_cylinder(c0, c1, r);
        }

        // find plane
        let plane = match k {
            0 => Plane::YZ,
            1 => Plane::ZX,
            2 => Plane::XY,
            _ => {
                error_axis_aligned_cylinder(c0, c1, r);
            }
        };

        // bottom is first
        let (center0, center1) = if c0[k] < c1[k] { (c0, c1) } else { (c1, c0) };

        Self {
            center0,
            center1,
            radius: r,
            plane,
            material,
        }
    }
}

impl Hittable for Cylinder {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<crate::hittable::HitRecord> {
        let (axis_a, axis_b, axis_c) = match self.plane {
            Plane::YZ => (1, 2, 0),
            Plane::ZX => (2, 0, 1),
            Plane::XY => (0, 1, 2),
        };

        // z^2 + x^2 = r^2
        let rd = ray.direction();
        let ro = ray.origin();

        let oc = ray.origin() - self.center0;
        let a = rd[axis_a] * rd[axis_a] + rd[axis_b] * rd[axis_b];
        let b = 2.0 * (rd[axis_a] * oc[axis_a] + rd[axis_b] * oc[axis_b]);
        let c = oc[axis_a] * oc[axis_a] + oc[axis_b] * oc[axis_b] - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }

        let (c_min, c_max) = (self.center0[axis_c], self.center1[axis_c]);
        let up_normal = match self.plane {
            Plane::YZ => vec3::XUP,
            Plane::ZX => vec3::YUP,
            Plane::XY => vec3::ZUP,
        };

        let sqrt_discr = discriminant.sqrt();
        let root1 = (-b - sqrt_discr) / (2.0 * a);
        let p1 = ro[axis_c] + root1 * rd[axis_c];

        let rooto: Option<f32> = if p1 > c_min && p1 < c_max {
            Some(root1)
        } else {
            let root2 = (-b + sqrt_discr) / (2.0 * a);
            let p2 = ro[axis_c] + root2 * rd[axis_c];
            if p2 > c_min && p2 < c_max {
                Some(root2)
            } else {
                None
            }
        };

        let t_normal: Option<(f32, Vec3)> = if let Some(t) = rooto {
            let p = ray.origin() + t * ray.direction();
            // hit sides
            let mut normal = p - self.center0;
            normal[axis_c] = p[axis_c];
            let normal = normal.normalize();

            Some((t, normal))
        } else {
            // may hit the base
            let base_normal = if ray.direction()[axis_c] < 0.0 {
                up_normal
            } else {
                -up_normal
            };
            let t = -base_normal.dot(&oc) / ray.direction().dot(&base_normal);
            let q = oc + ray.direction() * t;

            if q.dot(&q) < self.radius * self.radius {
                Some((t, base_normal))
            } else {
                None
            }
        };

        if let Some((t, normal)) = t_normal {
            if t < t_min || t > t_max {
                return None;
            }

            let p = ray.origin() + t * ray.direction();
            // TODO: uv
            let mut rec = HitRecord::new(t, 0.0, 0.0, p, self.material.as_ref());
            rec.set_face_normal(ray, &normal);
            return Some(rec);
        };

        return None;
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        let a = self.center1 - self.center0;

        let aa = -vec3::elementwise_mult(&a, &a) / a.dot(&a);
        let e = self.radius * vec3::sqrt(aa.add_scalar(1.0));

        println!("a={:?}, aa={:?}, e={:?}", a, aa, e);

        return Some(AABB::new(
            vec3::min(&(self.center0 - e), &(self.center1 - e)),
            vec3::max(&(self.center0 + e), &(self.center1 + e)),
        ));
    }

    fn accept(&self, visitor: &mut dyn crate::geometry::GeometryVisitor) {
        visitor.visit_cylinder(self)
    }

    fn walk(&self, walker: &mut dyn crate::geometry::GeometryWalker) {
        walker.enter_cylinder(EnterContext::new(self))
    }
}
