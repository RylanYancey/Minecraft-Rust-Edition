use std::ops::*;
use super::{one::{One, Zero}, Vec2};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Vec3<T>(pub T, pub T, pub T);

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self(x, y, z)
    }

    pub fn x(&self) -> &T {
        &self.0
    }

    pub fn y(&self) -> &T {
        &self.1
    }

    pub fn z(&self) -> &T {
        &self.2
    }

    pub fn map<F, K>(&self, f: F) -> Vec3<K>
    where
        F: Fn(&T) -> K
    {
        Vec3(
            (f)(&self.0),
            (f)(&self.1),
            (f)(&self.2)
        )
    }

    pub fn map_with<F, U, K>(&self, with: Vec3<U>, f: F) -> Vec3<K> 
    where
        F: Fn(&T, &U) -> K
    {
        Vec3(
            (f)(&self.0, &with.0),
            (f)(&self.1, &with.1),
            (f)(&self.2, &with.2)
        )
    }
}

impl<T: Clone> Vec3<T> {
    pub fn splat(n: T) -> Self {
        Self(n.clone(), n.clone(), n.clone())
    }

    pub fn xz(&self) -> Vec2<T> {
        Vec2(self.0.clone(), self.2.clone())
    }

    pub fn xy(&self) -> Vec2<T> {
        Vec2(self.0.clone(), self.1.clone())
    }

    pub fn yz(&self) -> Vec2<T> {
        Vec2(self.1.clone(), self.2.clone())
    }
}

impl Vec3<i32> {
    pub fn containing_subchunk(&self) -> Self {
        self.map(|v| v - (v & 15))
    }

    pub fn subchunk_index(&self) -> usize {
        ((self.y() & 15) + (self.x() & 15) * 16 + (self.z() & 15) * 256) as usize
    }
}

impl<T: One + Zero + Neg<Output=T>> Vec3<T> {
    pub fn up() -> Self {
        Self(T::zero(), T::one(), T::zero())
    }

    pub fn down() -> Self {
        -Self::up()
    }

    pub fn north() -> Self {
        Self(T::zero(), T::zero(), T::one())
    }

    pub fn south() -> Self {
        -Self::north()
    }

    pub fn east() -> Self {
        Self(T::one(), T::zero(), T::zero())
    }

    pub fn west() -> Self {
        -Self::east()
    }
}

impl<T: Neg<Output=T>> Neg for Vec3<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

macro_rules! impl_ops {
    ($op_trait: tt, $op_fn: ident, $op: tt) => {
        impl<T: $op_trait<Output=T>> $op_trait for Vec3<T> {
            type Output = Self;

            fn $op_fn(self, rhs: Self) -> Self {
                Self(
                    self.0 $op rhs.0,
                    self.1 $op rhs.1,
                    self.2 $op rhs.2
                )
            }
        }
    }
}

macro_rules! impl_assign_ops {
    ($op_trait: tt, $op_fn: ident, $op: tt) => {
        impl<T: $op_trait> $op_trait for Vec3<T> {
            fn $op_fn(&mut self, rhs: Self) {
                self.0 $op rhs.0;
                self.1 $op rhs.1;
                self.2 $op rhs.2;
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