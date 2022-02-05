use crate::TryConvert;
use core::convert::TryFrom;
use sp_runtime::{traits::Convert, AccountId32};
use sp_std::vec::Vec;

pub struct Converter;

impl Convert<u32, u32> for Converter {
    fn convert(x: u32) -> u32 {
        x
    }
}

impl Convert<u64, u64> for Converter {
    fn convert(x: u64) -> u64 {
        x
    }
}

impl Convert<u128, u128> for Converter {
    fn convert(x: u128) -> u128 {
        x
    }
}

impl Convert<i128, i128> for Converter {
    fn convert(x: i128) -> i128 {
        x
    }
}

impl Convert<[u8; 32], AccountId32> for Converter {
    fn convert(a: [u8; 32]) -> AccountId32 {
        AccountId32::new(a)
    }
}

impl Convert<Vec<u8>, [u8; 8]> for Converter {
    fn convert(mut v: Vec<u8>) -> [u8; 8] {
        let mut a = [0; 8];

        v.resize(8, 0);
        a.copy_from_slice(&v);

        a
    }
}

impl TryConvert<i128, u128> for Converter {
    fn try_convert(x: i128) -> Option<u128> {
        x.checked_abs().and_then(|i| u128::try_from(i).ok())
    }
}

impl TryConvert<u128, i128> for Converter {
    fn try_convert(x: u128) -> Option<i128> {
        i128::try_from(x).ok()
    }
}
