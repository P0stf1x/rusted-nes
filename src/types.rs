use std::num::Wrapping;

#[derive(Debug, Copy, Clone)]
pub struct Wrapped {
    pub value: isize,
    size: isize,
}

// Constructors
impl Wrapped {
    pub fn byte(value: isize) -> Self {
        if value >= 256 || value < 0 {
            panic!("Wrong value");
        }
        return Self{
            value,
            size: 256,
        };
    }

    pub fn word(value: isize) -> Self {
        if value >= 256*256 || value < 0 {
            panic!("Wrong value");
        }
        return Self{
            value,
            size: 256*256
        };
    }
}

impl PartialEq for Wrapped {
    fn eq(&self, other: &Self) -> bool {
        return self.value == other.value;
    }
}

impl PartialEq<isize> for Wrapped {
    fn eq(&self, &other: &isize) -> bool {
        return self.value == other;
    }
}

impl From<Wrapping<u8>> for Wrapped {
    fn from(value: Wrapping<u8>) -> Self {
        return Wrapped::byte(value.0 as isize);
    }
}

impl From<Wrapping<u16>> for Wrapped {
    fn from(value: Wrapping<u16>) -> Self {
        return Wrapped::word(value.0 as isize);
    }
}

impl std::ops::Add for Wrapped {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        return Self{
            value: (self.value + other.value).rem_euclid(self.size),
            size: self.size,
        };
    }
}

impl std::ops::Add<isize> for Wrapped {
    type Output = Self;

    fn add(self, other: isize) -> Self {
        return Self{
            value: (self.value + other).rem_euclid(self.size),
            size: self.size,
        };
    }
}

impl std::ops::AddAssign for Wrapped {
    fn add_assign(&mut self, other: Self) {
        self.value = (self.value + other.value).rem_euclid(self.size);
    }
}

impl std::ops::AddAssign<isize> for Wrapped {
    fn add_assign(&mut self, other: isize) {
        self.value = (self.value + other).rem_euclid(self.size);
    }
}

impl std::ops::Sub for Wrapped {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        return Self{
            value: (self.value - other.value).rem_euclid(self.size),
            size: self.size,
        };
    }
}

impl std::ops::Sub<isize> for Wrapped {
    type Output = Self;

    fn sub(self, other: isize) -> Self::Output {
        return Self{
            value: (self.value - other).rem_euclid(self.size),
            size: self.size,
        };
    }
}

impl std::ops::SubAssign for Wrapped {
    fn sub_assign(&mut self, other: Self) {
        self.value = (self.value - other.value).rem_euclid(self.size);
    }
}

impl std::ops::SubAssign<isize> for Wrapped {
    fn sub_assign(&mut self, other: isize) {
        self.value = (self.value - other).rem_euclid(self.size);
    }
}

#[cfg(test)]
mod wrapped_tests {
    use super::*;

    use proptest::prelude::*;
    proptest! {
        #[test]
        fn from_wrapping_u8(val in 0u8..=255) {
            prop_assert_eq!(Wrapped::from(Wrapping(val)).value as u8, val);
        }

        #[test]
        fn from_wrapping_u16(val in 0u16..=65535) {
            prop_assert_eq!(Wrapped::from(Wrapping(val)).value as u16, val);
        }

        #[test]
        fn to_wrapping_u8(val in 0u8..=255) {
            prop_assert_eq!(Into::<Wrapped>::into(Wrapping(val)), Wrapped::byte(val as isize));
        }

        #[test]
        fn to_wrapping_u16(val in 0u16..=65535) {
            prop_assert_eq!(Into::<Wrapped>::into(Wrapping(val)), Wrapped::word(val as isize));
        }
    }

    #[test]
    fn compare_with_self() {
        assert_eq!(Wrapped::byte(0x01), Wrapped::byte(0x01));
        assert_eq!(Wrapped::byte(0x01), Wrapped::word(0x01));
        assert_eq!(Wrapped::word(0x01), Wrapped::byte(0x01));
        assert_eq!(Wrapped::word(0x01), Wrapped::word(0x01));
    }

    #[test]
    fn compare_with_isize() {
        assert_eq!(Wrapped::byte(0x01), 0x01);
        assert_eq!(Wrapped::word(0x01), 0x01);
    }

    #[test]
    fn addition() {
        let a = Wrapped::byte(0x01);
        let b = Wrapped::byte(0x01);
        let c = 0x01;

        assert_eq!(a + b, 0x02);
        assert_eq!(a + c, 0x02);
    }

    #[test]
    fn addition_wrapped() {
        let a = Wrapped::byte(0xff);
        let b = Wrapped::byte(0x01);
        let c = 0x01;

        assert_eq!(a + b, 0x00);
        assert_eq!(a + c, 0x00);
    }

    #[test]
    fn subtraction() {
        let a = Wrapped::byte(0x01);
        let b = Wrapped::byte(0x01);
        let c = 0x01;

        assert_eq!(a - b, 0x00);
        assert_eq!(a - c, 0x00);
    }

    #[test]
    fn subtraction_wrapped() {
        let a = Wrapped::byte(0x00);
        let b = Wrapped::byte(0x01);
        let c = 0x01;

        assert_eq!(a - b, 0xff);
        assert_eq!(a - c, 0xff);
    }

    #[test]
    fn addition_assign() {
        let mut a1 = Wrapped::byte(0x01);
        let mut a2 = Wrapped::byte(0x01);
        a1 += Wrapped::byte(0x01);
        a2 += 0x01;

        assert_eq!(a1, 0x02);
        assert_eq!(a2, 0x02);
    }

    #[test]
    fn addition_assign_wrapped() {
        let mut a1 = Wrapped::byte(0xff);
        let mut a2 = Wrapped::byte(0xff);
        a1 += Wrapped::byte(0x01);
        a2 += 0x01;

        assert_eq!(a1, 0x00);
        assert_eq!(a2, 0x00);
    }

    #[test]
    fn subtraction_assign() {
        let mut a1 = Wrapped::byte(0x01);
        let mut a2 = Wrapped::byte(0x01);
        a1 -= Wrapped::byte(0x01);
        a2 -= 0x01;

        assert_eq!(a1, 0x00);
        assert_eq!(a2, 0x00);
    }

    #[test]
    fn subtraction_assign_wrapped() {
        let mut a1 = Wrapped::byte(0x00);
        let mut a2 = Wrapped::byte(0x00);
        a1 -= Wrapped::byte(0x01);
        a2 -= 0x01;

        assert_eq!(a1, 0xff);
        assert_eq!(a2, 0xff);
    }
}
