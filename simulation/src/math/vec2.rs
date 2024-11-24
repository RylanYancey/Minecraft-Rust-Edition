use std::ops::*;
use super::{one::{One, Zero}, Vec3};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub struct Vec2<T>(pub T, pub T);

impl<T> Vec2<T> {
    pub fn new(x: T, z: T) -> Self {
        Self(x, z)
    }

    pub fn x(&self) -> &T {
        &self.0
    }

    pub fn z(&self) -> &T {
        &self.1
    }

    pub fn map<F, K>(&self, f: F) -> Vec2<K>
    where
        F: Fn(&T) -> K
    {
        Vec2(
            (f)(&self.0),
            (f)(&self.1)
        )
    }

    pub fn map_with<F, U, K>(&self, with: &Vec2<U>, f: F) -> Vec2<K> 
    where
        F: Fn(&T, &U) -> K
    {
        Vec2(
            (f)(&self.0, &with.0),
            (f)(&self.1, &with.1)
        )
    }
}

impl<T: Clone> Vec2<T> {
    pub fn splat(n: T) -> Self {
        Self(n.clone(), n.clone())
    }

    pub fn extend_y(&self, y: T) -> Vec3<T> {
        Vec3(self.0.clone(), y, self.1.clone())
    }

    pub fn extend(&self, v: T) -> Vec3<T> {
        Vec3(self.0.clone(), self.1.clone(), v)
    }
}

impl<T: One + Zero + Neg<Output=T>> Vec2<T> {
    pub fn north() -> Self {
        Self(T::zero(), T::one())
    }

    pub fn south() -> Self {
        -Self::north()
    }

    pub fn east() -> Self {
        Self(T::one(), T::zero())
    }

    pub fn west() -> Self {
        -Self::east()
    }
}

impl Vec2<i32> {
    #[inline]
    pub fn to_i64(self) -> i64 {
        ((self.0 as i64) << 32) | (self.1 as i64 & 0xFFFF_FFFF)
    }

    #[inline]
    pub fn from_i64(n: i64) -> Self {
        Self((n >> 32) as i32, (n & 0xFFFF_FFFF) as i32)
    }

    #[inline]
    pub fn containing_region(&self) -> Self {
        Self(self.0 - (self.0 & 511), self.1 - (self.1 & 511))
    }

    #[inline]
    pub fn containing_chunk(&self) -> Self {
        Self(self.0 - (self.0 & 15), self.1 - (self.1 & 15))
    }
}

impl<T: Neg<Output=T> + Ord + Zero + Clone> Vec2<T> {
    pub fn abs(&self) -> Self {
        Self(
            if self.0 >= T::zero() {
                self.0.clone()
            } else {
                -self.0.clone()
            },
            if self.1 >= T::zero() {
                self.1.clone()
            } else {
                -self.1.clone()
            }
        )
    }
}

impl<T: Neg<Output=T>> Neg for Vec2<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec2(-self.0, -self.1)
    }
}

macro_rules! impl_ops {
    ($op_trait: tt, $op_fn: ident, $op: tt) => {
        impl<T: $op_trait<Output=T>> $op_trait for Vec2<T> {
            type Output = Self;

            fn $op_fn(self, rhs: Self) -> Self {
                Self(
                    self.0 $op rhs.0,
                    self.1 $op rhs.1,
                )
            }
        }
    }
}

macro_rules! impl_assign_ops {
    ($op_trait: tt, $op_fn: ident, $op: tt) => {
        impl<T: $op_trait> $op_trait for Vec2<T> {
            fn $op_fn(&mut self, rhs: Self) {
                self.0 $op rhs.0;
                self.1 $op rhs.1;
            }
        }
    }
}

impl_ops!(Add, add, +);
impl_ops!(Sub, sub, -);
impl_ops!(Mul, mul, *);
impl_ops!(Div, div, /);
impl_ops!(Rem, rem, %);
impl_ops!(BitAnd, bitand, &);
impl_ops!(BitOr, bitor, |);
impl_ops!(BitXor, bitxor, ^);

impl_assign_ops!(AddAssign, add_assign, +=);
impl_assign_ops!(SubAssign, sub_assign, -=);
impl_assign_ops!(MulAssign, mul_assign, *=);
impl_assign_ops!(DivAssign, div_assign, /=);
impl_assign_ops!(RemAssign, rem_assign, %=);
impl_assign_ops!(BitAndAssign, bitand_assign, &=);
impl_assign_ops!(BitOrAssign, bitor_assign, |=);
impl_assign_ops!(BitXorAssign, bitxor_assign, ^=);