use crate::prelude::{Num, Signed};

pub fn round_to<T: Num + PartialEq + PartialOrd + Signed + Copy + Clone>(num: T, multiple: T) -> T {
    if multiple == T::zero() {
        return num;
    }

    let remainder = num.abs() % multiple;
    if remainder == T::zero() {
        return num;
    }

    if num < T::zero() {
        return -(num.abs() - remainder);
    } else {
        return num + multiple - remainder;
    }
}
