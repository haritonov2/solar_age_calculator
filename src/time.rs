use std::collections::HashMap;

#[derive(Debug)]
pub struct Duration<'a> {
    pub earth_years_old_seconds: f64,
    pub earth_year_seconds: f64,
    ratios: HashMap<&'a str, f64>
}

impl<'a> From<u64> for Duration<'a> {
    fn from(s: u64) -> Self {
        let ratios = HashMap::from([
            ("mercury_ratio", 0.2_408_467),
            ("venus_ratio", 0.61_519_726),
            ("earth_ratio", 1.0),
            ("mars_ratio", 1.8_808_158),
            ("jupiter_ratio", 11.862_615),
            ("saturn_ratio", 29.447_498),
            ("uranus_ratio", 84.016_846),
            ("neptune_ratio", 164.79_132),
        ]);

        Self {
            earth_years_old_seconds: s as f64,
            earth_year_seconds: 31_557_600.00,
            ratios,
        }
    }
}

impl<'a> Duration<'a> {
    pub fn ratio_for(&self, planet_name: &str) -> &f64 {
        let key = format!("{}_ratio", planet_name.to_lowercase());

        self.ratios.get(key.as_str()).unwrap()
    }
}
