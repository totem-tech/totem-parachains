//                              Næ§@@@ÑÉ©
//                        æ@@@@@@@@@@@@@@@@@@
//                    Ñ@@@@?.?@@@@@@@@@@@@@@@@@@@N
//                 ¶@@@@@?^%@@.=@@@@@@@@@@@@@@@@@@@@
//               N@@@@@@@?^@@@»^@@@@@@@@@@@@@@@@@@@@@@
//               @@@@@@@@?^@@@».............?@@@@@@@@@É
//              Ñ@@@@@@@@?^@@@@@@@@@@@@@@@@@@'?@@@@@@@@Ñ
//              @@@@@@@@@?^@@@»..............»@@@@@@@@@@
//              @@@@@@@@@?^@@@»^@@@@@@@@@@@@@@@@@@@@@@@@
//              @@@@@@@@@?^ë@@&.@@@@@@@@@@@@@@@@@@@@@@@@
//               @@@@@@@@?^´@@@o.%@@@@@@@@@@@@@@@@@@@@©
//                @@@@@@@?.´@@@@@ë.........*.±@@@@@@@æ
//                 @@@@@@@@?´.I@@@@@@@@@@@@@@.&@@@@@N
//                  N@@@@@@@@@@ë.*=????????=?@@@@@Ñ
//                    @@@@@@@@@@@@@@@@@@@@@@@@@@@¶
//                        É@@@@@@@@@@@@@@@@Ñ¶
//                             Næ§@@@ÑÉ©

// Copyright 2020 Chris D'Costa
// This file is part of Totem Live Accounting.
// Authors:
// - Félix Daudré-Vignier   email: felix@totemaccounting.com
// - Chris D'Costa          email: chris.dcosta@totemaccounting.com

// Totem is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Totem is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Totem.  If not, see <http://www.gnu.org/licenses/>.

use codec::{Decode, Encode, EncodeLike, FullCodec, FullEncode};
use frame_support::{dispatch::DispatchResultWithPostInfo, storage::StorageMap};

/// Adds behavior to `StorageMap`s.
pub trait StorageMapExt<K, V>
where
    Self: StorageMap<K, V>,
    K: FullEncode + Encode + EncodeLike,
    V: FullCodec + Decode + FullEncode + Encode + EncodeLike,
{
    /// If the key exists in the map, modifies it with the provided function,
    /// otherwise, an error is returned.
    fn mutate_or_err<KeyArg: EncodeLike<K>, F: FnOnce(&mut V)>(
        key: KeyArg,
        f: F,
    ) -> DispatchResultWithPostInfo {
        Self::mutate_exists(key, |option| match option.as_mut() {
            Some(value) => Ok(f(value).into()),
            None => Err("Cannot recover the value".into()),
        })
    }
}

impl<T, K, V> StorageMapExt<K, V> for T
where
    T: StorageMap<K, V>,
    K: FullEncode + Encode + EncodeLike,
    V: FullCodec + Decode + FullEncode + Encode + EncodeLike,
{
}
