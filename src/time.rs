#[derive(Debug)]
pub struct Duration { seconds: u64 }

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { seconds: s }
    }
}
