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
