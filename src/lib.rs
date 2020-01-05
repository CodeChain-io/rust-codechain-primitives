// Copyright 2018, 2020 Kodebox, Inc.
// This file is part of CodeChain.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

mod hash;

pub use crate::hash::{H128, H160, H256, H264, H512, H520};
pub use ethereum_types::U256;

pub fn h128_from_u128(u: u128) -> H128 {
    H128(u.to_be_bytes())
}

pub fn u256_from_u128(u: u128) -> U256 {
    let mut arr: [u64; 4] = [0, 0, 0, 0];
    arr[0] = (u & u128::from(std::u64::MAX)) as u64;
    arr[1] = (u >> 64) as u64;
    U256(arr)
}

/// Return `s` without the `0x` at the beginning of it, if any.
pub fn remove_0x_prefix(s: &str) -> &str {
    if s.starts_with("0x") {
        &s[2..]
    } else {
        s
    }
}

pub type Bytes = Vec<u8>;

#[cfg(test)]
mod tests_h128_from_u128 {
    use super::*;
    use ethereum_types::U128;

    #[test]
    fn zero() {
        assert_eq!(H128::zero(), h128_from_u128(0));
    }

    #[test]
    fn one() {
        assert_eq!(H128::from(1), h128_from_u128(1));
    }

    #[test]
    fn u64_max_plus_1() {
        assert_eq!(H128::from(U128::from(std::u64::MAX) + 1), h128_from_u128(u128::from(std::u64::MAX) + 1));
    }

    #[test]
    fn max_minus_1() {
        assert_eq!(H128::from(U128::max_value() - 1), h128_from_u128(std::u128::MAX - 1));
    }

    #[test]
    fn u128_max() {
        assert_eq!(H128::from(U128::max_value()), h128_from_u128(std::u128::MAX));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethereum_types::U128;

    #[test]
    fn u128_zero() {
        assert_eq!(u256_from_u128(0), U128::zero().into());
    }

    #[test]
    fn u128_one() {
        assert_eq!(u256_from_u128(1), U128::one().into());
    }

    #[test]
    fn u64_max_plus_1() {
        let u128: U128 = U128::from(std::u64::MAX) + 1;
        assert_eq!(u256_from_u128(u128::from(std::u64::MAX) + 1), u128.into());
    }

    #[test]
    fn u128_max_minus_1() {
        let u128: U128 = U128::max_value() - 1;
        assert_eq!(u256_from_u128(std::u128::MAX - 1), u128.into());
    }

    #[test]
    fn u128_max() {
        assert_eq!(u256_from_u128(std::u128::MAX), U128::max_value().into());
    }

    #[test]
    fn remove_0x_prefix_returns_the_same_if_it_does_not_start_with_0x() {
        const S: &str = "abcdef";
        assert_eq!("abcdef", remove_0x_prefix(S));
    }

    #[test]
    fn remove_0x_prefix_returns_the_same_if_it_starts_with_0() {
        const S: &str = "0abcdef";
        assert_eq!("0abcdef", remove_0x_prefix(S));
    }

    #[test]
    fn remove_0x_prefix_works() {
        const S: &str = "0xabcdef";
        assert_eq!("abcdef", remove_0x_prefix(S));
    }

    #[test]
    fn remove_0x_prefix_returns_empty_string_if_input_is_0x() {
        const S: &str = "0x";
        assert_eq!("", remove_0x_prefix(S));
    }
}
