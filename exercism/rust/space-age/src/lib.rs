/// Earth year duration
pub struct Duration(f64);

/// Earth year duration from seconds
impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self(s as f64 / (365.25 * 24.0 * 60.0 * 60.0))
    }
}

pub trait Planet {
    /// Planet orbit periods per earth orbit
    const EARTH_PERIODS: f64;

    /// Planet years during earth year duration
    fn years_during(d: &Duration) -> f64 {
        d.0 / Self::EARTH_PERIODS
    }
}

macro_rules! planets {
    (@planet $name:tt, $earth_periods:tt) => {
        pub struct $name;

        impl Planet for $name {
            const EARTH_PERIODS: f64 = $earth_periods;
        }
    };

    ($(($name:tt, $earth_periods:tt),)*) => {
        $(planets!(@planet $name, $earth_periods);)*
    };
}

planets!(
    (Mercury, 0.2408467),
    (Venus, 0.61519726),
    (Earth, 1.0),
    (Mars, 1.8808158),
    (Jupiter, 11.862615),
    (Saturn, 29.447498),
    (Uranus, 84.016846),
    (Neptune, 164.79132),
);
