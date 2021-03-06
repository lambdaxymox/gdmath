use crate::base::{
    Magnitude,
    Scalar,
    ScalarSigned,
    ScalarFloat,
    Unit,
};
use crate::angle::{
    Angle,
    Radians,
};
use crate::matrix::{
    Matrix3x3, 
    Matrix4x4,
};
use crate::vector::{
    Vector3,
};

use num_traits::NumCast;
use core::fmt;
use core::ops;


/// A quaternion is a generalization of vectors in three dimensions that 
/// enables one to perform rotations about an arbitrary axis. They are a
/// three-dimensional analogue of complex numbers. In geometric algebra terms,
/// a complex number is a **scalar + bivector** form whereas a quaternion is
/// a **scalar + vector** form. 
///
/// Analogous to the complex numbers, quaternions can be written in polar form.
/// polar form reveals the fact that it encodes rotations. A quaternion `q` can
/// be written in polar form as
/// ```text
/// q = s + v := |q| * exp(-theta * v) := |q| * (cos(theta) + v * sin(theta))
/// ```
/// where `v` is a unit vector in the direction of the axis of rotation, `theta`
/// is the angle of rotation, and `|q|` denotes the length of the quaternion.
///
/// Quaternions are stored in [s, x, y, z] storage order, where `s` is the scalar
/// part and `(x, y, z)` are the vector components.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Quaternion<S> {
    /// The scalar component of a quaternion.
    pub s: S,
    /// The vector component of a quaternion.
    pub v: Vector3<S>,
}

impl<S> Quaternion<S> {
    /// Construct a new quaternion from its scalar component and its three
    /// vector components.
    #[inline]
    pub const fn new(qs: S, qx: S, qy: S, qz: S) -> Quaternion<S> {
        Quaternion { 
            s: qs, 
            v: Vector3::new(qx, qy, qz)
        }
    }

    /// Construct a quaternion from its scalar and vector parts.
    #[inline]
    pub fn from_parts(qs: S, qv: Vector3<S>) -> Quaternion<S> {
        Quaternion { 
            s: qs, 
            v: qv 
        }
    }
}

impl<S> Quaternion<S> 
where 
    S: Copy 
{
    /// Construct a new quaternion from a fill value. 
    ///
    /// Every component of the resulting vector will have the same value
    /// supplied by the `value` argument.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion, 
    /// # };
    /// #
    /// let result = Quaternion::from_fill(1_f64);
    /// let expected = Quaternion::new(1_f64, 1_f64, 1_f64, 1_f64);
    /// 
    /// assert_eq!(result, expected);
    /// ```
    #[inline]
    pub fn from_fill(value: S) -> Quaternion<S> {
        Quaternion::new(value, value, value, value)
    }

    /// The length of the the underlying array storing the quaternion components.
    #[inline]
    pub fn len(&self) -> usize {
        4
    }

    /// The shape of the underlying array storing the quaternion components.
    ///
    /// The shape is the equivalent number of columns and rows of the 
    /// array as though it represents a matrix. The order of the descriptions 
    /// of the shape of the array is **(rows, columns)**.
    #[inline]
    pub fn shape(&self) -> (usize, usize) {
        (4, 1)
    }

    /// Get a pointer to the underlying array.
    #[inline]
    pub fn as_ptr(&self) -> *const S {
        &self.s
    }

    /// Get a mutable pointer to the underlying array.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut S {
        &mut self.s
    }

    /// Get a slice of the underlying elements of the data type.
    #[inline]
    pub fn as_slice(&self) -> &[S] {
        <Self as AsRef<[S; 4]>>::as_ref(self)
    }

    /// Map an operation on that acts on the coordinates of a vector, returning 
    /// a vector whose coordinates are of the new scalar type.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion,  
    /// # };
    /// #
    /// let vector: Quaternion<f32> = Quaternion::new(1_f32, 2_f32, 3_f32, 4_f32);
    /// let expected: Quaternion<f64> = Quaternion::new(-2_f64, -3_f64, -4_f64, -5_f64);
    /// let result: Quaternion<f64> = vector.map(|comp| -(comp + 1_f32) as f64);
    ///
    /// assert_eq!(result, expected);
    /// ```
    #[inline]
    pub fn map<T, F>(self, mut op: F) -> Quaternion<T> 
    where 
        F: FnMut(S) -> T 
    {
        Quaternion::new(
            op(self.s),
            op(self.v.x),
            op(self.v.y),
            op(self.v.z),
        )
    }
}

impl<S> Quaternion<S> 
where 
    S: NumCast + Copy 
{
    /// Cast a quaternion from one type of scalars to another type of scalars.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion,   
    /// # };
    /// #
    /// let quaternion: Quaternion<u32> = Quaternion::new(1_u32, 2_u32, 3_u32, 4_u32);
    /// let expected: Option<Quaternion<i32>> = Some(Quaternion::new(1_i32, 2_i32, 3_i32, 4_i32));
    /// let result = quaternion.cast::<i32>();
    ///
    /// assert_eq!(result, expected);
    /// ```
    #[inline]
    pub fn cast<T: NumCast>(&self) -> Option<Quaternion<T>> {
        let s = match num_traits::cast(self.s) {
            Some(value) => value,
            None => return None,
        };
        let v = match self.v.cast() {
            Some(value) => value,
            None => return None,
        };

        Some(Quaternion::from_parts(s, v))
    }
}

impl<S> Quaternion<S> 
where 
    S: Scalar 
{
    /// Returns the unit real quaternion. 
    ///
    /// A real quaternion is a quaternion with zero vector part.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion, 
    /// # };
    /// #
    /// let unit_s: Quaternion<f64> = Quaternion::unit_s();
    ///
    /// assert!(unit_s.is_real());
    /// assert!(!unit_s.is_pure());
    /// ```
    #[inline]
    pub fn unit_s() -> Quaternion<S> {
        Quaternion::from_parts(S::one(), Vector3::zero())
    }

    /// Return the **x-axis** unit pure quaternion.
    ///
    /// A pure quaternion is a quaternion with zero scalar part.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion, 
    /// # };
    /// #
    /// let unit_x: Quaternion<f64> = Quaternion::unit_x();
    ///
    /// assert!(!unit_x.is_real());
    /// assert!(unit_x.is_pure());
    /// ```
    #[inline]
    pub fn unit_x() -> Quaternion<S> {
        Quaternion::from_parts(
            S::zero(), 
            Vector3::new(S::one(), S::zero(), S::zero())
        )
    }

    /// Returns the **y-axis** unit pure quaternion.
    ///
    /// A pure quaternion is a quaternion with zero scalar part.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion, 
    /// # };
    /// #
    /// let unit_y: Quaternion<f64> = Quaternion::unit_y();
    ///
    /// assert!(!unit_y.is_real());
    /// assert!(unit_y.is_pure());
    /// ```
    #[inline]
    pub fn unit_y() -> Quaternion<S> {
        Quaternion::from_parts(
            S::zero(), 
            Vector3::new(S::zero(), S::one(), S::zero())
        )
    }

    /// Returns the **z-axis** unit pure quaternion.
    ///
    /// A pure quaternion is a quaternion with zero scalar part.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion, 
    /// # };
    /// #
    /// let unit_z: Quaternion<f64> = Quaternion::unit_z();
    ///
    /// assert!(!unit_z.is_real());
    /// assert!(unit_z.is_pure());
    /// ```
    #[inline]
    pub fn unit_z() -> Quaternion<S> {
        Quaternion::from_parts(
            S::zero(), 
            Vector3::new(S::zero(), S::zero(), S::one())
        )
    }

    /// Construct a zero quaternion.
    ///
    /// A zero quaternion is a quaternion `zero` such that given another 
    /// quaternion `q`
    /// ```text
    /// q + zero = zero + q = q
    /// ```
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion,
    /// # };
    /// #
    /// let zero_quat = Quaternion::zero();
    /// let quaternion = Quaternion::new(1_f64, 2_f64, 3_f64, 4_f64);
    ///
    /// assert_eq!(zero_quat + quaternion, quaternion);
    /// assert_eq!(quaternion + zero_quat, quaternion);
    /// ```
    #[inline]
    pub fn zero() -> Quaternion<S> {
        let zero = S::zero();

        Quaternion::new(zero, zero, zero, zero)
    }
    
    /// Determine whether is a quaternion is the zero quaternion.
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.s.is_zero() 
            && self.v.x.is_zero() && self.v.y.is_zero() && self.v.z.is_zero()
    }
    
    /// Construct the multiplicate identity quaternion.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion,
    /// # };
    /// #
    /// let identity = Quaternion::identity();
    /// let quaternion = Quaternion::new(1_f64, 2_f64, 3_f64, 4_f64);
    ///
    /// assert_eq!(identity * quaternion, quaternion);
    /// assert_eq!(quaternion * identity, quaternion);
    /// ```
    #[inline]
    pub fn identity() -> Quaternion<S> {
        let one = S::one();
        let zero = S::zero();

        Quaternion::new(one, zero, zero, zero)
    }
    
    /// Determine whether a quaternion is equal to the identity quaternion.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion, 
    /// # };
    /// #
    /// let identity: Quaternion<f64> = Quaternion::identity();
    /// assert!(identity.is_identity());
    ///
    /// let unit_x: Quaternion<f64> = Quaternion::unit_x();
    /// assert!(!unit_x.is_identity());
    ///
    /// let unit_y: Quaternion<f64> = Quaternion::unit_y();
    /// assert!(!unit_y.is_identity());
    ///
    /// let unit_z: Quaternion<f64> = Quaternion::unit_z();
    /// assert!(!unit_z.is_identity());
    /// ```
    #[inline]
    pub fn is_identity(&self) -> bool {
        self.s.is_one() && self.v.is_zero()
    }

    /// Check whether a quaternion is a pure quaternion.
    ///
    /// A pure quaternion is a quaternion with zero scalar part.
    /// 
    /// ## Example
    /// 
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion,
    /// #     Vector3,
    /// # };
    /// #
    /// let pure = Quaternion::from_parts(0_f64, Vector3::unit_z());
    /// 
    /// assert!(pure.is_pure());
    /// 
    /// let not_pure = Quaternion::from_parts(1_f64, Vector3::unit_z());
    ///
    /// assert!(!not_pure.is_pure());
    /// ```
    #[inline]
    pub fn is_pure(&self) -> bool {
        self.s.is_zero()
    }

    /// Check whether a quaternion is a real quaternion.
    ///
    /// A real quaternion is a quaternion with zero vector part.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion,
    /// #     Vector3, 
    /// # };
    /// # 
    /// let real = Quaternion::from_parts(1_f64, Vector3::zero());
    /// 
    /// assert!(real.is_real());
    /// 
    /// let not_real = Quaternion::from_parts(1_f64, Vector3::new(2_f64, 3_f64, 4_f64));
    ///
    /// assert!(!not_real.is_real());
    /// ```
    #[inline]
    pub fn is_real(&self) -> bool {
        self.v.is_zero()
    }

    /// Construct a real quaternion from a scalar value.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion,
    /// # };
    /// #
    /// let quaternion = Quaternion::from_real(1_u32);
    ///
    /// assert!(quaternion.is_real());
    /// assert!(!quaternion.is_pure());
    /// ```
    #[inline]
    pub fn from_real(value: S) -> Quaternion<S> {
        Quaternion::from_parts(value, Vector3::zero())
    }

    /// Construct a pure quaternion from a vector value.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion,
    /// #     Vector3,
    /// # };
    /// #
    /// let vector = Vector3::new(1_u32, 2_u32, 3_u32);
    /// let quaternion = Quaternion::from_pure(vector);
    ///
    /// assert!(quaternion.is_pure());
    /// assert!(!quaternion.is_real());
    /// ```
    #[inline]
    pub fn from_pure(vector: Vector3<S>) -> Quaternion<S> {
        Quaternion::from_parts(S::zero(), vector)
    }
    
    /// Compute the dot product of two quaternions.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion, 
    /// # };
    /// #
    /// let quaternion1 = Quaternion::new(1_i32, 2_i32, 3_i32, 4_i32);
    /// let quaternion2 = Quaternion::new(5_i32, 6_i32, 7_i32, 8_i32);
    /// let expected = 70_i32;
    /// let result = quaternion1.dot(&quaternion2);
    /// 
    /// assert_eq!(result, expected);
    /// ```
    #[inline]
    pub fn dot(self, other: &Quaternion<S>) -> S {
        self.s * other.s + self.v.dot(&other.v)
    }
}

impl<S> Quaternion<S> 
where 
    S: ScalarSigned 
{
    /// Compute the conjugate of a quaternion.
    ///
    /// Given a quaternion `q := s + v` where `s` is a scalar and `v` is a vector,
    /// the conjugate of `q` is the quaternion `q* := s - v`.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion,
    /// #     Vector3,
    /// # };
    /// #
    /// let scalar = 1_f64;
    /// let vector = Vector3::new(2_f64, 3_f64, 4_f64);
    /// let quaternion = Quaternion::from_parts(scalar, vector);
    /// let expected = Quaternion::from_parts(scalar, -vector);
    /// let result = quaternion.conjugate();
    ///
    /// assert_eq!(result, expected);
    /// ```
    #[inline]
    pub fn conjugate(&self) -> Quaternion<S> {
        Quaternion::from_parts(self.s, -self.v)
    }
}

impl<S> Quaternion<S> 
where 
    S: ScalarFloat 
{
    /// Construct a quaternion corresponding to rotating about an axis `axis` 
    /// by an angle `angle` in radians from its unit polar decomposition.
    ///
    /// ## Example
    ///
    /// Construct a quaternion for performing a 30 degree rotation about the **z-axis**.
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion,
    /// #     Unit,
    /// #     Vector3,
    /// #     Angle,
    /// #     Radians,
    /// #     Magnitude,
    /// # };
    /// # use approx::{
    /// #     relative_eq,
    /// # };
    /// #
    /// let axis: Unit<Vector3<f64>> = Unit::from_value(Vector3::unit_z());
    /// let pi_over_6: Radians<f64> = Radians::full_turn() / 12_f64;
    /// let cos_pi_over_12 = (f64::sqrt(3_f64) + 1_f64) / (2_f64 * f64::sqrt(2_f64));
    /// let sin_pi_over_12 = (f64::sqrt(3_f64) - 1_f64) / (2_f64 * f64::sqrt(2_f64));
    /// let expected = Quaternion::new(cos_pi_over_12, 0_f64, 0_f64, sin_pi_over_12);
    ///
    /// assert!(relative_eq!(expected.magnitude_squared(), 1_f64, epsilon = 1e-10));
    /// 
    /// let result = Quaternion::from_axis_angle(&axis, pi_over_6);
    /// 
    /// assert!(relative_eq!(result, expected, epsilon = 1e-10));
    /// ```
    ///
    /// A quaternion constructed from an axis and an angle will have the 
    /// cosine and sine of half of the angle in its components because a
    /// quaternion used for rotation will be used as a quaternion rotor. In 
    /// order to do a full rotation of a vector by the full angle `angle`, the 
    /// quaternion generated by `from_axis_angle` must be used as a quaternion 
    /// rotor.
    ///
    /// Let `p` be a three-dimensional vector we wish to rotate, 
    /// represented as a pure quaternion, and `q` be the quaternion that 
    /// rotates the vector `p` by an angle `angle`. Then the vector rotated 
    /// by the quaternion by an angle `angle` about the axis `axis` is given by
    /// ```text
    /// q * p * q^-1
    /// ```
    /// 
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Angle, 
    /// #     Quaternion,
    /// #     Radians,
    /// #     Unit,
    /// #     Vector3,
    /// # };
    /// # use approx::{
    /// #     relative_eq, 
    /// # };
    /// #
    /// let axis: Unit<Vector3<f64>> = Unit::from_value(Vector3::unit_z());
    /// let angle: Radians<f64> = Radians::full_turn_div_4();
    /// let p: Quaternion<f64> = Quaternion::unit_x();
    /// let expected : Quaternion<f64> = Quaternion::unit_y();
    /// let q = Quaternion::from_axis_angle(&axis, angle);
    /// let q_inv = q.inverse().unwrap();
    /// let result = q * p * q_inv;
    ///
    /// assert!(relative_eq!(result, expected, epsilon = 1e-10));
    /// ```
    #[inline]
    pub fn from_axis_angle<A: Into<Radians<S>>>(axis: &Unit<Vector3<S>>, angle: A) -> Quaternion<S> {
        let one_half = num_traits::cast(0.5_f64).unwrap();
        let (sin_angle, cos_angle) = Radians::sin_cos(angle.into() * one_half);
        let _axis = axis.into_inner();
    
        Quaternion::from_parts(cos_angle, _axis * sin_angle)
    }

    /// Construct a quaternion from an equivalent 3x3 matrix.
    ///
    /// A quaternion can be constructed by starting from the Euler-Rodrigues 
    /// formula and working backwards to extract the components of the quaternion
    /// from the components of the matrix.
    ///
    /// ## Example
    /// Here we extract a quaternion from a 60-degree rotation about 
    /// the **z-axis**.
    /// ```
    /// # use cglinalg::{
    /// #     Matrix3x3,
    /// #     Quaternion,
    /// #     Vector3,
    /// # };
    /// #
    /// let matrix = Matrix3x3::new(
    ///       1_f64 / 2_f64,             f64::sqrt(3_f64) / 2_f64, 0_f64,
    ///      -f64::sqrt(3_f64) / 2_f64, 1_f64 / 2_f64,            0_f64,
    ///       0_f64,                    0_f64,                    1_f64
    /// );
    /// let scalar = f64::sqrt(3_f64) / 2_f64;
    /// let vector = Vector3::new(0_f64, 0_f64, 1_f64 / 2_f64);
    /// let expected = Quaternion::from_parts(scalar, vector);
    /// let result = Quaternion::from_matrix(&matrix);
    ///
    /// assert_eq!(result, expected);
    /// ```
    #[inline]
    pub fn from_matrix(matrix: &Matrix3x3<S>) -> Quaternion<S> {
        let trace = matrix.trace();
        let one_half: S = num_traits::cast(0.5_f64).unwrap();
        if trace >= S::zero() {
            let s = (S::one() + trace).sqrt();
            let qs = one_half * s;
            let s = one_half / s;
            let qx = (matrix[1][2] - matrix[2][1]) * s;
            let qy = (matrix[2][0] - matrix[0][2]) * s;
            let qz = (matrix[0][1] - matrix[1][0]) * s;
            
            Quaternion::new(qs, qx, qy, qz)
        } else if (matrix[0][0] > matrix[1][1]) && (matrix[0][0] > matrix[2][2]) {
            let s = ((matrix[0][0] - matrix[1][1] - matrix[2][2]) + S::one()).sqrt();
            let qx = one_half * s;
            let s = one_half / s;
            let qy = (matrix[1][0] + matrix[0][1]) * s;
            let qz = (matrix[0][2] + matrix[2][0]) * s;
            let qs = (matrix[1][2] - matrix[2][1]) * s;
            
            Quaternion::new(qs, qx, qy, qz)
        } else if matrix[1][1] > matrix[2][2] {
            let s = ((matrix[1][1] - matrix[0][0] - matrix[2][2]) + S::one()).sqrt();
            let qy = one_half * s;
            let s = one_half / s;
            let qz = (matrix[2][1] + matrix[1][2]) * s;
            let qx = (matrix[1][0] + matrix[0][1]) * s;
            let qs = (matrix[2][0] - matrix[0][2]) * s;
            
            Quaternion::new(qs, qx, qy, qz)
        } else {
            let s = ((matrix[2][2] - matrix[0][0] - matrix[1][1]) + S::one()).sqrt();
            let qz = one_half * s;
            let s = one_half / s;
            let qx = (matrix[0][2] + matrix[2][0]) * s;
            let qy = (matrix[2][1] + matrix[1][2]) * s;
            let qs = (matrix[0][1] - matrix[1][0]) * s;
            
            Quaternion::new(qs, qx, qy, qz)
        }
    }

    /// Convert a quaternion to its equivalent 3x3 matrix form.
    ///
    /// The following example shows the result of converting an arbitrary 
    /// quaternion to its matrix form using the Euler-Rodrigues formula.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion,
    /// #     Matrix3x3,  
    /// #     Magnitude,
    /// # };
    /// # use approx::{
    /// #     relative_eq,  
    /// # };
    /// #
    /// let quaternion = Quaternion::new(1_f64, 2_f64, 3_f64, 4_f64);
    /// let scale = 2_f64 / quaternion.magnitude_squared();
    /// let expected = Matrix3x3::new(
    ///     1_f64 - scale * 25_f64, scale * 10_f64,         scale * 5_f64,
    ///     scale * 2_f64,          1_f64 - scale * 20_f64, scale * 14_f64,
    ///     scale * 11_f64,         scale * 10_f64,         1_f64 - scale * 13_f64
    /// );
    /// let result = quaternion.to_matrix3x3();
    /// 
    /// assert!(relative_eq!(result, expected, epsilon = 1e-8));
    /// ```
    ///
    /// The following example shows the result of converting an unit 
    /// quaternion to its matrix form using the Euler-Rodrigues formula.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion,
    /// #     Matrix3x3,  
    /// #     Magnitude,
    /// # };
    /// # use approx::{
    /// #     relative_eq,  
    /// # };
    /// #
    /// let quaternion = Quaternion::new(1_f64, 1_f64, 1_f64, 1_f64) / 2_f64;
    ///
    /// assert_eq!(quaternion.magnitude_squared(), 1_f64);
    /// 
    /// let scale = 2_f64;
    /// let expected = Matrix3x3::new(
    ///     1_f64 - scale * (1_f64 / 2_f64), scale * (1_f64 / 2_f64),         scale * 0_f64,
    ///     scale * 0_f64,                   1_f64 - scale * (1_f64 / 2_f64), scale * (1_f64 / 2_f64),
    ///     scale * (1_f64 / 2_f64),         scale * 0_f64,                   1_f64 - scale * (1_f64 / 2_f64)
    /// );
    /// let result = quaternion.to_matrix3x3();
    /// 
    /// assert_eq!(result, expected);
    /// ```
    #[rustfmt::skip]
    #[inline]
    pub fn to_matrix3x3(&self) -> Matrix3x3<S> {
        let qs = self.s;
        let qx = self.v.x;
        let qy = self.v.y;
        let qz = self.v.z;
        let one = S::one();
        let two = one + one;
        let s = two / self.magnitude_squared();

        let c0r0 = one - s * qy * qy - s * qz * qz;
        let c0r1 = s * qx * qy + s * qs * qz;
        let c0r2 = s * qx * qz - s * qs * qy;

        let c1r0 = s * qx * qy - s * qs * qz;
        let c1r1 = one - s * qx * qx - s * qz * qz;
        let c1r2 = s * qy * qz + s * qs * qx;

        let c2r0 = s * qx * qz + s * qs * qy;
        let c2r1 = s * qy * qz - s * qs * qx;
        let c2r2 = one - s * qx * qx - s * qy * qy;
    
        Matrix3x3::new(
            c0r0, c0r1, c0r2,
            c1r0, c1r1, c1r2,
            c2r0, c2r1, c2r2
        )
    }

    /// Convert a quaternion to its equivalent 3x3 matrix form using 
    /// preallocated storage.
    ///
    /// The following example shows the result of converting an arbitrary 
    /// quaternion to its matrix form using the Euler-Rodrigues formula.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion,
    /// #     Matrix3x3,  
    /// #     Magnitude,
    /// # };
    /// # use approx::{
    /// #     relative_eq,  
    /// # };
    /// #
    /// let quaternion = Quaternion::new(1_f64, 2_f64, 3_f64, 4_f64);
    /// let scale = 2_f64 / quaternion.magnitude_squared();
    /// let expected = Matrix3x3::new(
    ///     1_f64 - scale * 25_f64, scale * 10_f64,         scale * 5_f64,
    ///     scale * 2_f64,          1_f64 - scale * 20_f64, scale * 14_f64,
    ///     scale * 11_f64,         scale * 10_f64,         1_f64 - scale * 13_f64
    /// );
    /// let mut result = Matrix3x3::zero();
    ///
    /// assert!(result.is_zero());
    ///
    /// quaternion.to_matrix3x3_mut(&mut result);
    /// 
    /// assert!(relative_eq!(result, expected, epsilon = 1e-8));
    /// ```
    /// The following example shows the result of converting an unit 
    /// quaternion to its matrix form using the Euler-Rodrigues formula.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion,
    /// #     Matrix3x3,  
    /// #     Magnitude,
    /// # };
    /// # use approx::{
    /// #     relative_eq,  
    /// # };
    /// #
    /// let quaternion = Quaternion::new(1_f64, 1_f64, 1_f64, 1_f64) / 2_f64;
    ///
    /// assert_eq!(quaternion.magnitude_squared(), 1_f64);
    /// 
    /// let scale = 2_f64;
    /// let expected = Matrix3x3::new(
    ///     1_f64 - scale * (1_f64 / 2_f64), scale * (1_f64 / 2_f64),         scale * 0_f64,
    ///     scale * 0_f64,                   1_f64 - scale * (1_f64 / 2_f64), scale * (1_f64 / 2_f64),
    ///     scale * (1_f64 / 2_f64),         scale * 0_f64,                   1_f64 - scale * (1_f64 / 2_f64)
    /// );
    /// let mut result = Matrix3x3::zero();
    ///
    /// assert!(result.is_zero());
    ///
    /// quaternion.to_matrix3x3_mut(&mut result);
    /// 
    /// assert_eq!(result, expected);
    /// ```
    #[inline]
    pub fn to_matrix3x3_mut(&self, matrix: &mut Matrix3x3<S>) {
        let qs = self.s;
        let qx = self.v.x;
        let qy = self.v.y;
        let qz = self.v.z;
        let one = S::one();
        let two = one + one;
        let s = two / self.magnitude_squared();
    
        matrix.c0r0 = one - s * qy * qy - s * qz * qz;
        matrix.c0r1 = s * qx * qy + s * qs * qz;
        matrix.c0r2 = s * qx * qz - s * qs * qy;
    
        matrix.c1r0 = s * qx * qy - s * qs * qz;
        matrix.c1r1 = one - s * qx * qx - s * qz * qz;
        matrix.c1r2 = s * qy * qz + s * qs * qx;
    
        matrix.c2r0 = s * qx * qz + s * qs * qy;
        matrix.c2r1 = s * qy * qz - s * qs * qx;
        matrix.c2r2 = one - s * qx * qx - s * qy * qy;
    }

    /// Convert a quaternion to its equivalent affine 4x4 matrix form.
    ///
    /// The following example shows the result of converting an arbitrary 
    /// quaternion to its affine matrix form using the Euler-Rodrigues formula.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion,
    /// #     Matrix4x4,  
    /// #     Magnitude,
    /// # };
    /// # use approx::{
    /// #     relative_eq,  
    /// # };
    /// #
    /// let quaternion = Quaternion::new(1_f64, 2_f64, 3_f64, 4_f64);
    /// let scale = 2_f64 / quaternion.magnitude_squared();
    /// let expected = Matrix4x4::new(
    ///     1_f64 - scale * 25_f64, scale * 10_f64,         scale * 5_f64,          0_f64,
    ///     scale * 2_f64,          1_f64 - scale * 20_f64, scale * 14_f64,         0_f64,
    ///     scale * 11_f64,         scale * 10_f64,         1_f64 - scale * 13_f64, 0_f64,
    ///     0_f64,                  0_f64,                  0_f64,                  1_f64
    /// );
    /// let result = quaternion.to_matrix4x4();
    /// 
    /// assert!(relative_eq!(result, expected, epsilon = 1e-8));
    /// ```
    ///
    /// The following example shows the result of converting an unit 
    /// quaternion to its affine matrix form using the Euler-Rodrigues formula.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion,
    /// #     Matrix4x4,  
    /// #     Magnitude,
    /// # };
    /// # use approx::{
    /// #     relative_eq,  
    /// # };
    /// #
    /// let quaternion = Quaternion::new(1_f64, 1_f64, 1_f64, 1_f64) / 2_f64;
    ///
    /// assert_eq!(quaternion.magnitude_squared(), 1_f64);
    /// 
    /// let scale = 2_f64;
    /// let expected = Matrix4x4::new(
    ///     1_f64 - scale * (1_f64 / 2_f64), scale * (1_f64 / 2_f64),         scale * 0_f64,                   0_f64,
    ///     scale * 0_f64,                   1_f64 - scale * (1_f64 / 2_f64), scale * (1_f64 / 2_f64),         0_f64,
    ///     scale * (1_f64 / 2_f64),         scale * 0_f64,                   1_f64 - scale * (1_f64 / 2_f64), 0_f64,
    ///     0_f64,                           0_f64,                           0_f64,                           1_f64
    /// );
    /// let result = quaternion.to_matrix4x4();
    /// 
    /// assert_eq!(result, expected);
    /// ```
    #[rustfmt::skip]
    #[inline]
    pub fn to_matrix4x4(&self) -> Matrix4x4<S> {
        let qs = self.s;
        let qx = self.v.x;
        let qy = self.v.y;
        let qz = self.v.z;
        let zero = S::zero();
        let one = S::one();
        let two = one + one;
        let s = two / self.magnitude_squared();

        let c0r0 = one - s * qy * qy - s * qz * qz;
        let c0r1 = s * qx * qy + s * qs * qz;
        let c0r2 = s * qx * qz - s * qs * qy;
        let c0r3 = zero;

        let c1r0 = s * qx * qy - s * qs * qz;
        let c1r1 = one - s * qx * qx - s * qz * qz;
        let c1r2 = s * qy * qz + s * qs * qx;
        let c1r3 = zero;

        let c2r0 = s * qx * qz + s * qs * qy;
        let c2r1 = s * qy * qz - s * qs * qx;
        let c2r2 = one - s * qx * qx - s * qy * qy;
        let c2r3 = zero;
        
        let c3r0 = zero;
        let c3r1 = zero;
        let c3r2 = zero;
        let c3r3 = one;
    
        Matrix4x4::new(
            c0r0, c0r1, c0r2, c0r3,
            c1r0, c1r1, c1r2, c1r3,
            c2r0, c2r1, c2r2, c2r3,
            c3r0, c3r1, c3r2, c3r3
        )
    }

    /// Convert a quaternion to its equivalent affine 4x4 matrix form using
    /// preallocated storage.
    ///
    /// The following example shows the result of converting an arbitrary 
    /// quaternion to its affine matrix form using the Euler-Rodrigues formula.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion,
    /// #     Matrix4x4,  
    /// #     Magnitude,
    /// # };
    /// # use approx::{
    /// #     relative_eq,  
    /// # };
    /// #
    /// let quaternion = Quaternion::new(1_f64, 2_f64, 3_f64, 4_f64);
    /// let scale = 2_f64 / quaternion.magnitude_squared();
    /// let expected = Matrix4x4::new(
    ///     1_f64 - scale * 25_f64, scale * 10_f64,         scale * 5_f64,          0_f64,
    ///     scale * 2_f64,          1_f64 - scale * 20_f64, scale * 14_f64,         0_f64,
    ///     scale * 11_f64,         scale * 10_f64,         1_f64 - scale * 13_f64, 0_f64,
    ///     0_f64,                  0_f64,                  0_f64,                  1_f64
    /// );
    /// let mut result = Matrix4x4::zero();
    ///
    /// assert!(result.is_zero());
    ///
    /// quaternion.to_matrix4x4_mut(&mut result);
    /// 
    /// assert!(relative_eq!(result, expected, epsilon = 1e-8));
    /// ```
    ///
    /// The following example shows the result of converting an unit 
    /// quaternion to its affine matrix form using the Euler-Rodrigues formula.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion,
    /// #     Matrix4x4,  
    /// #     Magnitude,
    /// # };
    /// # use approx::{
    /// #     relative_eq,  
    /// # };
    /// #
    /// let quaternion = Quaternion::new(1_f64, 1_f64, 1_f64, 1_f64) / 2_f64;
    ///
    /// assert_eq!(quaternion.magnitude_squared(), 1_f64);
    /// 
    /// let scale = 2_f64;
    /// let expected = Matrix4x4::new(
    ///     1_f64 - scale * (1_f64 / 2_f64), scale * (1_f64 / 2_f64),         scale * 0_f64,                   0_f64,
    ///     scale * 0_f64,                   1_f64 - scale * (1_f64 / 2_f64), scale * (1_f64 / 2_f64),         0_f64,
    ///     scale * (1_f64 / 2_f64),         scale * 0_f64,                   1_f64 - scale * (1_f64 / 2_f64), 0_f64,
    ///     0_f64,                           0_f64,                           0_f64,                           1_f64
    /// );
    /// let mut result = Matrix4x4::zero();
    ///
    /// assert!(result.is_zero());
    ///
    /// quaternion.to_matrix4x4_mut(&mut result);
    /// 
    /// assert!(relative_eq!(result, expected, epsilon = 1e-8));
    /// ```
    #[inline]
    pub fn to_matrix4x4_mut(&self, matrix: &mut Matrix4x4<S>) {
        let qs = self.s;
        let qx = self.v.x;
        let qy = self.v.y;
        let qz = self.v.z;
        let zero = S::zero();
        let one = S::one();
        let two = one + one;
        let s = two / self.magnitude_squared();

        matrix.c0r0 = one - s * qy * qy - s * qz * qz;
        matrix.c0r1 = s * qx * qy + s * qs * qz;
        matrix.c0r2 = s * qx * qz - s * qs * qy;
        matrix.c0r3 = zero;

        matrix.c1r0 = s * qx * qy - s * qs * qz;
        matrix.c1r1 = one - s * qx * qx - s * qz * qz;
        matrix.c1r2 = s * qy * qz + s * qs * qx;
        matrix.c1r3 = zero;

        matrix.c2r0 = s * qx * qz + s * qs * qy;
        matrix.c2r1 = s * qy * qz - s * qs * qx;
        matrix.c2r2 = one - s * qx * qx - s * qy * qy;
        matrix.c2r3 = zero;

        matrix.c3r0 = zero;
        matrix.c3r1 = zero;
        matrix.c3r2 = zero;
        matrix.c3r3 = one;
    }

    /// Compute the inverse of a quaternion.
    ///
    /// If the quaternion `self` has zero magnitude, it does not have an 
    /// inverse. In this case the function return `None`. Otherwise it returns 
    /// the inverse of `self`.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion,
    /// # };
    /// #
    /// let zero_quat: Quaternion<f64> = Quaternion::zero();
    /// 
    /// assert!(zero_quat.is_zero());
    /// assert!(zero_quat.inverse().is_none());
    ///
    /// let quaternion = Quaternion::new(1_f64, 1_f64, 0_f64, 0_f64);
    /// let expected = Some(Quaternion::new(1_f64 / 2_f64, -1_f64 / 2_f64, 0_f64, 0_f64));
    /// let result = quaternion.inverse();
    /// 
    /// assert_eq!(result, expected);
    /// ```
    #[inline]
    pub fn inverse(&self) -> Option<Quaternion<S>> {
        self.inverse_eps(S::default_epsilon())
    }

    #[inline]
    fn inverse_eps(&self, epsilon: S) -> Option<Quaternion<S>> {
        let magnitude_squared = self.magnitude_squared();
        if magnitude_squared <= epsilon * epsilon {
            None
        } else {
            Some(self.conjugate() / magnitude_squared)
        }
    }

    /// Determine whether a quaternion is invertible.
    ///
    /// If the quaternion `self` has zero magnitude, it does not have an 
    /// inverse. In this case the function return `None`. Otherwise it returns 
    /// the inverse of `self`.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion,
    /// # };
    /// #
    /// let zero_quat: Quaternion<f64> = Quaternion::zero();
    /// 
    /// assert!(zero_quat.is_zero());
    /// assert!(!zero_quat.is_invertible());
    ///
    /// let quaternion = Quaternion::new(1_f64, 1_f64, 0_f64, 0_f64);
    /// 
    /// assert!(quaternion.is_invertible());
    /// ```
    #[inline]
    pub fn is_invertible(&self) -> bool {
        self.is_invertible_eps(S::default_epsilon())
    }

    #[inline]
    fn is_invertible_eps(&self, epsilon: S) -> bool {
        self.magnitude_squared() >= epsilon * epsilon
    }

    /// Compute the principal argument of a quaternion.
    ///
    /// Every quaternion can be written in polar form. Let `q` be a quaternion
    /// and let `q = qs + qv` where `qs` is the scalar part of `q` and `qv` is 
    /// the vector part of `q`. The polar form of q can be written as
    /// ```text
    /// q = |q| * (cos(theta) + (qv / |qv|) * sin(theta))
    /// ```
    /// The argument of `q` is the set of angles `theta` that satisfy the 
    /// relation above which we denote `arg(q)`. The principal argument of `q` 
    /// is the angle `theta` satisfying the polar decomposition of `q` above 
    /// such that `theta` lies in the closed interval `[0, pi]`. For each 
    /// element `theta` in `arg(q)`, there is an integer `n` such that
    /// ```text
    /// theta = Arg(q) + 2 * pi * n
    /// ```
    /// In the case of `theta = Arg(q)`, we have `n = 0`.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion,
    /// #     Vector3,
    /// # };
    /// # use approx::{
    /// #     relative_eq,  
    /// # };
    /// # use core::f64;
    /// #
    /// let zero_quat: Quaternion<f64> = Quaternion::zero();
    /// let pi_over_two = f64::consts::FRAC_PI_2;
    ///
    /// assert!(zero_quat.is_zero());
    /// assert_eq!(zero_quat.arg(), pi_over_two);
    /// 
    /// let quaternion = Quaternion::new(1_f64, 1_f64, 1_f64, 1_f64);
    /// let pi_over_three = f64::consts::FRAC_PI_3;
    /// let expected = pi_over_three;
    /// let result = quaternion.arg();
    ///
    /// assert!(relative_eq!(result, expected, epsilon = 1e-10));
    /// ```
    #[inline]
    pub fn arg(&self) -> S {
        self.arg_eps(S::default_epsilon())
    }

    #[inline]
    fn arg_eps(&self, epsilon: S) -> S {
        if self.s * self.s <= epsilon * epsilon {
            num_traits::cast(core::f64::consts::FRAC_PI_2).unwrap()
        } else {
            S::atan(self.v.magnitude() / self.s)
        }
    }

    /// Calculate the exponential of a quaternion.
    ///
    /// ## Examples
    ///
    /// Compute the exponential of a scalar quaternion.
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion,
    /// #     Vector3,
    /// # };
    /// # use core::f64;
    /// #
    /// let zero_vec = Vector3::zero();
    /// let s = 3_f64;
    /// let qs = Quaternion::from_parts(s, zero_vec);
    /// let result = qs.exp();
    /// let expected = Quaternion::from_parts(s.exp(), zero_vec);
    ///
    /// assert_eq!(result, expected);
    /// ```
    ///
    /// A computation involving the unit **x-axis** pure quaternion. 
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion,
    /// #     Vector3,
    /// # };
    /// # use approx::{
    /// #     relative_eq, 
    /// # };
    /// # use core::f64;
    /// #
    /// let zero_vec = Vector3::zero();
    /// let unit_x = Quaternion::unit_x();
    /// let pi = f64::consts::PI;
    /// let result = (unit_x * pi).exp();
    /// let expected = Quaternion::from_parts(-1_f64, zero_vec);
    /// 
    /// assert!(relative_eq!(result, expected, epsilon = 1e-10));
    /// ```
    ///
    /// A computation involving the unit **y-axis** pure quaternion. 
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion,
    /// #     Vector3,
    /// # };
    /// # use approx::{
    /// #     relative_eq, 
    /// # };
    /// # use core::f64;
    /// #
    /// let zero_vec = Vector3::zero();
    /// let unit_y = Quaternion::unit_y();
    /// let pi = f64::consts::PI;
    /// let result = (unit_y * pi).exp();
    /// let expected = Quaternion::from_parts(-1_f64, zero_vec);
    /// 
    /// assert!(relative_eq!(result, expected, epsilon = 1e-10));
    /// ```
    ///
    /// A computation involving the unit **z-axis** pure quaternion. 
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion,
    /// #     Vector3,
    /// # };
    /// # use approx::{
    /// #     relative_eq, 
    /// # };
    /// # use core::f64;
    /// #
    /// let zero_vec = Vector3::zero();
    /// let unit_z = Quaternion::unit_z();
    /// let pi = f64::consts::PI;
    /// let result = (unit_z * pi).exp();
    /// let expected = Quaternion::from_parts(-1_f64, zero_vec);
    /// 
    /// assert!(relative_eq!(result, expected, epsilon = 1e-10));
    /// ```
    ///
    /// A computation involving the unit **z-axis** pure quaternion again.
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion,
    /// #     Vector3,
    /// # };
    /// # use approx::{
    /// #     relative_eq, 
    /// # };
    /// # use core::f64;
    /// #
    /// let unit_z = Quaternion::unit_z();
    /// let pi_over_two = f64::consts::PI / 2_f64;
    /// let result = (unit_z * pi_over_two).exp();
    /// let expected = unit_z;
    ///
    /// assert!(relative_eq!(result, expected, epsilon = 1e-10));
    /// ```
    #[inline]
    pub fn exp(&self) -> Quaternion<S> {
        self.exp_eps(S::default_epsilon())
    }

    #[inline]
    fn exp_eps(&self, epsilon: S) -> Quaternion<S> {
        let magnitude_v_squared = self.v.magnitude_squared();
        if magnitude_v_squared <= epsilon * epsilon {
            Quaternion::from_parts(self.s.exp(), Vector3::zero())
        } else {
            let magnitude_v = magnitude_v_squared.sqrt();
            let exp_s = self.s.exp();
            let q_scalar = exp_s * S::cos(magnitude_v);
            let q_vector = self.v * (exp_s * S::sin(magnitude_v) / magnitude_v);
            
            Quaternion::from_parts(q_scalar, q_vector)
        }
    }

    /// Calculate the principal value of the natural logarithm of a quaternion.
    ///
    /// Like the natural logarithm of a complex number, the natural 
    /// logarithm of a quaternion has multiple possible values. We define the 
    /// principal value of the quaternion logarithm
    /// ```text
    /// Ln(q) = log(||q||, e) + sgn(Vec(q)) * Arg(q)
    /// ```
    /// where `Arg(q)` is the principal argument of `q`, `||q||` is the 
    /// magnitude of `q`, `Vec(q)` is the vector part of `q`, `sgn(.)` is the 
    /// signum function, and `log(., e)` denotes the natural logarithm of a 
    /// scalar. Returning the principal value allows us to define a unique 
    /// natural logarithm for each quaternion `q`.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion,
    /// #     Vector3,
    /// # };
    /// # use approx::{
    /// #     relative_eq, 
    /// # };
    /// # use core::f64;
    /// #
    /// let unit_z: Vector3<f64> = Vector3::unit_z();
    /// let pi = f64::consts::PI;
    /// let quaternion = Quaternion::from_parts(1_f64, unit_z);
    /// let expected = Quaternion::new(f64::ln(f64::sqrt(2_f64)), 0_f64, 0_f64, pi / 4_f64);
    /// let result = quaternion.ln(); 
    ///
    /// assert!(relative_eq!(result, expected, epsilon = 1e-10));
    /// ```
    ///
    /// ## Example
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion,
    /// #     Vector3,
    /// # };
    /// # use approx::{
    /// #     relative_eq, 
    /// # };
    /// # use core::f64;
    /// #
    /// let quaternion = Quaternion::new(1_f64, 1_f64, 1_f64, 1_f64);
    /// let pi_over_three = f64::consts::FRAC_PI_3;
    /// let scalar = f64::ln(2_f64);
    /// let vector = (1_f64 / f64::sqrt(3_f64)) * pi_over_three * Vector3::new(1_f64, 1_f64, 1_f64);
    /// let expected = Quaternion::from_parts(scalar, vector);
    /// let result = quaternion.ln();
    ///
    /// assert!(relative_eq!(result, expected, epsilon = 1e-10));
    /// ```
    #[inline]
    pub fn ln(&self) -> Quaternion<S> {
        self.ln_eps(S::default_epsilon())
    }

    #[inline]
    fn ln_eps(&self, epsilon: S) -> Quaternion<S> {
        let magnitude_v_squared = self.v.magnitude_squared();
        if magnitude_v_squared <= epsilon * epsilon {
            let magnitude = self.s.abs();
            Quaternion::from_parts(magnitude.ln(), Vector3::zero())
        } else {
            let magnitude = self.magnitude();
            let arccos_s_over_mag_q = S::acos(self.s / magnitude);
            let q_scalar = S::ln(magnitude);
            let q_vector = self.v * (arccos_s_over_mag_q / magnitude_v_squared.sqrt());

            Quaternion::from_parts(q_scalar, q_vector)
        }
    }

    /// Calculate the power of a quaternion where the exponent is a real number.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion,
    /// #     Vector3, 
    /// # };
    /// # use approx::{
    /// #     relative_eq, 
    /// # };
    /// #
    /// let scalar = 1_f64;
    /// let vector = Vector3::unit_z();
    /// let quaternion = Quaternion::from_parts(scalar, vector);
    /// let exponent = 2_f64;
    /// let expected = 2_f64 * Quaternion::unit_z();
    /// let result = quaternion.powf(exponent);
    ///
    /// assert!(relative_eq!(result, expected, epsilon = 1e-10));
    /// ```
    #[inline]
    pub fn powf(&self, exponent: S) -> Quaternion<S> {
        (self.ln() * exponent).exp()
    }

    /// Construct a quaternion that rotates the shortest angular distance 
    /// between two vectors.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion,
    /// #     Radians,
    /// #     Angle,
    /// #     Vector3,
    /// #     Unit,
    /// # };
    /// # use approx::{
    /// #     relative_eq,   
    /// # };
    /// #
    /// let v1: Vector3<f64> = Vector3::unit_x() * 2_f64;
    /// let v2: Vector3<f64> = Vector3::unit_y() * 3_f64;
    /// let unit_z: Unit<Vector3<f64>> = Unit::from_value(Vector3::unit_z());
    /// let expected = Quaternion::from_axis_angle(
    ///    &unit_z, 
    ///    Radians::full_turn_div_4()
    /// );
    /// let result = Quaternion::rotation_between(&v1, &v2).unwrap();
    /// 
    /// assert!(relative_eq!(result, expected));
    /// ```
    #[inline]
    pub fn rotation_between(
        v1: &Vector3<S>, v2: &Vector3<S>) -> Option<Quaternion<S>>
    {
        if let (Some(unit_v1), Some(unit_v2)) = (
            Unit::try_from_value(*v1, S::zero()),
            Unit::try_from_value(*v2, S::zero()),
        ) {
            Self::rotation_between_axis(&unit_v1, &unit_v2)
        } else {
            Some(Self::identity())
        }
    }

    /// Construct a quaternion that rotates the shortest angular distance 
    /// between two unit vectors.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion,
    /// #     Radians,
    /// #     Angle,
    /// #     Unit,
    /// #     Vector3,
    /// # };
    /// # use approx::{
    /// #     relative_eq,   
    /// # };
    /// #
    /// let unit_x: Unit<Vector3<f64>> = Unit::from_value(Vector3::unit_x());
    /// let unit_y: Unit<Vector3<f64>> = Unit::from_value(Vector3::unit_y());
    /// let unit_z: Unit<Vector3<f64>> = Unit::from_value(Vector3::unit_z());
    /// let expected = Quaternion::from_axis_angle(
    ///    &unit_z, 
    ///    Radians::full_turn_div_4()
    /// );
    /// let result = Quaternion::rotation_between_axis(&unit_x, &unit_y).unwrap();
    /// 
    /// assert!(relative_eq!(result, expected));
    /// ```
    #[inline]
    pub fn rotation_between_axis(
        unit_v1: &Unit<Vector3<S>>, unit_v2: &Unit<Vector3<S>>) -> Option<Quaternion<S>> 
    {
        let v1 = unit_v1.as_ref();
        let v2 = unit_v2.as_ref();
        let v1_cross_v2 = v1.cross(v2);

        if let Some(axis) = Unit::try_from_value(v1_cross_v2, S::default_epsilon()) {
            let cos_theta = v1.dot(v2);

            if cos_theta <= -S::one() {
                // The cosines may fall outside the interval [-1, 1] because of floating 
                // point inaccuracies.
                None
            } else if cos_theta >= S::one() {
                // The cosines may fall outside the interval [-1, 1] because of floating 
                // point inaccuracies.
                Some(Self::identity())
            } else {
                // The cosine falls inside the interval [-1, 1].
                Some(Self::from_axis_angle(&axis, Radians::acos(cos_theta)))
            }
        } else if v1.dot(v2) < S::zero() {
            // There are two ways to rotate around the unit circle between two vectors.
            // If both one vector is the negation of the other one, i.e. they are pi radians
            // apart, then the arc distance along either direction around the unit circle is
            // identical, so we have no way of discerning one path from the other.
            None
        } else {
            // Both vectors point in the same direction.
            Some(Self::identity())
        }
    }

    /// Construct a quaternion corresponding to a rotation of an observer 
    /// standing at the origin facing the **positive z-axis** to an observer 
    /// standing at the origin facing the direction `direction`. 
    ///
    /// This rotation maps the **z-axis** to the direction `direction`.
    #[inline]
    pub fn face_towards(direction: &Vector3<S>, up: &Vector3<S>) -> Quaternion<S> {
        Self::from(&Matrix3x3::face_towards(direction, up))
    }

    /// Construct a quaternion corresponding to a right-handed viewing 
    /// transformation without translation. 
    ///
    /// This transformation maps the viewing direction `direction` to the 
    /// **negative z-axis**. It is conventionally used in computer graphics for
    /// camera view transformations.
    #[inline]
    pub fn look_at_rh(direction: &Vector3<S>, up: &Vector3<S>) -> Quaternion<S> {
        Self::from(&Matrix3x3::face_towards(direction, up).transpose())
    }

    /// Construct a quaternion corresponding to a left-handed viewing 
    /// transformation without translation. 
    ///
    /// This transformation maps the viewing direction `direction` to the 
    /// **negative z-axis**. It is conventionally used in computer graphics for
    /// camera view transformations.
    #[inline]
    pub fn look_at_lh(direction: &Vector3<S>, up: &Vector3<S>) -> Quaternion<S> {
        Self::from(&Matrix3x3::face_towards(direction, up).transpose())
    }

    /// Linearly interpolate between two quaternions.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion,  
    /// # };
    /// #
    /// let v0 = Quaternion::new(0_f64, 0_f64, 0_f64, 0_f64);
    /// let v1 = Quaternion::new(10_f64, 20_f64, 30_f64, 40_f64);
    /// let amount = 0.7;
    /// let expected = Quaternion::new(7_f64, 14_f64, 21_f64, 28_f64);
    /// let result = v0.lerp(&v1, amount);
    ///
    /// assert_eq!(result, expected);
    /// ```
    #[inline]
    pub fn lerp(&self, other: &Quaternion<S>, amount: S) -> Quaternion<S> {
        self + (other - self) * amount
    }

    /// Spherically linearly interpolate between two unit quaternions.
    ///
    /// In the case where the angle between quaternions is 180 degrees, the
    /// slerp function is not well defined because we can rotate about any axis
    /// normal to the plane swept out by the quaternions to get from one to the 
    /// other. The vector normal to the quaternions is not unique in this case.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Angle,
    /// #     Quaternion,
    /// #     Vector3,
    /// #     Degrees,
    /// # };
    /// # use approx::{
    /// #     relative_eq,   
    /// # };
    /// #
    /// let angle1 = Degrees(30_f64);
    /// let angle2 = Degrees(150_f64);
    /// let unit_z = Vector3::unit_z();
    /// let q1 = Quaternion::from_parts(
    ///    Angle::cos(angle1 / 2_f64), 
    ///    Angle::sin(angle1 / 2_f64) * unit_z
    /// );
    /// let q2 = Quaternion::from_parts(
    ///    Angle::cos(angle2 / 2_f64), 
    ///    Angle::sin(angle2 / 2_f64) * unit_z
    /// );
    /// let angle_expected = Degrees(90_f64);
    /// let expected = Quaternion::from_parts(
    ///    Angle::cos(angle_expected / 2_f64), 
    ///    Angle::sin(angle_expected / 2_f64) * unit_z
    /// );
    /// let result = q1.slerp(&q2, 0.5);
    ///
    /// assert!(relative_eq!(result, expected, epsilon = 1e-7));
    /// ```
    #[inline]
    pub fn slerp(&self, other: &Quaternion<S>, amount: S) -> Quaternion<S> {
        let zero = S::zero();
        let one = S::one();
        // There are two possible routes along a great circle arc between two 
        // quaternions on the three-sphere. By definition the slerp function 
        // computes the shortest path between two points on a sphere, hence we 
        // must determine which of two directions around the great circle arc 
        // swept out by the slerp function is the shortest one. 
        let (result, cos_half_theta) = if self.dot(other) < zero {
            // If the dot product is negative, the shortest path between two 
            // points on the great circle arc swept out by the quaternions runs 
            // in the opposite direction from the positive case, so we must 
            // negate one of the quaterions to take the short way around 
            // instead of the long way around.
            let _result = -self;
            (_result, (_result).dot(other))
        } else {
            let _result = *self;
            (_result, _result.dot(other))
        };

        // We have two opportunities for performance optimizations:
        //
        // If `result` == `other`, there is no curve to interpolate; the angle 
        // between `result` and  `other` is zero. In this case we can return 
        // `result`.
        if S::abs(cos_half_theta) >= one {
            return result;
        }

        // If `result` == `minus other` then the angle between them is 180 degrees.
        // In this case the slerp function is not well defined because we can 
        // rotate around any axis normal to the plane swept out by `result` and
        // `other`.
        //
        // For very small angles, `sin_half_theta` is approximately equal to the 
        // angle `half_theta`. That is, `sin(theta / 2) ~= theta / 2` as 
        // `theta -> 0`. Therefore, we can use the sine of the angle between two 
        // quaternions to determine when to approximate spherical linear 
        // interpolation with normalized linear interpolation. Using the sine of 
        // the angle is also cheaper to calculate since we can derive it from the
        // cosine we already calculated instead of calculating the angle from 
        // an inverse trigonometric function.
        let sin_half_theta = S::sqrt(one - cos_half_theta * cos_half_theta);
        let threshold = num_traits::cast(0.001).unwrap();
        if S::abs(sin_half_theta) < threshold {
            return result.nlerp(other, amount);
        }
        
        let half_theta = S::acos(cos_half_theta);
        let a = S::sin((one - amount) * half_theta) / sin_half_theta;
        let b = S::sin(amount * half_theta) / sin_half_theta;
        
        let s   = result.s   * a + other.s   * b;
        let v_x = result.v.x * a + other.v.x * b;
        let v_y = result.v.y * a + other.v.y * b;
        let v_z = result.v.z * a + other.v.z * b;

        Quaternion::new(s, v_x, v_y, v_z)
    }

    /// Compute the normalized linear interpolation between two quaternions.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion,
    /// #     Magnitude,
    /// # };
    /// #
    /// let v0 = Quaternion::new(0_f64, 0_f64, 0_f64, 0_f64);
    /// let v1 = Quaternion::new(10_f64, 20_f64, 30_f64, 40_f64);
    /// let amount = 0.7;
    /// let expected = v0.lerp(&v1, amount).normalize();
    /// let result = v0.nlerp(&v1, amount);
    ///
    /// assert_eq!(result, expected);
    /// ```
    #[inline]
    pub fn nlerp(&self, other: &Quaternion<S>, amount: S) -> Quaternion<S> {
        (self * (S::one() - amount) + other * amount).normalize()
    }

    /// Returns `true` if the elements of a quaternion are all finite. 
    /// Otherwise, it returns `false`. 
    ///
    /// A quaternion is finite when all of its elements are finite.
    ///
    /// ## Example (Finite Quaternion)
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion,
    /// # };
    /// #
    /// let quaternion = Quaternion::new(1_f64, 2_f64, 3_f64, 4_f64);
    ///
    /// assert!(quaternion.is_finite()); 
    /// ```
    ///
    /// ## Example (Not A Finite Vector)
    /// 
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion,
    /// # };
    /// #
    /// let quaternion = Quaternion::new(1_f64, f64::NAN, f64::NEG_INFINITY, 4_f64);
    ///
    /// assert!(!quaternion.is_finite());
    /// ```
    #[inline]
    pub fn is_finite(&self) -> bool {
        self.s.is_finite() && self.v.is_finite()
    }

    /// Compute the projection of the quaternion `self` onto the quaternion
    /// `other`.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion, 
    /// #     Magnitude,
    /// # };
    /// # 
    /// let quaternion = Quaternion::new(1_f64, 2_f64, 3_f64, 4_f64);
    /// let unit_x = Quaternion::unit_x();
    /// let unit_y = Quaternion::unit_y();
    /// let unit_z = Quaternion::unit_z();
    /// let unit_s = Quaternion::unit_s();
    /// let projected_x = quaternion.project(&unit_x);
    /// let projected_y = quaternion.project(&unit_y);
    /// let projected_z = quaternion.project(&unit_z);
    /// let projected_s = quaternion.project(&unit_s);
    ///
    /// assert_eq!(projected_x, quaternion.v.x * unit_x);
    /// assert_eq!(projected_y, quaternion.v.y * unit_y);
    /// assert_eq!(projected_z, quaternion.v.z * unit_z);
    /// assert_eq!(projected_s, quaternion.s * unit_s);
    /// ```
    #[inline]
    pub fn project(&self, other: &Quaternion<S>) -> Quaternion<S> {
        other * (self.dot(other) / other.magnitude_squared())
    }

    /// Compute the polar decomposition of a quaternion.
    ///
    /// Every quaternion `q` can be decomposed into a polar form. A
    /// quaternion `q`, can be written in polar form as
    /// ```text
    /// q := s + v := |q| * exp((-theta / 2) * vhat) 
    ///            := |q| * (cos(theta / 2) + vhat * sin(theta / 2))
    /// ```
    /// where `s` is the scalar part, `v` is the vector part, |q| is the length
    /// of the quaternion, `vhat` denotes the normalized part of the vector 
    /// part, and `theta` is the angle of rotation about the axis `vhat` 
    /// encoded by the quaternion.
    ///
    /// The output of the function is a triple containing the magnitude of 
    /// the quaternion, followed by the angle of rotation, followed optionally
    /// by the axis of rotation, if there is one. There may not be one in the 
    /// case where the quaternion is a real quaternion.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion,
    /// #     Vector3,
    /// #     Radians,
    /// #     Unit,
    /// # };
    /// # use approx::{
    /// #     relative_eq, 
    /// # };
    /// # use core::f64;
    /// #
    /// let scalar = 3_f64 * (f64::sqrt(3_f64) / 2_f64);
    /// let vector = (3_f64  / 2_f64) * Vector3::unit_z();
    /// let quaternion = Quaternion::from_parts(scalar, vector);
    /// let magnitude = 3_f64;
    /// let angle_over_two = Radians(f64::consts::FRAC_PI_6);
    /// let axis = Unit::from_value(Vector3::unit_z());
    /// let expected = (magnitude, angle_over_two, Some(axis));
    /// let result = quaternion.polar_decomposition();
    ///
    /// assert!(relative_eq!(result.0, expected.0, epsilon = 1e-8));
    /// assert!(relative_eq!(result.1, expected.1, epsilon = 1e-8));
    /// assert!(result.2.is_some());
    /// assert!(relative_eq!(
    ///     result.2.unwrap().as_ref(), 
    ///     expected.2.unwrap().as_ref(), 
    ///     epsilon = 1e-8
    /// ));
    /// ```
    #[inline]
    pub fn polar_decomposition(&self) -> (S, Radians<S>, Option<Unit<Vector3<S>>>) {
        let pair = Unit::try_from_value_with_magnitude(*self, S::zero());
        if let Some((unit_q, magnitude_q)) = pair {
            if let Some(axis) = Unit::try_from_value(self.v, S::zero()) {
                let cos_angle_over_two = unit_q.s.abs();
                let sin_angle_over_two = unit_q.v.magnitude();
                let angle_over_two = Radians::atan2(
                    sin_angle_over_two, 
                    cos_angle_over_two
                );

                (magnitude_q, angle_over_two, Some(axis))
            } else {
                (magnitude_q, Radians::zero(), None)
            }
        } else {
            (S::zero(), Radians::zero(), None)
        }
    }

    /// Construct a quaternion from its polar decomposition.
    ///
    /// Every quaternion can be written in polar form. Let `q` be a quaternion.
    /// It can be written as
    /// ```text
    /// q := |q| * (cos(angle / 2) + sin(angle / 2 * axis))
    ///   == |q| * cos(angle / 2) + |q| * sin(angle / 2) * axis
    ///   =: s + v
    /// ```
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Quaternion,
    /// #     Vector3,
    /// #     Radians,
    /// #     Unit,
    /// # };
    /// #
    /// # use core::f64;
    /// #
    /// let scale = 3_f64;
    /// let angle = Radians(f64::consts::FRAC_PI_3);
    /// let axis = Unit::from_value(Vector3::unit_z());
    /// let scalar = 3_f64 * f64::sqrt(3_f64) / 2_f64;
    /// let vector = Vector3::new(0_f64, 0_f64, 3_f64 / 2_f64);
    /// let expected = Quaternion::from_parts(scalar, vector);
    /// let result = Quaternion::from_polar_decomposition(scale, angle, &axis);
    ///
    /// assert_eq!(result, expected);
    /// ```
    pub fn from_polar_decomposition(
        scale: S, angle: Radians<S>, axis: &Unit<Vector3<S>>) -> Self {

        let two: S = num_traits::cast(2_i8).unwrap();
        let (sin_angle_over_two, cos_angle_over_two) = (angle / two).sin_cos();
        let scalar = cos_angle_over_two * scale;
        let vector = axis.as_ref() * sin_angle_over_two * scale;

        Quaternion::from_parts(scalar, vector)
    }
}

impl<S> AsRef<[S; 4]> for Quaternion<S> {
    #[inline]
    fn as_ref(&self) -> &[S; 4] {
        unsafe { 
            &*(self as *const Quaternion<S> as *const [S; 4])
        }
    }
}

impl<S> AsRef<(S, S, S, S)> for Quaternion<S> {
    #[inline]
    fn as_ref(&self) -> &(S, S, S, S) {
        unsafe { 
            &*(self as *const Quaternion<S> as *const (S, S, S, S))
        }
    }
}

impl<S> AsMut<[S; 4]> for Quaternion<S> {
    #[inline]
    fn as_mut(&mut self) -> &mut [S; 4] {
        unsafe { 
            &mut *(self as *mut Quaternion<S> as *mut [S; 4])
        }
    }
}

impl<S> AsMut<(S, S, S, S)> for Quaternion<S> {
    #[inline]
    fn as_mut(&mut self) -> &mut (S, S, S, S) {
        unsafe { 
            &mut *(self as *mut Quaternion<S> as *mut (S, S, S, S))
        }
    }
}

impl<S> From<Quaternion<S>> for Matrix3x3<S> 
where 
    S: ScalarFloat 
{
    #[inline]
    fn from(quaternion: Quaternion<S>) -> Matrix3x3<S> {
        quaternion.to_matrix3x3()
    }
}

impl<S> From<&Quaternion<S>> for Matrix3x3<S> 
where 
    S: ScalarFloat 
{
    #[inline]
    fn from(quaternion: &Quaternion<S>) -> Matrix3x3<S> {
        quaternion.to_matrix3x3()
    }
}

impl<S> From<Quaternion<S>> for Matrix4x4<S> 
where 
    S: ScalarFloat 
{
    #[inline]
    fn from(quaternion: Quaternion<S>) -> Matrix4x4<S> {
        quaternion.to_matrix4x4()
    }
}

impl<S> From<&Quaternion<S>> for Matrix4x4<S> 
where 
    S: ScalarFloat 
{
    #[inline]
    fn from(quaternion: &Quaternion<S>) -> Matrix4x4<S> {
        quaternion.to_matrix4x4()
    }
}

impl<S> From<[S; 4]> for Quaternion<S> 
where 
    S: Scalar 
{
    #[inline]
    fn from(v: [S; 4]) -> Quaternion<S> {
        Quaternion::new(v[0], v[1], v[2], v[3])
    }
}

impl<'a, S> From<&'a [S; 4]> for &'a Quaternion<S> 
where 
    S: Scalar 
{
    #[inline]
    fn from(v: &'a [S; 4]) -> &'a Quaternion<S> {
        unsafe { 
            &*(v as *const [S; 4] as *const Quaternion<S>)
        }
    }
}

impl<S> From<(S, S, S, S)> for Quaternion<S> 
where 
    S: Scalar 
{
    #[inline]
    fn from(v: (S, S, S, S)) -> Quaternion<S> {
        Quaternion::new(v.0, v.1, v.2, v.3)
    }
}

impl<'a, S> From<&'a (S, S, S, S)> for &'a Quaternion<S> 
where 
    S: Scalar 
{
    #[inline]
    fn from(v: &'a (S, S, S, S)) -> &'a Quaternion<S> {
        unsafe { 
            &*(v as *const (S, S, S, S) as *const Quaternion<S>)
        }
    }
}

impl<S> From<Matrix3x3<S>> for Quaternion<S> 
where 
    S: ScalarFloat 
{
    #[inline]
    fn from(matrix: Matrix3x3<S>) -> Quaternion<S> {
        Self::from_matrix(&matrix)
    }
}

impl<S> From<&Matrix3x3<S>> for Quaternion<S> 
where 
    S: ScalarFloat 
{
    #[inline]
    fn from(matrix: &Matrix3x3<S>) -> Quaternion<S> {
        Self::from_matrix(matrix)
    }
}

impl<S> ops::Index<usize> for Quaternion<S> 
where 
    S: Scalar 
{
    type Output = S;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        let v: &[S; 4] = self.as_ref();
        &v[index]
    }
}

impl<S> ops::Index<ops::Range<usize>> for Quaternion<S> 
where 
    S: Scalar 
{
    type Output = [S];

    #[inline]
    fn index(&self, index: ops::Range<usize>) -> &Self::Output {
        let v: &[S; 4] = self.as_ref();
        &v[index]
    }
}

impl<S> ops::Index<ops::RangeTo<usize>> for Quaternion<S> 
where 
    S: Scalar 
{
    type Output = [S];

    #[inline]
    fn index(&self, index: ops::RangeTo<usize>) -> &Self::Output {
        let v: &[S; 4] = self.as_ref();
        &v[index]
    }
}

impl<S> ops::Index<ops::RangeFrom<usize>> for Quaternion<S> 
where 
    S: Scalar 
{
    type Output = [S];

    #[inline]
    fn index(&self, index: ops::RangeFrom<usize>) -> &Self::Output {
        let v: &[S; 4] = self.as_ref();
        &v[index]
    }
}

impl<S> ops::Index<ops::RangeFull> for Quaternion<S> 
where 
    S: Scalar 
{
    type Output = [S];

    #[inline]
    fn index(&self, index: ops::RangeFull) -> &Self::Output {
        let v: &[S; 4] = self.as_ref();
        &v[index]
    }
}

impl<S> ops::IndexMut<usize> for Quaternion<S> 
where 
    S: Scalar 
{
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut S {
        let v: &mut [S; 4] = self.as_mut();
        &mut v[index]
    }
}

impl<S> ops::IndexMut<ops::Range<usize>> for Quaternion<S> 
where 
    S: Scalar 
{
    #[inline]
    fn index_mut(&mut self, index: ops::Range<usize>) -> &mut [S] {
        let v: &mut [S; 4] = self.as_mut();
        &mut v[index]
    }
}

impl<S> ops::IndexMut<ops::RangeTo<usize>> for Quaternion<S> 
where 
    S: Scalar 
{
    #[inline]
    fn index_mut(&mut self, index: ops::RangeTo<usize>) -> &mut [S] {
        let v: &mut [S; 4] = self.as_mut();
        &mut v[index]
    }
}

impl<S> ops::IndexMut<ops::RangeFrom<usize>> for Quaternion<S> 
where 
    S: Scalar 
{
    #[inline]
    fn index_mut(&mut self, index: ops::RangeFrom<usize>) -> &mut [S] {
        let v: &mut [S; 4] = self.as_mut();
        &mut v[index]
    }
}

impl<S> ops::IndexMut<ops::RangeFull> for Quaternion<S> 
where 
    S: Scalar 
{
    #[inline]
    fn index_mut(&mut self, index: ops::RangeFull) -> &mut [S] {
        let v: &mut [S; 4] = self.as_mut();
        &mut v[index]
    }
}

impl<S> fmt::Display for Quaternion<S> 
where 
    S: fmt::Display 
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            formatter, 
            "Quaternion [s: {}, v: [{}, {}, {}]]", 
            self.s, self.v[0], self.v[1], self.v[2]
        )
    }
}

impl<S> ops::Neg for Quaternion<S> 
where 
    S: ScalarSigned 
{
    type Output = Quaternion<S>;

    #[inline]
    fn neg(self) -> Self::Output {
        Quaternion::from_parts(-self.s, -self.v)
    }
}

impl<'a, S> ops::Neg for &'a Quaternion<S> 
where 
    S: ScalarSigned 
{
    type Output = Quaternion<S>;

    #[inline]
    fn neg(self) -> Self::Output {
        Quaternion::from_parts(-self.s, -self.v)
    }
}

impl<S> ops::Add<Quaternion<S>> for Quaternion<S> 
where 
    S: Scalar 
{
    type Output = Quaternion<S>;

    #[inline]
    fn add(self, other: Quaternion<S>) -> Self::Output {
        Quaternion::from_parts(self.s + other.s, self.v + other.v)
    }
}

impl<'a, S> ops::Add<Quaternion<S>> for &'a Quaternion<S> 
where 
    S: Scalar 
{
    type Output = Quaternion<S>;

    #[inline]
    fn add(self, other: Quaternion<S>) -> Self::Output {
        Quaternion::from_parts(self.s + other.s, self.v + other.v)
    }
}

impl<'a, S> ops::Add<&'a Quaternion<S>> for Quaternion<S> 
where 
    S: Scalar 
{
    type Output = Quaternion<S>;

    #[inline]
    fn add(self, other: &'a Quaternion<S>) -> Self::Output {
        Quaternion::from_parts(self.s + other.s, self.v + other.v)
    }
}

impl<'a, 'b, S> ops::Add<&'a Quaternion<S>> for &'b Quaternion<S> 
where 
    S: Scalar 
{
    type Output = Quaternion<S>;

    #[inline]
    fn add(self, other: &'a Quaternion<S>) -> Self::Output {
        Quaternion::from_parts(self.s + other.s, self.v + other.v)
    }
}

impl<S> ops::Sub<Quaternion<S>> for Quaternion<S> 
where 
    S: Scalar 
{
    type Output = Quaternion<S>;

    #[inline]
    fn sub(self, other: Quaternion<S>) -> Self::Output {
        Quaternion::from_parts(self.s - other.s, self.v - other.v)
    }
}

impl<'a, S> ops::Sub<Quaternion<S>> for &'a Quaternion<S> 
where 
    S: Scalar 
{
    type Output = Quaternion<S>;

    #[inline]
    fn sub(self, other: Quaternion<S>) -> Self::Output {
        Quaternion::from_parts(self.s - other.s, self.v - other.v)
    }
}

impl<'a, S> ops::Sub<&'a Quaternion<S>> for Quaternion<S> 
where 
    S: Scalar 
{
    type Output = Quaternion<S>;

    #[inline]
    fn sub(self, other: &'a Quaternion<S>) -> Self::Output {
        Quaternion::from_parts(self.s - other.s, self.v - other.v)
    }
}

impl<'a, 'b, S> ops::Sub<&'a Quaternion<S>> for &'b Quaternion<S> 
where 
    S: Scalar 
{
    type Output = Quaternion<S>;

    #[inline]
    fn sub(self, other: &'a Quaternion<S>) -> Self::Output {
        Quaternion::from_parts(self.s - other.s, self.v - other.v)
    }
}

impl<S> ops::Mul<S> for Quaternion<S> 
where 
    S: Scalar 
{
    type Output = Quaternion<S>;

    #[inline]
    fn mul(self, other: S) -> Quaternion<S> {
        Quaternion::new(
            self.s * other,
            self.v.x * other, self.v.y * other, self.v.z * other,
        )
    }
}

impl<S> ops::Mul<S> for &Quaternion<S> 
where 
    S: Scalar 
{
    type Output = Quaternion<S>;

    #[rustfmt::skip]
    #[inline]
    fn mul(self, other: S) -> Quaternion<S> {
        Quaternion::new(
            self.s * other,
            self.v.x * other, self.v.y * other, self.v.z * other,
        )
    }
}

impl<'a, S> ops::Mul<Quaternion<S>> for Quaternion<S> 
where 
    S: Scalar 
{
    type Output = Quaternion<S>;

    #[rustfmt::skip]
    #[inline]
    fn mul(self, other: Quaternion<S>) -> Self::Output {
        Quaternion::new(
            other.s * self.s   - other.v.x * self.v.x - other.v.y * self.v.y - other.v.z * self.v.z,
            other.s * self.v.x + other.v.x * self.s   - other.v.y * self.v.z + other.v.z * self.v.y,
            other.s * self.v.y + other.v.x * self.v.z + other.v.y * self.s   - other.v.z * self.v.x,
            other.s * self.v.z - other.v.x * self.v.y + other.v.y * self.v.x + other.v.z * self.s,
        )
    }
}

impl<'a, S> ops::Mul<&'a Quaternion<S>> for Quaternion<S> 
where 
    S: Scalar 
{
    type Output = Quaternion<S>;

    #[rustfmt::skip]
    #[inline]
    fn mul(self, other: &'a Quaternion<S>) -> Self::Output {
        Quaternion::new(
            other.s * self.s   - other.v.x * self.v.x - other.v.y * self.v.y - other.v.z * self.v.z,
            other.s * self.v.x + other.v.x * self.s   - other.v.y * self.v.z + other.v.z * self.v.y,
            other.s * self.v.y + other.v.x * self.v.z + other.v.y * self.s   - other.v.z * self.v.x,
            other.s * self.v.z - other.v.x * self.v.y + other.v.y * self.v.x + other.v.z * self.s,
        )
    }
}

impl<'a, S> ops::Mul<Quaternion<S>> for &'a Quaternion<S> 
where 
    S: Scalar 
{
    type Output = Quaternion<S>;

    #[rustfmt::skip]
    #[inline]
    fn mul(self, other: Quaternion<S>) -> Self::Output {
        Quaternion::new(
            other.s * self.s   - other.v.x * self.v.x - other.v.y * self.v.y - other.v.z * self.v.z,
            other.s * self.v.x + other.v.x * self.s   - other.v.y * self.v.z + other.v.z * self.v.y,
            other.s * self.v.y + other.v.x * self.v.z + other.v.y * self.s   - other.v.z * self.v.x,
            other.s * self.v.z - other.v.x * self.v.y + other.v.y * self.v.x + other.v.z * self.s,
        )
    }
}

impl<'a, 'b, S> ops::Mul<&'a Quaternion<S>> for &'b Quaternion<S> 
where 
    S: Scalar 
{
    type Output = Quaternion<S>;

    #[rustfmt::skip]
    #[inline]
    fn mul(self, other: &'a Quaternion<S>) -> Self::Output {
        Quaternion::new(
            other.s * self.s   - other.v.x * self.v.x - other.v.y * self.v.y - other.v.z * self.v.z,
            other.s * self.v.x + other.v.x * self.s   - other.v.y * self.v.z + other.v.z * self.v.y,
            other.s * self.v.y + other.v.x * self.v.z + other.v.y * self.s   - other.v.z * self.v.x,
            other.s * self.v.z - other.v.x * self.v.y + other.v.y * self.v.x + other.v.z * self.s,
        )
    }
}

macro_rules! impl_scalar_quaternion_mul_ops {
    ($Lhs:ty, { $scalar:ident, { $($field:ident),* } }) => {
        impl ops::Mul<Quaternion<$Lhs>> for $Lhs {
            type Output = Quaternion<$Lhs>;

            #[inline]
            fn mul(self, other: Quaternion<$Lhs>) -> Self::Output {
                Self::Output::new( self * other.$scalar, $(self * other.v.$field),*)
            }
        }

        impl ops::Mul<&Quaternion<$Lhs>> for $Lhs {
            type Output = Quaternion<$Lhs>;

            #[inline]
            fn mul(self, other: &Quaternion<$Lhs>) -> Self::Output {
                Self::Output::new( self * other.$scalar, $(self * other.v.$field),*)
            }
        }

        impl ops::Mul<Quaternion<$Lhs>> for &$Lhs {
            type Output = Quaternion<$Lhs>;

            #[inline]
            fn mul(self, other: Quaternion<$Lhs>) -> Self::Output {
                Self::Output::new( self * other.$scalar, $(self * other.v.$field),*)
            }
        }

        impl<'a, 'b> ops::Mul<&'a Quaternion<$Lhs>> for &'b $Lhs {
            type Output = Quaternion<$Lhs>;

            #[inline]
            fn mul(self, other: &'a Quaternion<$Lhs>) -> Self::Output {
                Self::Output::new( self * other.$scalar, $(self * other.v.$field),*)
            }
        }
    }
}

impl_scalar_quaternion_mul_ops!(u8,    { s, { x, y, z } });
impl_scalar_quaternion_mul_ops!(u16,   { s, { x, y, z } });
impl_scalar_quaternion_mul_ops!(u32,   { s, { x, y, z } });
impl_scalar_quaternion_mul_ops!(u64,   { s, { x, y, z } });
impl_scalar_quaternion_mul_ops!(u128,  { s, { x, y, z } });
impl_scalar_quaternion_mul_ops!(usize, { s, { x, y, z } });
impl_scalar_quaternion_mul_ops!(i8,    { s, { x, y, z } });
impl_scalar_quaternion_mul_ops!(i16,   { s, { x, y, z } });
impl_scalar_quaternion_mul_ops!(i32,   { s, { x, y, z } });
impl_scalar_quaternion_mul_ops!(i64,   { s, { x, y, z } });
impl_scalar_quaternion_mul_ops!(i128,  { s, { x, y, z } });
impl_scalar_quaternion_mul_ops!(isize, { s, { x, y, z } });
impl_scalar_quaternion_mul_ops!(f32,   { s, { x, y, z } });
impl_scalar_quaternion_mul_ops!(f64,   { s, { x, y, z } });


impl<S> ops::Div<S> for Quaternion<S> 
where 
    S: Scalar 
{
    type Output = Quaternion<S>;

    #[rustfmt::skip]
    #[inline]
    fn div(self, other: S) -> Quaternion<S> {
        Quaternion::new(
            self.s / other, 
            self.v.x / other, self.v.y / other, self.v.z / other,
        )
    }
}

impl<'a, S> ops::Div<S> for &'a Quaternion<S> 
where 
    S: Scalar 
{
    type Output = Quaternion<S>;

    #[rustfmt::skip]
    #[inline]
    fn div(self, other: S) -> Quaternion<S> {
        Quaternion::new(
            self.s / other, 
            self.v.x / other, self.v.y / other, self.v.z / other,
        )
    }
}

impl<S> ops::Rem<S> for Quaternion<S> 
where 
    S: Scalar 
{
    type Output = Quaternion<S>;

    #[rustfmt::skip]
    #[inline]
    fn rem(self, other: S) -> Self::Output {
        Quaternion::new(
            self.s % other,
            self.v.x % other, self.v.y % other, self.v.z % other,
        )
    }
}

impl<S> ops::Rem<S> for &Quaternion<S> 
where 
    S: Scalar 
{
    type Output = Quaternion<S>;

    #[rustfmt::skip]
    #[inline]
    fn rem(self, other: S) -> Self::Output {
        Quaternion::new(
            self.s % other,
            self.v.x % other, self.v.y % other, self.v.z % other,
        )
    }
}

impl<S> ops::AddAssign<Quaternion<S>> for Quaternion<S> 
where 
    S: Scalar 
{
    #[inline]
    fn add_assign(&mut self, other: Quaternion<S>) {
        self.s += other.s;
        self.v += other.v;
    }
}

impl<S> ops::AddAssign<&Quaternion<S>> for Quaternion<S> 
where 
    S: Scalar 
{
    #[inline]
    fn add_assign(&mut self, other: &Quaternion<S>) {
        self.s += other.s;
        self.v += other.v;
    }
}

impl<S> ops::SubAssign<Quaternion<S>> for Quaternion<S> 
where 
    S: Scalar 
{
    #[inline]
    fn sub_assign(&mut self, other: Quaternion<S>) {
        self.s -= other.s;
        self.v -= other.v;
    }
}

impl<S> ops::SubAssign<&Quaternion<S>> for Quaternion<S> 
where 
    S: Scalar 
{
    #[inline]
    fn sub_assign(&mut self, other: &Quaternion<S>) {
        self.s -= other.s;
        self.v -= other.v;
    }
}

impl<S> ops::MulAssign<S> for Quaternion<S> 
where 
    S: Scalar 
{
    #[inline]
    fn mul_assign(&mut self, other: S) {
        self.s *= other;
        self.v *= other;
    }
}

impl<S> ops::DivAssign<S> for Quaternion<S> 
where 
    S: Scalar 
{
    #[inline]
    fn div_assign(&mut self, other: S) {
        self.s /= other;
        self.v /= other;
    }
}

impl<S> ops::RemAssign<S> for Quaternion<S> 
where 
    S: Scalar 
{
    #[inline]
    fn rem_assign(&mut self, other: S) {
        self.s %= other;
        self.v %= other;
    }
}

impl<S> Magnitude for Quaternion<S> 
where 
    S: ScalarFloat 
{
    type Output = S;

    #[inline]
    fn magnitude_squared(&self) -> Self::Output {
        self.dot(self)
    }

    #[inline]
    fn magnitude(&self) -> Self::Output {
        self.magnitude_squared().sqrt()
    }

    #[inline]
    fn normalize(&self) -> Self {
        self / self.magnitude()
    }

    #[inline]
    fn normalize_to(&self, magnitude: Self::Output) -> Self {
        self * (magnitude / self.magnitude())
    }

    #[inline]
    fn try_normalize(&self, threshold: Self::Output) -> Option<Self> {
        let magnitude = self.magnitude();

        if magnitude <= threshold {
            None
        } else {
            Some(self.normalize())
        }
    }

    #[inline]
    fn distance_squared(&self, other: &Quaternion<S>) -> S {
        (self - other).magnitude_squared()
    }

    #[inline]
    fn distance(&self, other: &Self) -> Self::Output {
        self.distance_squared(other).sqrt()
    }
}

impl<S> approx::AbsDiffEq for Quaternion<S> 
where 
    S: ScalarFloat 
{
    type Epsilon = <S as approx::AbsDiffEq>::Epsilon;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        S::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        S::abs_diff_eq(&self.s, &other.s, epsilon) &&
        Vector3::abs_diff_eq(&self.v, &other.v, epsilon)
    }
}

impl<S> approx::RelativeEq for Quaternion<S> 
where 
    S: ScalarFloat 
{
    #[inline]
    fn default_max_relative() -> S::Epsilon {
        S::default_max_relative()
    }

    #[inline]
    fn relative_eq(&self, other: &Self, epsilon: S::Epsilon, max_relative: S::Epsilon) -> bool {
        S::relative_eq(&self.s, &other.s, epsilon, max_relative) &&
        Vector3::relative_eq(&self.v, &other.v, epsilon, max_relative)
    }
}

impl<S> approx::UlpsEq for Quaternion<S> 
where 
    S: ScalarFloat
{
    #[inline]
    fn default_max_ulps() -> u32 {
        S::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: S::Epsilon, max_ulps: u32) -> bool {
        S::ulps_eq(&self.s, &other.s, epsilon, max_ulps) &&
        Vector3::ulps_eq(&self.v, &other.v, epsilon, max_ulps)
    }
}

