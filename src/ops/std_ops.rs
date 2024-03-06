use crate::{FloatX, NumBits, RoundoffImpl};

/// `a + b`
impl<M: NumBits, Impl: RoundoffImpl<M>, T: Into<FloatX<M, Impl>>> std::ops::Add<T> for FloatX<M, Impl> {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        let mantissa_len = self.mantissa_len.least_bits(&rhs.mantissa_len);
        FloatX::new(self.roundoff_impl.add(self.repr, rhs.repr, &mantissa_len), mantissa_len, self.roundoff_impl)
    }
}

/// `a += b`
impl<M: NumBits, Impl: RoundoffImpl<M>, T: Into<FloatX<M, Impl>>> std::ops::AddAssign<T> for FloatX<M, Impl> {
    fn add_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        let mantissa_len = self.mantissa_len.least_bits(&rhs.mantissa_len);
        self.repr = self.roundoff_impl.add(self.repr, rhs.repr, &mantissa_len);
        self.mantissa_len = mantissa_len;
    }
}

/// `a - b`
impl<M: NumBits, Impl: RoundoffImpl<M>, T: Into<FloatX<M, Impl>>> std::ops::Sub<T> for FloatX<M, Impl> {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        let mantissa_len = self.mantissa_len.least_bits(&rhs.mantissa_len);
        FloatX::new(self.roundoff_impl.sub(self.repr, rhs.repr, &mantissa_len), self.mantissa_len, self.roundoff_impl)
    }
}

/// `a -= b`
impl<M: NumBits, Impl: RoundoffImpl<M>, T: Into<FloatX<M, Impl>>> std::ops::SubAssign<T> for FloatX<M, Impl> {
    fn sub_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        let mantissa_len = self.mantissa_len.least_bits(&rhs.mantissa_len);
        self.repr = self.roundoff_impl.sub(self.repr, rhs.repr, &mantissa_len);
        self.mantissa_len = mantissa_len;
    }
}

/// `a * b`
impl<M: NumBits, Impl: RoundoffImpl<M>, T: Into<FloatX<M, Impl>>> std::ops::Mul<T> for FloatX<M, Impl> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        let mantissa_len = self.mantissa_len.least_bits(&rhs.mantissa_len);
        FloatX::new(self.roundoff_impl.mul(self.repr, rhs.repr, &mantissa_len), self.mantissa_len, self.roundoff_impl)
    }
}

/// `a *= b`
impl<M: NumBits, Impl: RoundoffImpl<M>, T: Into<FloatX<M, Impl>>> std::ops::MulAssign<T> for FloatX<M, Impl> {
    fn mul_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        let mantissa_len = self.mantissa_len.least_bits(&rhs.mantissa_len);
        self.repr = self.roundoff_impl.mul(self.repr, rhs.repr, &mantissa_len);
        self.mantissa_len = mantissa_len;
    }
}

/// `a / b`
impl<M: NumBits, Impl: RoundoffImpl<M>, T: Into<FloatX<M, Impl>>> std::ops::Div<T> for FloatX<M, Impl> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        let mantissa_len = self.mantissa_len.least_bits(&rhs.mantissa_len);
        FloatX::new(self.roundoff_impl.div(self.repr, rhs.repr, &mantissa_len), self.mantissa_len, self.roundoff_impl)
    }
}

/// `a /= b`
impl<M: NumBits, Impl: RoundoffImpl<M>, T: Into<FloatX<M, Impl>>> std::ops::DivAssign<T> for FloatX<M, Impl> {
    fn div_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        let mantissa_len = self.mantissa_len.least_bits(&rhs.mantissa_len);
        self.repr = self.roundoff_impl.div(self.repr, rhs.repr, &mantissa_len);
        self.mantissa_len = mantissa_len;
    }
}

/// `-a`
impl<M: NumBits, Impl: RoundoffImpl<M>> std::ops::Neg for FloatX<M, Impl> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        FloatX::new(-self.repr, self.mantissa_len, self.roundoff_impl)
    }
}

/// `a % b`
impl<M: NumBits, Impl: RoundoffImpl<M>, T: Into<FloatX<M, Impl>>> std::ops::Rem<T> for FloatX<M, Impl> {
    type Output = Self;

    fn rem(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        let mantissa_len = self.mantissa_len.least_bits(&rhs.mantissa_len);
        FloatX::new(self.roundoff_impl.rem(self.repr, rhs.repr, &mantissa_len), self.mantissa_len, self.roundoff_impl)
    }
}

/// `a %= b`
impl<M: NumBits, Impl: RoundoffImpl<M>, T: Into<FloatX<M, Impl>>> std::ops::RemAssign<T> for FloatX<M, Impl> {
    fn rem_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        let mantissa_len = self.mantissa_len.least_bits(&rhs.mantissa_len);
        self.repr = self.roundoff_impl.rem(self.repr, rhs.repr, &mantissa_len);
        self.mantissa_len = mantissa_len;
    }
}