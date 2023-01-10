/*
 *    Copyright (c) 2023.  Michael Pacheco
 *    Licensed under the Apache License, Version 2.0 (the "License");
 *    you may not use this file except in compliance with the License.
 *    You may obtain a copy of the License at
 *
 *        http://www.apache.org/licenses/LICENSE-2.0
 *
 *    Unless required by applicable law or agreed to in writing, software
 *    distributed under the License is distributed on an "AS IS" BASIS,
 *    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *    See the License for the specific language governing permissions and
 *    limitations under the License.
 */
//! Library to method chain to other types.
//! # The goal of the library
//! This library abstracts a commonly some commonly functions in a format that allows for method
//! chaining.

use core::time::Duration;

/// Trait for converting a value to a `Duration`.
pub trait ToDuration<T> {
    ///The type used for intermediary conversion (in this case, `Duration` expects an u64)
    type Intermediary;
    /// Converts `self` to a `Duration` consuming it.
    ///
    /// # Returns
    /// A `Result` containing a new `Duration` instance if `T` was converted.
    ///
    /// # Errors
    /// If `T` overflows, return `TryFrom<T>::Error`.
    fn to_duration(self) -> Result<Duration, <Self::Intermediary as TryFrom<T>>::Error>
    where
        Self::Intermediary: TryFrom<T>;
}

impl<T> ToDuration<T> for T {
    type Intermediary = u64;
    #[inline]
    fn to_duration(self) -> Result<Duration, <Self::Intermediary as TryFrom<T>>::Error>
    where
        Self::Intermediary: TryFrom<T>,
    {
        Ok(Duration::from_secs(Self::Intermediary::try_from(self)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_duration() -> Result<(), Box<dyn std::error::Error>> {
        let initial: i64 = 5;
        let solution = initial.to_duration()?;
        let expected = Duration::from_secs(initial as u64);
        if solution == expected {
            Ok(())
        } else {
            Err(Box::from(liberr::Err::new(format!(
                "SOLUTION {solution:?} DOES NOT EQUAL EXPECTED {expected:?}"
            ))))
        }
    }
}
