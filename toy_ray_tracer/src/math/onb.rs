use crate::core::Vec3f;

#[derive(Clone, Copy)]
pub struct ONB {
    pub axis: [Vec3f; 3],
}

impl ONB {
    pub fn build_form_w(n: &Vec3f) -> ONB {
        let az = n.normalize();
        let a = if az[0].abs() > 0.9 {
            Vec3f::new(0.0, 1.0, 0.0)
        } else {
            Vec3f::new(1.0, 0.0, 0.0)
        };
        let ay = az.cross(&a).normalize();
        let ax = az.cross(&ay);
        ONB { axis: [ax, ay, az] }
    }

    pub fn u(&self) -> Vec3f {
        self.axis[0]
    }

    pub fn v(&self) -> Vec3f {
        self.axis[1]
    }

    pub fn w(&self) -> Vec3f {
        self.axis[2]
    }

    pub fn local(&self, a: Vec3f) -> Vec3f {
        self.u() * a.x + self.v() * a.y + self.w() * a.z
    }
}
