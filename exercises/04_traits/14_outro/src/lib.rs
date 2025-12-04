// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::ops::Add;
#[derive(PartialEq, Debug, Clone, Copy)]
// copy is needed here as the add always consumes the self, copy trait dodges this by passing a
// copy of the original variable into the function instead of the original one to
// solve ownership issues.
pub struct SaturatingU16 {
    value: u16,
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, info: &u16) -> bool {
        &self.value == info
    }

    // no need to write this methods as it just uses eq and shows ops
    // output of it.
    fn ne(&self, other: &u16) -> bool {
        !self.eq(other)
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: u16) -> Self::Output {
        //by going into a type scope we can access its
        //respective wrapping and saturating methods
        let added: u16 = u16::saturating_add(self.value, rhs);
        SaturatingU16 { value: added }
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: &u16) -> Self::Output {
        let added: u16 = u16::saturating_add(self.value, *rhs);
        SaturatingU16 { value: added }
    }
}

impl Add for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: Self) -> Self::Output {
        let added: u16 = u16::saturating_add(self.value, rhs.value);
        SaturatingU16 { value: added }
    }
}

impl Add<&Self> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: &Self) -> Self::Output {
        let added: u16 = u16::saturating_add(self.value, (*rhs).value);
        SaturatingU16 { value: added }
    }
}

impl From<u16> for SaturatingU16 {
    fn from(info: u16) -> Self {
        SaturatingU16 { value: info as u16 }
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(info: &u16) -> Self {
        SaturatingU16 { value: *info }
    }
}

//into automatically safely coerces types by inferreing the
//type of varibale , it is safer than 'as'
impl From<u8> for SaturatingU16 {
    fn from(info: u8) -> Self {
        SaturatingU16 { value: info.into() }
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(info: &u8) -> Self {
        SaturatingU16 {
            value: (*info).into(),
        }
    }
}
