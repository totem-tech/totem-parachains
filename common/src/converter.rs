use crate::TryConvert;
use core::convert::TryFrom;
use sp_runtime::{traits::Convert, AccountId32};
use sp_std::vec::Vec;

pub struct Converter;

impl Convert<u32, u64> for Converter {
	fn convert(x: u32) -> u64 {
		x as u64
	}
}

impl Convert<u32, f64> for Converter {
	fn convert(x: u32) -> f64 {
		x as f64
	}
}

impl Convert<u64, f64> for Converter {
	fn convert(x: u64) -> f64 {
		x as f64
	}
}

impl Convert<u64, i128> for Converter {
	fn convert(x: u64) -> i128 {
		if x <= i128::MAX as u64 {
			x as i128
		} else {
			0 // default value
		}
	}
}

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

impl Convert<u128, i128> for Converter {
    fn convert(x: u128) -> i128 {
        x as i128
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

impl Convert<[u8; 32], u64> for Converter {
    fn convert(a: [u8; 32]) -> u64 {
        let mut result: u64 = 0;
        for &byte in a.iter() {
            result = (result << 8) | u64::from(byte);
        }
        result
    }
}

impl Convert<u64, [u8; 32]> for Converter {
    fn convert(a: u64) -> [u8; 32] {
        let mut result: [u8; 32] = [0; 32];
        let mut temp = a;
        for i in 0..result.len() {
            result[31 - i] = temp as u8;
            temp >>= 8;
        }
        result
    }
}

impl Convert<Vec<u8>, [u8; 8]> for Converter {
    fn convert(v: Vec<u8>) -> [u8; 8] {
        let mut a = [0; 8];
        if v.len() >= 8 {
            a.copy_from_slice(&v[..8]);
        } else {
            a[..v.len()].copy_from_slice(&v);
        }
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

impl TryConvert<u64, i128> for Converter {
	fn try_convert(x: u64) -> Option<i128> {
		i128::try_from(x).ok()
	}
}
