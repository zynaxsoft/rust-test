use std::convert::TryFrom;

pub struct ABC {
    pub x: u32,
}

impl TryFrom<ABC> for f64 {
    type Error = &'static str;

    fn try_from(_value: ABC) -> Result<Self, Self::Error> {
        Ok(111.11)
    }
}
