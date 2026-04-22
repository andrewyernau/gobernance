#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct UnitF32(f32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitF32Error {
    NotFinite,
    OutOfRange,
}

impl UnitF32 {
    pub const ZERO: Self = Self(0.0);
    pub const ONE: Self = Self(1.0);

    pub fn new(value: f32) -> Result<Self, UnitF32Error> {
        if !value.is_finite() {
            return Err(UnitF32Error::NotFinite);
        }

        if !(0.0..=1.0).contains(&value) {
            return Err(UnitF32Error::OutOfRange);
        }

        Ok(Self(value))
    }

    pub fn get(&self) -> f32 {
        self.0
    }

    pub fn into_inner(self) -> f32 {
        self.0
    }
}

impl TryFrom<f32> for UnitF32 {
    type Error = UnitF32Error;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<UnitF32> for f32 {
    fn from(value: UnitF32) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_zero() {
        assert_eq!(UnitF32::new(0.0), Ok(UnitF32::ZERO));
    }

    #[test]
    fn accepts_one() {
        assert_eq!(UnitF32::new(1.0), Ok(UnitF32::ONE));
    }

    #[test]
    fn accepts_middle_value() {
        assert!(UnitF32::new(0.5).is_ok());
    }

    #[test]
    fn rejects_negative() {
        assert_eq!(UnitF32::new(-0.1), Err(UnitF32Error::OutOfRange));
    }

    #[test]
    fn rejects_above_one() {
        assert_eq!(UnitF32::new(1.1), Err(UnitF32Error::OutOfRange));
    }

    #[test]
    fn rejects_nan() {
        assert_eq!(UnitF32::new(f32::NAN), Err(UnitF32Error::NotFinite));
    }

    #[test]
    fn rejects_infinity() {
        assert_eq!(UnitF32::new(f32::INFINITY), Err(UnitF32Error::NotFinite));
    }
}
