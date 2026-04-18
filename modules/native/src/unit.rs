use std::fmt::{Display, Formatter, Result as FmtResult};

pub const KIBI: u64 = 1024;
pub const KIBI_F64: f64 = 1024.0;
pub const KIBI_STR: &'static str = "Ki";

pub const MEBI: u64 = KIBI * 1024;
pub const MEBI_F64: f64 = KIBI_F64 * 1024.0;
pub const MEBI_STR: &'static str = "Mi";

pub const GIBI: u64 = MEBI * 1024;
pub const GIBI_F64: f64 = MEBI_F64 * 1024.0;
pub const GIBI_STR: &'static str = "Gi";

pub const TEBI: u64 = GIBI * 1024;
pub const TEBI_F64: f64 = GIBI_F64 * 1024.0;
pub const TEBI_STR: &'static str = "Ti";

pub const PEBI: u64 = TEBI * 1024;
pub const PEBI_F64: f64 = TEBI_F64 * 1024.0;
pub const PEBI_STR: &'static str = "Pi";

const UNIT_VALUES: [(u64, &'static str); 6] = [
    (1, ""),
    (KIBI, KIBI_STR),
    (MEBI, MEBI_STR),
    (GIBI, GIBI_STR),
    (TEBI, TEBI_STR),
    (PEBI, PEBI_STR),
];

const UNIT_VALUES_F64: [(f64, &'static str); 6] = [
    (1.0, ""),
    (KIBI_F64, KIBI_STR),
    (MEBI_F64, MEBI_STR),
    (GIBI_F64, GIBI_STR),
    (TEBI_F64, TEBI_STR),
    (PEBI_F64, PEBI_STR),
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

pub struct FmtUnitF64<'a>(f64, &'a str);

impl<'a> FmtUnitF64<'a> {
    pub fn new(bytes: f64, prefix: &'a str) -> Self {
        Self(bytes, prefix)
    }
}

impl<'a> Display for FmtUnitF64<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut index = 0;
        let len = UNIT_VALUES_F64.len();

        while index < len {
            let calc = self.0 / UNIT_VALUES_F64[index].0;

            if calc < KIBI_F64 {
                return write!(f, "{calc:.2}{}{}", UNIT_VALUES_F64[index].1, self.1);
            } else {
                index += 1;
            }
        }

        write!(
            f,
            "{:.2}{}{}",
            self.0 / UNIT_VALUES_F64[index - 1].0,
            UNIT_VALUES_F64[len - 1].1,
            self.0
        )
    }
}
