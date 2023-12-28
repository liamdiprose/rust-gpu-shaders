#![allow(clippy::assign_op_pattern)]
use spirv_std::glam::{Vec2, Vec3, Vec4, Vec2Swizzles, Vec3Swizzles, Vec4Swizzles};
use spirv_std::num_traits::Float;
use crate::*;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use geometric_algebra::*;

#[derive(Clone, Copy)]
struct MultiVectorGroups {
    /// 1, e12, e1, e2
    g0: Vec4,
    /// e0, e012, e01, -e02
    g1: Vec4,
}

#[derive(Clone, Copy)]
pub union MultiVector {
    groups: MultiVectorGroups,
    /// 1, e12, e1, e2, e0, e012, e01, -e02
    elements: [f32; 8],
}

impl MultiVector {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(scalar: f32, e12: f32, e1: f32, e2: f32, e0: f32, e012: f32, e01: f32, _e02: f32) -> Self {
        Self { elements: [scalar, e12, e1, e2, e0, e012, e01, _e02] }
    }
    pub const fn from_groups(g0: Vec4, g1: Vec4) -> Self {
        Self { groups: MultiVectorGroups { g0, g1 } }
    }
    #[inline(always)]
    pub fn group0(&self) -> Vec4 {
        unsafe { self.groups.g0 }
    }
    #[inline(always)]
    pub fn group0_mut(&mut self) -> &mut Vec4 {
        unsafe { &mut self.groups.g0 }
    }
    #[inline(always)]
    pub fn group1(&self) -> Vec4 {
        unsafe { self.groups.g1 }
    }
    #[inline(always)]
    pub fn group1_mut(&mut self) -> &mut Vec4 {
        unsafe { &mut self.groups.g1 }
    }
}

const MULTIVECTOR_INDEX_REMAP: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

impl core::ops::Index<usize> for MultiVector {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[MULTIVECTOR_INDEX_REMAP[index]] }
    }
}

impl core::ops::IndexMut<usize> for MultiVector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[MULTIVECTOR_INDEX_REMAP[index]] }
    }
}

impl core::convert::From<MultiVector> for [f32; 8] {
    fn from(vector: MultiVector) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[7]] }
    }
}

impl core::convert::From<[f32; 8]> for MultiVector {
    fn from(array: [f32; 8]) -> Self {
        Self { elements: [array[0], array[1], array[2], array[3], array[4], array[5], array[6], array[7]] }
    }
}

impl core::fmt::Debug for MultiVector {
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter
            .debug_struct("MultiVector")
            .field("1", &self[0])
            .field("e12", &self[1])
            .field("e1", &self[2])
            .field("e2", &self[3])
            .field("e0", &self[4])
            .field("e012", &self[5])
            .field("e01", &self[6])
            .field("-e02", &self[7])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct RotorGroups {
    /// 1, e12
    g0: Vec2,
}

#[derive(Clone, Copy)]
pub union Rotor {
    groups: RotorGroups,
    /// 1, e12, 0, 0
    elements: [f32; 4],
}

impl Rotor {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(scalar: f32, e12: f32) -> Self {
        Self { elements: [scalar, e12, 0.0, 0.0] }
    }
    pub const fn from_groups(g0: Vec2) -> Self {
        Self { groups: RotorGroups { g0 } }
    }
    #[inline(always)]
    pub fn group0(&self) -> Vec2 {
        unsafe { self.groups.g0 }
    }
    #[inline(always)]
    pub fn group0_mut(&mut self) -> &mut Vec2 {
        unsafe { &mut self.groups.g0 }
    }
}

const ROTOR_INDEX_REMAP: [usize; 2] = [0, 1];

impl core::ops::Index<usize> for Rotor {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[ROTOR_INDEX_REMAP[index]] }
    }
}

impl core::ops::IndexMut<usize> for Rotor {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[ROTOR_INDEX_REMAP[index]] }
    }
}

impl core::convert::From<Rotor> for [f32; 2] {
    fn from(vector: Rotor) -> Self {
        unsafe { [vector.elements[0], vector.elements[1]] }
    }
}

impl core::convert::From<[f32; 2]> for Rotor {
    fn from(array: [f32; 2]) -> Self {
        Self { elements: [array[0], array[1], 0.0, 0.0] }
    }
}

impl core::fmt::Debug for Rotor {
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter
            .debug_struct("Rotor")
            .field("1", &self[0])
            .field("e12", &self[1])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct PointGroups {
    /// e12, e01, -e02
    g0: Vec3,
}

#[derive(Clone, Copy)]
pub union Point {
    groups: PointGroups,
    /// e12, e01, -e02, 0
    elements: [f32; 4],
}

impl Point {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(e12: f32, e01: f32, _e02: f32) -> Self {
        Self { elements: [e12, e01, _e02, 0.0] }
    }
    pub const fn from_groups(g0: Vec3) -> Self {
        Self { groups: PointGroups { g0 } }
    }
    #[inline(always)]
    pub fn group0(&self) -> Vec3 {
        unsafe { self.groups.g0 }
    }
    #[inline(always)]
    pub fn group0_mut(&mut self) -> &mut Vec3 {
        unsafe { &mut self.groups.g0 }
    }
}

const POINT_INDEX_REMAP: [usize; 3] = [0, 1, 2];

impl core::ops::Index<usize> for Point {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[POINT_INDEX_REMAP[index]] }
    }
}

impl core::ops::IndexMut<usize> for Point {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[POINT_INDEX_REMAP[index]] }
    }
}

impl core::convert::From<Point> for [f32; 3] {
    fn from(vector: Point) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2]] }
    }
}

impl core::convert::From<[f32; 3]> for Point {
    fn from(array: [f32; 3]) -> Self {
        Self { elements: [array[0], array[1], array[2], 0.0] }
    }
}

impl core::fmt::Debug for Point {
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter
            .debug_struct("Point")
            .field("e12", &self[0])
            .field("e01", &self[1])
            .field("-e02", &self[2])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct IdealPointGroups {
    /// e01, -e02
    g0: Vec2,
}

#[derive(Clone, Copy)]
pub union IdealPoint {
    groups: IdealPointGroups,
    /// e01, -e02, 0, 0
    elements: [f32; 4],
}

impl IdealPoint {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(e01: f32, _e02: f32) -> Self {
        Self { elements: [e01, _e02, 0.0, 0.0] }
    }
    pub const fn from_groups(g0: Vec2) -> Self {
        Self { groups: IdealPointGroups { g0 } }
    }
    #[inline(always)]
    pub fn group0(&self) -> Vec2 {
        unsafe { self.groups.g0 }
    }
    #[inline(always)]
    pub fn group0_mut(&mut self) -> &mut Vec2 {
        unsafe { &mut self.groups.g0 }
    }
}

const IDEALPOINT_INDEX_REMAP: [usize; 2] = [0, 1];

impl core::ops::Index<usize> for IdealPoint {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[IDEALPOINT_INDEX_REMAP[index]] }
    }
}

impl core::ops::IndexMut<usize> for IdealPoint {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[IDEALPOINT_INDEX_REMAP[index]] }
    }
}

impl core::convert::From<IdealPoint> for [f32; 2] {
    fn from(vector: IdealPoint) -> Self {
        unsafe { [vector.elements[0], vector.elements[1]] }
    }
}

impl core::convert::From<[f32; 2]> for IdealPoint {
    fn from(array: [f32; 2]) -> Self {
        Self { elements: [array[0], array[1], 0.0, 0.0] }
    }
}

impl core::fmt::Debug for IdealPoint {
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter
            .debug_struct("IdealPoint")
            .field("e01", &self[0])
            .field("-e02", &self[1])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct PlaneGroups {
    /// e0, e2, e1
    g0: Vec3,
}

#[derive(Clone, Copy)]
pub union Plane {
    groups: PlaneGroups,
    /// e0, e2, e1, 0
    elements: [f32; 4],
}

impl Plane {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(e0: f32, e2: f32, e1: f32) -> Self {
        Self { elements: [e0, e2, e1, 0.0] }
    }
    pub const fn from_groups(g0: Vec3) -> Self {
        Self { groups: PlaneGroups { g0 } }
    }
    #[inline(always)]
    pub fn group0(&self) -> Vec3 {
        unsafe { self.groups.g0 }
    }
    #[inline(always)]
    pub fn group0_mut(&mut self) -> &mut Vec3 {
        unsafe { &mut self.groups.g0 }
    }
}

const PLANE_INDEX_REMAP: [usize; 3] = [0, 1, 2];

impl core::ops::Index<usize> for Plane {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[PLANE_INDEX_REMAP[index]] }
    }
}

impl core::ops::IndexMut<usize> for Plane {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[PLANE_INDEX_REMAP[index]] }
    }
}

impl core::convert::From<Plane> for [f32; 3] {
    fn from(vector: Plane) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2]] }
    }
}

impl core::convert::From<[f32; 3]> for Plane {
    fn from(array: [f32; 3]) -> Self {
        Self { elements: [array[0], array[1], array[2], 0.0] }
    }
}

impl core::fmt::Debug for Plane {
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter
            .debug_struct("Plane")
            .field("e0", &self[0])
            .field("e2", &self[1])
            .field("e1", &self[2])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct TranslatorGroups {
    /// 1, e01, -e02
    g0: Vec3,
}

#[derive(Clone, Copy)]
pub union Translator {
    groups: TranslatorGroups,
    /// 1, e01, -e02, 0
    elements: [f32; 4],
}

impl Translator {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(scalar: f32, e01: f32, _e02: f32) -> Self {
        Self { elements: [scalar, e01, _e02, 0.0] }
    }
    pub const fn from_groups(g0: Vec3) -> Self {
        Self { groups: TranslatorGroups { g0 } }
    }
    #[inline(always)]
    pub fn group0(&self) -> Vec3 {
        unsafe { self.groups.g0 }
    }
    #[inline(always)]
    pub fn group0_mut(&mut self) -> &mut Vec3 {
        unsafe { &mut self.groups.g0 }
    }
}

const TRANSLATOR_INDEX_REMAP: [usize; 3] = [0, 1, 2];

impl core::ops::Index<usize> for Translator {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[TRANSLATOR_INDEX_REMAP[index]] }
    }
}

impl core::ops::IndexMut<usize> for Translator {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[TRANSLATOR_INDEX_REMAP[index]] }
    }
}

impl core::convert::From<Translator> for [f32; 3] {
    fn from(vector: Translator) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2]] }
    }
}

impl core::convert::From<[f32; 3]> for Translator {
    fn from(array: [f32; 3]) -> Self {
        Self { elements: [array[0], array[1], array[2], 0.0] }
    }
}

impl core::fmt::Debug for Translator {
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter
            .debug_struct("Translator")
            .field("1", &self[0])
            .field("e01", &self[1])
            .field("-e02", &self[2])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct MotorGroups {
    /// 1, e12, e01, -e02
    g0: Vec4,
}

#[derive(Clone, Copy)]
pub union Motor {
    groups: MotorGroups,
    /// 1, e12, e01, -e02
    elements: [f32; 4],
}

impl Motor {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(scalar: f32, e12: f32, e01: f32, _e02: f32) -> Self {
        Self { elements: [scalar, e12, e01, _e02] }
    }
    pub const fn from_groups(g0: Vec4) -> Self {
        Self { groups: MotorGroups { g0 } }
    }
    #[inline(always)]
    pub fn group0(&self) -> Vec4 {
        unsafe { self.groups.g0 }
    }
    #[inline(always)]
    pub fn group0_mut(&mut self) -> &mut Vec4 {
        unsafe { &mut self.groups.g0 }
    }
}

const MOTOR_INDEX_REMAP: [usize; 4] = [0, 1, 2, 3];

impl core::ops::Index<usize> for Motor {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[MOTOR_INDEX_REMAP[index]] }
    }
}

impl core::ops::IndexMut<usize> for Motor {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[MOTOR_INDEX_REMAP[index]] }
    }
}

impl core::convert::From<Motor> for [f32; 4] {
    fn from(vector: Motor) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3]] }
    }
}

impl core::convert::From<[f32; 4]> for Motor {
    fn from(array: [f32; 4]) -> Self {
        Self { elements: [array[0], array[1], array[2], array[3]] }
    }
}

impl core::fmt::Debug for Motor {
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter
            .debug_struct("Motor")
            .field("1", &self[0])
            .field("e12", &self[1])
            .field("e01", &self[2])
            .field("-e02", &self[3])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct MotorDualGroups {
    /// e012, e0, e2, e1
    g0: Vec4,
}

#[derive(Clone, Copy)]
pub union MotorDual {
    groups: MotorDualGroups,
    /// e012, e0, e2, e1
    elements: [f32; 4],
}

impl MotorDual {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(e012: f32, e0: f32, e2: f32, e1: f32) -> Self {
        Self { elements: [e012, e0, e2, e1] }
    }
    pub const fn from_groups(g0: Vec4) -> Self {
        Self { groups: MotorDualGroups { g0 } }
    }
    #[inline(always)]
    pub fn group0(&self) -> Vec4 {
        unsafe { self.groups.g0 }
    }
    #[inline(always)]
    pub fn group0_mut(&mut self) -> &mut Vec4 {
        unsafe { &mut self.groups.g0 }
    }
}

const MOTORDUAL_INDEX_REMAP: [usize; 4] = [0, 1, 2, 3];

impl core::ops::Index<usize> for MotorDual {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[MOTORDUAL_INDEX_REMAP[index]] }
    }
}

impl core::ops::IndexMut<usize> for MotorDual {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[MOTORDUAL_INDEX_REMAP[index]] }
    }
}

impl core::convert::From<MotorDual> for [f32; 4] {
    fn from(vector: MotorDual) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3]] }
    }
}

impl core::convert::From<[f32; 4]> for MotorDual {
    fn from(array: [f32; 4]) -> Self {
        Self { elements: [array[0], array[1], array[2], array[3]] }
    }
}

impl core::fmt::Debug for MotorDual {
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter
            .debug_struct("MotorDual")
            .field("e012", &self[0])
            .field("e0", &self[1])
            .field("e2", &self[2])
            .field("e1", &self[3])
            .finish()
    }
}

impl Add<MultiVector> for f32 {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self) * Vec4::from([1.0, 0.0, 0.0, 0.0]) + other.group0(), g1: other.group1() } }
    }
}

impl Sub<MultiVector> for f32 {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self) * Vec4::from([1.0, 0.0, 0.0, 0.0]) - other.group0(), g1: Vec4::splat(0.0) - other.group1() } }
    }
}

impl GeometricProduct<MultiVector> for f32 {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self) * other.group0(), g1: Vec4::splat(self) * other.group1() } }
    }
}

impl RegressiveProduct<MultiVector> for f32 {
    type Output = f32;

    fn regressive_product(self, other: MultiVector) -> f32 {
        self * other.group1()[1]
    }
}

impl OuterProduct<MultiVector> for f32 {
    type Output = MultiVector;

    fn outer_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self) * other.group0(), g1: Vec4::splat(self) * other.group1() } }
    }
}

impl InnerProduct<MultiVector> for f32 {
    type Output = MultiVector;

    fn inner_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self) * other.group0(), g1: Vec4::splat(self) * other.group1() } }
    }
}

impl LeftContraction<MultiVector> for f32 {
    type Output = MultiVector;

    fn left_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self) * other.group0(), g1: Vec4::splat(self) * other.group1() } }
    }
}

impl RightContraction<MultiVector> for f32 {
    type Output = f32;

    fn right_contraction(self, other: MultiVector) -> f32 {
        self * other.group0()[0]
    }
}

impl ScalarProduct<MultiVector> for f32 {
    type Output = f32;

    fn scalar_product(self, other: MultiVector) -> f32 {
        self * other.group0()[0]
    }
}

impl Add<Rotor> for f32 {
    type Output = Rotor;

    fn add(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Vec2::splat(self) * Vec2::from([1.0, 0.0]) + other.group0() } }
    }
}

impl Sub<Rotor> for f32 {
    type Output = Rotor;

    fn sub(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Vec2::splat(self) * Vec2::from([1.0, 0.0]) - other.group0() } }
    }
}

impl GeometricProduct<Rotor> for f32 {
    type Output = Rotor;

    fn geometric_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Vec2::splat(self) * other.group0() } }
    }
}

impl OuterProduct<Rotor> for f32 {
    type Output = Rotor;

    fn outer_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Vec2::splat(self) * other.group0() } }
    }
}

impl InnerProduct<Rotor> for f32 {
    type Output = Rotor;

    fn inner_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Vec2::splat(self) * other.group0() } }
    }
}

impl LeftContraction<Rotor> for f32 {
    type Output = Rotor;

    fn left_contraction(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Vec2::splat(self) * other.group0() } }
    }
}

impl RightContraction<Rotor> for f32 {
    type Output = f32;

    fn right_contraction(self, other: Rotor) -> f32 {
        self * other.group0()[0]
    }
}

impl ScalarProduct<Rotor> for f32 {
    type Output = f32;

    fn scalar_product(self, other: Rotor) -> f32 {
        self * other.group0()[0]
    }
}

impl Add<Point> for f32 {
    type Output = Motor;

    fn add(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self) * Vec4::from([1.0, 0.0, 0.0, 0.0]) + Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl Sub<Point> for f32 {
    type Output = Motor;

    fn sub(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self) * Vec4::from([1.0, 0.0, 0.0, 0.0]) - Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl GeometricProduct<Point> for f32 {
    type Output = Point;

    fn geometric_product(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Vec3::splat(self) * other.group0() } }
    }
}

impl OuterProduct<Point> for f32 {
    type Output = Point;

    fn outer_product(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Vec3::splat(self) * other.group0() } }
    }
}

impl InnerProduct<Point> for f32 {
    type Output = Point;

    fn inner_product(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Vec3::splat(self) * other.group0() } }
    }
}

impl LeftContraction<Point> for f32 {
    type Output = Point;

    fn left_contraction(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Vec3::splat(self) * other.group0() } }
    }
}

impl Add<IdealPoint> for f32 {
    type Output = Translator;

    fn add(self, other: IdealPoint) -> Translator {
        Translator { groups: TranslatorGroups { g0: Vec3::splat(self) * Vec3::from([1.0, 0.0, 0.0]) + Vec3::from([other.group0()[0], other.group0()[0], other.group0()[1]]) * Vec3::from([0.0, 1.0, 1.0]) } }
    }
}

impl Sub<IdealPoint> for f32 {
    type Output = Translator;

    fn sub(self, other: IdealPoint) -> Translator {
        Translator { groups: TranslatorGroups { g0: Vec3::splat(self) * Vec3::from([1.0, 0.0, 0.0]) - Vec3::from([other.group0()[0], other.group0()[0], other.group0()[1]]) * Vec3::from([0.0, 1.0, 1.0]) } }
    }
}

impl GeometricProduct<IdealPoint> for f32 {
    type Output = IdealPoint;

    fn geometric_product(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Vec2::splat(self) * other.group0() } }
    }
}

impl OuterProduct<IdealPoint> for f32 {
    type Output = IdealPoint;

    fn outer_product(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Vec2::splat(self) * other.group0() } }
    }
}

impl InnerProduct<IdealPoint> for f32 {
    type Output = IdealPoint;

    fn inner_product(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Vec2::splat(self) * other.group0() } }
    }
}

impl LeftContraction<IdealPoint> for f32 {
    type Output = IdealPoint;

    fn left_contraction(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Vec2::splat(self) * other.group0() } }
    }
}

impl GeometricProduct<Plane> for f32 {
    type Output = Plane;

    fn geometric_product(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self) * other.group0() } }
    }
}

impl OuterProduct<Plane> for f32 {
    type Output = Plane;

    fn outer_product(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self) * other.group0() } }
    }
}

impl InnerProduct<Plane> for f32 {
    type Output = Plane;

    fn inner_product(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self) * other.group0() } }
    }
}

impl LeftContraction<Plane> for f32 {
    type Output = Plane;

    fn left_contraction(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self) * other.group0() } }
    }
}

impl Add<Translator> for f32 {
    type Output = Translator;

    fn add(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Vec3::splat(self) * Vec3::from([1.0, 0.0, 0.0]) + other.group0() } }
    }
}

impl Sub<Translator> for f32 {
    type Output = Translator;

    fn sub(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Vec3::splat(self) * Vec3::from([1.0, 0.0, 0.0]) - other.group0() } }
    }
}

impl GeometricProduct<Translator> for f32 {
    type Output = Translator;

    fn geometric_product(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Vec3::splat(self) * other.group0() } }
    }
}

impl OuterProduct<Translator> for f32 {
    type Output = Translator;

    fn outer_product(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Vec3::splat(self) * other.group0() } }
    }
}

impl InnerProduct<Translator> for f32 {
    type Output = Translator;

    fn inner_product(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Vec3::splat(self) * other.group0() } }
    }
}

impl LeftContraction<Translator> for f32 {
    type Output = Translator;

    fn left_contraction(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Vec3::splat(self) * other.group0() } }
    }
}

impl RightContraction<Translator> for f32 {
    type Output = f32;

    fn right_contraction(self, other: Translator) -> f32 {
        self * other.group0()[0]
    }
}

impl ScalarProduct<Translator> for f32 {
    type Output = f32;

    fn scalar_product(self, other: Translator) -> f32 {
        self * other.group0()[0]
    }
}

impl Add<Motor> for f32 {
    type Output = Motor;

    fn add(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self) * Vec4::from([1.0, 0.0, 0.0, 0.0]) + other.group0() } }
    }
}

impl Sub<Motor> for f32 {
    type Output = Motor;

    fn sub(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self) * Vec4::from([1.0, 0.0, 0.0, 0.0]) - other.group0() } }
    }
}

impl GeometricProduct<Motor> for f32 {
    type Output = Motor;

    fn geometric_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self) * other.group0() } }
    }
}

impl OuterProduct<Motor> for f32 {
    type Output = Motor;

    fn outer_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self) * other.group0() } }
    }
}

impl InnerProduct<Motor> for f32 {
    type Output = Motor;

    fn inner_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self) * other.group0() } }
    }
}

impl LeftContraction<Motor> for f32 {
    type Output = Motor;

    fn left_contraction(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self) * other.group0() } }
    }
}

impl RightContraction<Motor> for f32 {
    type Output = f32;

    fn right_contraction(self, other: Motor) -> f32 {
        self * other.group0()[0]
    }
}

impl ScalarProduct<Motor> for f32 {
    type Output = f32;

    fn scalar_product(self, other: Motor) -> f32 {
        self * other.group0()[0]
    }
}

impl GeometricProduct<MotorDual> for f32 {
    type Output = MotorDual;

    fn geometric_product(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self) * other.group0() } }
    }
}

impl RegressiveProduct<MotorDual> for f32 {
    type Output = f32;

    fn regressive_product(self, other: MotorDual) -> f32 {
        self * other.group0()[0]
    }
}

impl OuterProduct<MotorDual> for f32 {
    type Output = MotorDual;

    fn outer_product(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self) * other.group0() } }
    }
}

impl InnerProduct<MotorDual> for f32 {
    type Output = MotorDual;

    fn inner_product(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self) * other.group0() } }
    }
}

impl LeftContraction<MotorDual> for f32 {
    type Output = MotorDual;

    fn left_contraction(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self) * other.group0() } }
    }
}

impl Zero for MultiVector {
    fn zero() -> Self {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(0.0), g1: Vec4::splat(0.0) } }
    }
}

impl One for MultiVector {
    fn one() -> Self {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::from([1.0, 0.0, 0.0, 0.0]), g1: Vec4::splat(0.0) } }
    }
}

impl Neg for MultiVector {
    type Output = MultiVector;

    fn neg(self) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Vec4::splat(-1.0), g1: self.group1() * Vec4::splat(-1.0) } }
    }
}

impl Automorphism for MultiVector {
    type Output = MultiVector;

    fn automorphism(self) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Vec4::from([1.0, 1.0, -1.0, -1.0]), g1: self.group1() * Vec4::from([-1.0, -1.0, 1.0, 1.0]) } }
    }
}

impl Reversal for MultiVector {
    type Output = MultiVector;

    fn reversal(self) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Vec4::from([1.0, -1.0, 1.0, 1.0]), g1: self.group1() * Vec4::from([1.0, -1.0, -1.0, -1.0]) } }
    }
}

impl Conjugation for MultiVector {
    type Output = MultiVector;

    fn conjugation(self) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Vec4::from([1.0, -1.0, -1.0, -1.0]), g1: self.group1() * Vec4::from([-1.0, 1.0, -1.0, -1.0]) } }
    }
}

impl Dual for MultiVector {
    type Output = MultiVector;

    fn dual(self) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group1().yxwz(), g1: self.group0().yxwz() } }
    }
}

impl Into<f32> for MultiVector {
    fn into(self) -> f32 {
        self.group0()[0]
    }
}

impl Add<f32> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: f32) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() + Vec4::splat(other) * Vec4::from([1.0, 0.0, 0.0, 0.0]), g1: self.group1() } }
    }
}

impl AddAssign<f32> for MultiVector {
    fn add_assign(&mut self, other: f32) {
        *self = (*self).add(other);
    }
}

impl Sub<f32> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: f32) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() - Vec4::splat(other) * Vec4::from([1.0, 0.0, 0.0, 0.0]), g1: self.group1() } }
    }
}

impl SubAssign<f32> for MultiVector {
    fn sub_assign(&mut self, other: f32) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<f32> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: f32) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Vec4::splat(other), g1: self.group1() * Vec4::splat(other) } }
    }
}

impl RegressiveProduct<f32> for MultiVector {
    type Output = f32;

    fn regressive_product(self, other: f32) -> f32 {
        self.group1()[1] * other
    }
}

impl OuterProduct<f32> for MultiVector {
    type Output = MultiVector;

    fn outer_product(self, other: f32) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Vec4::splat(other), g1: self.group1() * Vec4::splat(other) } }
    }
}

impl InnerProduct<f32> for MultiVector {
    type Output = MultiVector;

    fn inner_product(self, other: f32) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Vec4::splat(other), g1: self.group1() * Vec4::splat(other) } }
    }
}

impl LeftContraction<f32> for MultiVector {
    type Output = f32;

    fn left_contraction(self, other: f32) -> f32 {
        self.group0()[0] * other
    }
}

impl RightContraction<f32> for MultiVector {
    type Output = MultiVector;

    fn right_contraction(self, other: f32) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Vec4::splat(other), g1: self.group1() * Vec4::splat(other) } }
    }
}

impl ScalarProduct<f32> for MultiVector {
    type Output = f32;

    fn scalar_product(self, other: f32) -> f32 {
        self.group0()[0] * other
    }
}

impl Add<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() + other.group0(), g1: self.group1() + other.group1() } }
    }
}

impl AddAssign<MultiVector> for MultiVector {
    fn add_assign(&mut self, other: MultiVector) {
        *self = (*self).add(other);
    }
}

impl Sub<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() - other.group0(), g1: self.group1() - other.group1() } }
    }
}

impl SubAssign<MultiVector> for MultiVector {
    fn sub_assign(&mut self, other: MultiVector) {
        *self = (*self).sub(other);
    }
}

impl Mul<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn mul(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * other.group0(), g1: self.group1() * other.group1() } }
    }
}

impl MulAssign<MultiVector> for MultiVector {
    fn mul_assign(&mut self, other: MultiVector) {
        *self = (*self).mul(other);
    }
}

impl Div<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn div(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[3]]) * Vec4::from([1.0, 1.0, 1.0, 1.0]) / Vec4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[3]]) * Vec4::from([1.0, 1.0, 1.0, 1.0]), g1: Vec4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[3]]) * Vec4::from([1.0, 1.0, 1.0, 1.0]) / Vec4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[3]]) * Vec4::from([1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl DivAssign<MultiVector> for MultiVector {
    fn div_assign(&mut self, other: MultiVector) {
        *self = (*self).div(other);
    }
}

impl GeometricProduct<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + Vec4::splat(self.group0()[1]) * other.group0().yxwz() * Vec4::from([-1.0, 1.0, 1.0, -1.0]) + Vec4::splat(self.group0()[2]) * other.group0().zwxy() + Vec4::splat(self.group0()[3]) * other.group0().wzyx() * Vec4::from([1.0, -1.0, -1.0, 1.0]) + Vec4::splat(self.group1()[0]) * other.group1() * Vec4::from([1.0, 1.0, 1.0, -1.0]) + Vec4::splat(self.group1()[1]) * other.group1().yxwz() * Vec4::from([-1.0, 1.0, -1.0, -1.0]) + Vec4::splat(self.group1()[2]) * other.group1().zwxy() * Vec4::from([-1.0, 1.0, -1.0, -1.0]) + Vec4::splat(self.group1()[3]) * other.group1().wzyx() * Vec4::from([-1.0, -1.0, -1.0, 1.0]), g1: Vec4::splat(self.group0()[0]) * other.group1() + Vec4::splat(self.group0()[1]) * other.group1().yxwz() * Vec4::from([-1.0, 1.0, -1.0, 1.0]) + Vec4::splat(self.group0()[2]) * other.group1().zwxy() * Vec4::from([-1.0, 1.0, -1.0, 1.0]) + Vec4::splat(self.group0()[3]) * other.group1().wzyx() + Vec4::splat(self.group1()[0]) * other.group0() * Vec4::from([1.0, 1.0, 1.0, -1.0]) + Vec4::splat(self.group1()[1]) * other.group0().yxwz() * Vec4::from([-1.0, 1.0, 1.0, 1.0]) + Vec4::splat(self.group1()[2]) * other.group0().zwxy() * Vec4::from([1.0, 1.0, 1.0, -1.0]) + Vec4::splat(self.group1()[3]) * other.group0().wzyx() * Vec4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl RegressiveProduct<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn regressive_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[1]) * other.group1() * Vec4::from([1.0, 1.0, -1.0, 1.0]) + Vec4::splat(self.group0()[2]) * other.group1().wwyw() * Vec4::from([1.0, 0.0, 1.0, 0.0]) + Vec4::splat(self.group0()[3]) * other.group1().zzzy() * Vec4::from([1.0, 0.0, 0.0, 1.0]) + Vec4::splat(self.group1()[0]) * Vec4::splat(other.group0()[1]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) + Vec4::splat(self.group1()[1]) * other.group0() + Vec4::splat(self.group1()[2]) * other.group0().wwyw() * Vec4::from([1.0, 0.0, 1.0, 0.0]) + Vec4::splat(self.group1()[3]) * other.group0().zzzy() * Vec4::from([1.0, 0.0, 0.0, -1.0]) + Vec4::splat(self.group0()[0]) * other.group1().yxxx() * Vec4::from([1.0, 0.0, 0.0, 0.0]), g1: Vec4::splat(self.group1()[1]) * other.group1() + Vec4::splat(self.group1()[2]) * other.group1().wwyw() * Vec4::from([-1.0, 0.0, 1.0, 0.0]) + Vec4::splat(self.group1()[3]) * other.group1().zzzy() * Vec4::from([1.0, 0.0, 0.0, 1.0]) + Vec4::splat(self.group1()[0]) * other.group1().yxxx() * Vec4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl OuterProduct<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn outer_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + Vec4::splat(self.group0()[2]) * other.group0().wwxw() * Vec4::from([0.0, 1.0, 1.0, 0.0]) + Vec4::splat(self.group0()[3]) * other.group0().zzzx() * Vec4::from([0.0, -1.0, 0.0, 1.0]) + self.group0().xyxx() * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 1.0, 0.0, 0.0]), g1: Vec4::splat(self.group0()[0]) * other.group1() + Vec4::splat(self.group0()[2]) * other.group1().wwxw() * Vec4::from([0.0, 1.0, -1.0, 0.0]) + Vec4::splat(self.group0()[3]) * other.group1().zzzx() * Vec4::from([0.0, 1.0, 0.0, 1.0]) + Vec4::splat(self.group1()[0]) * other.group0() * Vec4::from([1.0, 1.0, 1.0, -1.0]) + Vec4::splat(self.group1()[1]) * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 1.0, 0.0, 0.0]) + Vec4::splat(self.group1()[2]) * other.group0().wwxw() * Vec4::from([0.0, 1.0, 1.0, 0.0]) + Vec4::splat(self.group1()[3]) * other.group0().zzzx() * Vec4::from([0.0, 1.0, 0.0, 1.0]) + self.group0().xyxx() * Vec4::splat(other.group1()[0]) * Vec4::from([0.0, 1.0, 0.0, 0.0]) } }
    }
}

impl InnerProduct<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn inner_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + Vec4::splat(self.group0()[1]) * other.group0().yxwz() * Vec4::from([-1.0, 1.0, 1.0, -1.0]) + Vec4::splat(self.group0()[3]) * other.group0().wwyx() * Vec4::from([1.0, 0.0, -1.0, 1.0]) + Vec4::splat(self.group1()[0]) * other.group1() * Vec4::from([1.0, 1.0, 1.0, -1.0]) + Vec4::splat(self.group1()[1]) * other.group1().yxwz() * Vec4::from([-1.0, 1.0, -1.0, -1.0]) + Vec4::splat(self.group1()[2]) * other.group1().zzxy() * Vec4::from([-1.0, 0.0, -1.0, -1.0]) + Vec4::splat(self.group1()[3]) * other.group1().wwyx() * Vec4::from([-1.0, 0.0, -1.0, 1.0]) + self.group0().zxzz() * other.group0().zxxy() * Vec4::from([1.0, 0.0, 1.0, 1.0]), g1: Vec4::splat(self.group0()[0]) * other.group1() + Vec4::splat(self.group0()[2]) * other.group1().zzzy() * Vec4::from([-1.0, 0.0, 0.0, 1.0]) + Vec4::splat(self.group0()[3]) * other.group1().wwyw() * Vec4::from([1.0, 0.0, 1.0, 0.0]) + Vec4::splat(self.group1()[0]) * Vec4::splat(other.group0()[0]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) + Vec4::splat(self.group1()[1]) * other.group0().yxwz() * Vec4::from([-1.0, 1.0, 1.0, 1.0]) + Vec4::splat(self.group1()[2]) * other.group0().zzxz() * Vec4::from([1.0, 0.0, 1.0, 0.0]) + Vec4::splat(self.group1()[3]) * other.group0().wwwx() * Vec4::from([-1.0, 0.0, 0.0, 1.0]) + self.group0().yxxx() * other.group1().yxxx() * Vec4::from([-1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl LeftContraction<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn left_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + Vec4::splat(self.group0()[2]) * other.group0().zzzy() * Vec4::from([1.0, 0.0, 0.0, 1.0]) + Vec4::splat(self.group0()[3]) * other.group0().wwyw() * Vec4::from([1.0, 0.0, -1.0, 0.0]) + Vec4::splat(self.group1()[0]) * other.group1() * Vec4::from([1.0, 1.0, 1.0, -1.0]) + Vec4::splat(self.group1()[1]) * Vec4::splat(other.group1()[1]) * Vec4::from([-1.0, 0.0, 0.0, 0.0]) + Vec4::splat(self.group1()[2]) * other.group1().zzzy() * Vec4::from([-1.0, 0.0, 0.0, -1.0]) + Vec4::splat(self.group1()[3]) * other.group1().wwyw() * Vec4::from([-1.0, 0.0, -1.0, 0.0]) + self.group0().yxxx() * other.group0().yxxx() * Vec4::from([-1.0, 0.0, 0.0, 0.0]), g1: Vec4::splat(self.group0()[0]) * other.group1() + Vec4::splat(self.group0()[2]) * other.group1().zzzy() * Vec4::from([-1.0, 0.0, 0.0, 1.0]) + Vec4::splat(self.group0()[3]) * other.group1().wwyw() * Vec4::from([1.0, 0.0, 1.0, 0.0]) + self.group0().yxxx() * other.group1().yxxx() * Vec4::from([-1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl RightContraction<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn right_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[1]) * other.group0().yxwz() * Vec4::from([-1.0, 1.0, 1.0, -1.0]) + Vec4::splat(self.group0()[2]) * other.group0().zzxz() * Vec4::from([1.0, 0.0, 1.0, 0.0]) + Vec4::splat(self.group0()[3]) * other.group0().wwwx() * Vec4::from([1.0, 0.0, 0.0, 1.0]) + Vec4::splat(self.group1()[0]) * Vec4::splat(other.group1()[0]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) + Vec4::splat(self.group1()[1]) * other.group1().yxwz() * Vec4::from([-1.0, 1.0, -1.0, -1.0]) + Vec4::splat(self.group1()[2]) * other.group1().zzxz() * Vec4::from([-1.0, 0.0, -1.0, 0.0]) + Vec4::splat(self.group1()[3]) * other.group1().wwwx() * Vec4::from([-1.0, 0.0, 0.0, 1.0]) + Vec4::splat(self.group0()[0]) * Vec4::splat(other.group0()[0]) * Vec4::from([1.0, 0.0, 0.0, 0.0]), g1: Vec4::splat(self.group1()[1]) * other.group0().yxwz() * Vec4::from([-1.0, 1.0, 1.0, 1.0]) + Vec4::splat(self.group1()[2]) * other.group0().zzxz() * Vec4::from([1.0, 0.0, 1.0, 0.0]) + Vec4::splat(self.group1()[3]) * other.group0().wwwx() * Vec4::from([-1.0, 0.0, 0.0, 1.0]) + Vec4::splat(self.group1()[0]) * Vec4::splat(other.group0()[0]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl ScalarProduct<MultiVector> for MultiVector {
    type Output = f32;

    fn scalar_product(self, other: MultiVector) -> f32 {
        self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] + self.group0()[3] * other.group0()[3] + self.group1()[0] * other.group1()[0] - self.group1()[1] * other.group1()[1] - self.group1()[2] * other.group1()[2] - self.group1()[3] * other.group1()[3]
    }
}

impl Into<Rotor> for MultiVector {
    fn into(self) -> Rotor {
        Rotor { groups: RotorGroups { g0: Vec2::from([self.group0()[0], self.group0()[1]]) } }
    }
}

impl Add<Rotor> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Rotor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() + Vec4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[0]]) * Vec4::from([1.0, 1.0, 0.0, 0.0]), g1: self.group1() } }
    }
}

impl AddAssign<Rotor> for MultiVector {
    fn add_assign(&mut self, other: Rotor) {
        *self = (*self).add(other);
    }
}

impl Sub<Rotor> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Rotor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() - Vec4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[0]]) * Vec4::from([1.0, 1.0, 0.0, 0.0]), g1: self.group1() } }
    }
}

impl SubAssign<Rotor> for MultiVector {
    fn sub_assign(&mut self, other: Rotor) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Rotor> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: Rotor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[1]) * Vec4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Vec4::from([-1.0, 1.0, 0.0, 0.0]) + Vec4::splat(self.group0()[3]) * Vec4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]) * Vec4::from([0.0, 0.0, -1.0, 1.0]) + self.group0().xxzz() * Vec4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[1]]), g1: Vec4::splat(self.group1()[1]) * Vec4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Vec4::from([-1.0, 1.0, 0.0, 0.0]) + Vec4::splat(self.group1()[3]) * Vec4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]) * Vec4::from([0.0, 0.0, 1.0, 1.0]) + self.group1().xxzz() * Vec4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[1]]) * Vec4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl OuterProduct<Rotor> for MultiVector {
    type Output = MultiVector;

    fn outer_product(self, other: Rotor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[1]) * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 1.0, 0.0, 0.0]) + self.group0().xxzw() * Vec4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[0]]), g1: Vec4::splat(self.group1()[1]) * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 1.0, 0.0, 0.0]) + self.group1().xxzw() * Vec4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[0]]) } }
    }
}

impl InnerProduct<Rotor> for MultiVector {
    type Output = MultiVector;

    fn inner_product(self, other: Rotor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[1]) * Vec4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Vec4::from([-1.0, 1.0, 0.0, 0.0]) + Vec4::splat(self.group0()[3]) * Vec4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]) * Vec4::from([0.0, 0.0, -1.0, 1.0]) + self.group0().xxzz() * Vec4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[1]]), g1: Vec4::splat(self.group1()[1]) * Vec4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Vec4::from([-1.0, 1.0, 0.0, 0.0]) + self.group1().xxzw() * Vec4::splat(other.group0()[0]) * Vec4::from([1.0, 0.0, 1.0, 1.0]) } }
    }
}

impl RightContraction<Rotor> for MultiVector {
    type Output = MultiVector;

    fn right_contraction(self, other: Rotor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[1]) * Vec4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Vec4::from([-1.0, 1.0, 0.0, 0.0]) + self.group0().xxzw() * Vec4::splat(other.group0()[0]) * Vec4::from([1.0, 0.0, 1.0, 1.0]), g1: Vec4::splat(self.group1()[1]) * Vec4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Vec4::from([-1.0, 1.0, 0.0, 0.0]) + self.group1().xxzw() * Vec4::splat(other.group0()[0]) * Vec4::from([1.0, 0.0, 1.0, 1.0]) } }
    }
}

impl ScalarProduct<Rotor> for MultiVector {
    type Output = f32;

    fn scalar_product(self, other: Rotor) -> f32 {
        self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1]
    }
}

impl Into<Point> for MultiVector {
    fn into(self) -> Point {
        Point { groups: PointGroups { g0: Vec3::from([self.group0()[1], self.group1()[2], self.group1()[3]]) } }
    }
}

impl Add<Point> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Point) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() + Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 1.0, 0.0, 0.0]), g1: self.group1() + Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([0.0, 0.0, 1.0, 1.0]) } }
    }
}

impl AddAssign<Point> for MultiVector {
    fn add_assign(&mut self, other: Point) {
        *self = (*self).add(other);
    }
}

impl Sub<Point> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Point) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() - Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 1.0, 0.0, 0.0]), g1: self.group1() - Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([0.0, 0.0, 1.0, 1.0]) } }
    }
}

impl SubAssign<Point> for MultiVector {
    fn sub_assign(&mut self, other: Point) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Point> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: Point) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group1()[0]) * Vec4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[2]]) * Vec4::from([0.0, 0.0, 1.0, -1.0]) + Vec4::splat(self.group1()[1]) * Vec4::from([other.group0()[2], other.group0()[2], other.group0()[2], other.group0()[1]]) * Vec4::from([0.0, 0.0, -1.0, -1.0]) + Vec4::splat(self.group1()[2]) * Vec4::from([other.group0()[1], other.group0()[2], other.group0()[1], other.group0()[1]]) * Vec4::from([-1.0, 1.0, 0.0, 0.0]) + Vec4::splat(self.group1()[3]) * Vec4::from([other.group0()[2], other.group0()[1], other.group0()[2], other.group0()[2]]) * Vec4::from([-1.0, -1.0, 0.0, 0.0]) + self.group0().yxwz() * Vec4::splat(other.group0()[0]) * Vec4::from([-1.0, 1.0, -1.0, 1.0]), g1: Vec4::splat(self.group0()[1]) * Vec4::from([other.group0()[2], other.group0()[2], other.group0()[2], other.group0()[1]]) * Vec4::from([0.0, 0.0, -1.0, 1.0]) + Vec4::splat(self.group0()[3]) * Vec4::from([other.group0()[2], other.group0()[1], other.group0()[2], other.group0()[2]]) * Vec4::from([1.0, 1.0, 0.0, 0.0]) + Vec4::splat(self.group1()[0]) * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 1.0, 0.0, 0.0]) + Vec4::splat(self.group1()[1]) * Vec4::splat(other.group0()[0]) * Vec4::from([-1.0, 0.0, 0.0, 0.0]) + Vec4::splat(self.group1()[2]) * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 0.0, 0.0, -1.0]) + Vec4::splat(self.group1()[3]) * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 0.0, 1.0, 0.0]) + self.group0().zzxx() * Vec4::from([other.group0()[1], other.group0()[2], other.group0()[1], other.group0()[2]]) * Vec4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl ScalarProduct<Point> for MultiVector {
    type Output = f32;

    fn scalar_product(self, other: Point) -> f32 {
        0.0 - self.group0()[1] * other.group0()[0] - self.group1()[2] * other.group0()[1] - self.group1()[3] * other.group0()[2]
    }
}

impl Into<IdealPoint> for MultiVector {
    fn into(self) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Vec2::from([self.group1()[2], self.group1()[3]]) } }
    }
}

impl Add<IdealPoint> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: IdealPoint) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0(), g1: self.group1() + Vec4::from([other.group0()[0], other.group0()[0], other.group0()[0], other.group0()[1]]) * Vec4::from([0.0, 0.0, 1.0, 1.0]) } }
    }
}

impl AddAssign<IdealPoint> for MultiVector {
    fn add_assign(&mut self, other: IdealPoint) {
        *self = (*self).add(other);
    }
}

impl Sub<IdealPoint> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: IdealPoint) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0(), g1: self.group1() - Vec4::from([other.group0()[0], other.group0()[0], other.group0()[0], other.group0()[1]]) * Vec4::from([0.0, 0.0, 1.0, 1.0]) } }
    }
}

impl SubAssign<IdealPoint> for MultiVector {
    fn sub_assign(&mut self, other: IdealPoint) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<IdealPoint> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: IdealPoint) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group1()[1]) * Vec4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]) * Vec4::from([0.0, 0.0, -1.0, -1.0]) + Vec4::splat(self.group1()[3]) * Vec4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Vec4::from([-1.0, -1.0, 0.0, 0.0]) + self.group1().zzxx() * Vec4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[1]]) * Vec4::from([-1.0, 1.0, 1.0, -1.0]), g1: Vec4::splat(self.group0()[1]) * Vec4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]) * Vec4::from([0.0, 0.0, -1.0, 1.0]) + Vec4::splat(self.group0()[3]) * Vec4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Vec4::from([1.0, 1.0, 0.0, 0.0]) + self.group0().zzxx() * Vec4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[1]]) * Vec4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl ScalarProduct<IdealPoint> for MultiVector {
    type Output = f32;

    fn scalar_product(self, other: IdealPoint) -> f32 {
        0.0 - self.group1()[2] * other.group0()[0] - self.group1()[3] * other.group0()[1]
    }
}

impl Into<Plane> for MultiVector {
    fn into(self) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::from([self.group1()[0], self.group0()[3], self.group0()[2]]) } }
    }
}

impl Add<Plane> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Plane) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() + Vec4::from([other.group0()[0], other.group0()[0], other.group0()[2], other.group0()[1]]) * Vec4::from([0.0, 0.0, 1.0, 1.0]), g1: self.group1() + Vec4::splat(other.group0()[0]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl AddAssign<Plane> for MultiVector {
    fn add_assign(&mut self, other: Plane) {
        *self = (*self).add(other);
    }
}

impl Sub<Plane> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Plane) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() - Vec4::from([other.group0()[0], other.group0()[0], other.group0()[2], other.group0()[1]]) * Vec4::from([0.0, 0.0, 1.0, 1.0]), g1: self.group1() - Vec4::splat(other.group0()[0]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl SubAssign<Plane> for MultiVector {
    fn sub_assign(&mut self, other: Plane) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Plane> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: Plane) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[1]) * Vec4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[2]]) * Vec4::from([0.0, 0.0, 1.0, -1.0]) + Vec4::splat(self.group0()[3]) * Vec4::from([other.group0()[1], other.group0()[2], other.group0()[1], other.group0()[1]]) * Vec4::from([1.0, -1.0, 0.0, 0.0]) + Vec4::splat(self.group1()[0]) * Vec4::splat(other.group0()[0]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) + Vec4::splat(self.group1()[1]) * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 1.0, 0.0, 0.0]) + Vec4::splat(self.group1()[2]) * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 0.0, -1.0, 0.0]) + Vec4::splat(self.group1()[3]) * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 0.0, 0.0, 1.0]) + self.group0().zzxx() * Vec4::from([other.group0()[2], other.group0()[1], other.group0()[2], other.group0()[1]]), g1: Vec4::splat(self.group1()[0]) * Vec4::from([other.group0()[2], other.group0()[2], other.group0()[2], other.group0()[1]]) * Vec4::from([0.0, 0.0, 1.0, -1.0]) + Vec4::splat(self.group1()[1]) * Vec4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[2]]) * Vec4::from([0.0, 0.0, 1.0, 1.0]) + Vec4::splat(self.group1()[2]) * Vec4::from([other.group0()[2], other.group0()[1], other.group0()[2], other.group0()[2]]) * Vec4::from([1.0, 1.0, 0.0, 0.0]) + Vec4::splat(self.group1()[3]) * Vec4::from([other.group0()[1], other.group0()[2], other.group0()[1], other.group0()[1]]) * Vec4::from([-1.0, 1.0, 0.0, 0.0]) + self.group0() * Vec4::splat(other.group0()[0]) * Vec4::from([1.0, 1.0, -1.0, 1.0]) } }
    }
}

impl ScalarProduct<Plane> for MultiVector {
    type Output = f32;

    fn scalar_product(self, other: Plane) -> f32 {
        self.group0()[2] * other.group0()[2] + self.group0()[3] * other.group0()[1] + self.group1()[0] * other.group0()[0]
    }
}

impl Into<Translator> for MultiVector {
    fn into(self) -> Translator {
        Translator { groups: TranslatorGroups { g0: Vec3::from([self.group0()[0], self.group1()[2], self.group1()[3]]) } }
    }
}

impl Add<Translator> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() + Vec4::splat(other.group0()[0]) * Vec4::from([1.0, 0.0, 0.0, 0.0]), g1: self.group1() + Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([0.0, 0.0, 1.0, 1.0]) } }
    }
}

impl AddAssign<Translator> for MultiVector {
    fn add_assign(&mut self, other: Translator) {
        *self = (*self).add(other);
    }
}

impl Sub<Translator> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() - Vec4::splat(other.group0()[0]) * Vec4::from([1.0, 0.0, 0.0, 0.0]), g1: self.group1() - Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([0.0, 0.0, 1.0, 1.0]) } }
    }
}

impl SubAssign<Translator> for MultiVector {
    fn sub_assign(&mut self, other: Translator) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Translator> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group1()[0]) * Vec4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[2]]) * Vec4::from([0.0, 0.0, 1.0, -1.0]) + Vec4::splat(self.group1()[1]) * Vec4::from([other.group0()[2], other.group0()[2], other.group0()[2], other.group0()[1]]) * Vec4::from([0.0, 0.0, -1.0, -1.0]) + Vec4::splat(self.group1()[2]) * Vec4::from([other.group0()[1], other.group0()[2], other.group0()[1], other.group0()[1]]) * Vec4::from([-1.0, 1.0, 0.0, 0.0]) + Vec4::splat(self.group1()[3]) * Vec4::from([other.group0()[2], other.group0()[1], other.group0()[2], other.group0()[2]]) * Vec4::from([-1.0, -1.0, 0.0, 0.0]) + self.group0() * Vec4::splat(other.group0()[0]), g1: Vec4::splat(self.group0()[1]) * Vec4::from([other.group0()[2], other.group0()[2], other.group0()[2], other.group0()[1]]) * Vec4::from([0.0, 0.0, -1.0, 1.0]) + Vec4::splat(self.group0()[3]) * Vec4::from([other.group0()[2], other.group0()[1], other.group0()[2], other.group0()[2]]) * Vec4::from([1.0, 1.0, 0.0, 0.0]) + Vec4::splat(self.group1()[0]) * Vec4::splat(other.group0()[0]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) + Vec4::splat(self.group1()[1]) * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 1.0, 0.0, 0.0]) + Vec4::splat(self.group1()[2]) * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 0.0, 1.0, 0.0]) + Vec4::splat(self.group1()[3]) * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 0.0, 0.0, 1.0]) + self.group0().zzxx() * Vec4::from([other.group0()[1], other.group0()[2], other.group0()[1], other.group0()[2]]) * Vec4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl OuterProduct<Translator> for MultiVector {
    type Output = MultiVector;

    fn outer_product(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Vec4::splat(other.group0()[0]), g1: Vec4::splat(self.group0()[3]) * Vec4::splat(other.group0()[1]) * Vec4::from([0.0, 1.0, 0.0, 0.0]) + Vec4::splat(self.group1()[1]) * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 1.0, 0.0, 0.0]) + Vec4::splat(self.group1()[2]) * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 0.0, 1.0, 0.0]) + Vec4::splat(self.group1()[3]) * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 0.0, 0.0, 1.0]) + Vec4::from([self.group1()[0], self.group0()[2], self.group0()[0], self.group0()[0]]) * Vec4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group0()[2]]) } }
    }
}

impl InnerProduct<Translator> for MultiVector {
    type Output = MultiVector;

    fn inner_product(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group1()[0]) * Vec4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[2]]) * Vec4::from([0.0, 0.0, 1.0, -1.0]) + Vec4::splat(self.group1()[1]) * Vec4::from([other.group0()[2], other.group0()[2], other.group0()[2], other.group0()[1]]) * Vec4::from([0.0, 0.0, -1.0, -1.0]) + Vec4::splat(self.group1()[2]) * Vec4::splat(other.group0()[1]) * Vec4::from([-1.0, 0.0, 0.0, 0.0]) + Vec4::splat(self.group1()[3]) * Vec4::splat(other.group0()[2]) * Vec4::from([-1.0, 0.0, 0.0, 0.0]) + self.group0() * Vec4::splat(other.group0()[0]), g1: Vec4::splat(self.group0()[3]) * Vec4::splat(other.group0()[2]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) + Vec4::splat(self.group1()[0]) * Vec4::splat(other.group0()[0]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) + Vec4::splat(self.group1()[2]) * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 0.0, 1.0, 0.0]) + Vec4::splat(self.group1()[3]) * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 0.0, 0.0, 1.0]) + Vec4::from([self.group0()[2], self.group1()[1], self.group0()[0], self.group0()[0]]) * Vec4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl RightContraction<Translator> for MultiVector {
    type Output = MultiVector;

    fn right_contraction(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group1()[1]) * Vec4::from([other.group0()[2], other.group0()[2], other.group0()[2], other.group0()[1]]) * Vec4::from([0.0, 0.0, -1.0, -1.0]) + Vec4::splat(self.group1()[2]) * Vec4::splat(other.group0()[1]) * Vec4::from([-1.0, 0.0, 0.0, 0.0]) + Vec4::splat(self.group1()[3]) * Vec4::splat(other.group0()[2]) * Vec4::from([-1.0, 0.0, 0.0, 0.0]) + self.group0() * Vec4::splat(other.group0()[0]), g1: self.group1() * Vec4::splat(other.group0()[0]) } }
    }
}

impl ScalarProduct<Translator> for MultiVector {
    type Output = f32;

    fn scalar_product(self, other: Translator) -> f32 {
        self.group0()[0] * other.group0()[0] - self.group1()[2] * other.group0()[1] - self.group1()[3] * other.group0()[2]
    }
}

impl Into<Motor> for MultiVector {
    fn into(self) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::from([self.group0()[0], self.group0()[1], self.group1()[2], self.group1()[3]]) } }
    }
}

impl Add<Motor> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() + other.group0().xyxx() * Vec4::from([1.0, 1.0, 0.0, 0.0]), g1: self.group1() + other.group0().xxzw() * Vec4::from([0.0, 0.0, 1.0, 1.0]) } }
    }
}

impl AddAssign<Motor> for MultiVector {
    fn add_assign(&mut self, other: Motor) {
        *self = (*self).add(other);
    }
}

impl Sub<Motor> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() - other.group0().xyxx() * Vec4::from([1.0, 1.0, 0.0, 0.0]), g1: self.group1() - other.group0().xxzw() * Vec4::from([0.0, 0.0, 1.0, 1.0]) } }
    }
}

impl SubAssign<Motor> for MultiVector {
    fn sub_assign(&mut self, other: Motor) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Motor> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[1]) * other.group0().yxyy() * Vec4::from([-1.0, 1.0, 0.0, 0.0]) + Vec4::splat(self.group0()[3]) * other.group0().yyyx() * Vec4::from([0.0, 0.0, -1.0, 1.0]) + Vec4::splat(self.group1()[0]) * other.group0().zzzw() * Vec4::from([0.0, 0.0, 1.0, -1.0]) + Vec4::splat(self.group1()[1]) * other.group0().wwwz() * Vec4::from([0.0, 0.0, -1.0, -1.0]) + Vec4::splat(self.group1()[2]) * other.group0().zwzz() * Vec4::from([-1.0, 1.0, 0.0, 0.0]) + Vec4::splat(self.group1()[3]) * other.group0().wzww() * Vec4::from([-1.0, -1.0, 0.0, 0.0]) + self.group0().xxzz() * other.group0().xyxy(), g1: Vec4::splat(self.group0()[1]) * other.group0().wwwz() * Vec4::from([0.0, 0.0, -1.0, 1.0]) + Vec4::splat(self.group0()[3]) * other.group0().wzww() * Vec4::from([1.0, 1.0, 0.0, 0.0]) + Vec4::splat(self.group1()[0]) * other.group0().xyxx() * Vec4::from([1.0, 1.0, 0.0, 0.0]) + Vec4::splat(self.group1()[1]) * other.group0().yxyy() * Vec4::from([-1.0, 1.0, 0.0, 0.0]) + Vec4::splat(self.group1()[2]) * other.group0().xxxy() * Vec4::from([0.0, 0.0, 1.0, -1.0]) + Vec4::splat(self.group1()[3]) * other.group0().yyyx() * Vec4::from([0.0, 0.0, 1.0, 1.0]) + self.group0().zzxx() * other.group0().zwzw() * Vec4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl OuterProduct<Motor> for MultiVector {
    type Output = MultiVector;

    fn outer_product(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[1]) * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 1.0, 0.0, 0.0]) + self.group0().xxzw() * other.group0().xyxx(), g1: Vec4::splat(self.group0()[3]) * Vec4::splat(other.group0()[2]) * Vec4::from([0.0, 1.0, 0.0, 0.0]) + Vec4::splat(self.group1()[0]) * other.group0().xyxx() * Vec4::from([1.0, 1.0, 0.0, 0.0]) + Vec4::splat(self.group1()[1]) * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 1.0, 0.0, 0.0]) + Vec4::splat(self.group1()[2]) * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 0.0, 1.0, 0.0]) + Vec4::splat(self.group1()[3]) * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 0.0, 0.0, 1.0]) + self.group0().xzxx() * other.group0().xwzw() * Vec4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl InnerProduct<Motor> for MultiVector {
    type Output = MultiVector;

    fn inner_product(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[1]) * other.group0().yxyy() * Vec4::from([-1.0, 1.0, 0.0, 0.0]) + Vec4::splat(self.group0()[3]) * other.group0().yyyx() * Vec4::from([0.0, 0.0, -1.0, 1.0]) + Vec4::splat(self.group1()[0]) * other.group0().zzzw() * Vec4::from([0.0, 0.0, 1.0, -1.0]) + Vec4::splat(self.group1()[1]) * other.group0().wwwz() * Vec4::from([0.0, 0.0, -1.0, -1.0]) + Vec4::splat(self.group1()[2]) * Vec4::splat(other.group0()[2]) * Vec4::from([-1.0, 0.0, 0.0, 0.0]) + Vec4::splat(self.group1()[3]) * Vec4::splat(other.group0()[3]) * Vec4::from([-1.0, 0.0, 0.0, 0.0]) + self.group0().xxzz() * other.group0().xyxy(), g1: Vec4::splat(self.group0()[3]) * Vec4::splat(other.group0()[3]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) + Vec4::splat(self.group1()[0]) * Vec4::splat(other.group0()[0]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) + Vec4::splat(self.group1()[1]) * other.group0().yxyy() * Vec4::from([-1.0, 1.0, 0.0, 0.0]) + Vec4::splat(self.group1()[2]) * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 0.0, 1.0, 0.0]) + Vec4::splat(self.group1()[3]) * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 0.0, 0.0, 1.0]) + self.group0().zxxx() * other.group0().zxzw() * Vec4::from([-1.0, 0.0, 1.0, 1.0]) } }
    }
}

impl RightContraction<Motor> for MultiVector {
    type Output = MultiVector;

    fn right_contraction(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[1]) * other.group0().yxyy() * Vec4::from([-1.0, 1.0, 0.0, 0.0]) + Vec4::splat(self.group1()[1]) * other.group0().wwwz() * Vec4::from([0.0, 0.0, -1.0, -1.0]) + Vec4::splat(self.group1()[2]) * Vec4::splat(other.group0()[2]) * Vec4::from([-1.0, 0.0, 0.0, 0.0]) + Vec4::splat(self.group1()[3]) * Vec4::splat(other.group0()[3]) * Vec4::from([-1.0, 0.0, 0.0, 0.0]) + self.group0().xxzw() * Vec4::splat(other.group0()[0]) * Vec4::from([1.0, 0.0, 1.0, 1.0]), g1: Vec4::splat(self.group1()[1]) * other.group0().yxyy() * Vec4::from([-1.0, 1.0, 0.0, 0.0]) + self.group1().xxzw() * Vec4::splat(other.group0()[0]) * Vec4::from([1.0, 0.0, 1.0, 1.0]) } }
    }
}

impl ScalarProduct<Motor> for MultiVector {
    type Output = f32;

    fn scalar_product(self, other: Motor) -> f32 {
        self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2] - self.group1()[3] * other.group0()[3]
    }
}

impl Into<MotorDual> for MultiVector {
    fn into(self) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::from([self.group1()[1], self.group1()[0], self.group0()[3], self.group0()[2]]) } }
    }
}

impl Add<MotorDual> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: MotorDual) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() + other.group0().xxwz() * Vec4::from([0.0, 0.0, 1.0, 1.0]), g1: self.group1() + other.group0().yxxx() * Vec4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl AddAssign<MotorDual> for MultiVector {
    fn add_assign(&mut self, other: MotorDual) {
        *self = (*self).add(other);
    }
}

impl Sub<MotorDual> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: MotorDual) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() - other.group0().xxwz() * Vec4::from([0.0, 0.0, 1.0, 1.0]), g1: self.group1() - other.group0().yxxx() * Vec4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl SubAssign<MotorDual> for MultiVector {
    fn sub_assign(&mut self, other: MotorDual) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<MotorDual> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: MotorDual) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[1]) * other.group0().zzzw() * Vec4::from([0.0, 0.0, 1.0, -1.0]) + Vec4::splat(self.group0()[3]) * other.group0().zwzz() * Vec4::from([1.0, -1.0, 0.0, 0.0]) + Vec4::splat(self.group1()[0]) * other.group0().yxyy() * Vec4::from([1.0, 1.0, 0.0, 0.0]) + Vec4::splat(self.group1()[1]) * other.group0().xyxx() * Vec4::from([-1.0, 1.0, 0.0, 0.0]) + Vec4::splat(self.group1()[2]) * other.group0().yyyx() * Vec4::from([0.0, 0.0, -1.0, -1.0]) + Vec4::splat(self.group1()[3]) * other.group0().xxxy() * Vec4::from([0.0, 0.0, -1.0, 1.0]) + self.group0().zzxx() * other.group0().wzwz(), g1: Vec4::splat(self.group0()[1]) * other.group0().xyxx() * Vec4::from([-1.0, 1.0, 0.0, 0.0]) + Vec4::splat(self.group0()[3]) * other.group0().xxxy() * Vec4::from([0.0, 0.0, 1.0, 1.0]) + Vec4::splat(self.group1()[0]) * other.group0().wwwz() * Vec4::from([0.0, 0.0, 1.0, -1.0]) + Vec4::splat(self.group1()[1]) * other.group0().zzzw() * Vec4::from([0.0, 0.0, 1.0, 1.0]) + Vec4::splat(self.group1()[2]) * other.group0().wzww() * Vec4::from([1.0, 1.0, 0.0, 0.0]) + Vec4::splat(self.group1()[3]) * other.group0().zwzz() * Vec4::from([-1.0, 1.0, 0.0, 0.0]) + self.group0().xxzz() * other.group0().yxyx() * Vec4::from([1.0, 1.0, -1.0, 1.0]) } }
    }
}

impl RegressiveProduct<MotorDual> for MultiVector {
    type Output = MultiVector;

    fn regressive_product(self, other: MotorDual) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[1]) * other.group0().yxyy() * Vec4::from([1.0, 1.0, 0.0, 0.0]) + Vec4::splat(self.group1()[1]) * other.group0().wwwz() * Vec4::from([0.0, 0.0, 1.0, 1.0]) + Vec4::splat(self.group1()[2]) * Vec4::splat(other.group0()[2]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) + Vec4::splat(self.group1()[3]) * Vec4::splat(other.group0()[3]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) + self.group0().xxzw() * Vec4::splat(other.group0()[0]) * Vec4::from([1.0, 0.0, 1.0, 1.0]), g1: Vec4::splat(self.group1()[1]) * other.group0().yxyy() * Vec4::from([1.0, 1.0, 0.0, 0.0]) + self.group1().xxzw() * Vec4::splat(other.group0()[0]) * Vec4::from([1.0, 0.0, 1.0, 1.0]) } }
    }
}

impl InnerProduct<MotorDual> for MultiVector {
    type Output = MultiVector;

    fn inner_product(self, other: MotorDual) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[1]) * other.group0().zzzw() * Vec4::from([0.0, 0.0, 1.0, -1.0]) + Vec4::splat(self.group0()[3]) * Vec4::splat(other.group0()[2]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) + Vec4::splat(self.group1()[0]) * other.group0().yxyy() * Vec4::from([1.0, 1.0, 0.0, 0.0]) + Vec4::splat(self.group1()[1]) * other.group0().xyxx() * Vec4::from([-1.0, 1.0, 0.0, 0.0]) + Vec4::splat(self.group1()[2]) * other.group0().yyyx() * Vec4::from([0.0, 0.0, -1.0, -1.0]) + Vec4::splat(self.group1()[3]) * other.group0().xxxy() * Vec4::from([0.0, 0.0, -1.0, 1.0]) + self.group0().zxxx() * other.group0().wxwz() * Vec4::from([1.0, 0.0, 1.0, 1.0]), g1: Vec4::splat(self.group0()[1]) * Vec4::splat(other.group0()[0]) * Vec4::from([-1.0, 0.0, 0.0, 0.0]) + Vec4::splat(self.group1()[1]) * other.group0().zzzw() * Vec4::from([0.0, 0.0, 1.0, 1.0]) + Vec4::splat(self.group1()[2]) * Vec4::splat(other.group0()[3]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) + Vec4::splat(self.group1()[3]) * Vec4::splat(other.group0()[2]) * Vec4::from([-1.0, 0.0, 0.0, 0.0]) + self.group0().xxwz() * other.group0().yxxx() } }
    }
}

impl LeftContraction<MotorDual> for MultiVector {
    type Output = MultiVector;

    fn left_contraction(self, other: MotorDual) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[3]) * Vec4::splat(other.group0()[2]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) + Vec4::splat(self.group1()[0]) * other.group0().yxyy() * Vec4::from([1.0, 1.0, 0.0, 0.0]) + Vec4::splat(self.group1()[1]) * Vec4::splat(other.group0()[0]) * Vec4::from([-1.0, 0.0, 0.0, 0.0]) + Vec4::splat(self.group1()[2]) * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 0.0, 0.0, -1.0]) + Vec4::splat(self.group1()[3]) * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 0.0, -1.0, 0.0]) + self.group0().zxxx() * other.group0().wxwz() * Vec4::from([1.0, 0.0, 1.0, 1.0]), g1: Vec4::splat(self.group0()[1]) * Vec4::splat(other.group0()[0]) * Vec4::from([-1.0, 0.0, 0.0, 0.0]) + self.group0().xxwz() * other.group0().yxxx() } }
    }
}

impl ScalarProduct<MotorDual> for MultiVector {
    type Output = f32;

    fn scalar_product(self, other: MotorDual) -> f32 {
        self.group0()[2] * other.group0()[3] + self.group0()[3] * other.group0()[2] + self.group1()[0] * other.group0()[1] - self.group1()[1] * other.group0()[0]
    }
}

impl SquaredMagnitude for MultiVector {
    type Output = f32;

    fn squared_magnitude(self) -> f32 {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for MultiVector {
    type Output = f32;

    fn magnitude(self) -> f32 {
        self.squared_magnitude().sqrt()
    }
}

impl Mul<f32> for MultiVector {
    type Output = MultiVector;

    fn mul(self, other: f32) -> MultiVector {
        self.geometric_product(other)
    }
}

impl MulAssign<f32> for MultiVector {
    fn mul_assign(&mut self, other: f32) {
        *self = (*self).mul(other);
    }
}

impl Signum for MultiVector {
    type Output = MultiVector;

    fn signum(self) -> MultiVector {
        self.geometric_product(1.0 / self.magnitude())
    }
}

impl Inverse for MultiVector {
    type Output = MultiVector;

    fn inverse(self) -> MultiVector {
        self.reversal().geometric_product(1.0 / self.squared_magnitude())
    }
}

impl Zero for Rotor {
    fn zero() -> Self {
        Rotor { groups: RotorGroups { g0: Vec2::splat(0.0) } }
    }
}

impl One for Rotor {
    fn one() -> Self {
        Rotor { groups: RotorGroups { g0: Vec2::from([1.0, 0.0]) } }
    }
}

impl Neg for Rotor {
    type Output = Rotor;

    fn neg(self) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Vec2::splat(-1.0) } }
    }
}

impl Automorphism for Rotor {
    type Output = Rotor;

    fn automorphism(self) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() } }
    }
}

impl Reversal for Rotor {
    type Output = Rotor;

    fn reversal(self) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Vec2::from([1.0, -1.0]) } }
    }
}

impl Conjugation for Rotor {
    type Output = Rotor;

    fn conjugation(self) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Vec2::from([1.0, -1.0]) } }
    }
}

impl Into<f32> for Rotor {
    fn into(self) -> f32 {
        self.group0()[0]
    }
}

impl Add<f32> for Rotor {
    type Output = Rotor;

    fn add(self, other: f32) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() + Vec2::splat(other) * Vec2::from([1.0, 0.0]) } }
    }
}

impl AddAssign<f32> for Rotor {
    fn add_assign(&mut self, other: f32) {
        *self = (*self).add(other);
    }
}

impl Sub<f32> for Rotor {
    type Output = Rotor;

    fn sub(self, other: f32) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() - Vec2::splat(other) * Vec2::from([1.0, 0.0]) } }
    }
}

impl SubAssign<f32> for Rotor {
    fn sub_assign(&mut self, other: f32) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<f32> for Rotor {
    type Output = Rotor;

    fn geometric_product(self, other: f32) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Vec2::splat(other) } }
    }
}

impl OuterProduct<f32> for Rotor {
    type Output = Rotor;

    fn outer_product(self, other: f32) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Vec2::splat(other) } }
    }
}

impl InnerProduct<f32> for Rotor {
    type Output = Rotor;

    fn inner_product(self, other: f32) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Vec2::splat(other) } }
    }
}

impl LeftContraction<f32> for Rotor {
    type Output = f32;

    fn left_contraction(self, other: f32) -> f32 {
        self.group0()[0] * other
    }
}

impl RightContraction<f32> for Rotor {
    type Output = Rotor;

    fn right_contraction(self, other: f32) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Vec2::splat(other) } }
    }
}

impl ScalarProduct<f32> for Rotor {
    type Output = f32;

    fn scalar_product(self, other: f32) -> f32 {
        self.group0()[0] * other
    }
}

impl Add<MultiVector> for Rotor {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::from([self.group0()[0], self.group0()[1], self.group0()[0], self.group0()[0]]) * Vec4::from([1.0, 1.0, 0.0, 0.0]) + other.group0(), g1: other.group1() } }
    }
}

impl Sub<MultiVector> for Rotor {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::from([self.group0()[0], self.group0()[1], self.group0()[0], self.group0()[0]]) * Vec4::from([1.0, 1.0, 0.0, 0.0]) - other.group0(), g1: Vec4::splat(0.0) - other.group1() } }
    }
}

impl GeometricProduct<MultiVector> for Rotor {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + Vec4::splat(self.group0()[1]) * other.group0().yxwz() * Vec4::from([-1.0, 1.0, 1.0, -1.0]), g1: Vec4::splat(self.group0()[0]) * other.group1() + Vec4::splat(self.group0()[1]) * other.group1().yxwz() * Vec4::from([-1.0, 1.0, -1.0, 1.0]) } }
    }
}

impl OuterProduct<MultiVector> for Rotor {
    type Output = MultiVector;

    fn outer_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + Vec4::from([self.group0()[0], self.group0()[1], self.group0()[0], self.group0()[0]]) * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 1.0, 0.0, 0.0]), g1: Vec4::splat(self.group0()[0]) * other.group1() + Vec4::from([self.group0()[0], self.group0()[1], self.group0()[0], self.group0()[0]]) * Vec4::splat(other.group1()[0]) * Vec4::from([0.0, 1.0, 0.0, 0.0]) } }
    }
}

impl InnerProduct<MultiVector> for Rotor {
    type Output = MultiVector;

    fn inner_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + Vec4::splat(self.group0()[1]) * other.group0().yxwz() * Vec4::from([-1.0, 1.0, 1.0, -1.0]), g1: Vec4::splat(self.group0()[0]) * other.group1() + Vec4::from([self.group0()[1], self.group0()[0], self.group0()[0], self.group0()[0]]) * other.group1().yxxx() * Vec4::from([-1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl LeftContraction<MultiVector> for Rotor {
    type Output = MultiVector;

    fn left_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + Vec4::from([self.group0()[1], self.group0()[0], self.group0()[0], self.group0()[0]]) * other.group0().yxxx() * Vec4::from([-1.0, 0.0, 0.0, 0.0]), g1: Vec4::splat(self.group0()[0]) * other.group1() + Vec4::from([self.group0()[1], self.group0()[0], self.group0()[0], self.group0()[0]]) * other.group1().yxxx() * Vec4::from([-1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl ScalarProduct<MultiVector> for Rotor {
    type Output = f32;

    fn scalar_product(self, other: MultiVector) -> f32 {
        self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1]
    }
}

impl Add<Rotor> for Rotor {
    type Output = Rotor;

    fn add(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() + other.group0() } }
    }
}

impl AddAssign<Rotor> for Rotor {
    fn add_assign(&mut self, other: Rotor) {
        *self = (*self).add(other);
    }
}

impl Sub<Rotor> for Rotor {
    type Output = Rotor;

    fn sub(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() - other.group0() } }
    }
}

impl SubAssign<Rotor> for Rotor {
    fn sub_assign(&mut self, other: Rotor) {
        *self = (*self).sub(other);
    }
}

impl Mul<Rotor> for Rotor {
    type Output = Rotor;

    fn mul(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * other.group0() } }
    }
}

impl MulAssign<Rotor> for Rotor {
    fn mul_assign(&mut self, other: Rotor) {
        *self = (*self).mul(other);
    }
}

impl Div<Rotor> for Rotor {
    type Output = Rotor;

    fn div(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Vec2::from([self.group0()[0], self.group0()[1]]) * Vec2::from([1.0, 1.0]) / Vec2::from([other.group0()[0], other.group0()[1]]) * Vec2::from([1.0, 1.0]) } }
    }
}

impl DivAssign<Rotor> for Rotor {
    fn div_assign(&mut self, other: Rotor) {
        *self = (*self).div(other);
    }
}

impl GeometricProduct<Rotor> for Rotor {
    type Output = Rotor;

    fn geometric_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Vec2::splat(self.group0()[0]) * other.group0() + Vec2::splat(self.group0()[1]) * other.group0().yx() * Vec2::from([-1.0, 1.0]) } }
    }
}

impl OuterProduct<Rotor> for Rotor {
    type Output = Rotor;

    fn outer_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Vec2::splat(self.group0()[0]) * other.group0() + self.group0() * Vec2::splat(other.group0()[0]) * Vec2::from([0.0, 1.0]) } }
    }
}

impl InnerProduct<Rotor> for Rotor {
    type Output = Rotor;

    fn inner_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Vec2::splat(self.group0()[0]) * other.group0() + Vec2::splat(self.group0()[1]) * other.group0().yx() * Vec2::from([-1.0, 1.0]) } }
    }
}

impl LeftContraction<Rotor> for Rotor {
    type Output = Rotor;

    fn left_contraction(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Vec2::splat(self.group0()[0]) * other.group0() + self.group0().yx() * other.group0().yx() * Vec2::from([-1.0, 0.0]) } }
    }
}

impl RightContraction<Rotor> for Rotor {
    type Output = Rotor;

    fn right_contraction(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Vec2::splat(self.group0()[1]) * other.group0().yx() * Vec2::from([-1.0, 1.0]) + Vec2::splat(self.group0()[0]) * Vec2::splat(other.group0()[0]) * Vec2::from([1.0, 0.0]) } }
    }
}

impl ScalarProduct<Rotor> for Rotor {
    type Output = f32;

    fn scalar_product(self, other: Rotor) -> f32 {
        self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1]
    }
}

impl Add<Point> for Rotor {
    type Output = Motor;

    fn add(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::from([self.group0()[0], self.group0()[1], self.group0()[0], self.group0()[0]]) * Vec4::from([1.0, 1.0, 0.0, 0.0]) + Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl Sub<Point> for Rotor {
    type Output = Motor;

    fn sub(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::from([self.group0()[0], self.group0()[1], self.group0()[0], self.group0()[0]]) * Vec4::from([1.0, 1.0, 0.0, 0.0]) - Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl GeometricProduct<Point> for Rotor {
    type Output = Motor;

    fn geometric_product(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[1]) * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[2], other.group0()[1]]) * Vec4::from([-1.0, 0.0, -1.0, 1.0]) + Vec4::splat(self.group0()[0]) * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl OuterProduct<Point> for Rotor {
    type Output = Point;

    fn outer_product(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Vec3::splat(self.group0()[0]) * other.group0() } }
    }
}

impl InnerProduct<Point> for Rotor {
    type Output = Motor;

    fn inner_product(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::from([self.group0()[1], self.group0()[0], self.group0()[0], self.group0()[0]]) * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl LeftContraction<Point> for Rotor {
    type Output = Motor;

    fn left_contraction(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::from([self.group0()[1], self.group0()[0], self.group0()[0], self.group0()[0]]) * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl RightContraction<Point> for Rotor {
    type Output = f32;

    fn right_contraction(self, other: Point) -> f32 {
        0.0 - self.group0()[1] * other.group0()[0]
    }
}

impl ScalarProduct<Point> for Rotor {
    type Output = f32;

    fn scalar_product(self, other: Point) -> f32 {
        0.0 - self.group0()[1] * other.group0()[0]
    }
}

impl Add<IdealPoint> for Rotor {
    type Output = Motor;

    fn add(self, other: IdealPoint) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::from([self.group0()[0], self.group0()[1], self.group0()[0], self.group0()[0]]) * Vec4::from([1.0, 1.0, 0.0, 0.0]) + Vec4::from([other.group0()[0], other.group0()[0], other.group0()[0], other.group0()[1]]) * Vec4::from([0.0, 0.0, 1.0, 1.0]) } }
    }
}

impl Sub<IdealPoint> for Rotor {
    type Output = Motor;

    fn sub(self, other: IdealPoint) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::from([self.group0()[0], self.group0()[1], self.group0()[0], self.group0()[0]]) * Vec4::from([1.0, 1.0, 0.0, 0.0]) - Vec4::from([other.group0()[0], other.group0()[0], other.group0()[0], other.group0()[1]]) * Vec4::from([0.0, 0.0, 1.0, 1.0]) } }
    }
}

impl GeometricProduct<IdealPoint> for Rotor {
    type Output = IdealPoint;

    fn geometric_product(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Vec2::splat(self.group0()[0]) * other.group0() + Vec2::splat(self.group0()[1]) * other.group0().yx() * Vec2::from([-1.0, 1.0]) } }
    }
}

impl OuterProduct<IdealPoint> for Rotor {
    type Output = IdealPoint;

    fn outer_product(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Vec2::splat(self.group0()[0]) * other.group0() } }
    }
}

impl InnerProduct<IdealPoint> for Rotor {
    type Output = IdealPoint;

    fn inner_product(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Vec2::splat(self.group0()[0]) * other.group0() } }
    }
}

impl LeftContraction<IdealPoint> for Rotor {
    type Output = IdealPoint;

    fn left_contraction(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Vec2::splat(self.group0()[0]) * other.group0() } }
    }
}

impl GeometricProduct<Plane> for Rotor {
    type Output = MotorDual;

    fn geometric_product(self, other: Plane) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[1]) * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[2], other.group0()[1]]) * Vec4::from([1.0, 0.0, -1.0, 1.0]) + Vec4::splat(self.group0()[0]) * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl RegressiveProduct<Plane> for Rotor {
    type Output = f32;

    fn regressive_product(self, other: Plane) -> f32 {
        self.group0()[1] * other.group0()[0]
    }
}

impl OuterProduct<Plane> for Rotor {
    type Output = MotorDual;

    fn outer_product(self, other: Plane) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::from([self.group0()[1], self.group0()[0], self.group0()[0], self.group0()[0]]) * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl InnerProduct<Plane> for Rotor {
    type Output = Plane;

    fn inner_product(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[0]) * other.group0() + Vec3::from([self.group0()[0], self.group0()[1], self.group0()[1]]) * other.group0().xzy() * Vec3::from([0.0, -1.0, 1.0]) } }
    }
}

impl LeftContraction<Plane> for Rotor {
    type Output = Plane;

    fn left_contraction(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[0]) * other.group0() } }
    }
}

impl Add<Translator> for Rotor {
    type Output = Motor;

    fn add(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::from([self.group0()[0], self.group0()[1], self.group0()[0], self.group0()[0]]) * Vec4::from([1.0, 1.0, 0.0, 0.0]) + Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([1.0, 0.0, 1.0, 1.0]) } }
    }
}

impl Sub<Translator> for Rotor {
    type Output = Motor;

    fn sub(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::from([self.group0()[0], self.group0()[1], self.group0()[0], self.group0()[0]]) * Vec4::from([1.0, 1.0, 0.0, 0.0]) - Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([1.0, 0.0, 1.0, 1.0]) } }
    }
}

impl GeometricProduct<Translator> for Rotor {
    type Output = Motor;

    fn geometric_product(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[1]) * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[2], other.group0()[1]]) * Vec4::from([0.0, 1.0, -1.0, 1.0]) + Vec4::splat(self.group0()[0]) * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([1.0, 0.0, 1.0, 1.0]) } }
    }
}

impl OuterProduct<Translator> for Rotor {
    type Output = Motor;

    fn outer_product(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::from([self.group0()[0], self.group0()[1], self.group0()[0], self.group0()[0]]) * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl InnerProduct<Translator> for Rotor {
    type Output = Motor;

    fn inner_product(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::from([self.group0()[0], self.group0()[1], self.group0()[0], self.group0()[0]]) * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl LeftContraction<Translator> for Rotor {
    type Output = Translator;

    fn left_contraction(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Vec3::splat(self.group0()[0]) * other.group0() } }
    }
}

impl RightContraction<Translator> for Rotor {
    type Output = Rotor;

    fn right_contraction(self, other: Translator) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Vec2::splat(other.group0()[0]) } }
    }
}

impl ScalarProduct<Translator> for Rotor {
    type Output = f32;

    fn scalar_product(self, other: Translator) -> f32 {
        self.group0()[0] * other.group0()[0]
    }
}

impl Add<Motor> for Rotor {
    type Output = Motor;

    fn add(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::from([self.group0()[0], self.group0()[1], self.group0()[0], self.group0()[0]]) * Vec4::from([1.0, 1.0, 0.0, 0.0]) + other.group0() } }
    }
}

impl Sub<Motor> for Rotor {
    type Output = Motor;

    fn sub(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::from([self.group0()[0], self.group0()[1], self.group0()[0], self.group0()[0]]) * Vec4::from([1.0, 1.0, 0.0, 0.0]) - other.group0() } }
    }
}

impl GeometricProduct<Motor> for Rotor {
    type Output = Motor;

    fn geometric_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + Vec4::splat(self.group0()[1]) * other.group0().yxwz() * Vec4::from([-1.0, 1.0, -1.0, 1.0]) } }
    }
}

impl OuterProduct<Motor> for Rotor {
    type Output = Motor;

    fn outer_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + Vec4::from([self.group0()[0], self.group0()[1], self.group0()[0], self.group0()[0]]) * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 1.0, 0.0, 0.0]) } }
    }
}

impl InnerProduct<Motor> for Rotor {
    type Output = Motor;

    fn inner_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + Vec4::from([self.group0()[1], self.group0()[1], self.group0()[0], self.group0()[0]]) * other.group0().yxxx() * Vec4::from([-1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl LeftContraction<Motor> for Rotor {
    type Output = Motor;

    fn left_contraction(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + Vec4::from([self.group0()[1], self.group0()[0], self.group0()[0], self.group0()[0]]) * other.group0().yxxx() * Vec4::from([-1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl RightContraction<Motor> for Rotor {
    type Output = Rotor;

    fn right_contraction(self, other: Motor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Vec2::splat(self.group0()[1]) * Vec2::from([other.group0()[1], other.group0()[0]]) * Vec2::from([-1.0, 1.0]) + Vec2::splat(self.group0()[0]) * Vec2::splat(other.group0()[0]) * Vec2::from([1.0, 0.0]) } }
    }
}

impl ScalarProduct<Motor> for Rotor {
    type Output = f32;

    fn scalar_product(self, other: Motor) -> f32 {
        self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1]
    }
}

impl GeometricProduct<MotorDual> for Rotor {
    type Output = MotorDual;

    fn geometric_product(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + Vec4::splat(self.group0()[1]) * other.group0().yxwz() * Vec4::from([1.0, -1.0, -1.0, 1.0]) } }
    }
}

impl RegressiveProduct<MotorDual> for Rotor {
    type Output = Rotor;

    fn regressive_product(self, other: MotorDual) -> Rotor {
        Rotor { groups: RotorGroups { g0: Vec2::splat(self.group0()[1]) * Vec2::from([other.group0()[1], other.group0()[0]]) + Vec2::splat(self.group0()[0]) * Vec2::splat(other.group0()[0]) * Vec2::from([1.0, 0.0]) } }
    }
}

impl OuterProduct<MotorDual> for Rotor {
    type Output = MotorDual;

    fn outer_product(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + Vec4::from([self.group0()[1], self.group0()[0], self.group0()[0], self.group0()[0]]) * other.group0().yxxx() * Vec4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl InnerProduct<MotorDual> for Rotor {
    type Output = MotorDual;

    fn inner_product(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + Vec4::from([self.group0()[0], self.group0()[1], self.group0()[1], self.group0()[1]]) * other.group0().xxwz() * Vec4::from([0.0, -1.0, -1.0, 1.0]) } }
    }
}

impl LeftContraction<MotorDual> for Rotor {
    type Output = MotorDual;

    fn left_contraction(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + Vec4::from([self.group0()[0], self.group0()[1], self.group0()[0], self.group0()[0]]) * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, -1.0, 0.0, 0.0]) } }
    }
}

impl SquaredMagnitude for Rotor {
    type Output = f32;

    fn squared_magnitude(self) -> f32 {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for Rotor {
    type Output = f32;

    fn magnitude(self) -> f32 {
        self.squared_magnitude().sqrt()
    }
}

impl Mul<f32> for Rotor {
    type Output = Rotor;

    fn mul(self, other: f32) -> Rotor {
        self.geometric_product(other)
    }
}

impl MulAssign<f32> for Rotor {
    fn mul_assign(&mut self, other: f32) {
        *self = (*self).mul(other);
    }
}

impl Signum for Rotor {
    type Output = Rotor;

    fn signum(self) -> Rotor {
        self.geometric_product(1.0 / self.magnitude())
    }
}

impl Inverse for Rotor {
    type Output = Rotor;

    fn inverse(self) -> Rotor {
        self.reversal().geometric_product(1.0 / self.squared_magnitude())
    }
}

impl Zero for Point {
    fn zero() -> Self {
        Point { groups: PointGroups { g0: Vec3::splat(0.0) } }
    }
}

impl One for Point {
    fn one() -> Self {
        Point { groups: PointGroups { g0: Vec3::splat(0.0) } }
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Vec3::splat(-1.0) } }
    }
}

impl Automorphism for Point {
    type Output = Point;

    fn automorphism(self) -> Point {
        Point { groups: PointGroups { g0: self.group0() } }
    }
}

impl Reversal for Point {
    type Output = Point;

    fn reversal(self) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Vec3::splat(-1.0) } }
    }
}

impl Conjugation for Point {
    type Output = Point;

    fn conjugation(self) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Vec3::splat(-1.0) } }
    }
}

impl Dual for Point {
    type Output = Plane;

    fn dual(self) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() } }
    }
}

impl Add<f32> for Point {
    type Output = Motor;

    fn add(self, other: f32) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Vec4::from([0.0, 1.0, 1.0, 1.0]) + Vec4::splat(other) * Vec4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl Sub<f32> for Point {
    type Output = Motor;

    fn sub(self, other: f32) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Vec4::from([0.0, 1.0, 1.0, 1.0]) - Vec4::splat(other) * Vec4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl GeometricProduct<f32> for Point {
    type Output = Point;

    fn geometric_product(self, other: f32) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Vec3::splat(other) } }
    }
}

impl OuterProduct<f32> for Point {
    type Output = Point;

    fn outer_product(self, other: f32) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Vec3::splat(other) } }
    }
}

impl InnerProduct<f32> for Point {
    type Output = Point;

    fn inner_product(self, other: f32) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Vec3::splat(other) } }
    }
}

impl RightContraction<f32> for Point {
    type Output = Point;

    fn right_contraction(self, other: f32) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Vec3::splat(other) } }
    }
}

impl Add<MultiVector> for Point {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[0]) * Vec4::from([0.0, 1.0, 0.0, 0.0]) + other.group0(), g1: Vec4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Vec4::from([0.0, 0.0, 1.0, 1.0]) + other.group1() } }
    }
}

impl Sub<MultiVector> for Point {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[0]) * Vec4::from([0.0, 1.0, 0.0, 0.0]) - other.group0(), g1: Vec4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Vec4::from([0.0, 0.0, 1.0, 1.0]) - other.group1() } }
    }
}

impl GeometricProduct<MultiVector> for Point {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[0]) * other.group0().yxwz() * Vec4::from([-1.0, 1.0, 1.0, -1.0]) + Vec4::splat(self.group0()[1]) * other.group1().zwxy() * Vec4::from([-1.0, 1.0, -1.0, -1.0]) + Vec4::splat(self.group0()[2]) * other.group1().wzyx() * Vec4::from([-1.0, -1.0, -1.0, 1.0]), g1: Vec4::splat(self.group0()[0]) * other.group1().yxwz() * Vec4::from([-1.0, 1.0, -1.0, 1.0]) + Vec4::splat(self.group0()[1]) * other.group0().zwxy() * Vec4::from([1.0, 1.0, 1.0, -1.0]) + Vec4::splat(self.group0()[2]) * other.group0().wzyx() * Vec4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl ScalarProduct<MultiVector> for Point {
    type Output = f32;

    fn scalar_product(self, other: MultiVector) -> f32 {
        0.0 - self.group0()[0] * other.group0()[1] - self.group0()[1] * other.group1()[2] - self.group0()[2] * other.group1()[3]
    }
}

impl Add<Rotor> for Point {
    type Output = Motor;

    fn add(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Vec4::from([0.0, 1.0, 1.0, 1.0]) + Vec4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[0]]) * Vec4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl Sub<Rotor> for Point {
    type Output = Motor;

    fn sub(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Vec4::from([0.0, 1.0, 1.0, 1.0]) - Vec4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[0]]) * Vec4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl GeometricProduct<Rotor> for Point {
    type Output = Motor;

    fn geometric_product(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[2]) * Vec4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]) * Vec4::from([0.0, 0.0, 1.0, 1.0]) + Vec4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[1]]) * Vec4::from([other.group0()[1], other.group0()[0], other.group0()[0], other.group0()[1]]) * Vec4::from([-1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl OuterProduct<Rotor> for Point {
    type Output = Point;

    fn outer_product(self, other: Rotor) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Vec3::splat(other.group0()[0]) } }
    }
}

impl InnerProduct<Rotor> for Point {
    type Output = Motor;

    fn inner_product(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Vec4::from([other.group0()[1], other.group0()[0], other.group0()[0], other.group0()[0]]) * Vec4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl LeftContraction<Rotor> for Point {
    type Output = f32;

    fn left_contraction(self, other: Rotor) -> f32 {
        0.0 - self.group0()[0] * other.group0()[1]
    }
}

impl RightContraction<Rotor> for Point {
    type Output = Motor;

    fn right_contraction(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Vec4::from([other.group0()[1], other.group0()[0], other.group0()[0], other.group0()[0]]) * Vec4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl ScalarProduct<Rotor> for Point {
    type Output = f32;

    fn scalar_product(self, other: Rotor) -> f32 {
        0.0 - self.group0()[0] * other.group0()[1]
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: self.group0() + other.group0() } }
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, other: Point) {
        *self = (*self).add(other);
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: self.group0() - other.group0() } }
    }
}

impl SubAssign<Point> for Point {
    fn sub_assign(&mut self, other: Point) {
        *self = (*self).sub(other);
    }
}

impl Mul<Point> for Point {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: self.group0() * other.group0() } }
    }
}

impl MulAssign<Point> for Point {
    fn mul_assign(&mut self, other: Point) {
        *self = (*self).mul(other);
    }
}

impl Div<Point> for Point {
    type Output = Point;

    fn div(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Vec3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Vec3::from([1.0, 1.0, 1.0]) / Vec3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec3::from([1.0, 1.0, 1.0]) } }
    }
}

impl DivAssign<Point> for Point {
    fn div_assign(&mut self, other: Point) {
        *self = (*self).div(other);
    }
}

impl GeometricProduct<Point> for Point {
    type Output = Motor;

    fn geometric_product(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[1]) * Vec4::from([other.group0()[1], other.group0()[2], other.group0()[1], other.group0()[0]]) * Vec4::from([-1.0, 1.0, 0.0, -1.0]) + Vec4::splat(self.group0()[2]) * Vec4::from([other.group0()[2], other.group0()[1], other.group0()[0], other.group0()[2]]) * Vec4::from([-1.0, -1.0, 1.0, 0.0]) + Vec4::splat(self.group0()[0]) * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[2], other.group0()[1]]) * Vec4::from([-1.0, 0.0, -1.0, 1.0]) } }
    }
}

impl RegressiveProduct<Point> for Point {
    type Output = Plane;

    fn regressive_product(self, other: Point) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[1]) * other.group0().zzx() * Vec3::from([-1.0, 0.0, 1.0]) + Vec3::splat(self.group0()[2]) * other.group0().yxy() * Vec3::from([1.0, -1.0, 0.0]) + Vec3::splat(self.group0()[0]) * other.group0().xzy() * Vec3::from([0.0, 1.0, -1.0]) } }
    }
}

impl InnerProduct<Point> for Point {
    type Output = f32;

    fn inner_product(self, other: Point) -> f32 {
        0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2]
    }
}

impl LeftContraction<Point> for Point {
    type Output = f32;

    fn left_contraction(self, other: Point) -> f32 {
        0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2]
    }
}

impl RightContraction<Point> for Point {
    type Output = f32;

    fn right_contraction(self, other: Point) -> f32 {
        0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2]
    }
}

impl ScalarProduct<Point> for Point {
    type Output = f32;

    fn scalar_product(self, other: Point) -> f32 {
        0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2]
    }
}

impl Into<IdealPoint> for Point {
    fn into(self) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Vec2::from([self.group0()[1], self.group0()[2]]) } }
    }
}

impl Add<IdealPoint> for Point {
    type Output = Point;

    fn add(self, other: IdealPoint) -> Point {
        Point { groups: PointGroups { g0: self.group0() + Vec3::from([other.group0()[0], other.group0()[0], other.group0()[1]]) * Vec3::from([0.0, 1.0, 1.0]) } }
    }
}

impl AddAssign<IdealPoint> for Point {
    fn add_assign(&mut self, other: IdealPoint) {
        *self = (*self).add(other);
    }
}

impl Sub<IdealPoint> for Point {
    type Output = Point;

    fn sub(self, other: IdealPoint) -> Point {
        Point { groups: PointGroups { g0: self.group0() - Vec3::from([other.group0()[0], other.group0()[0], other.group0()[1]]) * Vec3::from([0.0, 1.0, 1.0]) } }
    }
}

impl SubAssign<IdealPoint> for Point {
    fn sub_assign(&mut self, other: IdealPoint) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<IdealPoint> for Point {
    type Output = Motor;

    fn geometric_product(self, other: IdealPoint) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[2]) * Vec4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Vec4::from([-1.0, -1.0, 0.0, 0.0]) + Vec4::from([self.group0()[1], self.group0()[1], self.group0()[0], self.group0()[0]]) * Vec4::from([other.group0()[0], other.group0()[1], other.group0()[1], other.group0()[0]]) * Vec4::from([-1.0, 1.0, -1.0, 1.0]) } }
    }
}

impl RegressiveProduct<IdealPoint> for Point {
    type Output = Plane;

    fn regressive_product(self, other: IdealPoint) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[2]) * Vec3::splat(other.group0()[0]) * Vec3::from([1.0, 0.0, 0.0]) + self.group0().yxx() * Vec3::from([other.group0()[1], other.group0()[1], other.group0()[0]]) * Vec3::from([-1.0, 1.0, -1.0]) } }
    }
}

impl InnerProduct<IdealPoint> for Point {
    type Output = f32;

    fn inner_product(self, other: IdealPoint) -> f32 {
        0.0 - self.group0()[1] * other.group0()[0] - self.group0()[2] * other.group0()[1]
    }
}

impl LeftContraction<IdealPoint> for Point {
    type Output = f32;

    fn left_contraction(self, other: IdealPoint) -> f32 {
        0.0 - self.group0()[1] * other.group0()[0] - self.group0()[2] * other.group0()[1]
    }
}

impl RightContraction<IdealPoint> for Point {
    type Output = f32;

    fn right_contraction(self, other: IdealPoint) -> f32 {
        0.0 - self.group0()[1] * other.group0()[0] - self.group0()[2] * other.group0()[1]
    }
}

impl ScalarProduct<IdealPoint> for Point {
    type Output = f32;

    fn scalar_product(self, other: IdealPoint) -> f32 {
        0.0 - self.group0()[1] * other.group0()[0] - self.group0()[2] * other.group0()[1]
    }
}

impl GeometricProduct<Plane> for Point {
    type Output = MotorDual;

    fn geometric_product(self, other: Plane) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[1]) * Vec4::from([other.group0()[1], other.group0()[2], other.group0()[1], other.group0()[0]]) * Vec4::from([1.0, 1.0, 0.0, -1.0]) + Vec4::splat(self.group0()[2]) * Vec4::from([other.group0()[2], other.group0()[1], other.group0()[0], other.group0()[2]]) * Vec4::from([1.0, -1.0, 1.0, 0.0]) + Vec4::splat(self.group0()[0]) * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[2], other.group0()[1]]) * Vec4::from([1.0, 0.0, -1.0, 1.0]) } }
    }
}

impl RegressiveProduct<Plane> for Point {
    type Output = f32;

    fn regressive_product(self, other: Plane) -> f32 {
        self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2]
    }
}

impl InnerProduct<Plane> for Point {
    type Output = Plane;

    fn inner_product(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[1]) * other.group0().zzx() * Vec3::from([1.0, 0.0, -1.0]) + Vec3::splat(self.group0()[2]) * other.group0().yxy() * Vec3::from([-1.0, 1.0, 0.0]) + Vec3::splat(self.group0()[0]) * other.group0().xzy() * Vec3::from([0.0, -1.0, 1.0]) } }
    }
}

impl RightContraction<Plane> for Point {
    type Output = Plane;

    fn right_contraction(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[1]) * other.group0().zzx() * Vec3::from([1.0, 0.0, -1.0]) + Vec3::splat(self.group0()[2]) * other.group0().yxy() * Vec3::from([-1.0, 1.0, 0.0]) + Vec3::splat(self.group0()[0]) * other.group0().xzy() * Vec3::from([0.0, -1.0, 1.0]) } }
    }
}

impl Add<Translator> for Point {
    type Output = Motor;

    fn add(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Vec4::from([0.0, 1.0, 1.0, 1.0]) + Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([1.0, 0.0, 1.0, 1.0]) } }
    }
}

impl Sub<Translator> for Point {
    type Output = Motor;

    fn sub(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Vec4::from([0.0, 1.0, 1.0, 1.0]) - Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([1.0, 0.0, 1.0, 1.0]) } }
    }
}

impl GeometricProduct<Translator> for Point {
    type Output = Motor;

    fn geometric_product(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[1]) * Vec4::from([other.group0()[1], other.group0()[2], other.group0()[0], other.group0()[1]]) * Vec4::from([-1.0, 1.0, 1.0, 0.0]) + Vec4::splat(self.group0()[2]) * Vec4::from([other.group0()[2], other.group0()[1], other.group0()[2], other.group0()[0]]) * Vec4::from([-1.0, -1.0, 0.0, 1.0]) + Vec4::splat(self.group0()[0]) * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[2], other.group0()[1]]) * Vec4::from([0.0, 1.0, -1.0, 1.0]) } }
    }
}

impl RegressiveProduct<Translator> for Point {
    type Output = Plane;

    fn regressive_product(self, other: Translator) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[2]) * Vec3::splat(other.group0()[1]) * Vec3::from([1.0, 0.0, 0.0]) + self.group0().yxx() * other.group0().zzy() * Vec3::from([-1.0, 1.0, -1.0]) } }
    }
}

impl OuterProduct<Translator> for Point {
    type Output = Point;

    fn outer_product(self, other: Translator) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Vec3::splat(other.group0()[0]) } }
    }
}

impl InnerProduct<Translator> for Point {
    type Output = Motor;

    fn inner_product(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[2]) * Vec4::from([other.group0()[2], other.group0()[2], other.group0()[2], other.group0()[0]]) * Vec4::from([-1.0, 0.0, 0.0, 1.0]) + Vec4::from([self.group0()[1], self.group0()[0], self.group0()[1], self.group0()[0]]) * Vec4::from([other.group0()[1], other.group0()[0], other.group0()[0], other.group0()[0]]) * Vec4::from([-1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl LeftContraction<Translator> for Point {
    type Output = f32;

    fn left_contraction(self, other: Translator) -> f32 {
        0.0 - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2]
    }
}

impl RightContraction<Translator> for Point {
    type Output = Motor;

    fn right_contraction(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[2]) * Vec4::from([other.group0()[2], other.group0()[2], other.group0()[2], other.group0()[0]]) * Vec4::from([-1.0, 0.0, 0.0, 1.0]) + Vec4::from([self.group0()[1], self.group0()[0], self.group0()[1], self.group0()[0]]) * Vec4::from([other.group0()[1], other.group0()[0], other.group0()[0], other.group0()[0]]) * Vec4::from([-1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl ScalarProduct<Translator> for Point {
    type Output = f32;

    fn scalar_product(self, other: Translator) -> f32 {
        0.0 - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2]
    }
}

impl Add<Motor> for Point {
    type Output = Motor;

    fn add(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Vec4::from([0.0, 1.0, 1.0, 1.0]) + other.group0() } }
    }
}

impl Sub<Motor> for Point {
    type Output = Motor;

    fn sub(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Vec4::from([0.0, 1.0, 1.0, 1.0]) - other.group0() } }
    }
}

impl GeometricProduct<Motor> for Point {
    type Output = Motor;

    fn geometric_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[0]) * other.group0().yxwz() * Vec4::from([-1.0, 1.0, -1.0, 1.0]) + Vec4::splat(self.group0()[1]) * other.group0().zwxy() * Vec4::from([-1.0, 1.0, 1.0, -1.0]) + Vec4::splat(self.group0()[2]) * other.group0().wzyx() * Vec4::from([-1.0, -1.0, 1.0, 1.0]) } }
    }
}

impl RegressiveProduct<Motor> for Point {
    type Output = Plane;

    fn regressive_product(self, other: Motor) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[1]) * Vec3::from([other.group0()[3], other.group0()[3], other.group0()[1]]) * Vec3::from([-1.0, 0.0, 1.0]) + Vec3::splat(self.group0()[2]) * Vec3::from([other.group0()[2], other.group0()[1], other.group0()[2]]) * Vec3::from([1.0, -1.0, 0.0]) + Vec3::splat(self.group0()[0]) * Vec3::from([other.group0()[0], other.group0()[3], other.group0()[2]]) * Vec3::from([0.0, 1.0, -1.0]) } }
    }
}

impl OuterProduct<Motor> for Point {
    type Output = Point;

    fn outer_product(self, other: Motor) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Vec3::splat(other.group0()[0]) } }
    }
}

impl InnerProduct<Motor> for Point {
    type Output = Motor;

    fn inner_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[1]) * other.group0().zzxz() * Vec4::from([-1.0, 0.0, 1.0, 0.0]) + Vec4::splat(self.group0()[2]) * other.group0().wwwx() * Vec4::from([-1.0, 0.0, 0.0, 1.0]) + Vec4::splat(self.group0()[0]) * other.group0().yxxx() * Vec4::from([-1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl LeftContraction<Motor> for Point {
    type Output = f32;

    fn left_contraction(self, other: Motor) -> f32 {
        0.0 - self.group0()[0] * other.group0()[1] - self.group0()[1] * other.group0()[2] - self.group0()[2] * other.group0()[3]
    }
}

impl RightContraction<Motor> for Point {
    type Output = Motor;

    fn right_contraction(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[1]) * other.group0().zzxz() * Vec4::from([-1.0, 0.0, 1.0, 0.0]) + Vec4::splat(self.group0()[2]) * other.group0().wwwx() * Vec4::from([-1.0, 0.0, 0.0, 1.0]) + Vec4::splat(self.group0()[0]) * other.group0().yxxx() * Vec4::from([-1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl ScalarProduct<Motor> for Point {
    type Output = f32;

    fn scalar_product(self, other: Motor) -> f32 {
        0.0 - self.group0()[0] * other.group0()[1] - self.group0()[1] * other.group0()[2] - self.group0()[2] * other.group0()[3]
    }
}

impl GeometricProduct<MotorDual> for Point {
    type Output = MotorDual;

    fn geometric_product(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[0]) * other.group0().yxwz() * Vec4::from([1.0, -1.0, -1.0, 1.0]) + Vec4::splat(self.group0()[1]) * other.group0().zwxy() * Vec4::from([1.0, 1.0, -1.0, -1.0]) + Vec4::splat(self.group0()[2]) * other.group0().wzyx() * Vec4::from([1.0, -1.0, 1.0, -1.0]) } }
    }
}

impl RegressiveProduct<MotorDual> for Point {
    type Output = Motor;

    fn regressive_product(self, other: MotorDual) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[1]) * other.group0().zzxz() * Vec4::from([1.0, 0.0, 1.0, 0.0]) + Vec4::splat(self.group0()[2]) * other.group0().wwwx() * Vec4::from([1.0, 0.0, 0.0, 1.0]) + Vec4::splat(self.group0()[0]) * other.group0().yxxx() * Vec4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl InnerProduct<MotorDual> for Point {
    type Output = Plane;

    fn inner_product(self, other: MotorDual) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[0]) * Vec3::from([other.group0()[0], other.group0()[3], other.group0()[2]]) * Vec3::from([-1.0, -1.0, 1.0]) + Vec3::splat(self.group0()[1]) * Vec3::from([other.group0()[3], other.group0()[0], other.group0()[1]]) * Vec3::from([1.0, -1.0, -1.0]) + Vec3::splat(self.group0()[2]) * Vec3::from([other.group0()[2], other.group0()[1], other.group0()[0]]) * Vec3::from([-1.0, 1.0, -1.0]) } }
    }
}

impl LeftContraction<MotorDual> for Point {
    type Output = Plane;

    fn left_contraction(self, other: MotorDual) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Vec3::splat(other.group0()[0]) * Vec3::splat(-1.0) } }
    }
}

impl RightContraction<MotorDual> for Point {
    type Output = Plane;

    fn right_contraction(self, other: MotorDual) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[1]) * Vec3::from([other.group0()[3], other.group0()[3], other.group0()[1]]) * Vec3::from([1.0, 0.0, -1.0]) + Vec3::splat(self.group0()[2]) * Vec3::from([other.group0()[2], other.group0()[1], other.group0()[2]]) * Vec3::from([-1.0, 1.0, 0.0]) + Vec3::splat(self.group0()[0]) * Vec3::from([other.group0()[0], other.group0()[3], other.group0()[2]]) * Vec3::from([0.0, -1.0, 1.0]) } }
    }
}

impl SquaredMagnitude for Point {
    type Output = f32;

    fn squared_magnitude(self) -> f32 {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for Point {
    type Output = f32;

    fn magnitude(self) -> f32 {
        self.squared_magnitude().sqrt()
    }
}

impl Mul<f32> for Point {
    type Output = Point;

    fn mul(self, other: f32) -> Point {
        self.geometric_product(other)
    }
}

impl MulAssign<f32> for Point {
    fn mul_assign(&mut self, other: f32) {
        *self = (*self).mul(other);
    }
}

impl Signum for Point {
    type Output = Point;

    fn signum(self) -> Point {
        self.geometric_product(1.0 / self.magnitude())
    }
}

impl Inverse for Point {
    type Output = Point;

    fn inverse(self) -> Point {
        self.reversal().geometric_product(1.0 / self.squared_magnitude())
    }
}

impl Zero for IdealPoint {
    fn zero() -> Self {
        IdealPoint { groups: IdealPointGroups { g0: Vec2::splat(0.0) } }
    }
}

impl One for IdealPoint {
    fn one() -> Self {
        IdealPoint { groups: IdealPointGroups { g0: Vec2::splat(0.0) } }
    }
}

impl Neg for IdealPoint {
    type Output = IdealPoint;

    fn neg(self) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Vec2::splat(-1.0) } }
    }
}

impl Automorphism for IdealPoint {
    type Output = IdealPoint;

    fn automorphism(self) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() } }
    }
}

impl Reversal for IdealPoint {
    type Output = IdealPoint;

    fn reversal(self) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Vec2::splat(-1.0) } }
    }
}

impl Conjugation for IdealPoint {
    type Output = IdealPoint;

    fn conjugation(self) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Vec2::splat(-1.0) } }
    }
}

impl Add<f32> for IdealPoint {
    type Output = Translator;

    fn add(self, other: f32) -> Translator {
        Translator { groups: TranslatorGroups { g0: Vec3::from([self.group0()[0], self.group0()[0], self.group0()[1]]) * Vec3::from([0.0, 1.0, 1.0]) + Vec3::splat(other) * Vec3::from([1.0, 0.0, 0.0]) } }
    }
}

impl Sub<f32> for IdealPoint {
    type Output = Translator;

    fn sub(self, other: f32) -> Translator {
        Translator { groups: TranslatorGroups { g0: Vec3::from([self.group0()[0], self.group0()[0], self.group0()[1]]) * Vec3::from([0.0, 1.0, 1.0]) - Vec3::splat(other) * Vec3::from([1.0, 0.0, 0.0]) } }
    }
}

impl GeometricProduct<f32> for IdealPoint {
    type Output = IdealPoint;

    fn geometric_product(self, other: f32) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Vec2::splat(other) } }
    }
}

impl OuterProduct<f32> for IdealPoint {
    type Output = IdealPoint;

    fn outer_product(self, other: f32) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Vec2::splat(other) } }
    }
}

impl InnerProduct<f32> for IdealPoint {
    type Output = IdealPoint;

    fn inner_product(self, other: f32) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Vec2::splat(other) } }
    }
}

impl RightContraction<f32> for IdealPoint {
    type Output = IdealPoint;

    fn right_contraction(self, other: f32) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Vec2::splat(other) } }
    }
}

impl Add<MultiVector> for IdealPoint {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: other.group0(), g1: Vec4::from([self.group0()[0], self.group0()[0], self.group0()[0], self.group0()[1]]) * Vec4::from([0.0, 0.0, 1.0, 1.0]) + other.group1() } }
    }
}

impl Sub<MultiVector> for IdealPoint {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(0.0) - other.group0(), g1: Vec4::from([self.group0()[0], self.group0()[0], self.group0()[0], self.group0()[1]]) * Vec4::from([0.0, 0.0, 1.0, 1.0]) - other.group1() } }
    }
}

impl GeometricProduct<MultiVector> for IdealPoint {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[0]) * other.group1().zwxy() * Vec4::from([-1.0, 1.0, -1.0, -1.0]) + Vec4::splat(self.group0()[1]) * other.group1().wzyx() * Vec4::from([-1.0, -1.0, -1.0, 1.0]), g1: Vec4::splat(self.group0()[0]) * other.group0().zwxy() * Vec4::from([1.0, 1.0, 1.0, -1.0]) + Vec4::splat(self.group0()[1]) * other.group0().wzyx() * Vec4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl ScalarProduct<MultiVector> for IdealPoint {
    type Output = f32;

    fn scalar_product(self, other: MultiVector) -> f32 {
        0.0 - self.group0()[0] * other.group1()[2] - self.group0()[1] * other.group1()[3]
    }
}

impl Add<Rotor> for IdealPoint {
    type Output = Motor;

    fn add(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::from([self.group0()[0], self.group0()[0], self.group0()[0], self.group0()[1]]) * Vec4::from([0.0, 0.0, 1.0, 1.0]) + Vec4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[0]]) * Vec4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl Sub<Rotor> for IdealPoint {
    type Output = Motor;

    fn sub(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::from([self.group0()[0], self.group0()[0], self.group0()[0], self.group0()[1]]) * Vec4::from([0.0, 0.0, 1.0, 1.0]) - Vec4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[0]]) * Vec4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl GeometricProduct<Rotor> for IdealPoint {
    type Output = IdealPoint;

    fn geometric_product(self, other: Rotor) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Vec2::splat(self.group0()[0]) * other.group0() * Vec2::from([1.0, -1.0]) + Vec2::splat(self.group0()[1]) * other.group0().yx() } }
    }
}

impl OuterProduct<Rotor> for IdealPoint {
    type Output = IdealPoint;

    fn outer_product(self, other: Rotor) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Vec2::splat(other.group0()[0]) } }
    }
}

impl InnerProduct<Rotor> for IdealPoint {
    type Output = IdealPoint;

    fn inner_product(self, other: Rotor) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Vec2::splat(other.group0()[0]) } }
    }
}

impl RightContraction<Rotor> for IdealPoint {
    type Output = IdealPoint;

    fn right_contraction(self, other: Rotor) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Vec2::splat(other.group0()[0]) } }
    }
}

impl Add<Point> for IdealPoint {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Vec3::from([self.group0()[0], self.group0()[0], self.group0()[1]]) * Vec3::from([0.0, 1.0, 1.0]) + other.group0() } }
    }
}

impl Sub<Point> for IdealPoint {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Vec3::from([self.group0()[0], self.group0()[0], self.group0()[1]]) * Vec3::from([0.0, 1.0, 1.0]) - other.group0() } }
    }
}

impl GeometricProduct<Point> for IdealPoint {
    type Output = Motor;

    fn geometric_product(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[1]) * Vec4::from([other.group0()[2], other.group0()[1], other.group0()[0], other.group0()[2]]) * Vec4::from([-1.0, -1.0, 1.0, 0.0]) + Vec4::splat(self.group0()[0]) * Vec4::from([other.group0()[1], other.group0()[2], other.group0()[0], other.group0()[0]]) * Vec4::from([-1.0, 1.0, 0.0, -1.0]) } }
    }
}

impl RegressiveProduct<Point> for IdealPoint {
    type Output = Plane;

    fn regressive_product(self, other: Point) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[1]) * other.group0().yxy() * Vec3::from([1.0, -1.0, 0.0]) + Vec3::splat(self.group0()[0]) * other.group0().zxx() * Vec3::from([-1.0, 0.0, 1.0]) } }
    }
}

impl InnerProduct<Point> for IdealPoint {
    type Output = f32;

    fn inner_product(self, other: Point) -> f32 {
        0.0 - self.group0()[0] * other.group0()[1] - self.group0()[1] * other.group0()[2]
    }
}

impl LeftContraction<Point> for IdealPoint {
    type Output = f32;

    fn left_contraction(self, other: Point) -> f32 {
        0.0 - self.group0()[0] * other.group0()[1] - self.group0()[1] * other.group0()[2]
    }
}

impl RightContraction<Point> for IdealPoint {
    type Output = f32;

    fn right_contraction(self, other: Point) -> f32 {
        0.0 - self.group0()[0] * other.group0()[1] - self.group0()[1] * other.group0()[2]
    }
}

impl ScalarProduct<Point> for IdealPoint {
    type Output = f32;

    fn scalar_product(self, other: Point) -> f32 {
        0.0 - self.group0()[0] * other.group0()[1] - self.group0()[1] * other.group0()[2]
    }
}

impl Add<IdealPoint> for IdealPoint {
    type Output = IdealPoint;

    fn add(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() + other.group0() } }
    }
}

impl AddAssign<IdealPoint> for IdealPoint {
    fn add_assign(&mut self, other: IdealPoint) {
        *self = (*self).add(other);
    }
}

impl Sub<IdealPoint> for IdealPoint {
    type Output = IdealPoint;

    fn sub(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() - other.group0() } }
    }
}

impl SubAssign<IdealPoint> for IdealPoint {
    fn sub_assign(&mut self, other: IdealPoint) {
        *self = (*self).sub(other);
    }
}

impl Mul<IdealPoint> for IdealPoint {
    type Output = IdealPoint;

    fn mul(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * other.group0() } }
    }
}

impl MulAssign<IdealPoint> for IdealPoint {
    fn mul_assign(&mut self, other: IdealPoint) {
        *self = (*self).mul(other);
    }
}

impl Div<IdealPoint> for IdealPoint {
    type Output = IdealPoint;

    fn div(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Vec2::from([self.group0()[0], self.group0()[1]]) * Vec2::from([1.0, 1.0]) / Vec2::from([other.group0()[0], other.group0()[1]]) * Vec2::from([1.0, 1.0]) } }
    }
}

impl DivAssign<IdealPoint> for IdealPoint {
    fn div_assign(&mut self, other: IdealPoint) {
        *self = (*self).div(other);
    }
}

impl GeometricProduct<IdealPoint> for IdealPoint {
    type Output = Rotor;

    fn geometric_product(self, other: IdealPoint) -> Rotor {
        Rotor { groups: RotorGroups { g0: Vec2::splat(self.group0()[0]) * other.group0() * Vec2::from([-1.0, 1.0]) - Vec2::splat(self.group0()[1]) * other.group0().yx() } }
    }
}

impl InnerProduct<IdealPoint> for IdealPoint {
    type Output = f32;

    fn inner_product(self, other: IdealPoint) -> f32 {
        0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1]
    }
}

impl LeftContraction<IdealPoint> for IdealPoint {
    type Output = f32;

    fn left_contraction(self, other: IdealPoint) -> f32 {
        0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1]
    }
}

impl RightContraction<IdealPoint> for IdealPoint {
    type Output = f32;

    fn right_contraction(self, other: IdealPoint) -> f32 {
        0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1]
    }
}

impl ScalarProduct<IdealPoint> for IdealPoint {
    type Output = f32;

    fn scalar_product(self, other: IdealPoint) -> f32 {
        0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1]
    }
}

impl GeometricProduct<Plane> for IdealPoint {
    type Output = MotorDual;

    fn geometric_product(self, other: Plane) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[1]) * Vec4::from([other.group0()[2], other.group0()[1], other.group0()[0], other.group0()[2]]) * Vec4::from([1.0, -1.0, 1.0, 0.0]) + Vec4::splat(self.group0()[0]) * Vec4::from([other.group0()[1], other.group0()[2], other.group0()[0], other.group0()[0]]) * Vec4::from([1.0, 1.0, 0.0, -1.0]) } }
    }
}

impl RegressiveProduct<Plane> for IdealPoint {
    type Output = f32;

    fn regressive_product(self, other: Plane) -> f32 {
        self.group0()[0] * other.group0()[1] + self.group0()[1] * other.group0()[2]
    }
}

impl InnerProduct<Plane> for IdealPoint {
    type Output = Plane;

    fn inner_product(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[1]) * other.group0().yxy() * Vec3::from([-1.0, 1.0, 0.0]) + Vec3::splat(self.group0()[0]) * other.group0().zxx() * Vec3::from([1.0, 0.0, -1.0]) } }
    }
}

impl RightContraction<Plane> for IdealPoint {
    type Output = Plane;

    fn right_contraction(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[1]) * other.group0().yxy() * Vec3::from([-1.0, 1.0, 0.0]) + Vec3::splat(self.group0()[0]) * other.group0().zxx() * Vec3::from([1.0, 0.0, -1.0]) } }
    }
}

impl Add<Translator> for IdealPoint {
    type Output = Translator;

    fn add(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Vec3::from([self.group0()[0], self.group0()[0], self.group0()[1]]) * Vec3::from([0.0, 1.0, 1.0]) + other.group0() } }
    }
}

impl Sub<Translator> for IdealPoint {
    type Output = Translator;

    fn sub(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Vec3::from([self.group0()[0], self.group0()[0], self.group0()[1]]) * Vec3::from([0.0, 1.0, 1.0]) - other.group0() } }
    }
}

impl GeometricProduct<Translator> for IdealPoint {
    type Output = Motor;

    fn geometric_product(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[1]) * Vec4::from([other.group0()[2], other.group0()[1], other.group0()[2], other.group0()[0]]) * Vec4::from([-1.0, -1.0, 0.0, 1.0]) + Vec4::splat(self.group0()[0]) * Vec4::from([other.group0()[1], other.group0()[2], other.group0()[0], other.group0()[0]]) * Vec4::from([-1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl OuterProduct<Translator> for IdealPoint {
    type Output = IdealPoint;

    fn outer_product(self, other: Translator) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Vec2::splat(other.group0()[0]) } }
    }
}

impl InnerProduct<Translator> for IdealPoint {
    type Output = Translator;

    fn inner_product(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Vec3::splat(self.group0()[1]) * other.group0().zzx() * Vec3::from([-1.0, 0.0, 1.0]) + Vec3::splat(self.group0()[0]) * other.group0().yxx() * Vec3::from([-1.0, 1.0, 0.0]) } }
    }
}

impl LeftContraction<Translator> for IdealPoint {
    type Output = f32;

    fn left_contraction(self, other: Translator) -> f32 {
        0.0 - self.group0()[0] * other.group0()[1] - self.group0()[1] * other.group0()[2]
    }
}

impl RightContraction<Translator> for IdealPoint {
    type Output = Translator;

    fn right_contraction(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Vec3::splat(self.group0()[1]) * other.group0().zzx() * Vec3::from([-1.0, 0.0, 1.0]) + Vec3::splat(self.group0()[0]) * other.group0().yxx() * Vec3::from([-1.0, 1.0, 0.0]) } }
    }
}

impl ScalarProduct<Translator> for IdealPoint {
    type Output = f32;

    fn scalar_product(self, other: Translator) -> f32 {
        0.0 - self.group0()[0] * other.group0()[1] - self.group0()[1] * other.group0()[2]
    }
}

impl Add<Motor> for IdealPoint {
    type Output = Motor;

    fn add(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::from([self.group0()[0], self.group0()[0], self.group0()[0], self.group0()[1]]) * Vec4::from([0.0, 0.0, 1.0, 1.0]) + other.group0() } }
    }
}

impl Sub<Motor> for IdealPoint {
    type Output = Motor;

    fn sub(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::from([self.group0()[0], self.group0()[0], self.group0()[0], self.group0()[1]]) * Vec4::from([0.0, 0.0, 1.0, 1.0]) - other.group0() } }
    }
}

impl GeometricProduct<Motor> for IdealPoint {
    type Output = Motor;

    fn geometric_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[0]) * other.group0().zwxy() * Vec4::from([-1.0, 1.0, 1.0, -1.0]) + Vec4::splat(self.group0()[1]) * other.group0().wzyx() * Vec4::from([-1.0, -1.0, 1.0, 1.0]) } }
    }
}

impl RegressiveProduct<Motor> for IdealPoint {
    type Output = Plane;

    fn regressive_product(self, other: Motor) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[1]) * Vec3::from([other.group0()[2], other.group0()[1], other.group0()[2]]) * Vec3::from([1.0, -1.0, 0.0]) + Vec3::splat(self.group0()[0]) * Vec3::from([other.group0()[3], other.group0()[0], other.group0()[1]]) * Vec3::from([-1.0, 0.0, 1.0]) } }
    }
}

impl OuterProduct<Motor> for IdealPoint {
    type Output = IdealPoint;

    fn outer_product(self, other: Motor) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Vec2::splat(other.group0()[0]) } }
    }
}

impl InnerProduct<Motor> for IdealPoint {
    type Output = Translator;

    fn inner_product(self, other: Motor) -> Translator {
        Translator { groups: TranslatorGroups { g0: Vec3::splat(self.group0()[1]) * Vec3::from([other.group0()[3], other.group0()[3], other.group0()[0]]) * Vec3::from([-1.0, 0.0, 1.0]) + Vec3::splat(self.group0()[0]) * Vec3::from([other.group0()[2], other.group0()[0], other.group0()[0]]) * Vec3::from([-1.0, 1.0, 0.0]) } }
    }
}

impl LeftContraction<Motor> for IdealPoint {
    type Output = f32;

    fn left_contraction(self, other: Motor) -> f32 {
        0.0 - self.group0()[0] * other.group0()[2] - self.group0()[1] * other.group0()[3]
    }
}

impl RightContraction<Motor> for IdealPoint {
    type Output = Translator;

    fn right_contraction(self, other: Motor) -> Translator {
        Translator { groups: TranslatorGroups { g0: Vec3::splat(self.group0()[1]) * Vec3::from([other.group0()[3], other.group0()[3], other.group0()[0]]) * Vec3::from([-1.0, 0.0, 1.0]) + Vec3::splat(self.group0()[0]) * Vec3::from([other.group0()[2], other.group0()[0], other.group0()[0]]) * Vec3::from([-1.0, 1.0, 0.0]) } }
    }
}

impl ScalarProduct<Motor> for IdealPoint {
    type Output = f32;

    fn scalar_product(self, other: Motor) -> f32 {
        0.0 - self.group0()[0] * other.group0()[2] - self.group0()[1] * other.group0()[3]
    }
}

impl GeometricProduct<MotorDual> for IdealPoint {
    type Output = MotorDual;

    fn geometric_product(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[0]) * other.group0().zwxy() * Vec4::from([1.0, 1.0, -1.0, -1.0]) + Vec4::splat(self.group0()[1]) * other.group0().wzyx() * Vec4::from([1.0, -1.0, 1.0, -1.0]) } }
    }
}

impl RegressiveProduct<MotorDual> for IdealPoint {
    type Output = Translator;

    fn regressive_product(self, other: MotorDual) -> Translator {
        Translator { groups: TranslatorGroups { g0: Vec3::splat(self.group0()[1]) * Vec3::from([other.group0()[3], other.group0()[3], other.group0()[0]]) * Vec3::from([1.0, 0.0, 1.0]) + Vec3::splat(self.group0()[0]) * Vec3::from([other.group0()[2], other.group0()[0], other.group0()[0]]) * Vec3::from([1.0, 1.0, 0.0]) } }
    }
}

impl InnerProduct<MotorDual> for IdealPoint {
    type Output = Plane;

    fn inner_product(self, other: MotorDual) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[0]) * Vec3::from([other.group0()[3], other.group0()[0], other.group0()[1]]) * Vec3::from([1.0, -1.0, -1.0]) + Vec3::splat(self.group0()[1]) * Vec3::from([other.group0()[2], other.group0()[1], other.group0()[0]]) * Vec3::from([-1.0, 1.0, -1.0]) } }
    }
}

impl RightContraction<MotorDual> for IdealPoint {
    type Output = Plane;

    fn right_contraction(self, other: MotorDual) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[1]) * Vec3::from([other.group0()[2], other.group0()[1], other.group0()[2]]) * Vec3::from([-1.0, 1.0, 0.0]) + Vec3::splat(self.group0()[0]) * Vec3::from([other.group0()[3], other.group0()[0], other.group0()[1]]) * Vec3::from([1.0, 0.0, -1.0]) } }
    }
}

impl SquaredMagnitude for IdealPoint {
    type Output = f32;

    fn squared_magnitude(self) -> f32 {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for IdealPoint {
    type Output = f32;

    fn magnitude(self) -> f32 {
        self.squared_magnitude().sqrt()
    }
}

impl Mul<f32> for IdealPoint {
    type Output = IdealPoint;

    fn mul(self, other: f32) -> IdealPoint {
        self.geometric_product(other)
    }
}

impl MulAssign<f32> for IdealPoint {
    fn mul_assign(&mut self, other: f32) {
        *self = (*self).mul(other);
    }
}

impl Signum for IdealPoint {
    type Output = IdealPoint;

    fn signum(self) -> IdealPoint {
        self.geometric_product(1.0 / self.magnitude())
    }
}

impl Inverse for IdealPoint {
    type Output = IdealPoint;

    fn inverse(self) -> IdealPoint {
        self.reversal().geometric_product(1.0 / self.squared_magnitude())
    }
}

impl Zero for Plane {
    fn zero() -> Self {
        Plane { groups: PlaneGroups { g0: Vec3::splat(0.0) } }
    }
}

impl One for Plane {
    fn one() -> Self {
        Plane { groups: PlaneGroups { g0: Vec3::splat(0.0) } }
    }
}

impl Neg for Plane {
    type Output = Plane;

    fn neg(self) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Vec3::splat(-1.0) } }
    }
}

impl Automorphism for Plane {
    type Output = Plane;

    fn automorphism(self) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Vec3::splat(-1.0) } }
    }
}

impl Reversal for Plane {
    type Output = Plane;

    fn reversal(self) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() } }
    }
}

impl Conjugation for Plane {
    type Output = Plane;

    fn conjugation(self) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Vec3::splat(-1.0) } }
    }
}

impl Dual for Plane {
    type Output = Point;

    fn dual(self) -> Point {
        Point { groups: PointGroups { g0: self.group0() } }
    }
}

impl GeometricProduct<f32> for Plane {
    type Output = Plane;

    fn geometric_product(self, other: f32) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Vec3::splat(other) } }
    }
}

impl OuterProduct<f32> for Plane {
    type Output = Plane;

    fn outer_product(self, other: f32) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Vec3::splat(other) } }
    }
}

impl InnerProduct<f32> for Plane {
    type Output = Plane;

    fn inner_product(self, other: f32) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Vec3::splat(other) } }
    }
}

impl RightContraction<f32> for Plane {
    type Output = Plane;

    fn right_contraction(self, other: f32) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Vec3::splat(other) } }
    }
}

impl Add<MultiVector> for Plane {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::from([self.group0()[0], self.group0()[0], self.group0()[2], self.group0()[1]]) * Vec4::from([0.0, 0.0, 1.0, 1.0]) + other.group0(), g1: Vec4::splat(self.group0()[0]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) + other.group1() } }
    }
}

impl Sub<MultiVector> for Plane {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::from([self.group0()[0], self.group0()[0], self.group0()[2], self.group0()[1]]) * Vec4::from([0.0, 0.0, 1.0, 1.0]) - other.group0(), g1: Vec4::splat(self.group0()[0]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) - other.group1() } }
    }
}

impl GeometricProduct<MultiVector> for Plane {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[0]) * other.group1() * Vec4::from([1.0, 1.0, 1.0, -1.0]) + Vec4::splat(self.group0()[1]) * other.group0().wzyx() * Vec4::from([1.0, -1.0, -1.0, 1.0]) + Vec4::splat(self.group0()[2]) * other.group0().zwxy(), g1: Vec4::splat(self.group0()[0]) * other.group0() * Vec4::from([1.0, 1.0, 1.0, -1.0]) + Vec4::splat(self.group0()[1]) * other.group1().wzyx() + Vec4::splat(self.group0()[2]) * other.group1().zwxy() * Vec4::from([-1.0, 1.0, -1.0, 1.0]) } }
    }
}

impl ScalarProduct<MultiVector> for Plane {
    type Output = f32;

    fn scalar_product(self, other: MultiVector) -> f32 {
        self.group0()[0] * other.group1()[0] + self.group0()[1] * other.group0()[3] + self.group0()[2] * other.group0()[2]
    }
}

impl GeometricProduct<Rotor> for Plane {
    type Output = MotorDual;

    fn geometric_product(self, other: Rotor) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[2]) * Vec4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]) * Vec4::from([0.0, 0.0, 1.0, 1.0]) + Vec4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[1]]) * Vec4::from([other.group0()[1], other.group0()[0], other.group0()[0], other.group0()[1]]) * Vec4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl RegressiveProduct<Rotor> for Plane {
    type Output = f32;

    fn regressive_product(self, other: Rotor) -> f32 {
        self.group0()[0] * other.group0()[1]
    }
}

impl OuterProduct<Rotor> for Plane {
    type Output = MotorDual;

    fn outer_product(self, other: Rotor) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Vec4::from([other.group0()[1], other.group0()[0], other.group0()[0], other.group0()[0]]) } }
    }
}

impl InnerProduct<Rotor> for Plane {
    type Output = Plane;

    fn inner_product(self, other: Rotor) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[2]) * Vec3::from([other.group0()[1], other.group0()[1], other.group0()[0]]) * Vec3::from([0.0, 1.0, 1.0]) + self.group0().xyy() * Vec3::from([other.group0()[0], other.group0()[0], other.group0()[1]]) * Vec3::from([1.0, 1.0, -1.0]) } }
    }
}

impl RightContraction<Rotor> for Plane {
    type Output = Plane;

    fn right_contraction(self, other: Rotor) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Vec3::splat(other.group0()[0]) } }
    }
}

impl GeometricProduct<Point> for Plane {
    type Output = MotorDual;

    fn geometric_product(self, other: Point) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[1]) * Vec4::from([other.group0()[1], other.group0()[2], other.group0()[1], other.group0()[0]]) * Vec4::from([1.0, 1.0, 0.0, -1.0]) + Vec4::splat(self.group0()[2]) * Vec4::from([other.group0()[2], other.group0()[1], other.group0()[0], other.group0()[2]]) * Vec4::from([1.0, -1.0, 1.0, 0.0]) + Vec4::splat(self.group0()[0]) * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[2], other.group0()[1]]) * Vec4::from([1.0, 0.0, -1.0, 1.0]) } }
    }
}

impl RegressiveProduct<Point> for Plane {
    type Output = f32;

    fn regressive_product(self, other: Point) -> f32 {
        self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2]
    }
}

impl InnerProduct<Point> for Plane {
    type Output = Plane;

    fn inner_product(self, other: Point) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[1]) * other.group0().zzx() * Vec3::from([1.0, 0.0, -1.0]) + Vec3::splat(self.group0()[2]) * other.group0().yxy() * Vec3::from([-1.0, 1.0, 0.0]) + Vec3::splat(self.group0()[0]) * other.group0().xzy() * Vec3::from([0.0, -1.0, 1.0]) } }
    }
}

impl LeftContraction<Point> for Plane {
    type Output = Plane;

    fn left_contraction(self, other: Point) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[1]) * other.group0().zzx() * Vec3::from([1.0, 0.0, -1.0]) + Vec3::splat(self.group0()[2]) * other.group0().yxy() * Vec3::from([-1.0, 1.0, 0.0]) + Vec3::splat(self.group0()[0]) * other.group0().xzy() * Vec3::from([0.0, -1.0, 1.0]) } }
    }
}

impl GeometricProduct<IdealPoint> for Plane {
    type Output = MotorDual;

    fn geometric_product(self, other: IdealPoint) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[2]) * Vec4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Vec4::from([1.0, -1.0, 0.0, 0.0]) + Vec4::from([self.group0()[1], self.group0()[1], self.group0()[0], self.group0()[0]]) * Vec4::from([other.group0()[0], other.group0()[1], other.group0()[1], other.group0()[0]]) * Vec4::from([1.0, 1.0, -1.0, 1.0]) } }
    }
}

impl RegressiveProduct<IdealPoint> for Plane {
    type Output = f32;

    fn regressive_product(self, other: IdealPoint) -> f32 {
        self.group0()[1] * other.group0()[0] + self.group0()[2] * other.group0()[1]
    }
}

impl InnerProduct<IdealPoint> for Plane {
    type Output = Plane;

    fn inner_product(self, other: IdealPoint) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[2]) * Vec3::splat(other.group0()[0]) * Vec3::from([-1.0, 0.0, 0.0]) + self.group0().yxx() * Vec3::from([other.group0()[1], other.group0()[1], other.group0()[0]]) * Vec3::from([1.0, -1.0, 1.0]) } }
    }
}

impl LeftContraction<IdealPoint> for Plane {
    type Output = Plane;

    fn left_contraction(self, other: IdealPoint) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[2]) * Vec3::splat(other.group0()[0]) * Vec3::from([-1.0, 0.0, 0.0]) + self.group0().yxx() * Vec3::from([other.group0()[1], other.group0()[1], other.group0()[0]]) * Vec3::from([1.0, -1.0, 1.0]) } }
    }
}

impl Add<Plane> for Plane {
    type Output = Plane;

    fn add(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() + other.group0() } }
    }
}

impl AddAssign<Plane> for Plane {
    fn add_assign(&mut self, other: Plane) {
        *self = (*self).add(other);
    }
}

impl Sub<Plane> for Plane {
    type Output = Plane;

    fn sub(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() - other.group0() } }
    }
}

impl SubAssign<Plane> for Plane {
    fn sub_assign(&mut self, other: Plane) {
        *self = (*self).sub(other);
    }
}

impl Mul<Plane> for Plane {
    type Output = Plane;

    fn mul(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * other.group0() } }
    }
}

impl MulAssign<Plane> for Plane {
    fn mul_assign(&mut self, other: Plane) {
        *self = (*self).mul(other);
    }
}

impl Div<Plane> for Plane {
    type Output = Plane;

    fn div(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Vec3::from([1.0, 1.0, 1.0]) / Vec3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec3::from([1.0, 1.0, 1.0]) } }
    }
}

impl DivAssign<Plane> for Plane {
    fn div_assign(&mut self, other: Plane) {
        *self = (*self).div(other);
    }
}

impl GeometricProduct<Plane> for Plane {
    type Output = Motor;

    fn geometric_product(self, other: Plane) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[1]) * Vec4::from([other.group0()[1], other.group0()[2], other.group0()[1], other.group0()[0]]) * Vec4::from([1.0, -1.0, 0.0, 1.0]) + Vec4::splat(self.group0()[2]) * Vec4::from([other.group0()[2], other.group0()[1], other.group0()[0], other.group0()[2]]) * Vec4::from([1.0, 1.0, -1.0, 0.0]) + Vec4::splat(self.group0()[0]) * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[2], other.group0()[1]]) * Vec4::from([1.0, 0.0, 1.0, -1.0]) } }
    }
}

impl OuterProduct<Plane> for Plane {
    type Output = Point;

    fn outer_product(self, other: Plane) -> Point {
        Point { groups: PointGroups { g0: Vec3::splat(self.group0()[1]) * other.group0().zzx() * Vec3::from([-1.0, 0.0, 1.0]) + Vec3::splat(self.group0()[2]) * other.group0().yxy() * Vec3::from([1.0, -1.0, 0.0]) + Vec3::splat(self.group0()[0]) * other.group0().xzy() * Vec3::from([0.0, 1.0, -1.0]) } }
    }
}

impl InnerProduct<Plane> for Plane {
    type Output = f32;

    fn inner_product(self, other: Plane) -> f32 {
        self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2]
    }
}

impl LeftContraction<Plane> for Plane {
    type Output = f32;

    fn left_contraction(self, other: Plane) -> f32 {
        self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2]
    }
}

impl RightContraction<Plane> for Plane {
    type Output = f32;

    fn right_contraction(self, other: Plane) -> f32 {
        self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2]
    }
}

impl ScalarProduct<Plane> for Plane {
    type Output = f32;

    fn scalar_product(self, other: Plane) -> f32 {
        self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2]
    }
}

impl GeometricProduct<Translator> for Plane {
    type Output = MotorDual;

    fn geometric_product(self, other: Translator) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[1]) * Vec4::from([other.group0()[1], other.group0()[2], other.group0()[0], other.group0()[1]]) * Vec4::from([1.0, 1.0, 1.0, 0.0]) + Vec4::splat(self.group0()[2]) * Vec4::from([other.group0()[2], other.group0()[1], other.group0()[2], other.group0()[0]]) * Vec4::from([1.0, -1.0, 0.0, 1.0]) + Vec4::splat(self.group0()[0]) * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[2], other.group0()[1]]) * Vec4::from([0.0, 1.0, -1.0, 1.0]) } }
    }
}

impl RegressiveProduct<Translator> for Plane {
    type Output = f32;

    fn regressive_product(self, other: Translator) -> f32 {
        self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2]
    }
}

impl OuterProduct<Translator> for Plane {
    type Output = MotorDual;

    fn outer_product(self, other: Translator) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[2]) * Vec4::from([other.group0()[2], other.group0()[2], other.group0()[2], other.group0()[0]]) * Vec4::from([1.0, 0.0, 0.0, 1.0]) + Vec4::from([self.group0()[1], self.group0()[0], self.group0()[1], self.group0()[0]]) * Vec4::from([other.group0()[1], other.group0()[0], other.group0()[0], other.group0()[0]]) * Vec4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl InnerProduct<Translator> for Plane {
    type Output = Plane;

    fn inner_product(self, other: Translator) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[0]) * other.group0().xzy() * Vec3::from([1.0, -1.0, 1.0]) + Vec3::splat(self.group0()[2]) * other.group0().yyx() * Vec3::from([-1.0, 0.0, 1.0]) + self.group0().yyx() * other.group0().zxx() * Vec3::from([1.0, 1.0, 0.0]) } }
    }
}

impl LeftContraction<Translator> for Plane {
    type Output = Plane;

    fn left_contraction(self, other: Translator) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[2]) * Vec3::splat(other.group0()[1]) * Vec3::from([-1.0, 0.0, 0.0]) + self.group0().yxx() * other.group0().zzy() * Vec3::from([1.0, -1.0, 1.0]) } }
    }
}

impl RightContraction<Translator> for Plane {
    type Output = Plane;

    fn right_contraction(self, other: Translator) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Vec3::splat(other.group0()[0]) } }
    }
}

impl GeometricProduct<Motor> for Plane {
    type Output = MotorDual;

    fn geometric_product(self, other: Motor) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[0]) * other.group0().yxwz() * Vec4::from([1.0, 1.0, -1.0, 1.0]) + Vec4::splat(self.group0()[1]) * other.group0().zwxy() * Vec4::from([1.0, 1.0, 1.0, -1.0]) + Vec4::splat(self.group0()[2]) * other.group0().wzyx() * Vec4::from([1.0, -1.0, 1.0, 1.0]) } }
    }
}

impl RegressiveProduct<Motor> for Plane {
    type Output = f32;

    fn regressive_product(self, other: Motor) -> f32 {
        self.group0()[0] * other.group0()[1] + self.group0()[1] * other.group0()[2] + self.group0()[2] * other.group0()[3]
    }
}

impl OuterProduct<Motor> for Plane {
    type Output = MotorDual;

    fn outer_product(self, other: Motor) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[1]) * other.group0().zzxz() * Vec4::from([1.0, 0.0, 1.0, 0.0]) + Vec4::splat(self.group0()[2]) * other.group0().wwwx() * Vec4::from([1.0, 0.0, 0.0, 1.0]) + Vec4::splat(self.group0()[0]) * other.group0().yxxx() * Vec4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl InnerProduct<Motor> for Plane {
    type Output = Plane;

    fn inner_product(self, other: Motor) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[0]) * Vec3::from([other.group0()[0], other.group0()[3], other.group0()[2]]) * Vec3::from([1.0, -1.0, 1.0]) + Vec3::splat(self.group0()[1]) * Vec3::from([other.group0()[3], other.group0()[0], other.group0()[1]]) * Vec3::from([1.0, 1.0, -1.0]) + Vec3::splat(self.group0()[2]) * Vec3::from([other.group0()[2], other.group0()[1], other.group0()[0]]) * Vec3::from([-1.0, 1.0, 1.0]) } }
    }
}

impl LeftContraction<Motor> for Plane {
    type Output = Plane;

    fn left_contraction(self, other: Motor) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[1]) * Vec3::from([other.group0()[3], other.group0()[3], other.group0()[1]]) * Vec3::from([1.0, 0.0, -1.0]) + Vec3::splat(self.group0()[2]) * Vec3::from([other.group0()[2], other.group0()[1], other.group0()[2]]) * Vec3::from([-1.0, 1.0, 0.0]) + Vec3::splat(self.group0()[0]) * Vec3::from([other.group0()[0], other.group0()[3], other.group0()[2]]) * Vec3::from([0.0, -1.0, 1.0]) } }
    }
}

impl RightContraction<Motor> for Plane {
    type Output = Plane;

    fn right_contraction(self, other: Motor) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Vec3::splat(other.group0()[0]) } }
    }
}

impl Add<MotorDual> for Plane {
    type Output = MotorDual;

    fn add(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Vec4::from([0.0, 1.0, 1.0, 1.0]) + other.group0() } }
    }
}

impl Sub<MotorDual> for Plane {
    type Output = MotorDual;

    fn sub(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Vec4::from([0.0, 1.0, 1.0, 1.0]) - other.group0() } }
    }
}

impl GeometricProduct<MotorDual> for Plane {
    type Output = Motor;

    fn geometric_product(self, other: MotorDual) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[0]) * other.group0().yxwz() * Vec4::from([1.0, 1.0, 1.0, -1.0]) + Vec4::splat(self.group0()[1]) * other.group0().zwxy() * Vec4::from([1.0, -1.0, 1.0, 1.0]) + Vec4::splat(self.group0()[2]) * other.group0().wzyx() * Vec4::from([1.0, 1.0, -1.0, 1.0]) } }
    }
}

impl RegressiveProduct<MotorDual> for Plane {
    type Output = Plane;

    fn regressive_product(self, other: MotorDual) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Vec3::splat(other.group0()[0]) } }
    }
}

impl OuterProduct<MotorDual> for Plane {
    type Output = Point;

    fn outer_product(self, other: MotorDual) -> Point {
        Point { groups: PointGroups { g0: Vec3::splat(self.group0()[1]) * Vec3::from([other.group0()[3], other.group0()[3], other.group0()[1]]) * Vec3::from([-1.0, 0.0, 1.0]) + Vec3::splat(self.group0()[2]) * Vec3::from([other.group0()[2], other.group0()[1], other.group0()[2]]) * Vec3::from([1.0, -1.0, 0.0]) + Vec3::splat(self.group0()[0]) * Vec3::from([other.group0()[0], other.group0()[3], other.group0()[2]]) * Vec3::from([0.0, 1.0, -1.0]) } }
    }
}

impl InnerProduct<MotorDual> for Plane {
    type Output = Motor;

    fn inner_product(self, other: MotorDual) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[1]) * other.group0().zzxz() * Vec4::from([1.0, 0.0, 1.0, 0.0]) + Vec4::splat(self.group0()[2]) * other.group0().wwwx() * Vec4::from([1.0, 0.0, 0.0, 1.0]) + Vec4::splat(self.group0()[0]) * other.group0().yxxx() * Vec4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl LeftContraction<MotorDual> for Plane {
    type Output = Motor;

    fn left_contraction(self, other: MotorDual) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[1]) * other.group0().zzxz() * Vec4::from([1.0, 0.0, 1.0, 0.0]) + Vec4::splat(self.group0()[2]) * other.group0().wwwx() * Vec4::from([1.0, 0.0, 0.0, 1.0]) + Vec4::splat(self.group0()[0]) * other.group0().yxxx() * Vec4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl RightContraction<MotorDual> for Plane {
    type Output = f32;

    fn right_contraction(self, other: MotorDual) -> f32 {
        self.group0()[0] * other.group0()[1] + self.group0()[1] * other.group0()[2] + self.group0()[2] * other.group0()[3]
    }
}

impl ScalarProduct<MotorDual> for Plane {
    type Output = f32;

    fn scalar_product(self, other: MotorDual) -> f32 {
        self.group0()[0] * other.group0()[1] + self.group0()[1] * other.group0()[2] + self.group0()[2] * other.group0()[3]
    }
}

impl SquaredMagnitude for Plane {
    type Output = f32;

    fn squared_magnitude(self) -> f32 {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for Plane {
    type Output = f32;

    fn magnitude(self) -> f32 {
        self.squared_magnitude().sqrt()
    }
}

impl Mul<f32> for Plane {
    type Output = Plane;

    fn mul(self, other: f32) -> Plane {
        self.geometric_product(other)
    }
}

impl MulAssign<f32> for Plane {
    fn mul_assign(&mut self, other: f32) {
        *self = (*self).mul(other);
    }
}

impl Signum for Plane {
    type Output = Plane;

    fn signum(self) -> Plane {
        self.geometric_product(1.0 / self.magnitude())
    }
}

impl Inverse for Plane {
    type Output = Plane;

    fn inverse(self) -> Plane {
        self.reversal().geometric_product(1.0 / self.squared_magnitude())
    }
}

impl Zero for Translator {
    fn zero() -> Self {
        Translator { groups: TranslatorGroups { g0: Vec3::splat(0.0) } }
    }
}

impl One for Translator {
    fn one() -> Self {
        Translator { groups: TranslatorGroups { g0: Vec3::from([1.0, 0.0, 0.0]) } }
    }
}

impl Neg for Translator {
    type Output = Translator;

    fn neg(self) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Vec3::splat(-1.0) } }
    }
}

impl Automorphism for Translator {
    type Output = Translator;

    fn automorphism(self) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() } }
    }
}

impl Reversal for Translator {
    type Output = Translator;

    fn reversal(self) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Vec3::from([1.0, -1.0, -1.0]) } }
    }
}

impl Conjugation for Translator {
    type Output = Translator;

    fn conjugation(self) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Vec3::from([1.0, -1.0, -1.0]) } }
    }
}

impl Into<f32> for Translator {
    fn into(self) -> f32 {
        self.group0()[0]
    }
}

impl Add<f32> for Translator {
    type Output = Translator;

    fn add(self, other: f32) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() + Vec3::splat(other) * Vec3::from([1.0, 0.0, 0.0]) } }
    }
}

impl AddAssign<f32> for Translator {
    fn add_assign(&mut self, other: f32) {
        *self = (*self).add(other);
    }
}

impl Sub<f32> for Translator {
    type Output = Translator;

    fn sub(self, other: f32) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() - Vec3::splat(other) * Vec3::from([1.0, 0.0, 0.0]) } }
    }
}

impl SubAssign<f32> for Translator {
    fn sub_assign(&mut self, other: f32) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<f32> for Translator {
    type Output = Translator;

    fn geometric_product(self, other: f32) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Vec3::splat(other) } }
    }
}

impl OuterProduct<f32> for Translator {
    type Output = Translator;

    fn outer_product(self, other: f32) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Vec3::splat(other) } }
    }
}

impl InnerProduct<f32> for Translator {
    type Output = Translator;

    fn inner_product(self, other: f32) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Vec3::splat(other) } }
    }
}

impl LeftContraction<f32> for Translator {
    type Output = f32;

    fn left_contraction(self, other: f32) -> f32 {
        self.group0()[0] * other
    }
}

impl RightContraction<f32> for Translator {
    type Output = Translator;

    fn right_contraction(self, other: f32) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Vec3::splat(other) } }
    }
}

impl ScalarProduct<f32> for Translator {
    type Output = f32;

    fn scalar_product(self, other: f32) -> f32 {
        self.group0()[0] * other
    }
}

impl Add<MultiVector> for Translator {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[0]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) + other.group0(), g1: Vec4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Vec4::from([0.0, 0.0, 1.0, 1.0]) + other.group1() } }
    }
}

impl Sub<MultiVector> for Translator {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[0]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) - other.group0(), g1: Vec4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Vec4::from([0.0, 0.0, 1.0, 1.0]) - other.group1() } }
    }
}

impl GeometricProduct<MultiVector> for Translator {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + Vec4::splat(self.group0()[1]) * other.group1().zwxy() * Vec4::from([-1.0, 1.0, -1.0, -1.0]) + Vec4::splat(self.group0()[2]) * other.group1().wzyx() * Vec4::from([-1.0, -1.0, -1.0, 1.0]), g1: Vec4::splat(self.group0()[0]) * other.group1() + Vec4::splat(self.group0()[1]) * other.group0().zwxy() * Vec4::from([1.0, 1.0, 1.0, -1.0]) + Vec4::splat(self.group0()[2]) * other.group0().wzyx() * Vec4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl OuterProduct<MultiVector> for Translator {
    type Output = MultiVector;

    fn outer_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[0]) * other.group0(), g1: Vec4::splat(self.group0()[0]) * other.group1() + Vec4::splat(self.group0()[2]) * other.group0().zzzx() * Vec4::from([0.0, 1.0, 0.0, 1.0]) + Vec4::from([self.group0()[0], self.group0()[1], self.group0()[1], self.group0()[0]]) * other.group0().xwxx() * Vec4::from([0.0, 1.0, 1.0, 0.0]) } }
    }
}

impl InnerProduct<MultiVector> for Translator {
    type Output = MultiVector;

    fn inner_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + Vec4::splat(self.group0()[2]) * other.group1().wwyx() * Vec4::from([-1.0, 0.0, -1.0, 1.0]) + Vec4::from([self.group0()[1], self.group0()[0], self.group0()[1], self.group0()[1]]) * other.group1().zxxy() * Vec4::from([-1.0, 0.0, -1.0, -1.0]), g1: Vec4::splat(self.group0()[0]) * other.group1() + Vec4::splat(self.group0()[2]) * other.group0().wwwx() * Vec4::from([-1.0, 0.0, 0.0, 1.0]) + Vec4::from([self.group0()[1], self.group0()[0], self.group0()[1], self.group0()[0]]) * other.group0().zxxx() * Vec4::from([1.0, 0.0, 1.0, 0.0]) } }
    }
}

impl LeftContraction<MultiVector> for Translator {
    type Output = MultiVector;

    fn left_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + Vec4::splat(self.group0()[2]) * other.group1().wwyw() * Vec4::from([-1.0, 0.0, -1.0, 0.0]) + Vec4::from([self.group0()[1], self.group0()[0], self.group0()[0], self.group0()[1]]) * other.group1().zxxy() * Vec4::from([-1.0, 0.0, 0.0, -1.0]), g1: Vec4::splat(self.group0()[0]) * other.group1() } }
    }
}

impl ScalarProduct<MultiVector> for Translator {
    type Output = f32;

    fn scalar_product(self, other: MultiVector) -> f32 {
        self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group1()[2] - self.group0()[2] * other.group1()[3]
    }
}

impl Add<Rotor> for Translator {
    type Output = Motor;

    fn add(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Vec4::from([1.0, 0.0, 1.0, 1.0]) + Vec4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[0]]) * Vec4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl Sub<Rotor> for Translator {
    type Output = Motor;

    fn sub(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Vec4::from([1.0, 0.0, 1.0, 1.0]) - Vec4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[0]]) * Vec4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl GeometricProduct<Rotor> for Translator {
    type Output = Motor;

    fn geometric_product(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[2]) * Vec4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]) * Vec4::from([0.0, 0.0, 1.0, 1.0]) + Vec4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[1]]) * Vec4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[1]]) * Vec4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl OuterProduct<Rotor> for Translator {
    type Output = Motor;

    fn outer_product(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Vec4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[0]]) } }
    }
}

impl InnerProduct<Rotor> for Translator {
    type Output = Motor;

    fn inner_product(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Vec4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[0]]) } }
    }
}

impl LeftContraction<Rotor> for Translator {
    type Output = Rotor;

    fn left_contraction(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Vec2::splat(self.group0()[0]) * other.group0() } }
    }
}

impl RightContraction<Rotor> for Translator {
    type Output = Translator;

    fn right_contraction(self, other: Rotor) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Vec3::splat(other.group0()[0]) } }
    }
}

impl ScalarProduct<Rotor> for Translator {
    type Output = f32;

    fn scalar_product(self, other: Rotor) -> f32 {
        self.group0()[0] * other.group0()[0]
    }
}

impl Add<Point> for Translator {
    type Output = Motor;

    fn add(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Vec4::from([1.0, 0.0, 1.0, 1.0]) + Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl Sub<Point> for Translator {
    type Output = Motor;

    fn sub(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Vec4::from([1.0, 0.0, 1.0, 1.0]) - Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl GeometricProduct<Point> for Translator {
    type Output = Motor;

    fn geometric_product(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[1]) * Vec4::from([other.group0()[1], other.group0()[2], other.group0()[1], other.group0()[0]]) * Vec4::from([-1.0, 1.0, 0.0, -1.0]) + Vec4::splat(self.group0()[2]) * Vec4::from([other.group0()[2], other.group0()[1], other.group0()[0], other.group0()[2]]) * Vec4::from([-1.0, -1.0, 1.0, 0.0]) + Vec4::splat(self.group0()[0]) * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl RegressiveProduct<Point> for Translator {
    type Output = Plane;

    fn regressive_product(self, other: Point) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[2]) * other.group0().yxy() * Vec3::from([1.0, -1.0, 0.0]) + self.group0().yxy() * other.group0().zxx() * Vec3::from([-1.0, 0.0, 1.0]) } }
    }
}

impl OuterProduct<Point> for Translator {
    type Output = Point;

    fn outer_product(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Vec3::splat(self.group0()[0]) * other.group0() } }
    }
}

impl InnerProduct<Point> for Translator {
    type Output = Motor;

    fn inner_product(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[2]) * Vec4::splat(other.group0()[2]) * Vec4::from([-1.0, 0.0, 0.0, 0.0]) + Vec4::from([self.group0()[1], self.group0()[0], self.group0()[0], self.group0()[0]]) * Vec4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl LeftContraction<Point> for Translator {
    type Output = Motor;

    fn left_contraction(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[2]) * Vec4::splat(other.group0()[2]) * Vec4::from([-1.0, 0.0, 0.0, 0.0]) + Vec4::from([self.group0()[1], self.group0()[0], self.group0()[0], self.group0()[0]]) * Vec4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl RightContraction<Point> for Translator {
    type Output = f32;

    fn right_contraction(self, other: Point) -> f32 {
        0.0 - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2]
    }
}

impl ScalarProduct<Point> for Translator {
    type Output = f32;

    fn scalar_product(self, other: Point) -> f32 {
        0.0 - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2]
    }
}

impl Into<IdealPoint> for Translator {
    fn into(self) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Vec2::from([self.group0()[1], self.group0()[2]]) } }
    }
}

impl Add<IdealPoint> for Translator {
    type Output = Translator;

    fn add(self, other: IdealPoint) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() + Vec3::from([other.group0()[0], other.group0()[0], other.group0()[1]]) * Vec3::from([0.0, 1.0, 1.0]) } }
    }
}

impl AddAssign<IdealPoint> for Translator {
    fn add_assign(&mut self, other: IdealPoint) {
        *self = (*self).add(other);
    }
}

impl Sub<IdealPoint> for Translator {
    type Output = Translator;

    fn sub(self, other: IdealPoint) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() - Vec3::from([other.group0()[0], other.group0()[0], other.group0()[1]]) * Vec3::from([0.0, 1.0, 1.0]) } }
    }
}

impl SubAssign<IdealPoint> for Translator {
    fn sub_assign(&mut self, other: IdealPoint) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<IdealPoint> for Translator {
    type Output = Motor;

    fn geometric_product(self, other: IdealPoint) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[2]) * Vec4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Vec4::from([-1.0, -1.0, 0.0, 0.0]) + Vec4::from([self.group0()[1], self.group0()[1], self.group0()[0], self.group0()[0]]) * Vec4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[1]]) * Vec4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl OuterProduct<IdealPoint> for Translator {
    type Output = IdealPoint;

    fn outer_product(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Vec2::splat(self.group0()[0]) * other.group0() } }
    }
}

impl InnerProduct<IdealPoint> for Translator {
    type Output = Translator;

    fn inner_product(self, other: IdealPoint) -> Translator {
        Translator { groups: TranslatorGroups { g0: Vec3::splat(self.group0()[2]) * Vec3::splat(other.group0()[1]) * Vec3::from([-1.0, 0.0, 0.0]) + self.group0().yxx() * Vec3::from([other.group0()[0], other.group0()[0], other.group0()[1]]) * Vec3::from([-1.0, 1.0, 1.0]) } }
    }
}

impl LeftContraction<IdealPoint> for Translator {
    type Output = Translator;

    fn left_contraction(self, other: IdealPoint) -> Translator {
        Translator { groups: TranslatorGroups { g0: Vec3::splat(self.group0()[2]) * Vec3::splat(other.group0()[1]) * Vec3::from([-1.0, 0.0, 0.0]) + self.group0().yxx() * Vec3::from([other.group0()[0], other.group0()[0], other.group0()[1]]) * Vec3::from([-1.0, 1.0, 1.0]) } }
    }
}

impl RightContraction<IdealPoint> for Translator {
    type Output = f32;

    fn right_contraction(self, other: IdealPoint) -> f32 {
        0.0 - self.group0()[1] * other.group0()[0] - self.group0()[2] * other.group0()[1]
    }
}

impl ScalarProduct<IdealPoint> for Translator {
    type Output = f32;

    fn scalar_product(self, other: IdealPoint) -> f32 {
        0.0 - self.group0()[1] * other.group0()[0] - self.group0()[2] * other.group0()[1]
    }
}

impl GeometricProduct<Plane> for Translator {
    type Output = MotorDual;

    fn geometric_product(self, other: Plane) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[1]) * Vec4::from([other.group0()[1], other.group0()[2], other.group0()[1], other.group0()[0]]) * Vec4::from([1.0, 1.0, 0.0, -1.0]) + Vec4::splat(self.group0()[2]) * Vec4::from([other.group0()[2], other.group0()[1], other.group0()[0], other.group0()[2]]) * Vec4::from([1.0, -1.0, 1.0, 0.0]) + Vec4::splat(self.group0()[0]) * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl RegressiveProduct<Plane> for Translator {
    type Output = f32;

    fn regressive_product(self, other: Plane) -> f32 {
        self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2]
    }
}

impl OuterProduct<Plane> for Translator {
    type Output = MotorDual;

    fn outer_product(self, other: Plane) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[2]) * Vec4::splat(other.group0()[2]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) + Vec4::from([self.group0()[1], self.group0()[0], self.group0()[0], self.group0()[0]]) * Vec4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl InnerProduct<Plane> for Translator {
    type Output = Plane;

    fn inner_product(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[0]) * other.group0() + Vec3::splat(self.group0()[2]) * other.group0().yxy() * Vec3::from([-1.0, 1.0, 0.0]) + self.group0().yxy() * other.group0().zxx() * Vec3::from([1.0, 0.0, -1.0]) } }
    }
}

impl LeftContraction<Plane> for Translator {
    type Output = Plane;

    fn left_contraction(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[0]) * other.group0() } }
    }
}

impl RightContraction<Plane> for Translator {
    type Output = Plane;

    fn right_contraction(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[2]) * other.group0().yxy() * Vec3::from([-1.0, 1.0, 0.0]) + self.group0().yxy() * other.group0().zxx() * Vec3::from([1.0, 0.0, -1.0]) } }
    }
}

impl Add<Translator> for Translator {
    type Output = Translator;

    fn add(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() + other.group0() } }
    }
}

impl AddAssign<Translator> for Translator {
    fn add_assign(&mut self, other: Translator) {
        *self = (*self).add(other);
    }
}

impl Sub<Translator> for Translator {
    type Output = Translator;

    fn sub(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() - other.group0() } }
    }
}

impl SubAssign<Translator> for Translator {
    fn sub_assign(&mut self, other: Translator) {
        *self = (*self).sub(other);
    }
}

impl Mul<Translator> for Translator {
    type Output = Translator;

    fn mul(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * other.group0() } }
    }
}

impl MulAssign<Translator> for Translator {
    fn mul_assign(&mut self, other: Translator) {
        *self = (*self).mul(other);
    }
}

impl Div<Translator> for Translator {
    type Output = Translator;

    fn div(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Vec3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Vec3::from([1.0, 1.0, 1.0]) / Vec3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec3::from([1.0, 1.0, 1.0]) } }
    }
}

impl DivAssign<Translator> for Translator {
    fn div_assign(&mut self, other: Translator) {
        *self = (*self).div(other);
    }
}

impl GeometricProduct<Translator> for Translator {
    type Output = Motor;

    fn geometric_product(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[1]) * Vec4::from([other.group0()[1], other.group0()[2], other.group0()[0], other.group0()[1]]) * Vec4::from([-1.0, 1.0, 1.0, 0.0]) + Vec4::splat(self.group0()[2]) * Vec4::from([other.group0()[2], other.group0()[1], other.group0()[2], other.group0()[0]]) * Vec4::from([-1.0, -1.0, 0.0, 1.0]) + Vec4::splat(self.group0()[0]) * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([1.0, 0.0, 1.0, 1.0]) } }
    }
}

impl OuterProduct<Translator> for Translator {
    type Output = Translator;

    fn outer_product(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Vec3::splat(self.group0()[0]) * other.group0() + self.group0() * Vec3::splat(other.group0()[0]) * Vec3::from([0.0, 1.0, 1.0]) } }
    }
}

impl InnerProduct<Translator> for Translator {
    type Output = Translator;

    fn inner_product(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Vec3::splat(self.group0()[0]) * other.group0() + Vec3::splat(self.group0()[2]) * other.group0().zzx() * Vec3::from([-1.0, 0.0, 1.0]) + self.group0().yyx() * other.group0().yxx() * Vec3::from([-1.0, 1.0, 0.0]) } }
    }
}

impl LeftContraction<Translator> for Translator {
    type Output = Translator;

    fn left_contraction(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Vec3::splat(self.group0()[0]) * other.group0() + Vec3::splat(self.group0()[2]) * Vec3::splat(other.group0()[2]) * Vec3::from([-1.0, 0.0, 0.0]) + self.group0().yxx() * other.group0().yxx() * Vec3::from([-1.0, 0.0, 0.0]) } }
    }
}

impl RightContraction<Translator> for Translator {
    type Output = Translator;

    fn right_contraction(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Vec3::splat(self.group0()[1]) * other.group0().yxy() * Vec3::from([-1.0, 1.0, 0.0]) + Vec3::splat(self.group0()[2]) * other.group0().zzx() * Vec3::from([-1.0, 0.0, 1.0]) + Vec3::splat(self.group0()[0]) * Vec3::splat(other.group0()[0]) * Vec3::from([1.0, 0.0, 0.0]) } }
    }
}

impl ScalarProduct<Translator> for Translator {
    type Output = f32;

    fn scalar_product(self, other: Translator) -> f32 {
        self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2]
    }
}

impl Add<Motor> for Translator {
    type Output = Motor;

    fn add(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Vec4::from([1.0, 0.0, 1.0, 1.0]) + other.group0() } }
    }
}

impl Sub<Motor> for Translator {
    type Output = Motor;

    fn sub(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Vec4::from([1.0, 0.0, 1.0, 1.0]) - other.group0() } }
    }
}

impl GeometricProduct<Motor> for Translator {
    type Output = Motor;

    fn geometric_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + Vec4::splat(self.group0()[1]) * other.group0().zwxy() * Vec4::from([-1.0, 1.0, 1.0, -1.0]) + Vec4::splat(self.group0()[2]) * other.group0().wzyx() * Vec4::from([-1.0, -1.0, 1.0, 1.0]) } }
    }
}

impl RegressiveProduct<Motor> for Translator {
    type Output = Plane;

    fn regressive_product(self, other: Motor) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[2]) * Vec3::from([other.group0()[2], other.group0()[1], other.group0()[2]]) * Vec3::from([1.0, -1.0, 0.0]) + self.group0().yxy() * Vec3::from([other.group0()[3], other.group0()[0], other.group0()[1]]) * Vec3::from([-1.0, 0.0, 1.0]) } }
    }
}

impl OuterProduct<Motor> for Translator {
    type Output = Motor;

    fn outer_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + Vec4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 0.0, 1.0, 1.0]) } }
    }
}

impl InnerProduct<Motor> for Translator {
    type Output = Motor;

    fn inner_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + Vec4::splat(self.group0()[2]) * other.group0().wwwx() * Vec4::from([-1.0, 0.0, 0.0, 1.0]) + Vec4::from([self.group0()[1], self.group0()[0], self.group0()[1], self.group0()[0]]) * other.group0().zxxx() * Vec4::from([-1.0, 0.0, 1.0, 0.0]) } }
    }
}

impl LeftContraction<Motor> for Translator {
    type Output = Motor;

    fn left_contraction(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + Vec4::splat(self.group0()[2]) * Vec4::splat(other.group0()[3]) * Vec4::from([-1.0, 0.0, 0.0, 0.0]) + Vec4::from([self.group0()[1], self.group0()[0], self.group0()[0], self.group0()[0]]) * other.group0().zxxx() * Vec4::from([-1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl RightContraction<Motor> for Translator {
    type Output = Translator;

    fn right_contraction(self, other: Motor) -> Translator {
        Translator { groups: TranslatorGroups { g0: Vec3::splat(self.group0()[1]) * Vec3::from([other.group0()[2], other.group0()[0], other.group0()[2]]) * Vec3::from([-1.0, 1.0, 0.0]) + Vec3::splat(self.group0()[2]) * Vec3::from([other.group0()[3], other.group0()[3], other.group0()[0]]) * Vec3::from([-1.0, 0.0, 1.0]) + Vec3::splat(self.group0()[0]) * Vec3::splat(other.group0()[0]) * Vec3::from([1.0, 0.0, 0.0]) } }
    }
}

impl ScalarProduct<Motor> for Translator {
    type Output = f32;

    fn scalar_product(self, other: Motor) -> f32 {
        self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[2] - self.group0()[2] * other.group0()[3]
    }
}

impl GeometricProduct<MotorDual> for Translator {
    type Output = MotorDual;

    fn geometric_product(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + Vec4::splat(self.group0()[1]) * other.group0().zwxy() * Vec4::from([1.0, 1.0, -1.0, -1.0]) + Vec4::splat(self.group0()[2]) * other.group0().wzyx() * Vec4::from([1.0, -1.0, 1.0, -1.0]) } }
    }
}

impl RegressiveProduct<MotorDual> for Translator {
    type Output = Translator;

    fn regressive_product(self, other: MotorDual) -> Translator {
        Translator { groups: TranslatorGroups { g0: Vec3::splat(self.group0()[1]) * Vec3::from([other.group0()[2], other.group0()[0], other.group0()[2]]) * Vec3::from([1.0, 1.0, 0.0]) + Vec3::splat(self.group0()[2]) * Vec3::from([other.group0()[3], other.group0()[3], other.group0()[0]]) * Vec3::from([1.0, 0.0, 1.0]) + Vec3::splat(self.group0()[0]) * Vec3::splat(other.group0()[0]) * Vec3::from([1.0, 0.0, 0.0]) } }
    }
}

impl OuterProduct<MotorDual> for Translator {
    type Output = MotorDual;

    fn outer_product(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + Vec4::splat(self.group0()[2]) * Vec4::splat(other.group0()[3]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) + Vec4::from([self.group0()[1], self.group0()[0], self.group0()[0], self.group0()[0]]) * other.group0().zxxx() * Vec4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl InnerProduct<MotorDual> for Translator {
    type Output = MotorDual;

    fn inner_product(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + Vec4::splat(self.group0()[2]) * other.group0().zzyx() * Vec4::from([0.0, -1.0, 1.0, -1.0]) + Vec4::from([self.group0()[0], self.group0()[1], self.group0()[1], self.group0()[1]]) * other.group0().xwxy() * Vec4::from([0.0, 1.0, -1.0, -1.0]) } }
    }
}

impl LeftContraction<MotorDual> for Translator {
    type Output = MotorDual;

    fn left_contraction(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + Vec4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 0.0, -1.0, -1.0]) } }
    }
}

impl RightContraction<MotorDual> for Translator {
    type Output = Plane;

    fn right_contraction(self, other: MotorDual) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[2]) * Vec3::from([other.group0()[2], other.group0()[1], other.group0()[2]]) * Vec3::from([-1.0, 1.0, 0.0]) + self.group0().yxy() * Vec3::from([other.group0()[3], other.group0()[0], other.group0()[1]]) * Vec3::from([1.0, 0.0, -1.0]) } }
    }
}

impl SquaredMagnitude for Translator {
    type Output = f32;

    fn squared_magnitude(self) -> f32 {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for Translator {
    type Output = f32;

    fn magnitude(self) -> f32 {
        self.squared_magnitude().sqrt()
    }
}

impl Mul<f32> for Translator {
    type Output = Translator;

    fn mul(self, other: f32) -> Translator {
        self.geometric_product(other)
    }
}

impl MulAssign<f32> for Translator {
    fn mul_assign(&mut self, other: f32) {
        *self = (*self).mul(other);
    }
}

impl Signum for Translator {
    type Output = Translator;

    fn signum(self) -> Translator {
        self.geometric_product(1.0 / self.magnitude())
    }
}

impl Inverse for Translator {
    type Output = Translator;

    fn inverse(self) -> Translator {
        self.reversal().geometric_product(1.0 / self.squared_magnitude())
    }
}

impl Zero for Motor {
    fn zero() -> Self {
        Motor { groups: MotorGroups { g0: Vec4::splat(0.0) } }
    }
}

impl One for Motor {
    fn one() -> Self {
        Motor { groups: MotorGroups { g0: Vec4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl Neg for Motor {
    type Output = Motor;

    fn neg(self) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Vec4::splat(-1.0) } }
    }
}

impl Automorphism for Motor {
    type Output = Motor;

    fn automorphism(self) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() } }
    }
}

impl Reversal for Motor {
    type Output = Motor;

    fn reversal(self) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Vec4::from([1.0, -1.0, -1.0, -1.0]) } }
    }
}

impl Conjugation for Motor {
    type Output = Motor;

    fn conjugation(self) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Vec4::from([1.0, -1.0, -1.0, -1.0]) } }
    }
}

impl Dual for Motor {
    type Output = MotorDual;

    fn dual(self) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: self.group0() } }
    }
}

impl Into<f32> for Motor {
    fn into(self) -> f32 {
        self.group0()[0]
    }
}

impl Add<f32> for Motor {
    type Output = Motor;

    fn add(self, other: f32) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() + Vec4::splat(other) * Vec4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl AddAssign<f32> for Motor {
    fn add_assign(&mut self, other: f32) {
        *self = (*self).add(other);
    }
}

impl Sub<f32> for Motor {
    type Output = Motor;

    fn sub(self, other: f32) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() - Vec4::splat(other) * Vec4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl SubAssign<f32> for Motor {
    fn sub_assign(&mut self, other: f32) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<f32> for Motor {
    type Output = Motor;

    fn geometric_product(self, other: f32) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Vec4::splat(other) } }
    }
}

impl OuterProduct<f32> for Motor {
    type Output = Motor;

    fn outer_product(self, other: f32) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Vec4::splat(other) } }
    }
}

impl InnerProduct<f32> for Motor {
    type Output = Motor;

    fn inner_product(self, other: f32) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Vec4::splat(other) } }
    }
}

impl LeftContraction<f32> for Motor {
    type Output = f32;

    fn left_contraction(self, other: f32) -> f32 {
        self.group0()[0] * other
    }
}

impl RightContraction<f32> for Motor {
    type Output = Motor;

    fn right_contraction(self, other: f32) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Vec4::splat(other) } }
    }
}

impl ScalarProduct<f32> for Motor {
    type Output = f32;

    fn scalar_product(self, other: f32) -> f32 {
        self.group0()[0] * other
    }
}

impl Add<MultiVector> for Motor {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0().xyxx() * Vec4::from([1.0, 1.0, 0.0, 0.0]) + other.group0(), g1: self.group0().xxzw() * Vec4::from([0.0, 0.0, 1.0, 1.0]) + other.group1() } }
    }
}

impl Sub<MultiVector> for Motor {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0().xyxx() * Vec4::from([1.0, 1.0, 0.0, 0.0]) - other.group0(), g1: self.group0().xxzw() * Vec4::from([0.0, 0.0, 1.0, 1.0]) - other.group1() } }
    }
}

impl GeometricProduct<MultiVector> for Motor {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + Vec4::splat(self.group0()[1]) * other.group0().yxwz() * Vec4::from([-1.0, 1.0, 1.0, -1.0]) + Vec4::splat(self.group0()[2]) * other.group1().zwxy() * Vec4::from([-1.0, 1.0, -1.0, -1.0]) + Vec4::splat(self.group0()[3]) * other.group1().wzyx() * Vec4::from([-1.0, -1.0, -1.0, 1.0]), g1: Vec4::splat(self.group0()[0]) * other.group1() + Vec4::splat(self.group0()[1]) * other.group1().yxwz() * Vec4::from([-1.0, 1.0, -1.0, 1.0]) + Vec4::splat(self.group0()[2]) * other.group0().zwxy() * Vec4::from([1.0, 1.0, 1.0, -1.0]) + Vec4::splat(self.group0()[3]) * other.group0().wzyx() * Vec4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl OuterProduct<MultiVector> for Motor {
    type Output = MultiVector;

    fn outer_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + self.group0().xyxx() * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 1.0, 0.0, 0.0]), g1: Vec4::splat(self.group0()[0]) * other.group1() + Vec4::splat(self.group0()[2]) * other.group0().wwxw() * Vec4::from([0.0, 1.0, 1.0, 0.0]) + Vec4::splat(self.group0()[3]) * other.group0().zzzx() * Vec4::from([0.0, 1.0, 0.0, 1.0]) + self.group0().xyxx() * Vec4::splat(other.group1()[0]) * Vec4::from([0.0, 1.0, 0.0, 0.0]) } }
    }
}

impl InnerProduct<MultiVector> for Motor {
    type Output = MultiVector;

    fn inner_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + Vec4::splat(self.group0()[1]) * other.group0().yxwz() * Vec4::from([-1.0, 1.0, 1.0, -1.0]) + Vec4::splat(self.group0()[3]) * other.group1().wwyx() * Vec4::from([-1.0, 0.0, -1.0, 1.0]) + self.group0().zxzz() * other.group1().zxxy() * Vec4::from([-1.0, 0.0, -1.0, -1.0]), g1: Vec4::splat(self.group0()[0]) * other.group1() + Vec4::splat(self.group0()[2]) * other.group0().zzxz() * Vec4::from([1.0, 0.0, 1.0, 0.0]) + Vec4::splat(self.group0()[3]) * other.group0().wwwx() * Vec4::from([-1.0, 0.0, 0.0, 1.0]) + self.group0().yxxx() * other.group1().yxxx() * Vec4::from([-1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl LeftContraction<MultiVector> for Motor {
    type Output = MultiVector;

    fn left_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + Vec4::splat(self.group0()[2]) * other.group1().zzzy() * Vec4::from([-1.0, 0.0, 0.0, -1.0]) + Vec4::splat(self.group0()[3]) * other.group1().wwyw() * Vec4::from([-1.0, 0.0, -1.0, 0.0]) + self.group0().yxxx() * other.group0().yxxx() * Vec4::from([-1.0, 0.0, 0.0, 0.0]), g1: Vec4::splat(self.group0()[0]) * other.group1() + self.group0().yxxx() * other.group1().yxxx() * Vec4::from([-1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl ScalarProduct<MultiVector> for Motor {
    type Output = f32;

    fn scalar_product(self, other: MultiVector) -> f32 {
        self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group1()[2] - self.group0()[3] * other.group1()[3]
    }
}

impl Into<Rotor> for Motor {
    fn into(self) -> Rotor {
        Rotor { groups: RotorGroups { g0: Vec2::from([self.group0()[0], self.group0()[1]]) } }
    }
}

impl Add<Rotor> for Motor {
    type Output = Motor;

    fn add(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() + Vec4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[0]]) * Vec4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl AddAssign<Rotor> for Motor {
    fn add_assign(&mut self, other: Rotor) {
        *self = (*self).add(other);
    }
}

impl Sub<Rotor> for Motor {
    type Output = Motor;

    fn sub(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() - Vec4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[0]]) * Vec4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl SubAssign<Rotor> for Motor {
    fn sub_assign(&mut self, other: Rotor) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Rotor> for Motor {
    type Output = Motor;

    fn geometric_product(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[1]) * Vec4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Vec4::from([-1.0, 1.0, 0.0, 0.0]) + Vec4::splat(self.group0()[3]) * Vec4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]) * Vec4::from([0.0, 0.0, 1.0, 1.0]) + self.group0().xxzz() * Vec4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[1]]) * Vec4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl OuterProduct<Rotor> for Motor {
    type Output = Motor;

    fn outer_product(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[1]) * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 1.0, 0.0, 0.0]) + self.group0().xxzw() * Vec4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[0]]) } }
    }
}

impl InnerProduct<Rotor> for Motor {
    type Output = Motor;

    fn inner_product(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[1]) * Vec4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Vec4::from([-1.0, 1.0, 0.0, 0.0]) + self.group0().xxzw() * Vec4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[0]]) } }
    }
}

impl LeftContraction<Rotor> for Motor {
    type Output = Rotor;

    fn left_contraction(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Vec2::splat(self.group0()[0]) * other.group0() + Vec2::from([self.group0()[1], self.group0()[0]]) * other.group0().yx() * Vec2::from([-1.0, 0.0]) } }
    }
}

impl RightContraction<Rotor> for Motor {
    type Output = Motor;

    fn right_contraction(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[1]) * Vec4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Vec4::from([-1.0, 1.0, 0.0, 0.0]) + self.group0().xxzw() * Vec4::splat(other.group0()[0]) * Vec4::from([1.0, 0.0, 1.0, 1.0]) } }
    }
}

impl ScalarProduct<Rotor> for Motor {
    type Output = f32;

    fn scalar_product(self, other: Rotor) -> f32 {
        self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1]
    }
}

impl Into<Point> for Motor {
    fn into(self) -> Point {
        Point { groups: PointGroups { g0: Vec3::from([self.group0()[1], self.group0()[2], self.group0()[3]]) } }
    }
}

impl Add<Point> for Motor {
    type Output = Motor;

    fn add(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() + Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl AddAssign<Point> for Motor {
    fn add_assign(&mut self, other: Point) {
        *self = (*self).add(other);
    }
}

impl Sub<Point> for Motor {
    type Output = Motor;

    fn sub(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() - Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl SubAssign<Point> for Motor {
    fn sub_assign(&mut self, other: Point) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Point> for Motor {
    type Output = Motor;

    fn geometric_product(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[1]) * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[2], other.group0()[1]]) * Vec4::from([-1.0, 0.0, -1.0, 1.0]) + Vec4::splat(self.group0()[2]) * Vec4::from([other.group0()[1], other.group0()[2], other.group0()[1], other.group0()[0]]) * Vec4::from([-1.0, 1.0, 0.0, -1.0]) + Vec4::splat(self.group0()[3]) * Vec4::from([other.group0()[2], other.group0()[1], other.group0()[0], other.group0()[2]]) * Vec4::from([-1.0, -1.0, 1.0, 0.0]) + Vec4::splat(self.group0()[0]) * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl RegressiveProduct<Point> for Motor {
    type Output = Plane;

    fn regressive_product(self, other: Point) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[2]) * other.group0().zzx() * Vec3::from([-1.0, 0.0, 1.0]) + Vec3::splat(self.group0()[3]) * other.group0().yxy() * Vec3::from([1.0, -1.0, 0.0]) + Vec3::from([self.group0()[0], self.group0()[1], self.group0()[1]]) * other.group0().xzy() * Vec3::from([0.0, 1.0, -1.0]) } }
    }
}

impl OuterProduct<Point> for Motor {
    type Output = Point;

    fn outer_product(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Vec3::splat(self.group0()[0]) * other.group0() } }
    }
}

impl InnerProduct<Point> for Motor {
    type Output = Motor;

    fn inner_product(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[2]) * Vec4::splat(other.group0()[1]) * Vec4::from([-1.0, 0.0, 0.0, 0.0]) + Vec4::splat(self.group0()[3]) * Vec4::splat(other.group0()[2]) * Vec4::from([-1.0, 0.0, 0.0, 0.0]) + self.group0().yxxx() * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl LeftContraction<Point> for Motor {
    type Output = Motor;

    fn left_contraction(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[2]) * Vec4::splat(other.group0()[1]) * Vec4::from([-1.0, 0.0, 0.0, 0.0]) + Vec4::splat(self.group0()[3]) * Vec4::splat(other.group0()[2]) * Vec4::from([-1.0, 0.0, 0.0, 0.0]) + self.group0().yxxx() * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl RightContraction<Point> for Motor {
    type Output = f32;

    fn right_contraction(self, other: Point) -> f32 {
        0.0 - self.group0()[1] * other.group0()[0] - self.group0()[2] * other.group0()[1] - self.group0()[3] * other.group0()[2]
    }
}

impl ScalarProduct<Point> for Motor {
    type Output = f32;

    fn scalar_product(self, other: Point) -> f32 {
        0.0 - self.group0()[1] * other.group0()[0] - self.group0()[2] * other.group0()[1] - self.group0()[3] * other.group0()[2]
    }
}

impl Into<IdealPoint> for Motor {
    fn into(self) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Vec2::from([self.group0()[2], self.group0()[3]]) } }
    }
}

impl Add<IdealPoint> for Motor {
    type Output = Motor;

    fn add(self, other: IdealPoint) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() + Vec4::from([other.group0()[0], other.group0()[0], other.group0()[0], other.group0()[1]]) * Vec4::from([0.0, 0.0, 1.0, 1.0]) } }
    }
}

impl AddAssign<IdealPoint> for Motor {
    fn add_assign(&mut self, other: IdealPoint) {
        *self = (*self).add(other);
    }
}

impl Sub<IdealPoint> for Motor {
    type Output = Motor;

    fn sub(self, other: IdealPoint) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() - Vec4::from([other.group0()[0], other.group0()[0], other.group0()[0], other.group0()[1]]) * Vec4::from([0.0, 0.0, 1.0, 1.0]) } }
    }
}

impl SubAssign<IdealPoint> for Motor {
    fn sub_assign(&mut self, other: IdealPoint) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<IdealPoint> for Motor {
    type Output = Motor;

    fn geometric_product(self, other: IdealPoint) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[1]) * Vec4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]) * Vec4::from([0.0, 0.0, -1.0, 1.0]) + Vec4::splat(self.group0()[3]) * Vec4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Vec4::from([-1.0, -1.0, 0.0, 0.0]) + self.group0().zzxx() * Vec4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[1]]) * Vec4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl RegressiveProduct<IdealPoint> for Motor {
    type Output = Plane;

    fn regressive_product(self, other: IdealPoint) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[3]) * Vec3::splat(other.group0()[0]) * Vec3::from([1.0, 0.0, 0.0]) + Vec3::from([self.group0()[2], self.group0()[1], self.group0()[1]]) * Vec3::from([other.group0()[1], other.group0()[1], other.group0()[0]]) * Vec3::from([-1.0, 1.0, -1.0]) } }
    }
}

impl OuterProduct<IdealPoint> for Motor {
    type Output = IdealPoint;

    fn outer_product(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Vec2::splat(self.group0()[0]) * other.group0() } }
    }
}

impl InnerProduct<IdealPoint> for Motor {
    type Output = Translator;

    fn inner_product(self, other: IdealPoint) -> Translator {
        Translator { groups: TranslatorGroups { g0: Vec3::splat(self.group0()[3]) * Vec3::splat(other.group0()[1]) * Vec3::from([-1.0, 0.0, 0.0]) + Vec3::from([self.group0()[2], self.group0()[0], self.group0()[0]]) * Vec3::from([other.group0()[0], other.group0()[0], other.group0()[1]]) * Vec3::from([-1.0, 1.0, 1.0]) } }
    }
}

impl LeftContraction<IdealPoint> for Motor {
    type Output = Translator;

    fn left_contraction(self, other: IdealPoint) -> Translator {
        Translator { groups: TranslatorGroups { g0: Vec3::splat(self.group0()[3]) * Vec3::splat(other.group0()[1]) * Vec3::from([-1.0, 0.0, 0.0]) + Vec3::from([self.group0()[2], self.group0()[0], self.group0()[0]]) * Vec3::from([other.group0()[0], other.group0()[0], other.group0()[1]]) * Vec3::from([-1.0, 1.0, 1.0]) } }
    }
}

impl RightContraction<IdealPoint> for Motor {
    type Output = f32;

    fn right_contraction(self, other: IdealPoint) -> f32 {
        0.0 - self.group0()[2] * other.group0()[0] - self.group0()[3] * other.group0()[1]
    }
}

impl ScalarProduct<IdealPoint> for Motor {
    type Output = f32;

    fn scalar_product(self, other: IdealPoint) -> f32 {
        0.0 - self.group0()[2] * other.group0()[0] - self.group0()[3] * other.group0()[1]
    }
}

impl GeometricProduct<Plane> for Motor {
    type Output = MotorDual;

    fn geometric_product(self, other: Plane) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[1]) * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[2], other.group0()[1]]) * Vec4::from([1.0, 0.0, -1.0, 1.0]) + Vec4::splat(self.group0()[2]) * Vec4::from([other.group0()[1], other.group0()[2], other.group0()[1], other.group0()[0]]) * Vec4::from([1.0, 1.0, 0.0, -1.0]) + Vec4::splat(self.group0()[3]) * Vec4::from([other.group0()[2], other.group0()[1], other.group0()[0], other.group0()[2]]) * Vec4::from([1.0, -1.0, 1.0, 0.0]) + Vec4::splat(self.group0()[0]) * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl RegressiveProduct<Plane> for Motor {
    type Output = f32;

    fn regressive_product(self, other: Plane) -> f32 {
        self.group0()[1] * other.group0()[0] + self.group0()[2] * other.group0()[1] + self.group0()[3] * other.group0()[2]
    }
}

impl OuterProduct<Plane> for Motor {
    type Output = MotorDual;

    fn outer_product(self, other: Plane) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[2]) * Vec4::splat(other.group0()[1]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) + Vec4::splat(self.group0()[3]) * Vec4::splat(other.group0()[2]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) + self.group0().yxxx() * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl InnerProduct<Plane> for Motor {
    type Output = Plane;

    fn inner_product(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[0]) * other.group0() + Vec3::splat(self.group0()[2]) * other.group0().zzx() * Vec3::from([1.0, 0.0, -1.0]) + Vec3::splat(self.group0()[3]) * other.group0().yxy() * Vec3::from([-1.0, 1.0, 0.0]) + Vec3::from([self.group0()[0], self.group0()[1], self.group0()[1]]) * other.group0().xzy() * Vec3::from([0.0, -1.0, 1.0]) } }
    }
}

impl LeftContraction<Plane> for Motor {
    type Output = Plane;

    fn left_contraction(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[0]) * other.group0() } }
    }
}

impl RightContraction<Plane> for Motor {
    type Output = Plane;

    fn right_contraction(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[2]) * other.group0().zzx() * Vec3::from([1.0, 0.0, -1.0]) + Vec3::splat(self.group0()[3]) * other.group0().yxy() * Vec3::from([-1.0, 1.0, 0.0]) + Vec3::from([self.group0()[0], self.group0()[1], self.group0()[1]]) * other.group0().xzy() * Vec3::from([0.0, -1.0, 1.0]) } }
    }
}

impl Into<Translator> for Motor {
    fn into(self) -> Translator {
        Translator { groups: TranslatorGroups { g0: Vec3::from([self.group0()[0], self.group0()[2], self.group0()[3]]) } }
    }
}

impl Add<Translator> for Motor {
    type Output = Motor;

    fn add(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() + Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([1.0, 0.0, 1.0, 1.0]) } }
    }
}

impl AddAssign<Translator> for Motor {
    fn add_assign(&mut self, other: Translator) {
        *self = (*self).add(other);
    }
}

impl Sub<Translator> for Motor {
    type Output = Motor;

    fn sub(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() - Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([1.0, 0.0, 1.0, 1.0]) } }
    }
}

impl SubAssign<Translator> for Motor {
    fn sub_assign(&mut self, other: Translator) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Translator> for Motor {
    type Output = Motor;

    fn geometric_product(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[1]) * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[2], other.group0()[1]]) * Vec4::from([0.0, 1.0, -1.0, 1.0]) + Vec4::splat(self.group0()[2]) * Vec4::from([other.group0()[1], other.group0()[2], other.group0()[0], other.group0()[1]]) * Vec4::from([-1.0, 1.0, 1.0, 0.0]) + Vec4::splat(self.group0()[3]) * Vec4::from([other.group0()[2], other.group0()[1], other.group0()[2], other.group0()[0]]) * Vec4::from([-1.0, -1.0, 0.0, 1.0]) + Vec4::splat(self.group0()[0]) * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([1.0, 0.0, 1.0, 1.0]) } }
    }
}

impl RegressiveProduct<Translator> for Motor {
    type Output = Plane;

    fn regressive_product(self, other: Translator) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[3]) * Vec3::splat(other.group0()[1]) * Vec3::from([1.0, 0.0, 0.0]) + Vec3::from([self.group0()[2], self.group0()[1], self.group0()[1]]) * other.group0().zzy() * Vec3::from([-1.0, 1.0, -1.0]) } }
    }
}

impl OuterProduct<Translator> for Motor {
    type Output = Motor;

    fn outer_product(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[2]) * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 0.0, 1.0, 0.0]) + Vec4::splat(self.group0()[3]) * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 0.0, 0.0, 1.0]) + self.group0().xyxx() * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl InnerProduct<Translator> for Motor {
    type Output = Motor;

    fn inner_product(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[2]) * Vec4::from([other.group0()[1], other.group0()[1], other.group0()[0], other.group0()[1]]) * Vec4::from([-1.0, 0.0, 1.0, 0.0]) + Vec4::splat(self.group0()[3]) * Vec4::from([other.group0()[2], other.group0()[2], other.group0()[2], other.group0()[0]]) * Vec4::from([-1.0, 0.0, 0.0, 1.0]) + self.group0().xyxx() * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl LeftContraction<Translator> for Motor {
    type Output = Translator;

    fn left_contraction(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Vec3::splat(self.group0()[0]) * other.group0() + Vec3::splat(self.group0()[3]) * Vec3::splat(other.group0()[2]) * Vec3::from([-1.0, 0.0, 0.0]) + Vec3::from([self.group0()[2], self.group0()[0], self.group0()[0]]) * other.group0().yxx() * Vec3::from([-1.0, 0.0, 0.0]) } }
    }
}

impl RightContraction<Translator> for Motor {
    type Output = Motor;

    fn right_contraction(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[2]) * Vec4::from([other.group0()[1], other.group0()[1], other.group0()[0], other.group0()[1]]) * Vec4::from([-1.0, 0.0, 1.0, 0.0]) + Vec4::splat(self.group0()[3]) * Vec4::from([other.group0()[2], other.group0()[2], other.group0()[2], other.group0()[0]]) * Vec4::from([-1.0, 0.0, 0.0, 1.0]) + self.group0().xyxx() * Vec4::splat(other.group0()[0]) * Vec4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl ScalarProduct<Translator> for Motor {
    type Output = f32;

    fn scalar_product(self, other: Translator) -> f32 {
        self.group0()[0] * other.group0()[0] - self.group0()[2] * other.group0()[1] - self.group0()[3] * other.group0()[2]
    }
}

impl Add<Motor> for Motor {
    type Output = Motor;

    fn add(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() + other.group0() } }
    }
}

impl AddAssign<Motor> for Motor {
    fn add_assign(&mut self, other: Motor) {
        *self = (*self).add(other);
    }
}

impl Sub<Motor> for Motor {
    type Output = Motor;

    fn sub(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() - other.group0() } }
    }
}

impl SubAssign<Motor> for Motor {
    fn sub_assign(&mut self, other: Motor) {
        *self = (*self).sub(other);
    }
}

impl Mul<Motor> for Motor {
    type Output = Motor;

    fn mul(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * other.group0() } }
    }
}

impl MulAssign<Motor> for Motor {
    fn mul_assign(&mut self, other: Motor) {
        *self = (*self).mul(other);
    }
}

impl Div<Motor> for Motor {
    type Output = Motor;

    fn div(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[3]]) * Vec4::from([1.0, 1.0, 1.0, 1.0]) / Vec4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[3]]) * Vec4::from([1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl DivAssign<Motor> for Motor {
    fn div_assign(&mut self, other: Motor) {
        *self = (*self).div(other);
    }
}

impl GeometricProduct<Motor> for Motor {
    type Output = Motor;

    fn geometric_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + Vec4::splat(self.group0()[1]) * other.group0().yxwz() * Vec4::from([-1.0, 1.0, -1.0, 1.0]) + Vec4::splat(self.group0()[2]) * other.group0().zwxy() * Vec4::from([-1.0, 1.0, 1.0, -1.0]) + Vec4::splat(self.group0()[3]) * other.group0().wzyx() * Vec4::from([-1.0, -1.0, 1.0, 1.0]) } }
    }
}

impl RegressiveProduct<Motor> for Motor {
    type Output = Plane;

    fn regressive_product(self, other: Motor) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[2]) * Vec3::from([other.group0()[3], other.group0()[3], other.group0()[1]]) * Vec3::from([-1.0, 0.0, 1.0]) + Vec3::splat(self.group0()[3]) * Vec3::from([other.group0()[2], other.group0()[1], other.group0()[2]]) * Vec3::from([1.0, -1.0, 0.0]) + Vec3::from([self.group0()[0], self.group0()[1], self.group0()[1]]) * Vec3::from([other.group0()[0], other.group0()[3], other.group0()[2]]) * Vec3::from([0.0, 1.0, -1.0]) } }
    }
}

impl OuterProduct<Motor> for Motor {
    type Output = Motor;

    fn outer_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + self.group0() * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl InnerProduct<Motor> for Motor {
    type Output = Motor;

    fn inner_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + Vec4::splat(self.group0()[2]) * other.group0().zzxz() * Vec4::from([-1.0, 0.0, 1.0, 0.0]) + Vec4::splat(self.group0()[3]) * other.group0().wwwx() * Vec4::from([-1.0, 0.0, 0.0, 1.0]) + self.group0().yyxx() * other.group0().yxxx() * Vec4::from([-1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl LeftContraction<Motor> for Motor {
    type Output = Motor;

    fn left_contraction(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + Vec4::splat(self.group0()[2]) * Vec4::splat(other.group0()[2]) * Vec4::from([-1.0, 0.0, 0.0, 0.0]) + Vec4::splat(self.group0()[3]) * Vec4::splat(other.group0()[3]) * Vec4::from([-1.0, 0.0, 0.0, 0.0]) + self.group0().yxxx() * other.group0().yxxx() * Vec4::from([-1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl RightContraction<Motor> for Motor {
    type Output = Motor;

    fn right_contraction(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[1]) * other.group0().yxyy() * Vec4::from([-1.0, 1.0, 0.0, 0.0]) + Vec4::splat(self.group0()[2]) * other.group0().zzxz() * Vec4::from([-1.0, 0.0, 1.0, 0.0]) + Vec4::splat(self.group0()[3]) * other.group0().wwwx() * Vec4::from([-1.0, 0.0, 0.0, 1.0]) + Vec4::splat(self.group0()[0]) * Vec4::splat(other.group0()[0]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl ScalarProduct<Motor> for Motor {
    type Output = f32;

    fn scalar_product(self, other: Motor) -> f32 {
        self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3]
    }
}

impl Add<MotorDual> for Motor {
    type Output = MultiVector;

    fn add(self, other: MotorDual) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0().xyxx() * Vec4::from([1.0, 1.0, 0.0, 0.0]) + other.group0().xxwz() * Vec4::from([0.0, 0.0, 1.0, 1.0]), g1: self.group0().xxzw() * Vec4::from([0.0, 0.0, 1.0, 1.0]) + other.group0().yxxx() * Vec4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl Sub<MotorDual> for Motor {
    type Output = MultiVector;

    fn sub(self, other: MotorDual) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0().xyxx() * Vec4::from([1.0, 1.0, 0.0, 0.0]) - other.group0().xxwz() * Vec4::from([0.0, 0.0, 1.0, 1.0]), g1: self.group0().xxzw() * Vec4::from([0.0, 0.0, 1.0, 1.0]) - other.group0().yxxx() * Vec4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl GeometricProduct<MotorDual> for Motor {
    type Output = MotorDual;

    fn geometric_product(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + Vec4::splat(self.group0()[1]) * other.group0().yxwz() * Vec4::from([1.0, -1.0, -1.0, 1.0]) + Vec4::splat(self.group0()[2]) * other.group0().zwxy() * Vec4::from([1.0, 1.0, -1.0, -1.0]) + Vec4::splat(self.group0()[3]) * other.group0().wzyx() * Vec4::from([1.0, -1.0, 1.0, -1.0]) } }
    }
}

impl RegressiveProduct<MotorDual> for Motor {
    type Output = Motor;

    fn regressive_product(self, other: MotorDual) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[1]) * other.group0().yxyy() * Vec4::from([1.0, 1.0, 0.0, 0.0]) + Vec4::splat(self.group0()[2]) * other.group0().zzxz() * Vec4::from([1.0, 0.0, 1.0, 0.0]) + Vec4::splat(self.group0()[3]) * other.group0().wwwx() * Vec4::from([1.0, 0.0, 0.0, 1.0]) + Vec4::splat(self.group0()[0]) * Vec4::splat(other.group0()[0]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl OuterProduct<MotorDual> for Motor {
    type Output = MotorDual;

    fn outer_product(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + Vec4::splat(self.group0()[2]) * Vec4::splat(other.group0()[2]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) + Vec4::splat(self.group0()[3]) * Vec4::splat(other.group0()[3]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) + self.group0().yxxx() * other.group0().yxxx() * Vec4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl InnerProduct<MotorDual> for Motor {
    type Output = MotorDual;

    fn inner_product(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + Vec4::splat(self.group0()[2]) * other.group0().wwxy() * Vec4::from([0.0, 1.0, -1.0, -1.0]) + Vec4::splat(self.group0()[3]) * other.group0().zzyx() * Vec4::from([0.0, -1.0, 1.0, -1.0]) + self.group0().xyyy() * other.group0().xxwz() * Vec4::from([0.0, -1.0, -1.0, 1.0]) } }
    }
}

impl LeftContraction<MotorDual> for Motor {
    type Output = MotorDual;

    fn left_contraction(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + self.group0() * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, -1.0, -1.0, -1.0]) } }
    }
}

impl RightContraction<MotorDual> for Motor {
    type Output = Plane;

    fn right_contraction(self, other: MotorDual) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[2]) * Vec3::from([other.group0()[3], other.group0()[3], other.group0()[1]]) * Vec3::from([1.0, 0.0, -1.0]) + Vec3::splat(self.group0()[3]) * Vec3::from([other.group0()[2], other.group0()[1], other.group0()[2]]) * Vec3::from([-1.0, 1.0, 0.0]) + Vec3::from([self.group0()[0], self.group0()[1], self.group0()[1]]) * Vec3::from([other.group0()[0], other.group0()[3], other.group0()[2]]) * Vec3::from([0.0, -1.0, 1.0]) } }
    }
}

impl SquaredMagnitude for Motor {
    type Output = f32;

    fn squared_magnitude(self) -> f32 {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for Motor {
    type Output = f32;

    fn magnitude(self) -> f32 {
        self.squared_magnitude().sqrt()
    }
}

impl Mul<f32> for Motor {
    type Output = Motor;

    fn mul(self, other: f32) -> Motor {
        self.geometric_product(other)
    }
}

impl MulAssign<f32> for Motor {
    fn mul_assign(&mut self, other: f32) {
        *self = (*self).mul(other);
    }
}

impl Signum for Motor {
    type Output = Motor;

    fn signum(self) -> Motor {
        self.geometric_product(1.0 / self.magnitude())
    }
}

impl Inverse for Motor {
    type Output = Motor;

    fn inverse(self) -> Motor {
        self.reversal().geometric_product(1.0 / self.squared_magnitude())
    }
}

impl Zero for MotorDual {
    fn zero() -> Self {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(0.0) } }
    }
}

impl One for MotorDual {
    fn one() -> Self {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(0.0) } }
    }
}

impl Neg for MotorDual {
    type Output = MotorDual;

    fn neg(self) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: self.group0() * Vec4::splat(-1.0) } }
    }
}

impl Automorphism for MotorDual {
    type Output = MotorDual;

    fn automorphism(self) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: self.group0() * Vec4::splat(-1.0) } }
    }
}

impl Reversal for MotorDual {
    type Output = MotorDual;

    fn reversal(self) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: self.group0() * Vec4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl Conjugation for MotorDual {
    type Output = MotorDual;

    fn conjugation(self) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: self.group0() * Vec4::from([1.0, -1.0, -1.0, -1.0]) } }
    }
}

impl Dual for MotorDual {
    type Output = Motor;

    fn dual(self) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() } }
    }
}

impl GeometricProduct<f32> for MotorDual {
    type Output = MotorDual;

    fn geometric_product(self, other: f32) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: self.group0() * Vec4::splat(other) } }
    }
}

impl RegressiveProduct<f32> for MotorDual {
    type Output = f32;

    fn regressive_product(self, other: f32) -> f32 {
        self.group0()[0] * other
    }
}

impl OuterProduct<f32> for MotorDual {
    type Output = MotorDual;

    fn outer_product(self, other: f32) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: self.group0() * Vec4::splat(other) } }
    }
}

impl InnerProduct<f32> for MotorDual {
    type Output = MotorDual;

    fn inner_product(self, other: f32) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: self.group0() * Vec4::splat(other) } }
    }
}

impl RightContraction<f32> for MotorDual {
    type Output = MotorDual;

    fn right_contraction(self, other: f32) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: self.group0() * Vec4::splat(other) } }
    }
}

impl Add<MultiVector> for MotorDual {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0().xxwz() * Vec4::from([0.0, 0.0, 1.0, 1.0]) + other.group0(), g1: self.group0().yxxx() * Vec4::from([1.0, 1.0, 0.0, 0.0]) + other.group1() } }
    }
}

impl Sub<MultiVector> for MotorDual {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0().xxwz() * Vec4::from([0.0, 0.0, 1.0, 1.0]) - other.group0(), g1: self.group0().yxxx() * Vec4::from([1.0, 1.0, 0.0, 0.0]) - other.group1() } }
    }
}

impl GeometricProduct<MultiVector> for MotorDual {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[0]) * other.group1().yxwz() * Vec4::from([-1.0, 1.0, -1.0, -1.0]) + Vec4::splat(self.group0()[1]) * other.group1() * Vec4::from([1.0, 1.0, 1.0, -1.0]) + Vec4::splat(self.group0()[2]) * other.group0().wzyx() * Vec4::from([1.0, -1.0, -1.0, 1.0]) + Vec4::splat(self.group0()[3]) * other.group0().zwxy(), g1: Vec4::splat(self.group0()[0]) * other.group0().yxwz() * Vec4::from([-1.0, 1.0, 1.0, 1.0]) + Vec4::splat(self.group0()[1]) * other.group0() * Vec4::from([1.0, 1.0, 1.0, -1.0]) + Vec4::splat(self.group0()[2]) * other.group1().wzyx() + Vec4::splat(self.group0()[3]) * other.group1().zwxy() * Vec4::from([-1.0, 1.0, -1.0, 1.0]) } }
    }
}

impl RegressiveProduct<MultiVector> for MotorDual {
    type Output = MultiVector;

    fn regressive_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + Vec4::splat(self.group0()[2]) * other.group1().zzzy() * Vec4::from([1.0, 0.0, 0.0, 1.0]) + Vec4::splat(self.group0()[3]) * other.group1().wwyw() * Vec4::from([1.0, 0.0, 1.0, 0.0]) + self.group0().yxxx() * other.group0().yxxx() * Vec4::from([1.0, 0.0, 0.0, 0.0]), g1: Vec4::splat(self.group0()[0]) * other.group1() + self.group0().yxxx() * other.group1().yxxx() * Vec4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl InnerProduct<MultiVector> for MotorDual {
    type Output = MultiVector;

    fn inner_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[0]) * other.group1().yxwz() * Vec4::from([-1.0, 1.0, -1.0, -1.0]) + Vec4::splat(self.group0()[1]) * other.group1() * Vec4::from([1.0, 1.0, 1.0, -1.0]) + Vec4::splat(self.group0()[3]) * other.group0().zzxy() * Vec4::from([1.0, 0.0, 1.0, 1.0]) + self.group0().zxzz() * other.group0().wxyx() * Vec4::from([1.0, 0.0, -1.0, 1.0]), g1: Vec4::splat(self.group0()[0]) * other.group0().yxwz() * Vec4::from([-1.0, 1.0, 1.0, 1.0]) + Vec4::splat(self.group0()[2]) * other.group1().wwyw() * Vec4::from([1.0, 0.0, 1.0, 0.0]) + Vec4::splat(self.group0()[3]) * other.group1().zzzy() * Vec4::from([-1.0, 0.0, 0.0, 1.0]) + self.group0().yxxx() * Vec4::splat(other.group0()[0]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl RightContraction<MultiVector> for MotorDual {
    type Output = MultiVector;

    fn right_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Vec4::splat(self.group0()[0]) * other.group1().yxwz() * Vec4::from([-1.0, 1.0, -1.0, -1.0]) + Vec4::splat(self.group0()[2]) * other.group0().wwwx() * Vec4::from([1.0, 0.0, 0.0, 1.0]) + Vec4::splat(self.group0()[3]) * other.group0().zzxz() * Vec4::from([1.0, 0.0, 1.0, 0.0]) + self.group0().yxxx() * Vec4::splat(other.group1()[0]) * Vec4::from([1.0, 0.0, 0.0, 0.0]), g1: Vec4::splat(self.group0()[0]) * other.group0().yxwz() * Vec4::from([-1.0, 1.0, 1.0, 1.0]) + self.group0().yxxx() * Vec4::splat(other.group0()[0]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl ScalarProduct<MultiVector> for MotorDual {
    type Output = f32;

    fn scalar_product(self, other: MultiVector) -> f32 {
        0.0 - self.group0()[0] * other.group1()[1] + self.group0()[1] * other.group1()[0] + self.group0()[2] * other.group0()[3] + self.group0()[3] * other.group0()[2]
    }
}

impl GeometricProduct<Rotor> for MotorDual {
    type Output = MotorDual;

    fn geometric_product(self, other: Rotor) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[1]) * Vec4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Vec4::from([1.0, 1.0, 0.0, 0.0]) + Vec4::splat(self.group0()[3]) * Vec4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]) * Vec4::from([0.0, 0.0, 1.0, 1.0]) + self.group0().xxzz() * Vec4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[1]]) * Vec4::from([1.0, -1.0, 1.0, -1.0]) } }
    }
}

impl RegressiveProduct<Rotor> for MotorDual {
    type Output = Rotor;

    fn regressive_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Vec2::splat(self.group0()[0]) * other.group0() + Vec2::from([self.group0()[1], self.group0()[0]]) * other.group0().yx() * Vec2::from([1.0, 0.0]) } }
    }
}

impl OuterProduct<Rotor> for MotorDual {
    type Output = MotorDual;

    fn outer_product(self, other: Rotor) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[1]) * Vec4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Vec4::from([1.0, 1.0, 0.0, 0.0]) + self.group0().xxzw() * Vec4::splat(other.group0()[0]) * Vec4::from([1.0, 0.0, 1.0, 1.0]) } }
    }
}

impl InnerProduct<Rotor> for MotorDual {
    type Output = MotorDual;

    fn inner_product(self, other: Rotor) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[1]) * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 1.0, 0.0, 0.0]) + Vec4::splat(self.group0()[3]) * Vec4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]) * Vec4::from([0.0, 0.0, 1.0, 1.0]) + self.group0().xxzz() * Vec4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[1]]) * Vec4::from([1.0, -1.0, 1.0, -1.0]) } }
    }
}

impl RightContraction<Rotor> for MotorDual {
    type Output = MotorDual;

    fn right_contraction(self, other: Rotor) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[1]) * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 1.0, 0.0, 0.0]) + self.group0().xxzw() * Vec4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[0]]) * Vec4::from([1.0, -1.0, 1.0, 1.0]) } }
    }
}

impl GeometricProduct<Point> for MotorDual {
    type Output = MotorDual;

    fn geometric_product(self, other: Point) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[1]) * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[2], other.group0()[1]]) * Vec4::from([1.0, 0.0, -1.0, 1.0]) + Vec4::splat(self.group0()[2]) * Vec4::from([other.group0()[1], other.group0()[2], other.group0()[1], other.group0()[0]]) * Vec4::from([1.0, 1.0, 0.0, -1.0]) + Vec4::splat(self.group0()[3]) * Vec4::from([other.group0()[2], other.group0()[1], other.group0()[0], other.group0()[2]]) * Vec4::from([1.0, -1.0, 1.0, 0.0]) + Vec4::splat(self.group0()[0]) * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([0.0, -1.0, -1.0, -1.0]) } }
    }
}

impl RegressiveProduct<Point> for MotorDual {
    type Output = Motor;

    fn regressive_product(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[2]) * Vec4::splat(other.group0()[1]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) + Vec4::splat(self.group0()[3]) * Vec4::splat(other.group0()[2]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) + self.group0().yxxx() * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl InnerProduct<Point> for MotorDual {
    type Output = Plane;

    fn inner_product(self, other: Point) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(0.0) - Vec3::splat(self.group0()[0]) * other.group0() + Vec3::splat(self.group0()[2]) * other.group0().zzx() * Vec3::from([1.0, 0.0, -1.0]) + Vec3::splat(self.group0()[3]) * other.group0().yxy() * Vec3::from([-1.0, 1.0, 0.0]) + Vec3::from([self.group0()[0], self.group0()[1], self.group0()[1]]) * other.group0().xzy() * Vec3::from([0.0, -1.0, 1.0]) } }
    }
}

impl LeftContraction<Point> for MotorDual {
    type Output = Plane;

    fn left_contraction(self, other: Point) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[2]) * other.group0().zzx() * Vec3::from([1.0, 0.0, -1.0]) + Vec3::splat(self.group0()[3]) * other.group0().yxy() * Vec3::from([-1.0, 1.0, 0.0]) + Vec3::from([self.group0()[0], self.group0()[1], self.group0()[1]]) * other.group0().xzy() * Vec3::from([0.0, -1.0, 1.0]) } }
    }
}

impl RightContraction<Point> for MotorDual {
    type Output = Plane;

    fn right_contraction(self, other: Point) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(0.0) - Vec3::splat(self.group0()[0]) * other.group0() } }
    }
}

impl GeometricProduct<IdealPoint> for MotorDual {
    type Output = MotorDual;

    fn geometric_product(self, other: IdealPoint) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[1]) * Vec4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]) * Vec4::from([0.0, 0.0, -1.0, 1.0]) + Vec4::splat(self.group0()[3]) * Vec4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Vec4::from([1.0, -1.0, 0.0, 0.0]) + self.group0().zzxx() * Vec4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[1]]) * Vec4::from([1.0, 1.0, -1.0, -1.0]) } }
    }
}

impl RegressiveProduct<IdealPoint> for MotorDual {
    type Output = Translator;

    fn regressive_product(self, other: IdealPoint) -> Translator {
        Translator { groups: TranslatorGroups { g0: Vec3::splat(self.group0()[3]) * Vec3::splat(other.group0()[1]) * Vec3::from([1.0, 0.0, 0.0]) + Vec3::from([self.group0()[2], self.group0()[0], self.group0()[0]]) * Vec3::from([other.group0()[0], other.group0()[0], other.group0()[1]]) } }
    }
}

impl InnerProduct<IdealPoint> for MotorDual {
    type Output = Plane;

    fn inner_product(self, other: IdealPoint) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[1]) * Vec3::from([other.group0()[1], other.group0()[1], other.group0()[0]]) * Vec3::from([0.0, -1.0, 1.0]) + Vec3::splat(self.group0()[3]) * Vec3::splat(other.group0()[0]) * Vec3::from([-1.0, 0.0, 0.0]) + Vec3::from([self.group0()[2], self.group0()[0], self.group0()[0]]) * Vec3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Vec3::from([1.0, -1.0, -1.0]) } }
    }
}

impl LeftContraction<IdealPoint> for MotorDual {
    type Output = Plane;

    fn left_contraction(self, other: IdealPoint) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[3]) * Vec3::splat(other.group0()[0]) * Vec3::from([-1.0, 0.0, 0.0]) + Vec3::from([self.group0()[2], self.group0()[1], self.group0()[1]]) * Vec3::from([other.group0()[1], other.group0()[1], other.group0()[0]]) * Vec3::from([1.0, -1.0, 1.0]) } }
    }
}

impl Into<Plane> for MotorDual {
    fn into(self) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::from([self.group0()[1], self.group0()[2], self.group0()[3]]) } }
    }
}

impl Add<Plane> for MotorDual {
    type Output = MotorDual;

    fn add(self, other: Plane) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: self.group0() + Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl AddAssign<Plane> for MotorDual {
    fn add_assign(&mut self, other: Plane) {
        *self = (*self).add(other);
    }
}

impl Sub<Plane> for MotorDual {
    type Output = MotorDual;

    fn sub(self, other: Plane) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: self.group0() - Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl SubAssign<Plane> for MotorDual {
    fn sub_assign(&mut self, other: Plane) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Plane> for MotorDual {
    type Output = Motor;

    fn geometric_product(self, other: Plane) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[1]) * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[2], other.group0()[1]]) * Vec4::from([1.0, 0.0, 1.0, -1.0]) + Vec4::splat(self.group0()[2]) * Vec4::from([other.group0()[1], other.group0()[2], other.group0()[1], other.group0()[0]]) * Vec4::from([1.0, -1.0, 0.0, 1.0]) + Vec4::splat(self.group0()[3]) * Vec4::from([other.group0()[2], other.group0()[1], other.group0()[0], other.group0()[2]]) * Vec4::from([1.0, 1.0, -1.0, 0.0]) + Vec4::splat(self.group0()[0]) * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl RegressiveProduct<Plane> for MotorDual {
    type Output = Plane;

    fn regressive_product(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[0]) * other.group0() } }
    }
}

impl OuterProduct<Plane> for MotorDual {
    type Output = Point;

    fn outer_product(self, other: Plane) -> Point {
        Point { groups: PointGroups { g0: Vec3::splat(self.group0()[2]) * other.group0().zzx() * Vec3::from([-1.0, 0.0, 1.0]) + Vec3::splat(self.group0()[3]) * other.group0().yxy() * Vec3::from([1.0, -1.0, 0.0]) + Vec3::from([self.group0()[0], self.group0()[1], self.group0()[1]]) * other.group0().xzy() * Vec3::from([0.0, 1.0, -1.0]) } }
    }
}

impl InnerProduct<Plane> for MotorDual {
    type Output = Motor;

    fn inner_product(self, other: Plane) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[2]) * Vec4::splat(other.group0()[1]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) + Vec4::splat(self.group0()[3]) * Vec4::splat(other.group0()[2]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) + self.group0().yxxx() * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl LeftContraction<Plane> for MotorDual {
    type Output = f32;

    fn left_contraction(self, other: Plane) -> f32 {
        self.group0()[1] * other.group0()[0] + self.group0()[2] * other.group0()[1] + self.group0()[3] * other.group0()[2]
    }
}

impl RightContraction<Plane> for MotorDual {
    type Output = Motor;

    fn right_contraction(self, other: Plane) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[2]) * Vec4::splat(other.group0()[1]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) + Vec4::splat(self.group0()[3]) * Vec4::splat(other.group0()[2]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) + self.group0().yxxx() * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl ScalarProduct<Plane> for MotorDual {
    type Output = f32;

    fn scalar_product(self, other: Plane) -> f32 {
        self.group0()[1] * other.group0()[0] + self.group0()[2] * other.group0()[1] + self.group0()[3] * other.group0()[2]
    }
}

impl GeometricProduct<Translator> for MotorDual {
    type Output = MotorDual;

    fn geometric_product(self, other: Translator) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[1]) * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[2], other.group0()[1]]) * Vec4::from([0.0, 1.0, -1.0, 1.0]) + Vec4::splat(self.group0()[2]) * Vec4::from([other.group0()[1], other.group0()[2], other.group0()[0], other.group0()[1]]) * Vec4::from([1.0, 1.0, 1.0, 0.0]) + Vec4::splat(self.group0()[3]) * Vec4::from([other.group0()[2], other.group0()[1], other.group0()[2], other.group0()[0]]) * Vec4::from([1.0, -1.0, 0.0, 1.0]) + Vec4::splat(self.group0()[0]) * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([1.0, 0.0, -1.0, -1.0]) } }
    }
}

impl RegressiveProduct<Translator> for MotorDual {
    type Output = Translator;

    fn regressive_product(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Vec3::splat(self.group0()[0]) * other.group0() + Vec3::splat(self.group0()[3]) * Vec3::splat(other.group0()[2]) * Vec3::from([1.0, 0.0, 0.0]) + Vec3::from([self.group0()[2], self.group0()[0], self.group0()[0]]) * other.group0().yxx() * Vec3::from([1.0, 0.0, 0.0]) } }
    }
}

impl OuterProduct<Translator> for MotorDual {
    type Output = MotorDual;

    fn outer_product(self, other: Translator) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[2]) * Vec4::from([other.group0()[1], other.group0()[1], other.group0()[0], other.group0()[1]]) * Vec4::from([1.0, 0.0, 1.0, 0.0]) + Vec4::splat(self.group0()[3]) * Vec4::from([other.group0()[2], other.group0()[2], other.group0()[2], other.group0()[0]]) * Vec4::from([1.0, 0.0, 0.0, 1.0]) + self.group0().xyxx() * Vec4::splat(other.group0()[0]) * Vec4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl InnerProduct<Translator> for MotorDual {
    type Output = MotorDual;

    fn inner_product(self, other: Translator) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[1]) * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[2], other.group0()[1]]) * Vec4::from([0.0, 1.0, -1.0, 1.0]) + Vec4::splat(self.group0()[2]) * Vec4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group0()[2]]) * Vec4::from([0.0, 1.0, 1.0, 0.0]) + Vec4::splat(self.group0()[3]) * Vec4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]) * Vec4::from([0.0, -1.0, 0.0, 1.0]) + Vec4::splat(self.group0()[0]) * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([1.0, 0.0, -1.0, -1.0]) } }
    }
}

impl LeftContraction<Translator> for MotorDual {
    type Output = Plane;

    fn left_contraction(self, other: Translator) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[3]) * Vec3::splat(other.group0()[1]) * Vec3::from([-1.0, 0.0, 0.0]) + Vec3::from([self.group0()[2], self.group0()[1], self.group0()[1]]) * other.group0().zzy() * Vec3::from([1.0, -1.0, 1.0]) } }
    }
}

impl RightContraction<Translator> for MotorDual {
    type Output = MotorDual;

    fn right_contraction(self, other: Translator) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[2]) * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 0.0, 1.0, 0.0]) + Vec4::splat(self.group0()[3]) * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 0.0, 0.0, 1.0]) + self.group0().xyxx() * Vec4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Vec4::from([1.0, 1.0, -1.0, -1.0]) } }
    }
}

impl Add<Motor> for MotorDual {
    type Output = MultiVector;

    fn add(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0().xxwz() * Vec4::from([0.0, 0.0, 1.0, 1.0]) + other.group0().xyxx() * Vec4::from([1.0, 1.0, 0.0, 0.0]), g1: self.group0().yxxx() * Vec4::from([1.0, 1.0, 0.0, 0.0]) + other.group0().xxzw() * Vec4::from([0.0, 0.0, 1.0, 1.0]) } }
    }
}

impl Sub<Motor> for MotorDual {
    type Output = MultiVector;

    fn sub(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0().xxwz() * Vec4::from([0.0, 0.0, 1.0, 1.0]) - other.group0().xyxx() * Vec4::from([1.0, 1.0, 0.0, 0.0]), g1: self.group0().yxxx() * Vec4::from([1.0, 1.0, 0.0, 0.0]) - other.group0().xxzw() * Vec4::from([0.0, 0.0, 1.0, 1.0]) } }
    }
}

impl GeometricProduct<Motor> for MotorDual {
    type Output = MotorDual;

    fn geometric_product(self, other: Motor) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() * Vec4::from([1.0, -1.0, -1.0, -1.0]) + Vec4::splat(self.group0()[1]) * other.group0().yxwz() * Vec4::from([1.0, 1.0, -1.0, 1.0]) + Vec4::splat(self.group0()[2]) * other.group0().zwxy() * Vec4::from([1.0, 1.0, 1.0, -1.0]) + Vec4::splat(self.group0()[3]) * other.group0().wzyx() * Vec4::from([1.0, -1.0, 1.0, 1.0]) } }
    }
}

impl RegressiveProduct<Motor> for MotorDual {
    type Output = Motor;

    fn regressive_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + Vec4::splat(self.group0()[2]) * Vec4::splat(other.group0()[2]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) + Vec4::splat(self.group0()[3]) * Vec4::splat(other.group0()[3]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) + self.group0().yxxx() * other.group0().yxxx() * Vec4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl OuterProduct<Motor> for MotorDual {
    type Output = MotorDual;

    fn outer_product(self, other: Motor) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[1]) * other.group0().yxyy() * Vec4::from([1.0, 1.0, 0.0, 0.0]) + Vec4::splat(self.group0()[2]) * other.group0().zzxz() * Vec4::from([1.0, 0.0, 1.0, 0.0]) + Vec4::splat(self.group0()[3]) * other.group0().wwwx() * Vec4::from([1.0, 0.0, 0.0, 1.0]) + Vec4::splat(self.group0()[0]) * Vec4::splat(other.group0()[0]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl InnerProduct<Motor> for MotorDual {
    type Output = MotorDual;

    fn inner_product(self, other: Motor) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() * Vec4::from([1.0, -1.0, -1.0, -1.0]) + Vec4::splat(self.group0()[2]) * other.group0().wwxy() * Vec4::from([0.0, 1.0, 1.0, -1.0]) + Vec4::splat(self.group0()[3]) * other.group0().zzyx() * Vec4::from([0.0, -1.0, 1.0, 1.0]) + self.group0().xyyy() * other.group0().xxwz() * Vec4::from([0.0, 1.0, -1.0, 1.0]) } }
    }
}

impl LeftContraction<Motor> for MotorDual {
    type Output = Plane;

    fn left_contraction(self, other: Motor) -> Plane {
        Plane { groups: PlaneGroups { g0: Vec3::splat(self.group0()[2]) * Vec3::from([other.group0()[3], other.group0()[3], other.group0()[1]]) * Vec3::from([1.0, 0.0, -1.0]) + Vec3::splat(self.group0()[3]) * Vec3::from([other.group0()[2], other.group0()[1], other.group0()[2]]) * Vec3::from([-1.0, 1.0, 0.0]) + Vec3::from([self.group0()[0], self.group0()[1], self.group0()[1]]) * Vec3::from([other.group0()[0], other.group0()[3], other.group0()[2]]) * Vec3::from([0.0, -1.0, 1.0]) } }
    }
}

impl RightContraction<Motor> for MotorDual {
    type Output = MotorDual;

    fn right_contraction(self, other: Motor) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() * Vec4::from([1.0, -1.0, -1.0, -1.0]) + self.group0() * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl Add<MotorDual> for MotorDual {
    type Output = MotorDual;

    fn add(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: self.group0() + other.group0() } }
    }
}

impl AddAssign<MotorDual> for MotorDual {
    fn add_assign(&mut self, other: MotorDual) {
        *self = (*self).add(other);
    }
}

impl Sub<MotorDual> for MotorDual {
    type Output = MotorDual;

    fn sub(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: self.group0() - other.group0() } }
    }
}

impl SubAssign<MotorDual> for MotorDual {
    fn sub_assign(&mut self, other: MotorDual) {
        *self = (*self).sub(other);
    }
}

impl Mul<MotorDual> for MotorDual {
    type Output = MotorDual;

    fn mul(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: self.group0() * other.group0() } }
    }
}

impl MulAssign<MotorDual> for MotorDual {
    fn mul_assign(&mut self, other: MotorDual) {
        *self = (*self).mul(other);
    }
}

impl Div<MotorDual> for MotorDual {
    type Output = MotorDual;

    fn div(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[3]]) * Vec4::from([1.0, 1.0, 1.0, 1.0]) / Vec4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[3]]) * Vec4::from([1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl DivAssign<MotorDual> for MotorDual {
    fn div_assign(&mut self, other: MotorDual) {
        *self = (*self).div(other);
    }
}

impl GeometricProduct<MotorDual> for MotorDual {
    type Output = Motor;

    fn geometric_product(self, other: MotorDual) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() * Vec4::from([-1.0, 1.0, 1.0, 1.0]) + Vec4::splat(self.group0()[1]) * other.group0().yxwz() * Vec4::from([1.0, 1.0, 1.0, -1.0]) + Vec4::splat(self.group0()[2]) * other.group0().zwxy() * Vec4::from([1.0, -1.0, 1.0, 1.0]) + Vec4::splat(self.group0()[3]) * other.group0().wzyx() * Vec4::from([1.0, 1.0, -1.0, 1.0]) } }
    }
}

impl RegressiveProduct<MotorDual> for MotorDual {
    type Output = MotorDual;

    fn regressive_product(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() + self.group0() * Vec4::splat(other.group0()[0]) * Vec4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl OuterProduct<MotorDual> for MotorDual {
    type Output = Point;

    fn outer_product(self, other: MotorDual) -> Point {
        Point { groups: PointGroups { g0: Vec3::splat(self.group0()[2]) * Vec3::from([other.group0()[3], other.group0()[3], other.group0()[1]]) * Vec3::from([-1.0, 0.0, 1.0]) + Vec3::splat(self.group0()[3]) * Vec3::from([other.group0()[2], other.group0()[1], other.group0()[2]]) * Vec3::from([1.0, -1.0, 0.0]) + Vec3::from([self.group0()[0], self.group0()[1], self.group0()[1]]) * Vec3::from([other.group0()[0], other.group0()[3], other.group0()[2]]) * Vec3::from([0.0, 1.0, -1.0]) } }
    }
}

impl InnerProduct<MotorDual> for MotorDual {
    type Output = Motor;

    fn inner_product(self, other: MotorDual) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() * Vec4::from([-1.0, 1.0, 1.0, 1.0]) + Vec4::splat(self.group0()[2]) * other.group0().zzxz() * Vec4::from([1.0, 0.0, 1.0, 0.0]) + Vec4::splat(self.group0()[3]) * other.group0().wwwx() * Vec4::from([1.0, 0.0, 0.0, 1.0]) + self.group0().yyxx() * other.group0().yxxx() * Vec4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl LeftContraction<MotorDual> for MotorDual {
    type Output = Motor;

    fn left_contraction(self, other: MotorDual) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[1]) * other.group0().yxyy() * Vec4::from([1.0, 1.0, 0.0, 0.0]) + Vec4::splat(self.group0()[2]) * other.group0().zzxz() * Vec4::from([1.0, 0.0, 1.0, 0.0]) + Vec4::splat(self.group0()[3]) * other.group0().wwwx() * Vec4::from([1.0, 0.0, 0.0, 1.0]) + Vec4::splat(self.group0()[0]) * Vec4::splat(other.group0()[0]) * Vec4::from([-1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl RightContraction<MotorDual> for MotorDual {
    type Output = Motor;

    fn right_contraction(self, other: MotorDual) -> Motor {
        Motor { groups: MotorGroups { g0: Vec4::splat(self.group0()[0]) * other.group0() * Vec4::from([-1.0, 1.0, 1.0, 1.0]) + Vec4::splat(self.group0()[2]) * Vec4::splat(other.group0()[2]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) + Vec4::splat(self.group0()[3]) * Vec4::splat(other.group0()[3]) * Vec4::from([1.0, 0.0, 0.0, 0.0]) + self.group0().yxxx() * other.group0().yxxx() * Vec4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl ScalarProduct<MotorDual> for MotorDual {
    type Output = f32;

    fn scalar_product(self, other: MotorDual) -> f32 {
        0.0 - self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] + self.group0()[3] * other.group0()[3]
    }
}

impl SquaredMagnitude for MotorDual {
    type Output = f32;

    fn squared_magnitude(self) -> f32 {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for MotorDual {
    type Output = f32;

    fn magnitude(self) -> f32 {
        self.squared_magnitude().sqrt()
    }
}

impl Mul<f32> for MotorDual {
    type Output = MotorDual;

    fn mul(self, other: f32) -> MotorDual {
        self.geometric_product(other)
    }
}

impl MulAssign<f32> for MotorDual {
    fn mul_assign(&mut self, other: f32) {
        *self = (*self).mul(other);
    }
}

impl Signum for MotorDual {
    type Output = MotorDual;

    fn signum(self) -> MotorDual {
        self.geometric_product(1.0 / self.magnitude())
    }
}

impl Inverse for MotorDual {
    type Output = MotorDual;

    fn inverse(self) -> MotorDual {
        self.reversal().geometric_product(1.0 / self.squared_magnitude())
    }
}

impl GeometricQuotient<IdealPoint> for IdealPoint {
    type Output = Rotor;

    fn geometric_quotient(self, other: IdealPoint) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<IdealPoint> for IdealPoint {
    type Output = IdealPoint;

    fn transformation(self, other: IdealPoint) -> IdealPoint {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Motor> for IdealPoint {
    type Output = Motor;

    fn geometric_quotient(self, other: Motor) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Motor> for IdealPoint {
    type Output = Motor;

    fn transformation(self, other: Motor) -> Motor {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<MotorDual> for IdealPoint {
    type Output = MotorDual;

    fn geometric_quotient(self, other: MotorDual) -> MotorDual {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MotorDual> for IdealPoint {
    type Output = MotorDual;

    fn transformation(self, other: MotorDual) -> MotorDual {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<MultiVector> for IdealPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MultiVector> for IdealPoint {
    type Output = MultiVector;

    fn transformation(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Plane> for IdealPoint {
    type Output = MotorDual;

    fn geometric_quotient(self, other: Plane) -> MotorDual {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Plane> for IdealPoint {
    type Output = Plane;

    fn transformation(self, other: Plane) -> Plane {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Point> for IdealPoint {
    type Output = Motor;

    fn geometric_quotient(self, other: Point) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Point> for IdealPoint {
    type Output = Point;

    fn transformation(self, other: Point) -> Point {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Rotor> for IdealPoint {
    type Output = IdealPoint;

    fn geometric_quotient(self, other: Rotor) -> IdealPoint {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Rotor> for IdealPoint {
    type Output = Rotor;

    fn transformation(self, other: Rotor) -> Rotor {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<f32> for IdealPoint {
    type Output = IdealPoint;

    fn geometric_quotient(self, other: f32) -> IdealPoint {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<f32> for IdealPoint {
    type Output = f32;

    fn transformation(self, other: f32) -> f32 {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Translator> for IdealPoint {
    type Output = Motor;

    fn geometric_quotient(self, other: Translator) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Translator> for IdealPoint {
    type Output = Translator;

    fn transformation(self, other: Translator) -> Translator {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<IdealPoint> for Motor {
    type Output = Motor;

    fn geometric_quotient(self, other: IdealPoint) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<IdealPoint> for Motor {
    type Output = IdealPoint;

    fn transformation(self, other: IdealPoint) -> IdealPoint {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl Powi for Motor {
    type Output = Motor;

    fn powi(self, exponent: isize) -> Motor {
        if exponent == 0 {
            return Motor::one();
        }
        let mut x: Motor = if exponent < 0 { self.inverse() } else { self };
        let mut y: Motor = Motor::one();
        let mut n: isize = exponent.abs();
        while 1 < n {
            if n & 1 == 1 {
                y = x.geometric_product(y);
            }
            x = x.geometric_product(x);
            n = n >> 1;
        }
        x.geometric_product(y)
    }
}

impl GeometricQuotient<Motor> for Motor {
    type Output = Motor;

    fn geometric_quotient(self, other: Motor) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Motor> for Motor {
    type Output = Motor;

    fn transformation(self, other: Motor) -> Motor {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<MotorDual> for Motor {
    type Output = MotorDual;

    fn geometric_quotient(self, other: MotorDual) -> MotorDual {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MotorDual> for Motor {
    type Output = MotorDual;

    fn transformation(self, other: MotorDual) -> MotorDual {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<MultiVector> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MultiVector> for Motor {
    type Output = MultiVector;

    fn transformation(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Plane> for Motor {
    type Output = MotorDual;

    fn geometric_quotient(self, other: Plane) -> MotorDual {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Plane> for Motor {
    type Output = Plane;

    fn transformation(self, other: Plane) -> Plane {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Point> for Motor {
    type Output = Motor;

    fn geometric_quotient(self, other: Point) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Point> for Motor {
    type Output = Point;

    fn transformation(self, other: Point) -> Point {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Rotor> for Motor {
    type Output = Motor;

    fn geometric_quotient(self, other: Rotor) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Rotor> for Motor {
    type Output = Rotor;

    fn transformation(self, other: Rotor) -> Rotor {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<f32> for Motor {
    type Output = Motor;

    fn geometric_quotient(self, other: f32) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<f32> for Motor {
    type Output = f32;

    fn transformation(self, other: f32) -> f32 {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Translator> for Motor {
    type Output = Motor;

    fn geometric_quotient(self, other: Translator) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Translator> for Motor {
    type Output = Translator;

    fn transformation(self, other: Translator) -> Translator {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<IdealPoint> for MotorDual {
    type Output = MotorDual;

    fn geometric_quotient(self, other: IdealPoint) -> MotorDual {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<IdealPoint> for MotorDual {
    type Output = IdealPoint;

    fn transformation(self, other: IdealPoint) -> IdealPoint {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Motor> for MotorDual {
    type Output = MotorDual;

    fn geometric_quotient(self, other: Motor) -> MotorDual {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Motor> for MotorDual {
    type Output = Motor;

    fn transformation(self, other: Motor) -> Motor {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<MotorDual> for MotorDual {
    type Output = Motor;

    fn geometric_quotient(self, other: MotorDual) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MotorDual> for MotorDual {
    type Output = MotorDual;

    fn transformation(self, other: MotorDual) -> MotorDual {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<MultiVector> for MotorDual {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MultiVector> for MotorDual {
    type Output = MultiVector;

    fn transformation(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Plane> for MotorDual {
    type Output = Motor;

    fn geometric_quotient(self, other: Plane) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Plane> for MotorDual {
    type Output = Plane;

    fn transformation(self, other: Plane) -> Plane {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Point> for MotorDual {
    type Output = MotorDual;

    fn geometric_quotient(self, other: Point) -> MotorDual {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Point> for MotorDual {
    type Output = Point;

    fn transformation(self, other: Point) -> Point {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Rotor> for MotorDual {
    type Output = MotorDual;

    fn geometric_quotient(self, other: Rotor) -> MotorDual {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Rotor> for MotorDual {
    type Output = Rotor;

    fn transformation(self, other: Rotor) -> Rotor {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<f32> for MotorDual {
    type Output = MotorDual;

    fn geometric_quotient(self, other: f32) -> MotorDual {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<f32> for MotorDual {
    type Output = f32;

    fn transformation(self, other: f32) -> f32 {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Translator> for MotorDual {
    type Output = MotorDual;

    fn geometric_quotient(self, other: Translator) -> MotorDual {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Translator> for MotorDual {
    type Output = Translator;

    fn transformation(self, other: Translator) -> Translator {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<IdealPoint> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: IdealPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<IdealPoint> for MultiVector {
    type Output = IdealPoint;

    fn transformation(self, other: IdealPoint) -> IdealPoint {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Motor> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Motor> for MultiVector {
    type Output = Motor;

    fn transformation(self, other: Motor) -> Motor {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<MotorDual> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MotorDual) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MotorDual> for MultiVector {
    type Output = MotorDual;

    fn transformation(self, other: MotorDual) -> MotorDual {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl Powi for MultiVector {
    type Output = MultiVector;

    fn powi(self, exponent: isize) -> MultiVector {
        if exponent == 0 {
            return MultiVector::one();
        }
        let mut x: MultiVector = if exponent < 0 { self.inverse() } else { self };
        let mut y: MultiVector = MultiVector::one();
        let mut n: isize = exponent.abs();
        while 1 < n {
            if n & 1 == 1 {
                y = x.geometric_product(y);
            }
            x = x.geometric_product(x);
            n = n >> 1;
        }
        x.geometric_product(y)
    }
}

impl GeometricQuotient<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn transformation(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Plane> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Plane> for MultiVector {
    type Output = Plane;

    fn transformation(self, other: Plane) -> Plane {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Point> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Point) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Point> for MultiVector {
    type Output = Point;

    fn transformation(self, other: Point) -> Point {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Rotor> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Rotor> for MultiVector {
    type Output = Rotor;

    fn transformation(self, other: Rotor) -> Rotor {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<f32> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: f32) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<f32> for MultiVector {
    type Output = f32;

    fn transformation(self, other: f32) -> f32 {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Translator> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Translator> for MultiVector {
    type Output = Translator;

    fn transformation(self, other: Translator) -> Translator {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<IdealPoint> for Plane {
    type Output = MotorDual;

    fn geometric_quotient(self, other: IdealPoint) -> MotorDual {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<IdealPoint> for Plane {
    type Output = IdealPoint;

    fn transformation(self, other: IdealPoint) -> IdealPoint {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Motor> for Plane {
    type Output = MotorDual;

    fn geometric_quotient(self, other: Motor) -> MotorDual {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Motor> for Plane {
    type Output = Motor;

    fn transformation(self, other: Motor) -> Motor {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<MotorDual> for Plane {
    type Output = Motor;

    fn geometric_quotient(self, other: MotorDual) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MotorDual> for Plane {
    type Output = MotorDual;

    fn transformation(self, other: MotorDual) -> MotorDual {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<MultiVector> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MultiVector> for Plane {
    type Output = MultiVector;

    fn transformation(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Plane> for Plane {
    type Output = Motor;

    fn geometric_quotient(self, other: Plane) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Plane> for Plane {
    type Output = Plane;

    fn transformation(self, other: Plane) -> Plane {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Point> for Plane {
    type Output = MotorDual;

    fn geometric_quotient(self, other: Point) -> MotorDual {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Point> for Plane {
    type Output = Point;

    fn transformation(self, other: Point) -> Point {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Rotor> for Plane {
    type Output = MotorDual;

    fn geometric_quotient(self, other: Rotor) -> MotorDual {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Rotor> for Plane {
    type Output = Rotor;

    fn transformation(self, other: Rotor) -> Rotor {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<f32> for Plane {
    type Output = Plane;

    fn geometric_quotient(self, other: f32) -> Plane {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<f32> for Plane {
    type Output = f32;

    fn transformation(self, other: f32) -> f32 {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Translator> for Plane {
    type Output = MotorDual;

    fn geometric_quotient(self, other: Translator) -> MotorDual {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Translator> for Plane {
    type Output = Translator;

    fn transformation(self, other: Translator) -> Translator {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<IdealPoint> for Point {
    type Output = Motor;

    fn geometric_quotient(self, other: IdealPoint) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<IdealPoint> for Point {
    type Output = IdealPoint;

    fn transformation(self, other: IdealPoint) -> IdealPoint {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Motor> for Point {
    type Output = Motor;

    fn geometric_quotient(self, other: Motor) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Motor> for Point {
    type Output = Motor;

    fn transformation(self, other: Motor) -> Motor {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<MotorDual> for Point {
    type Output = MotorDual;

    fn geometric_quotient(self, other: MotorDual) -> MotorDual {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MotorDual> for Point {
    type Output = MotorDual;

    fn transformation(self, other: MotorDual) -> MotorDual {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<MultiVector> for Point {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MultiVector> for Point {
    type Output = MultiVector;

    fn transformation(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Plane> for Point {
    type Output = MotorDual;

    fn geometric_quotient(self, other: Plane) -> MotorDual {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Plane> for Point {
    type Output = Plane;

    fn transformation(self, other: Plane) -> Plane {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Point> for Point {
    type Output = Motor;

    fn geometric_quotient(self, other: Point) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Point> for Point {
    type Output = Point;

    fn transformation(self, other: Point) -> Point {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Rotor> for Point {
    type Output = Motor;

    fn geometric_quotient(self, other: Rotor) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Rotor> for Point {
    type Output = Rotor;

    fn transformation(self, other: Rotor) -> Rotor {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<f32> for Point {
    type Output = Point;

    fn geometric_quotient(self, other: f32) -> Point {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<f32> for Point {
    type Output = f32;

    fn transformation(self, other: f32) -> f32 {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Translator> for Point {
    type Output = Motor;

    fn geometric_quotient(self, other: Translator) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Translator> for Point {
    type Output = Translator;

    fn transformation(self, other: Translator) -> Translator {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<IdealPoint> for Rotor {
    type Output = IdealPoint;

    fn geometric_quotient(self, other: IdealPoint) -> IdealPoint {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<IdealPoint> for Rotor {
    type Output = IdealPoint;

    fn transformation(self, other: IdealPoint) -> IdealPoint {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Motor> for Rotor {
    type Output = Motor;

    fn geometric_quotient(self, other: Motor) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Motor> for Rotor {
    type Output = Motor;

    fn transformation(self, other: Motor) -> Motor {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<MotorDual> for Rotor {
    type Output = MotorDual;

    fn geometric_quotient(self, other: MotorDual) -> MotorDual {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MotorDual> for Rotor {
    type Output = MotorDual;

    fn transformation(self, other: MotorDual) -> MotorDual {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<MultiVector> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MultiVector> for Rotor {
    type Output = MultiVector;

    fn transformation(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Plane> for Rotor {
    type Output = MotorDual;

    fn geometric_quotient(self, other: Plane) -> MotorDual {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Plane> for Rotor {
    type Output = Plane;

    fn transformation(self, other: Plane) -> Plane {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Point> for Rotor {
    type Output = Motor;

    fn geometric_quotient(self, other: Point) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Point> for Rotor {
    type Output = Point;

    fn transformation(self, other: Point) -> Point {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl Powi for Rotor {
    type Output = Rotor;

    fn powi(self, exponent: isize) -> Rotor {
        if exponent == 0 {
            return Rotor::one();
        }
        let mut x: Rotor = if exponent < 0 { self.inverse() } else { self };
        let mut y: Rotor = Rotor::one();
        let mut n: isize = exponent.abs();
        while 1 < n {
            if n & 1 == 1 {
                y = x.geometric_product(y);
            }
            x = x.geometric_product(x);
            n = n >> 1;
        }
        x.geometric_product(y)
    }
}

impl GeometricQuotient<Rotor> for Rotor {
    type Output = Rotor;

    fn geometric_quotient(self, other: Rotor) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Rotor> for Rotor {
    type Output = Rotor;

    fn transformation(self, other: Rotor) -> Rotor {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<f32> for Rotor {
    type Output = Rotor;

    fn geometric_quotient(self, other: f32) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<f32> for Rotor {
    type Output = f32;

    fn transformation(self, other: f32) -> f32 {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Translator> for Rotor {
    type Output = Motor;

    fn geometric_quotient(self, other: Translator) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Translator> for Rotor {
    type Output = Translator;

    fn transformation(self, other: Translator) -> Translator {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<IdealPoint> for f32 {
    type Output = IdealPoint;

    fn geometric_quotient(self, other: IdealPoint) -> IdealPoint {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<IdealPoint> for f32 {
    type Output = IdealPoint;

    fn transformation(self, other: IdealPoint) -> IdealPoint {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Motor> for f32 {
    type Output = Motor;

    fn geometric_quotient(self, other: Motor) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Motor> for f32 {
    type Output = Motor;

    fn transformation(self, other: Motor) -> Motor {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<MotorDual> for f32 {
    type Output = MotorDual;

    fn geometric_quotient(self, other: MotorDual) -> MotorDual {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MotorDual> for f32 {
    type Output = MotorDual;

    fn transformation(self, other: MotorDual) -> MotorDual {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<MultiVector> for f32 {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MultiVector> for f32 {
    type Output = MultiVector;

    fn transformation(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Plane> for f32 {
    type Output = Plane;

    fn geometric_quotient(self, other: Plane) -> Plane {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Plane> for f32 {
    type Output = Plane;

    fn transformation(self, other: Plane) -> Plane {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Point> for f32 {
    type Output = Point;

    fn geometric_quotient(self, other: Point) -> Point {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Point> for f32 {
    type Output = Point;

    fn transformation(self, other: Point) -> Point {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Rotor> for f32 {
    type Output = Rotor;

    fn geometric_quotient(self, other: Rotor) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Rotor> for f32 {
    type Output = Rotor;

    fn transformation(self, other: Rotor) -> Rotor {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Translator> for f32 {
    type Output = Translator;

    fn geometric_quotient(self, other: Translator) -> Translator {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Translator> for f32 {
    type Output = Translator;

    fn transformation(self, other: Translator) -> Translator {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<IdealPoint> for Translator {
    type Output = Motor;

    fn geometric_quotient(self, other: IdealPoint) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<IdealPoint> for Translator {
    type Output = IdealPoint;

    fn transformation(self, other: IdealPoint) -> IdealPoint {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Motor> for Translator {
    type Output = Motor;

    fn geometric_quotient(self, other: Motor) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Motor> for Translator {
    type Output = Motor;

    fn transformation(self, other: Motor) -> Motor {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<MotorDual> for Translator {
    type Output = MotorDual;

    fn geometric_quotient(self, other: MotorDual) -> MotorDual {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MotorDual> for Translator {
    type Output = MotorDual;

    fn transformation(self, other: MotorDual) -> MotorDual {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<MultiVector> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MultiVector> for Translator {
    type Output = MultiVector;

    fn transformation(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Plane> for Translator {
    type Output = MotorDual;

    fn geometric_quotient(self, other: Plane) -> MotorDual {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Plane> for Translator {
    type Output = Plane;

    fn transformation(self, other: Plane) -> Plane {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Point> for Translator {
    type Output = Motor;

    fn geometric_quotient(self, other: Point) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Point> for Translator {
    type Output = Point;

    fn transformation(self, other: Point) -> Point {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Rotor> for Translator {
    type Output = Motor;

    fn geometric_quotient(self, other: Rotor) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Rotor> for Translator {
    type Output = Rotor;

    fn transformation(self, other: Rotor) -> Rotor {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<f32> for Translator {
    type Output = Translator;

    fn geometric_quotient(self, other: f32) -> Translator {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<f32> for Translator {
    type Output = f32;

    fn transformation(self, other: f32) -> f32 {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Translator> for Translator {
    type Output = Motor;

    fn geometric_quotient(self, other: Translator) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Translator> for Translator {
    type Output = Translator;

    fn transformation(self, other: Translator) -> Translator {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

