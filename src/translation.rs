use crate::base::{
    ScalarSigned,
    ScalarFloat,
};
use crate::matrix::{
    Matrix3x3,
    Matrix4x4,
};
use crate::vector::{
    Vector2,
    Vector3,
};
use crate::point::{
    Point2,
    Point3,
};
use crate::transform::{
    Transform2,
    Transform3,
};

use core::fmt;
use core::ops;


/// A translation transformation in two dimensions.
///
/// A translation is an operation that creates displacement motions. 
/// In a Euclidean setting, translations preserve differences between two points 
/// and acts as the identity on vectors.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Translation2<S> {
    /// The vector along which a vector or point is displaced.
    pub(crate) vector: Vector2<S>,
}

impl<S> Translation2<S> 
where 
    S: ScalarSigned 
{
    /// Construct a translation from the components of the translation.
    #[inline]
    pub fn new(x: S, y: S) -> Translation2<S> {
        Translation2 {
            vector: Vector2::new(x, y)
        }
    }

    /// Construct a translation operator from a vector of displacements.
    #[inline]
    pub fn from_vector(vector: &Vector2<S>) -> Translation2<S> {
        Translation2 {
            vector: *vector,
        }
    }

    /// Construct a translation between two vectors.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Translation2,
    /// #     Vector2,
    /// #     Point2,
    /// # };
    /// #
    /// let vector1 = Vector2::new(1_f64, 2_f64);
    /// let vector2 = Vector2::new(3_f64, 4_f64);
    /// let translation = Translation2::between_vectors(&vector1, &vector2);
    /// let point = Point2::new(0_f64, 0_f64);
    /// let expected = Point2::new(2_f64, 2_f64);
    /// let result = translation.translate_point(&point);
    ///
    /// assert_eq!(result, expected);
    /// ```
    #[inline]
    pub fn between_vectors(vector1: &Vector2<S>, vector2: &Vector2<S>) -> Self {
        let distance = vector2 - vector1;

        Translation2::from_vector(&distance)
    }

    /// Construct a translation between two points.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Translation2,
    /// #     Point2,
    /// # };
    /// #
    /// let point1 = Point2::new(1_f64, 2_f64);
    /// let point2 = Point2::new(3_f64, 4_f64);
    /// let translation = Translation2::between_points(&point1, &point2);
    /// let point = Point2::new(0_f64, 0_f64);
    /// let expected = Point2::new(2_f64, 2_f64);
    /// let result = translation.translate_point(&point);
    ///
    /// assert_eq!(result, expected);
    /// ```
    #[inline]
    pub fn between_points(point1: &Point2<S>, point2: &Point2<S>) -> Self {
        let distance = point2 - point1;

        Translation2::from_vector(&distance)
    }

    /// Construct a translation that translates a vector or point in the opposite
    /// direction of the translation applied by `self`.
    ///
    /// If `self` is a translation of a vector by a displacement `distance`, then its
    /// inverse will be a translation by a displacement `-distance`.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Translation2,
    /// #     Vector2, 
    /// # };
    /// #
    /// let distance = Vector2::new(1_f64, 2_f64);
    /// let translation = Translation2::from_vector(&distance);
    /// let expected = Translation2::from_vector(&(-distance));
    /// let result = translation.inverse();
    ///
    /// assert_eq!(result, expected);
    /// ```
    #[inline]
    pub fn inverse(&self) -> Self {
        Translation2::from_vector(&(-self.vector))
    }

    /// Mutably invert a translation in place.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Translation2,
    /// #     Vector2, 
    /// # };
    /// #
    /// let mut result = Translation2::new(1_f64, 2_f64);
    /// let expected = Translation2::new(-1_f64, -2_f64);
    /// result.inverse_mut();
    ///
    /// assert_eq!(result, expected);
    /// ```
    #[inline]
    pub fn inverse_mut(&mut self) {
        self.vector.neg_mut();
    }
    
    /// Apply the translation operation to a point.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Translation2,
    /// #     Vector2,
    /// #     Point2,
    /// # };
    /// #
    /// let distance = Vector2::new(4_f64, 8_f64);
    /// let translation = Translation2::from_vector(&distance);
    /// let point = Point2::new(0_f64, 0_f64);
    /// let expected = Point2::new(4_f64, 8_f64);
    /// let result = translation.translate_point(&point);
    ///
    /// assert_eq!(result, expected);
    /// ```
    #[inline]
    pub fn translate_point(&self, point: &Point2<S>) -> Point2<S> {
        point + self.vector
    }

    /// Apply the translation operation to a vector. 
    ///
    /// This should act as the identity since vectors represent differences 
    /// between points. Let `p1` and `p2` be points and let `v = p2 - p1` 
    /// be their difference. If we translate each point by a vector `a`, 
    /// then `(p2 + a) - (p1 + a) = p2 - p1 = v`.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Translation2,
    /// #     Vector2,
    /// # };
    /// #
    /// let distance = Vector2::new(4_f64, 8_f64);
    /// let translation = Translation2::from_vector(&distance);
    /// let vector = Vector2::new(0_f64, 0_f64);
    /// let expected = vector;
    /// let result = translation.translate_vector(&vector);
    ///
    /// assert_eq!(result, expected);
    /// ```
    #[inline]
    pub fn translate_vector(&self, vector: &Vector2<S>) -> Vector2<S> {
        *vector
    }

    /// Apply the inverse translation to a point.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Translation2,
    /// #     Point2,
    /// #     Vector2, 
    /// # };
    /// #
    /// let distance = Vector2::new(13_f64, 30_f64);
    /// let translation = Translation2::from_vector(&distance);
    /// let translation_inv = translation.inverse();
    /// let point = Point2::new(1_f64, 2_f64);
    /// let expected = translation_inv.translate_point(&point);
    /// let result = translation.inverse_translate_point(&point);
    ///
    /// assert_eq!(result, expected);
    /// ```
    #[inline]
    pub fn inverse_translate_point(&self, point: &Point2<S>) -> Point2<S> {
        point - self.vector
    }

    /// Apply the inverse translation to a vector.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Translation2,
    /// #     Vector2, 
    /// # };
    /// #
    /// let distance = Vector2::new(13_f64, 30_f64);
    /// let translation = Translation2::from_vector(&distance);
    /// let vector = Vector2::new(1_f64, 2_f64);
    ///
    /// assert_eq!(translation.inverse_translate_vector(&vector), vector);
    /// ```
    #[inline]
    pub fn inverse_translate_vector(&self, vector: &Vector2<S>) -> Vector2<S> {
        *vector
    }

    /// The identity transformation for translations, which displaces
    /// a vector or point zero distance.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Translation2,
    /// #     Point2, 
    /// # };
    /// #
    /// let translation = Translation2::identity();
    /// let point = Point2::new(1_f64, 2_f64);
    /// 
    /// assert_eq!(translation.translate_point(&point), point);
    /// ```
    #[inline]
    pub fn identity() -> Translation2<S> {
        Translation2 { 
            vector: Vector2::zero(),
        }
    }

    /// Convert a translation into a generic two-dimensional transformation.
    #[inline]
    pub fn to_transform(&self) -> Transform2<S> {
        Transform2::from_specialized(self)
    }
}

impl<S> AsRef<Vector2<S>> for Translation2<S> {
    #[inline]
    fn as_ref(&self) -> &Vector2<S> {
        &self.vector
    }
}

impl<S> fmt::Display for Translation2<S> 
where 
    S: fmt::Display 
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter, 
            "Translation2 [x={}, y={}]", 
            self.vector[0], self.vector[1]
        )
    }
}

impl<S> From<Translation2<S>> for Matrix3x3<S> 
where 
    S: ScalarSigned 
{
    #[inline]
    fn from(transform: Translation2<S>) -> Matrix3x3<S> {
        Matrix3x3::from_affine_translation(&transform.vector)
    }
}

impl<S> From<&Translation2<S>> for Matrix3x3<S> 
where 
    S: ScalarSigned 
{
    #[inline]
    fn from(transform: &Translation2<S>) -> Matrix3x3<S> {
        Matrix3x3::from_affine_translation(&transform.vector)
    }
}

impl<S> approx::AbsDiffEq for Translation2<S> 
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
        Vector2::abs_diff_eq(&self.vector, &other.vector, epsilon)
    }
}

impl<S> approx::RelativeEq for Translation2<S> 
where 
    S: ScalarFloat 
{
    #[inline]
    fn default_max_relative() -> S::Epsilon {
        S::default_max_relative()
    }

    #[inline]
    fn relative_eq(&self, other: &Self, epsilon: S::Epsilon, max_relative: S::Epsilon) -> bool {
        Vector2::relative_eq(&self.vector, &other.vector, epsilon, max_relative)
    }
}

impl<S> approx::UlpsEq for Translation2<S> 
where 
    S: ScalarFloat 
{
    #[inline]
    fn default_max_ulps() -> u32 {
        S::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: S::Epsilon, max_ulps: u32) -> bool {
        Vector2::ulps_eq(&self.vector, &other.vector, epsilon, max_ulps)
    }
}

impl<S> ops::Mul<Point2<S>> for Translation2<S> 
where 
    S: ScalarFloat 
{
    type Output = Point2<S>;

    #[inline]
    fn mul(self, other: Point2<S>) -> Self::Output {
        self.translate_point(&other)
    }
}

impl<S> ops::Mul<&Point2<S>> for Translation2<S> 
where 
    S: ScalarFloat 
{
    type Output = Point2<S>;

    #[inline]
    fn mul(self, other: &Point2<S>) -> Self::Output {
        self.translate_point(other)
    }
}

impl<S> ops::Mul<Point2<S>> for &Translation2<S> 
where 
    S: ScalarFloat 
{
    type Output = Point2<S>;

    #[inline]
    fn mul(self, other: Point2<S>) -> Self::Output {
        self.translate_point(&other)
    }
}

impl<'a, 'b, S> ops::Mul<&'a Point2<S>> for &'b Translation2<S> 
where 
    S: ScalarFloat 
{
    type Output = Point2<S>;

    #[inline]
    fn mul(self, other: &'a Point2<S>) -> Self::Output {
        self.translate_point(other)
    }
}

impl<S> ops::Mul<Translation2<S>> for Translation2<S> 
where 
    S: ScalarFloat 
{
    type Output = Translation2<S>;

    #[inline]
    fn mul(self, other: Translation2<S>) -> Self::Output {
        Translation2::from_vector(&(self.vector + other.vector))
    }
}

impl<S> ops::Mul<&Translation2<S>> for Translation2<S> 
where 
    S: ScalarFloat 
{
    type Output = Translation2<S>;

    #[inline]
    fn mul(self, other: &Translation2<S>) -> Self::Output {
        Translation2::from_vector(&(self.vector + other.vector))
    }
}

impl<S> ops::Mul<Translation2<S>> for &Translation2<S> 
where 
    S: ScalarFloat 
{
    type Output = Translation2<S>;

    #[inline]
    fn mul(self, other: Translation2<S>) -> Self::Output {
        Translation2::from_vector(&(self.vector + other.vector))
    }
}

impl<'a, 'b, S> ops::Mul<&'a Translation2<S>> for &'b Translation2<S> 
where 
    S: ScalarFloat 
{
    type Output = Translation2<S>;

    #[inline]
    fn mul(self, other: &'a Translation2<S>) -> Self::Output {
        Translation2::from_vector(&(self.vector + other.vector))
    }
}



/// A translation transformation in three dimensions.
///
/// A translation is an operation that creates displacement motions. 
/// In a Euclidean setting, translations preserve differences between two points 
/// and acts as the identity on vectors.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Translation3<S> {
    pub(crate) vector: Vector3<S>,
}

impl<S> Translation3<S> 
where 
    S: ScalarSigned 
{
    /// Construct a translation from the components of the translation.
    #[inline]
    pub fn new(x: S, y: S, z: S) -> Translation3<S> {
        Translation3 {
            vector: Vector3::new(x, y, z)
        }
    }

    /// Construct a translation operator from a vector of displacements.
    pub fn from_vector(vector: &Vector3<S>) -> Translation3<S> {
        Translation3 {
            vector: *vector,
        }
    }

    /// Construct a translation between two vectors.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Translation3,
    /// #     Vector3,
    /// #     Point3,
    /// # };
    /// #
    /// let vector1 = Vector3::new(1_f64, 2_f64, 3_f64);
    /// let vector2 = Vector3::new(7_f64, 9_f64, 11_f64);
    /// let translation = Translation3::between_vectors(&vector1, &vector2);
    /// let point = Point3::new(0_f64, 0_f64, 0_f64);
    /// let expected = Point3::new(6_f64, 7_f64, 8_f64);
    /// let result = translation.translate_point(&point);
    ///
    /// assert_eq!(result, expected);
    /// ```
    #[inline]
    pub fn between_vectors(vector1: &Vector3<S>, vector2: &Vector3<S>) -> Self {
        let distance = vector2 - vector1;

        Translation3::from_vector(&distance)
    }

    /// Construct a translation between two points.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Translation3,
    /// #     Point3,
    /// # };
    /// #
    /// let point1 = Point3::new(1_f64, 2_f64, 3_f64);
    /// let point2 = Point3::new(7_f64, 9_f64, 11_f64);
    /// let translation = Translation3::between_points(&point1, &point2);
    /// let point = Point3::new(0_f64, 0_f64, 0_f64);
    /// let expected = Point3::new(6_f64, 7_f64, 8_f64);
    /// let result = translation.translate_point(&point);
    ///
    /// assert_eq!(result, expected);
    /// ```
    #[inline]
    pub fn between_points(point1: &Point3<S>, point2: &Point3<S>) -> Self {
        let distance = point2 - point1;

        Translation3::from_vector(&distance)
    }

    /// Construct a translation that translates a vector or point in the opposite
    /// direction of the translation applied by `self`.
    ///
    /// If `self` is a translation of a vector by a displacement `distance`, then its
    /// inverse will be a translation by a displacement `-distance`.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Translation3,
    /// #     Vector3, 
    /// # };
    /// #
    /// let distance = Vector3::new(1_f64, 2_f64, 3_f64);
    /// let translation = Translation3::from_vector(&distance);
    /// let expected = Translation3::from_vector(&(-distance));
    /// let result = translation.inverse();
    ///
    /// assert_eq!(result, expected);
    /// ```
    #[inline]
    pub fn inverse(&self) -> Self {
        Translation3::from_vector(&(-self.vector))
    }

    /// Mutably invert a translation in place.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Translation3,
    /// #     Vector3, 
    /// # };
    /// #
    /// let mut result = Translation3::new(1_f64, 2_f64, 3_f64);
    /// let expected = Translation3::new(-1_f64, -2_f64, -3_f64);
    /// result.inverse_mut();
    ///
    /// assert_eq!(result, expected);
    /// ```
    #[inline]
    pub fn inverse_mut(&mut self) {
        self.vector.neg_mut();
    }
    
    /// Apply the translation transformation to a point.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Translation3,
    /// #     Point3,
    /// #     Vector3,
    /// # };
    /// #
    /// let distance = Vector3::new(1_f64, 2_f64, 3_f64);
    /// let translation = Translation3::from_vector(&distance);
    /// let point = Point3::new(0_f64, 0_f64, 0_f64);
    /// let expected = Point3::new(1_f64, 2_f64, 3_f64);
    /// let result = translation.translate_point(&point);
    ///
    /// assert_eq!(result, expected);
    /// ```
    #[inline]
    pub fn translate_point(&self, point: &Point3<S>) -> Point3<S> {
        point + self.vector
    }

    /// Apply the translation transformation to a vector.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Translation3,
    /// #     Vector3,
    /// # };
    /// #
    /// let distance = Vector3::new(100_f64, 200_f64, 300_f64);
    /// let translation = Translation3::from_vector(&distance);
    /// let vector = Vector3::new(0_f64, 0_f64, 0_f64);
    /// let expected = vector;
    /// let result = translation.translate_vector(&vector);
    ///
    /// assert_eq!(result, expected);
    /// ```
    #[inline]
    pub fn translate_vector(&self, vector: &Vector3<S>) -> Vector3<S> {
        *vector
    }

    /// Apply the inverse of the translation to a point.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Translation3,
    /// #     Point3,
    /// #     Vector3,
    /// # };
    /// #
    /// let distance = Vector3::new(1_f64, 2_f64, 3_f64);
    /// let translation = Translation3::from_vector(&distance);
    /// let point = Point3::new(0_f64, 0_f64, 0_f64);
    /// let expected = point;
    /// let translated_point = translation.translate_point(&point);
    /// let result = translation.inverse_translate_point(&translated_point);
    ///
    /// assert_eq!(result, expected);
    /// ```
    #[inline]
    pub fn inverse_translate_point(&self, point: &Point3<S>) -> Point3<S> {
        point - self.vector
    }

    /// Apply the inverse of the translation to a vector.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Translation3,
    /// #     Vector3,
    /// # };
    /// #
    /// let distance = Vector3::new(100_f64, 200_f64, 300_f64);
    /// let translation = Translation3::from_vector(&distance);
    /// let vector = Vector3::new(0_f64, 0_f64, 0_f64);
    /// let expected = vector;
    /// let result = translation.inverse_translate_vector(&vector);
    ///
    /// assert_eq!(result, expected);
    /// ```
    #[inline]
    pub fn inverse_translate_vector(&self, vector: &Vector3<S>) -> Vector3<S> {
        *vector
    }

    /// The identity transformation for translations, which displaces
    /// a vector or point zero distance.
    ///
    /// ## Example
    ///
    /// ```
    /// # use cglinalg::{
    /// #     Translation3,
    /// #     Point3, 
    /// # };
    /// #
    /// let translation = Translation3::identity();
    /// let point = Point3::new(1_f64, 2_f64, 3_f64);
    /// 
    /// assert_eq!(translation.translate_point(&point), point);
    /// ```
    #[inline]
    pub fn identity() -> Translation3<S> {
        Translation3 { 
            vector: Vector3::zero(),
        }
    }

    /// Convert a translation to a generic transformation.
    #[inline]
    pub fn to_transform(&self) -> Transform3<S> {
        Transform3::from_specialized(self)
    }
}


impl<S> AsRef<Vector3<S>> for Translation3<S> {
    #[inline]
    fn as_ref(&self) -> &Vector3<S> {
        &self.vector
    }
}

impl<S> fmt::Display for Translation3<S> 
where 
    S: fmt::Display 
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter, 
            "Translation3 [x-{}, y={}, z={}]", 
            self.vector[0], self.vector[1], self.vector[2]
        )
    }
}

impl<S> From<Translation3<S>> for Matrix4x4<S> 
where 
    S: ScalarSigned 
{
    #[inline]
    fn from(transform: Translation3<S>) -> Matrix4x4<S> {
        Matrix4x4::from_affine_translation(&transform.vector)
    }
}

impl<S> From<&Translation3<S>> for Matrix4x4<S> 
where 
    S: ScalarSigned 
{
    #[inline]
    fn from(transform: &Translation3<S>) -> Matrix4x4<S> {
        Matrix4x4::from_affine_translation(&transform.vector)
    }
}

impl<S> approx::AbsDiffEq for Translation3<S> 
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
        Vector3::abs_diff_eq(&self.vector, &other.vector, epsilon)
    }
}

impl<S> approx::RelativeEq for Translation3<S> 
where 
    S: ScalarFloat 
{
    #[inline]
    fn default_max_relative() -> S::Epsilon {
        S::default_max_relative()
    }

    #[inline]
    fn relative_eq(&self, other: &Self, epsilon: S::Epsilon, max_relative: S::Epsilon) -> bool {
        Vector3::relative_eq(&self.vector, &other.vector, epsilon, max_relative)
    }
}

impl<S> approx::UlpsEq for Translation3<S> 
where 
    S: ScalarFloat 
{
    #[inline]
    fn default_max_ulps() -> u32 {
        S::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: S::Epsilon, max_ulps: u32) -> bool {
        Vector3::ulps_eq(&self.vector, &other.vector, epsilon, max_ulps)
    }
}

impl<S> ops::Mul<Point3<S>> for Translation3<S> 
where 
    S: ScalarFloat 
{
    type Output = Point3<S>;

    #[inline]
    fn mul(self, other: Point3<S>) -> Self::Output {
        self.translate_point(&other)
    }
}

impl<S> ops::Mul<&Point3<S>> for Translation3<S> 
where 
    S: ScalarFloat 
{
    type Output = Point3<S>;

    #[inline]
    fn mul(self, other: &Point3<S>) -> Self::Output {
        self.translate_point(other)
    }
}

impl<S> ops::Mul<Point3<S>> for &Translation3<S> 
where 
    S: ScalarFloat 
{
    type Output = Point3<S>;

    #[inline]
    fn mul(self, other: Point3<S>) -> Self::Output {
        self.translate_point(&other)
    }
}

impl<'a, 'b, S> ops::Mul<&'a Point3<S>> for &'b Translation3<S> 
where 
    S: ScalarFloat 
{
    type Output = Point3<S>;

    #[inline]
    fn mul(self, other: &'a Point3<S>) -> Self::Output {
        self.translate_point(other)
    }
}

impl<S> ops::Mul<Translation3<S>> for Translation3<S> 
where 
    S: ScalarFloat 
{
    type Output = Translation3<S>;

    #[inline]
    fn mul(self, other: Translation3<S>) -> Self::Output {
        Translation3::from_vector(&(self.vector + other.vector))
    }
}

impl<S> ops::Mul<&Translation3<S>> for Translation3<S> 
where 
    S: ScalarFloat 
{
    type Output = Translation3<S>;

    #[inline]
    fn mul(self, other: &Translation3<S>) -> Self::Output {
        Translation3::from_vector(&(self.vector + other.vector))
    }
}

impl<S> ops::Mul<Translation3<S>> for &Translation3<S> 
where 
    S: ScalarFloat 
{
    type Output = Translation3<S>;

    #[inline]
    fn mul(self, other: Translation3<S>) -> Self::Output {
        Translation3::from_vector(&(self.vector + other.vector))
    }
}

impl<'a, 'b, S> ops::Mul<&'a Translation3<S>> for &'b Translation3<S> 
where 
    S: ScalarFloat 
{
    type Output = Translation3<S>;

    #[inline]
    fn mul(self, other: &'a Translation3<S>) -> Self::Output {
        Translation3::from_vector(&(self.vector + other.vector))
    }
}

