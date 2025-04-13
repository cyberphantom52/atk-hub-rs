use libatk_rs::types::Error;

use crate::proto;
static DPI_STEP: u16 = 50;

pub struct Milliseconds;
pub struct Seconds;
pub struct Decaseconds;

pub trait TimeUnit {
    const FACTOR: u32;
    const LABEL: &'static str;
}

impl TimeUnit for Milliseconds {
    const FACTOR: u32 = 1;
    const LABEL: &'static str = "ms";
}

impl TimeUnit for Seconds {
    const FACTOR: u32 = 1000;
    const LABEL: &'static str = "s";
}

impl TimeUnit for Decaseconds {
    const FACTOR: u32 = 10000;
    const LABEL: &'static str = "ds";
}

pub struct Duration<T: TimeUnit> {
    value: u32,
    marker: std::marker::PhantomData<T>,
}

impl<T: TimeUnit> Clone for Duration<T> {
    fn clone(&self) -> Self {
        Duration {
            value: self.value,
            marker: std::marker::PhantomData,
        }
    }
}

impl<T: TimeUnit> Copy for Duration<T> {}

impl<T: TimeUnit> Default for Duration<T> {
    fn default() -> Self {
        Duration {
            value: 0,
            marker: std::marker::PhantomData,
        }
    }
}

impl<T: TimeUnit> std::fmt::Debug for Duration<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<T: TimeUnit> std::fmt::Display for Duration<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.as_unit(), T::LABEL)
    }
}

impl<T: TimeUnit> Duration<T> {
    pub fn new(value: u32) -> Self {
        Duration {
            value: value.saturating_mul(T::FACTOR),
            marker: std::marker::PhantomData,
        }
    }

    pub fn as_unit(&self) -> u32 {
        self.value / T::FACTOR
    }

    pub fn convert<U: TimeUnit>(self) -> Duration<U> {
        Duration {
            value: self.value,
            marker: std::marker::PhantomData,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Dpi(u16);

impl std::fmt::Display for Dpi {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.dpi())
    }
}

impl Default for Dpi {
    fn default() -> Self {
        Dpi(1600)
    }
}

impl From<u16> for Dpi {
    fn from(value: u16) -> Self {
        Dpi(value)
    }
}

impl TryFrom<&[u8]> for Dpi {
    type Error = Error;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() != 4 {
            return Err(Error::ParseError(format!(
                "DPI: Invalid data length: expected 4 got {}",
                data.len()
            )));
        }

        let checksum = 0xff
            & 0x55u8
                .wrapping_sub(data[0])
                .wrapping_sub(data[1])
                .wrapping_sub(data[2]);
        if checksum != data[3] {
            return Err(Error::ParseError("DPI: Invalid checksum".to_string()));
        }

        let x_dpi = data[0];
        let dpi_ex = data[2];

        Ok(Self(
            (((u8::MAX as u16 + 1) * dpi_ex as u16 / 0x44) + (x_dpi as u16 + 1)) * DPI_STEP,
        ))
    }
}

impl Into<[u8; 4]> for Dpi {
    fn into(self) -> [u8; 4] {
        let steps = (self.dpi() / DPI_STEP) - 1;

        let x_dpi = u8::MAX & steps as u8;
        let y_dpi = x_dpi;
        let dpi_ex = (0x44 * steps / (u8::MAX as u16 + 1)) as u8;
        let checksum = u8::MAX
            & 0x55u8
                .wrapping_sub(x_dpi)
                .wrapping_sub(y_dpi)
                .wrapping_sub(dpi_ex);

        [x_dpi, y_dpi, dpi_ex, checksum]
    }
}

impl Into<proto::Dpi> for Dpi {
    fn into(self) -> proto::Dpi {
        proto::Dpi {
            x: self.0 as i32,
            y: self.0 as i32,
        }
    }
}

impl From<proto::Dpi> for Dpi {
    fn from(proto: proto::Dpi) -> Self {
        Dpi(proto.x as u16)
    }
}

impl Dpi {
    pub fn dpi(&self) -> u16 {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Into<proto::Color> for Color {
    fn into(self) -> proto::Color {
        proto::Color {
            red: self.red as i32,
            green: self.green as i32,
            blue: self.blue as i32,
        }
    }
}

impl From<proto::Color> for Color {
    fn from(proto: proto::Color) -> Self {
        Color {
            red: proto.red as u8,
            green: proto.green as u8,
            blue: proto.blue as u8,
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color {
            red: 0xFF,
            green: 0xFF,
            blue: 0xFF,
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.red, self.green, self.blue)
    }
}

impl TryFrom<&[u8]> for Color {
    type Error = Error;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() != 4 {
            return Err(Error::ParseError("Color: Invalid data length".to_string()));
        }

        let checksum = 0xff
            & 0x55u8
                .wrapping_sub(data[0])
                .wrapping_sub(data[1])
                .wrapping_sub(data[2]);

        if checksum != data[3] {
            return Err(Error::ParseError("Color: Invalid checksum".to_string()));
        }

        Ok(Self {
            red: data[0],
            green: data[1],
            blue: data[2],
        })
    }
}

impl Into<[u8; 4]> for Color {
    fn into(self) -> [u8; 4] {
        let checksum = 0xff
            & 0x55u8
                .wrapping_sub(self.red)
                .wrapping_sub(self.green)
                .wrapping_sub(self.blue);

        [self.red, self.green, self.blue, checksum]
    }
}
