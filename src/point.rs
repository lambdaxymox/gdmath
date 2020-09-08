use num_traits::NumCast;
use scalar::{
    Scalar,
};
use structure::{
    Storage,
};
use std::mem;


#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Point1<S> {
    pub x: S,
}

impl<S> Point1<S> {
    /// Construct a new point.
    pub const fn new(x: S) -> Point1<S> {
        Point1 { x: x }
    }

    /// Map an operation on the elements of a point, returning a point of the 
    /// new underlying type.
    pub fn map<T, F>(self, mut op: F) -> Point1<T> where F: FnMut(S) -> T {
        Point1 { x: op(self.x) }
    }
}

impl<S> Point1<S> where S: NumCast + Copy {
    /// Cast a point of one type of scalars to a point of another type of scalars.
    pub fn cast<T: NumCast>(&self) -> Option<Point1<T>> {
        let x = match num_traits::cast(self.x) {
            Some(value) => value,
            None => return None,
        };

        Some(Point1::new(x))
    }
}

impl<S> Storage for Point1<S> where S: Scalar {
    type Element = S;

    #[inline]
    fn len() -> usize {
        1
    }

    #[inline]
    fn shape() -> (usize, usize) {
        (1, 1)
    }

    #[inline]
    fn from_value(value: Self::Element) -> Self {
        Point1::new(value)
    }

    #[inline]
    fn sum(&self) -> Self::Element {
        self.x
    }

    #[inline]
    fn product(&self) -> Self::Element {
        self.x
    }

    #[inline]
    fn as_ptr(&self) -> *const Self::Element {
        &self.x
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut Self::Element {
        &mut self.x
    }

    #[inline]
    fn as_slice(&self) -> &[Self::Element] {
        <Self as AsRef<[Self::Element; 1]>>::as_ref(self)
    }
}

impl<S> AsRef<[S; 1]> for Point1<S> {
    fn as_ref(&self) -> &[S; 1] {
        unsafe { mem::transmute(self) }
    }
}

impl<S> AsRef<S> for Point1<S> {
    fn as_ref(&self) -> &S {
        unsafe { mem::transmute(self) }
    }
}

impl<S> AsRef<(S,)> for Point1<S> {
    fn as_ref(&self) -> &(S,) {
        unsafe { mem::transmute(self) }
    }
}

impl<S> AsMut<[S; 1]> for Point1<S> {
    fn as_mut(&mut self) -> &mut [S; 1] {
        unsafe { mem::transmute(self) }
    }
}

impl<S> AsMut<S> for Point1<S> {
    fn as_mut(&mut self) -> &mut S {
        unsafe { mem::transmute(self) }
    }
}

impl<S> AsMut<(S,)> for Point1<S> {
    fn as_mut(&mut self) -> &mut (S,) {
        unsafe { mem::transmute(self) }
    }
}



/// A representation of two-dimensional points with a Euclidean metric.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Point2<S> {
   pub x: S,
   pub y: S,
}

impl<S> Point2<S> {
    /// Construct a new point.
    pub const fn new(x: S, y: S) -> Point2<S> {
        Point2 { x: x, y: y }
    }

    /// Map an operation on the elements of a point, returning a point of the 
    /// new underlying type.
    pub fn map<T, F>(self, mut op: F) -> Point2<T> where F: FnMut(S) -> T {
        Point2 {
            x: op(self.x),
            y: op(self.y),
        }
    }
}

impl<S> Point2<S> where S: NumCast + Copy {
    /// Cast a point of one type of scalars to a point of another type of scalars.
    pub fn cast<T: NumCast>(&self) -> Option<Point2<T>> {
        let x = match num_traits::cast(self.x) {
            Some(value) => value,
            None => return None,
        };
        let y = match num_traits::cast(self.y) {
            Some(value) => value,
            None => return None,
        };

        Some(Point2::new(x, y))
    }
}

impl<S> Storage for Point2<S> where S: Scalar {
    type Element = S;

    #[inline]
    fn len() -> usize {
        2
    }

    #[inline]
    fn shape() -> (usize, usize) {
        (2, 1)
    }

    #[inline]
    fn from_value(value: Self::Element) -> Self {
        Point2::new(value, value)
    }

    #[inline]
    fn sum(&self) -> Self::Element {
        self.x + self.y
    }

    #[inline]
    fn product(&self) -> Self::Element {
        self.x * self.y
    }

    #[inline]
    fn as_ptr(&self) -> *const Self::Element {
        &self.x
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut Self::Element {
        &mut self.x
    }

    #[inline]
    fn as_slice(&self) -> &[Self::Element] {
        <Self as AsRef<[Self::Element; 2]>>::as_ref(self)
    }
}

impl<S> AsRef<[S; 2]> for Point2<S> {
    fn as_ref(&self) -> &[S; 2] {
        unsafe { mem::transmute(self) }
    }
}

impl<S> AsRef<(S, S)> for Point2<S> {
    fn as_ref(&self) -> &(S, S) {
        unsafe { mem::transmute(self) }
    }
}

impl<S> AsMut<[S; 2]> for Point2<S> {
    fn as_mut(&mut self) -> &mut [S; 2] {
        unsafe { mem::transmute(self) }
    }
}

impl<S> AsMut<(S, S)> for Point2<S> {
    fn as_mut(&mut self) -> &mut (S, S) {
        unsafe { mem::transmute(self) }
    }
}


/// A representation of three-dimensional points in a Euclidean space.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Point3<S> {
    pub x: S,
    pub y: S,
    pub z: S,
}

impl<S> Point3<S> {
    /// Construct a new point.
    pub const fn new(x: S, y: S, z: S) -> Point3<S> {
        Point3 { x: x, y: y, z: z }
    }

    /// Map an operation on the elements of a point, returning a point of the 
    /// new underlying type.
    pub fn map<T, F>(self, mut op: F) -> Point3<T> where F: FnMut(S) -> T {
        Point3 {
            x: op(self.x),
            y: op(self.y),
            z: op(self.z),
        }
    }
}

impl<S> Point3<S> where S: NumCast + Copy {
    /// Cast a point from one type of scalars to another type of scalars.
    pub fn cast<T: NumCast>(&self) -> Option<Point3<T>> {
        let x = match num_traits::cast(self.x) {
            Some(value) => value,
            None => return None,
        };
        let y = match num_traits::cast(self.y) {
            Some(value) => value,
            None => return None,
        };
        let z = match num_traits::cast(self.z) {
            Some(value) => value,
            None => return None,
        };

        Some(Point3::new(x, y, z))
    }
}

impl<S> Storage for Point3<S> where S: Scalar {
    type Element = S;

    #[inline]
    fn len() -> usize {
        3
    }

    #[inline]
    fn shape() -> (usize, usize) {
        (3, 1)
    }

    #[inline]
    fn from_value(value: Self::Element) -> Self {
        Point3::new(value, value, value)
    }

    #[inline]
    fn sum(&self) -> Self::Element {
        self.x + self.y + self.z
    }

    #[inline]
    fn product(&self) -> Self::Element {
        self.x * self.y * self.z
    }

    #[inline]
    fn as_ptr(&self) -> *const Self::Element {
        &self.x
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut Self::Element {
        &mut self.x
    }

    #[inline]
    fn as_slice(&self) -> &[Self::Element] {
        <Self as AsRef<[Self::Element; 3]>>::as_ref(self)
    }
}


impl<S> AsRef<[S; 3]> for Point3<S> {
    fn as_ref(&self) -> &[S; 3] {
        unsafe { mem::transmute(self) }
    }
}

impl<S> AsRef<(S, S, S)> for Point3<S> {
    fn as_ref(&self) -> &(S, S, S) {
        unsafe { mem::transmute(self) }
    }
}

impl<S> AsMut<[S; 3]> for Point3<S> {
    fn as_mut(&mut self) -> &mut [S; 3] {
        unsafe { mem::transmute(self) }
    }
}

impl<S> AsMut<(S, S, S)> for Point3<S> {
    fn as_mut(&mut self) -> &mut (S, S, S) {
        unsafe { mem::transmute(self) }
    }
}
