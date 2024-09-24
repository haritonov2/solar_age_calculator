use crate::Duration;

pub trait Planet {
    fn title() -> &'static str;

    fn years_during(d: &Duration) -> f64 {
        let planet_name = Self::title();
        let earth_ratio = d.ratio_for(planet_name);

        (d.earth_years_old_seconds / d.earth_year_seconds) / earth_ratio
    }
}

macro_rules! struct_planets {
    (names $($p:ident), +) => {
        $(
            pub struct $p;

            impl Planet for $p {
                fn title() -> &'static str {
                    stringify!($p)
                }
            }
        )*
    }
}

struct_planets!(names Mercury, Venus, Earth, Mars, Jupiter, Saturn, Uranus, Neptune);
