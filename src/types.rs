#[derive(Debug, Copy, Clone)]
pub struct Wrapped {
    value: isize,
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

impl std::ops::Add for Wrapped {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        return Self{
            value: (self.value + other.value).rem_euclid(self.size),
            size: self.size,
        };
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

#[cfg(test)]
mod wrapped_tests {
    use super::*;

    #[test]
    fn addition() {
        let a = Wrapped::byte(0x01);
        let b = Wrapped::byte(0x01);

        assert_eq!(a + b, Wrapped::byte(0x02));
    }

    #[test]
    fn addition_wrapped() {
        let a = Wrapped::byte(0xff);
        let b = Wrapped::byte(0x01);

        assert_eq!(a + b, Wrapped::byte(0x00));
    }

    #[test]
    fn subtraction() {
        let a = Wrapped::byte(0x01);
        let b = Wrapped::byte(0x01);

        assert_eq!(a - b, Wrapped::byte(0x00));
    }

    #[test]
    fn subtraction_wrapped() {
        let a = Wrapped::byte(0x00);
        let b = Wrapped::byte(0x01);

        assert_eq!(a - b, Wrapped::byte(0xff));
    }
}