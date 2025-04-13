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
pub struct Dpi {
    x: u8,
    y: u8,
    dpi_ex: u8,
}

impl std::fmt::Display for Dpi {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "X: {}, Y: {}", self.x_dpi(), self.y_dpi())
    }
}

impl Default for Dpi {
    fn default() -> Self {
        Self::new(1600, 1600)
    }
}

impl Dpi {
    fn new(x: u16, y: u16) -> Dpi {
        let x_steps = (x / DPI_STEP) - 1;
        let y_steps = (y / DPI_STEP) - 1;

        let x: u8 = u8::MAX & x_steps as u8;
        let y: u8 = u8::MAX & y_steps as u8;

        let x_high: u8 = ((x_steps >> 8) & 0x0F) as u8;
        let y_high: u8 = ((y_steps >> 8) & 0x0F) as u8;

        let dpi_ex = (x_high << 6) | (y_high << 2);

        Self { x, y, dpi_ex }
    }

    fn x_dpi(&self) -> u16 {
        let x_high: u16 = (self.dpi_ex >> 6) as u16;
        let x_steps: u16 = (x_high << 8) | (self.x as u16);

        (x_steps + 1) * DPI_STEP
    }

    fn y_dpi(&self) -> u16 {
        let y_high: u16 = ((self.dpi_ex >> 2) & 0x0F) as u16;
        let y_steps: u16 = (y_high << 8) | (self.y as u16);

        (y_steps + 1) * DPI_STEP
    }
}

impl TryFrom<&[u8]> for Dpi {
    type Error = Error;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() != 4 {
            return Err(Error::ParseError("DPI2: Invalid data length".to_string()));
        }

        let checksum = u8::MAX
            & 0x55u8
                .wrapping_sub(data[0])
                .wrapping_sub(data[1])
                .wrapping_sub(data[2]);
        if checksum != data[3] {
            return Err(Error::ParseError("DPI2: Invalid checksum".to_string()));
        }

        Ok(Self {
            x: data[0],
            y: data[1],
            dpi_ex: data[2],
        })
    }
}

impl Into<[u8; 4]> for Dpi {
    fn into(self) -> [u8; 4] {
        let checksum = u8::MAX
            & 0x55u8
                .wrapping_sub(self.x)
                .wrapping_sub(self.y)
                .wrapping_sub(self.dpi_ex);

        [self.x, self.y, self.dpi_ex, checksum]
    }
}

impl From<proto::Dpi> for Dpi {
    fn from(proto: proto::Dpi) -> Self {
        Self::new(proto.x as u16, proto.y as u16)
    }
}

impl Into<proto::Dpi> for Dpi {
    fn into(self) -> proto::Dpi {
        proto::Dpi {
            x: self.x_dpi() as i32,
            y: self.y_dpi() as i32,
        }
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
