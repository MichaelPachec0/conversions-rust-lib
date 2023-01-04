/*
 *    Copyright (c) 2023  Michael Pacheco
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

use std::time::Duration;

pub trait ToDuration<T>
    where
        u64: TryFrom<T>,
{
    fn to_duration(self) -> Result<Duration, <u64 as TryFrom<T>>::Error>;
}

impl<T> ToDuration<T> for T
    where
        u64: TryFrom<T>,
{
    fn to_duration(self) -> Result<Duration, <u64 as TryFrom<T>>::Error> {
        Ok(Duration::from_secs(u64::try_from(self)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_duration() -> Result<(), Box<dyn std::error::Error>> {
        let initial = 5;
        let solution = initial.to_duration()?;
        let expected = Duration::from_secs(5);
        assert_eq!(solution, expected, "SOLUTION {solution:?} DOES NOT EQUAL EXPECTED {expected:?}");
        Ok(())
    }
}
