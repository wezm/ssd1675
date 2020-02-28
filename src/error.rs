use core::fmt::{self, Debug};
use hal::digital::v2::OutputPin;

pub enum Error<GPIO: OutputPin> {
    /// A GPIO could not be set.
    Gpio(GPIO::Error),
}

impl<GPIO: OutputPin> Debug for Error<GPIO>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Gpio(_) => write!(f, "GPIO error"),
        }
    }
}