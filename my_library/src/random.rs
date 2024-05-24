use rand::prelude::*;
use std::ops::Range;
pub struct RandomNumberGenerator{
    rng: StdRng,
}

impl RandomNumberGenerator{
    pub fn new() -> RandomNumberGenerator {
        RandomNumberGenerator {
            rng: StdRng::from_entropy(),
        }
    }
    pub fn seeded(seed: u64) -> RandomNumberGenerator {
        Self { rng: StdRng::seed_from_u64(seed) }
    }
    
    pub fn random_in_range(&mut self, range: Range<u32>) -> u32 {
        self.rng.gen_range(range)
    }
    pub fn range(&mut self, range: Range<u32>) -> u32 {
        self.rng.gen_range(range)
    }
}

impl Default for RandomNumberGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_range_bounds() {
        let mut rng = RandomNumberGenerator::new();
        let range = 0..10;
        for _ in 0..1000 {
            let n = rng.range(range.clone());
            assert!(n >= 0 && n < 10);
        }
    }

    #[test]
    fn test_reproducibility() {
        let mut rng = (
            RandomNumberGenerator::seeded(1),
            RandomNumberGenerator::seeded(1),
        );
        (0..1000).for_each(|_| {
            assert_eq!(
                rng.0.range(u32::MIN..u32::MAX), 
                rng.1.range(u32::MIN..u32::MAX));
        });
    }
}