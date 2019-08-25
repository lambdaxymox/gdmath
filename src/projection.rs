use std::fmt;
use std::mem;
use std::ops;
use std::cmp;

use crate::vector::Vector3;
use crate::matrix::{Matrix3, Matrix4};
use crate::traits::{Array, Zero, VectorSpace, Metric, DotProduct, Lerp};


const M_PI: f32 = 3.14159265358979323846264338327950288;
const ONE_DEG_IN_RAD: f32 = (2.0 * M_PI) / 360.0; // == 0.017444444


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Orthographic {
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    near: f32,
    far: f32,
}

impl Into<Orthographic> for (f32, f32, f32, f32, f32, f32) {
    #[inline]
    fn into(self) -> Orthographic {
        match self {
            (left, right, bottom, top, near, far) => {
                Orthographic {
                    left: left,
                    right: right,
                    bottom: bottom,
                    top: top,
                    near: near,
                    far: far,
                }
            }
        }
    }
}

impl<'a> Into<Orthographic> for &'a (f32, f32, f32, f32, f32, f32) {
    #[inline]
    fn into(self) -> Orthographic {
        match *self {
            (left, right, bottom, top, near, far) => {
                Orthographic {
                    left: left,
                    right: right,
                    bottom: bottom,
                    top: top,
                    near: near,
                    far: far,
                }
            }
        }
    }
}

impl Into<Orthographic> for [f32; 6] {
    #[inline]
    fn into(self) -> Orthographic {
        match self {
            [left, right, bottom, top, near, far] => {
                Orthographic {
                    left: left,
                    right: right,
                    bottom: bottom,
                    top: top,
                    near: near,
                    far: far,
                }
            }
        }
    }
}

impl<'a> Into<Orthographic> for &'a [f32; 6] {
    #[inline]
    fn into(self) -> Orthographic {
        match *self {
            [left, right, bottom, top, near, far] => {
                Orthographic {
                    left: left,
                    right: right,
                    bottom: bottom,
                    top: top,
                    near: near,
                    far: far,
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Perspective {
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    near: f32,
    far: f32,
}

impl Into<Perspective> for (f32, f32, f32, f32, f32, f32) {
    #[inline]
    fn into(self) -> Perspective {
        match self {
            (left, right, bottom, top, near, far) => {
                Perspective {
                    left: left,
                    right: right,
                    bottom: bottom,
                    top: top,
                    near: near,
                    far: far,
                }
            }
        }
    }
}

impl<'a> Into<Perspective> for &'a (f32, f32, f32, f32, f32, f32) {
    #[inline]
    fn into(self) -> Perspective {
        match *self {
            (left, right, bottom, top, near, far) => {
                Perspective {
                    left: left,
                    right: right,
                    bottom: bottom,
                    top: top,
                    near: near,
                    far: far,
                }
            }
        }
    }
}

impl Into<Perspective> for [f32; 6] {
    #[inline]
    fn into(self) -> Perspective {
        match self {
            [left, right, bottom, top, near, far] => {
                Perspective {
                    left: left,
                    right: right,
                    bottom: bottom,
                    top: top,
                    near: near,
                    far: far,
                }
            }
        }
    }
}

impl<'a> Into<Perspective> for &'a [f32; 6] {
    #[inline]
    fn into(self) -> Perspective {
        match *self {
            [left, right, bottom, top, near, far] => {
                Perspective {
                    left: left,
                    right: right,
                    bottom: bottom,
                    top: top,
                    near: near,
                    far: far,
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PerspectiveFov {
    fovy: f32,
    aspect: f32,
    near: f32,
    far: f32,
}

impl Into<PerspectiveFov> for (f32, f32, f32, f32) {
    #[inline]
    fn into(self) -> PerspectiveFov {
        match self {
            (fovy, aspect, near, far) => {
                PerspectiveFov {
                    fovy: fovy,
                    aspect: aspect,
                    near: near,
                    far: far,
                }
            }
        }
    }
}

impl<'a> Into<PerspectiveFov> for &'a (f32, f32, f32, f32) {
    #[inline]
    fn into(self) -> PerspectiveFov {
        match *self {
            (fovy, aspect, near, far) => {
                PerspectiveFov {
                    fovy: fovy,
                    aspect: aspect,
                    near: near,
                    far: far,
                }
            }
        }
    }
}

impl Into<PerspectiveFov> for [f32; 4] {
    #[inline]
    fn into(self) -> PerspectiveFov {
        match self {
            [fovy, aspect, near, far] => {
                PerspectiveFov {
                    fovy: fovy,
                    aspect: aspect,
                    near: near,
                    far: far,
                }
            }
        }
    }
}

impl<'a> Into<PerspectiveFov> for &'a [f32; 4] {
    #[inline]
    fn into(self) -> PerspectiveFov {
        match *self {
            [fovy, aspect, near, far] => {
                PerspectiveFov {
                    fovy: fovy,
                    aspect: aspect,
                    near: near,
                    far: far,
                }
            }
        }
    }
}


impl From<PerspectiveFov> for Matrix4 {
    fn from(persp: PerspectiveFov) -> Matrix4 {
        let fov_rad = persp.fovy * ONE_DEG_IN_RAD;
        let range = f32::tan(fov_rad / 2.0) * persp.near;
        let sx = (2.0 * persp.near) / (range * persp.aspect + range * persp.aspect);
        let sy = persp.near / range;
        let sz = (persp.far + persp.near) / (persp.near - persp.far);
        let pz = (2.0 * persp.far * persp.near) / (persp.near - persp.far);
        
        Matrix4::new(
             sx, 0.0, 0.0,  0.0,
            0.0,  sy, 0.0,  0.0,
            0.0, 0.0,  sz, -1.0,
            0.0, 0.0,  pz,  0.0
        )
    }
}

impl From<Perspective> for Matrix4 {
    fn from(persp: Perspective) -> Matrix4 {
        let c0r0 = (2.0 * persp.near) / (persp.right - persp.left);
        let c0r1 = 0.0;
        let c0r2 = 0.0;
        let c0r3 = 0.0;

        let c1r0 = 0.0;
        let c1r1 = (2.0 * persp.near) / (persp.top - persp.bottom);
        let c1r2 = 0.0;
        let c1r3 = 0.0;

        let c2r0 =  (persp.right + persp.left)   / (persp.right - persp.left);
        let c2r1 =  (persp.top   + persp.bottom) / (persp.top   - persp.bottom);
        let c2r2 = -(persp.far   + persp.near)   / (persp.far   - persp.near);
        let c2r3 = -1.0;

        let c3r0 = 0.0;
        let c3r1 = 0.0;
        let c3r2 = -(2.0 * persp.far * persp.near) / (persp.far - persp.near);
        let c3r3 = 0.0;

        Matrix4::new(
            c0r0, c0r1, c0r2, c0r3,
            c1r0, c1r1, c1r2, c1r3,
            c2r0, c2r1, c2r2, c2r3,
            c3r0, c3r1, c3r2, c3r3,
        )
    }
}

impl From<Orthographic> for Matrix4 {
    fn from(ortho: Orthographic) -> Matrix4 {
        let sx = 2.0 / (ortho.right - ortho.left);
        let sy = 2.0 / (ortho.top - ortho.bottom);
        let sz = 2.0 / (ortho.far - ortho.near);
        let tx = (ortho.right + ortho.left) / (ortho.right - ortho.left);
        let ty = (ortho.top + ortho.bottom) / (ortho.top - ortho.bottom);
        let tz = (ortho.far + ortho.near) / (ortho.far - ortho.near);

        Matrix4::new(
             sx, 0.0, 0.0, 0.0,
            0.0,  sy, 0.0, 0.0,
            0.0, 0.0,  sz, 0.0,
             -tx, -ty, -tz, 1.0
        )
    }
}
