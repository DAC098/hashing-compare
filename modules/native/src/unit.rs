use std::fmt::{Display, Formatter, Result as FmtResult};

pub const KIBI: u64 = 1024;
pub const KIBI_STR: &str = "Ki";

pub const MEBI: u64 = KIBI * 1024;
pub const MEBI_STR: &str = "Mi";

pub const GIBI: u64 = MEBI * 1024;
pub const GIBI_STR: &str = "Gi";

pub const TEBI: u64 = GIBI * 1024;
pub const TEBI_STR: &str = "Ti";

pub const PEBI: u64 = TEBI * 1024;
pub const PEBI_STR: &str = "Pi";

const UNIT_VALUES: [(u64, &str); 6] = [
    (1, ""),
    (KIBI, KIBI_STR),
    (MEBI, MEBI_STR),
    (GIBI, GIBI_STR),
    (TEBI, TEBI_STR),
    (PEBI, PEBI_STR),
];

pub struct FmtUnit<'a>(u64, &'a str);

impl<'a> FmtUnit<'a> {
    pub fn new(value: u64, prefix: &'a str) -> Self {
        Self(value, prefix)
    }
}

impl<'a> Display for FmtUnit<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut index = 0;
        let len = UNIT_VALUES.len();

        while index < len {
            let calc = self.0 / UNIT_VALUES[index].0;

            if calc < KIBI {
                return write!(f, "{calc}{}{}", UNIT_VALUES[index].1, self.1);
            } else {
                index += 1;
            }
        }

        write!(
            f,
            "{}{}{}",
            self.0 / UNIT_VALUES[len - 1].0,
            UNIT_VALUES[len - 1].1,
            self.1
        )
    }
}
