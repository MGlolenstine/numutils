#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Radix {
    Hexadecimal,
    #[default]
    Decimal,
    Octal,
    Binary,
}

impl Radix {
    pub fn get_short_text(&self) -> String {
        match self {
            Radix::Hexadecimal => "Hex".to_string(),
            Radix::Decimal => "Dec".to_string(),
            Radix::Octal => "Oct".to_string(),
            Radix::Binary => "Bin".to_string(),
        }
    }

    pub fn get_text(&self) -> String {
        match self {
            Radix::Hexadecimal => "Hexadecimal".to_string(),
            Radix::Decimal => "Decimal".to_string(),
            Radix::Octal => "Octal".to_string(),
            Radix::Binary => "Binary".to_string(),
        }
    }

    pub fn iter() -> Vec<Radix> {
        vec![
            Radix::Hexadecimal,
            Radix::Decimal,
            Radix::Octal,
            Radix::Binary,
        ]
    }
}

pub fn format_for_radix(radix: Radix, value: u64) -> String {
    match radix {
        Radix::Hexadecimal => format!("{:x}", value),
        Radix::Decimal => format!("{}", value),
        Radix::Octal => format!("{:o}", value),
        Radix::Binary => format!("{:b}", value),
    }
}

impl From<u32> for Radix {
    fn from(value: u32) -> Self {
        match value {
            2 => Radix::Binary,
            8 => Radix::Octal,
            10 => Radix::Decimal,
            16 => Radix::Hexadecimal,
            _ => unimplemented!(),
        }
    }
}

impl From<Radix> for u32 {
    fn from(value: Radix) -> Self {
        match value {
            Radix::Binary => 2,
            Radix::Octal => 8,
            Radix::Decimal => 10,
            Radix::Hexadecimal => 16,
        }
    }
}
