use crate::Duration;

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        todo!("convert a duration ({d:?}) to the number of years on this planet for that duration");
    }
}

macro_rules! struct_planets {
    (names $($p:ident), +) => {
        $(
            pub struct $p;
        )*
    }
}

macro_rules! impl_Planet {
    (for $($p:ty), +) => {
        $(
            impl Planet for $p {}
        )*
    }
}

struct_planets!(names Mercury, Venus, Earth, Mars, Jupiter, Saturn, Uranus, Neptune);
impl_Planet!(for Mercury, Venus, Earth, Mars, Jupiter, Saturn, Uranus, Neptune);
