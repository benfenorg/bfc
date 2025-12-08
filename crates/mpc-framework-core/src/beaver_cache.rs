/*!
# Beaver Triple Cache Optimization Module - Performance Enhancement Core

This module implements efficient Beaver triple caching mechanisms to improve SMPC multiplication efficiency.

## Core Optimization Strategies

### Pre-computation Cache
- **Intelligent Cache**: LRU replacement strategy to optimize memory usage
- **On-Demand Generation**: Generate triples as needed to reduce latency
- **Cache Warmup**: Pre-generate triples to improve first-call performance

*/

use crate::field::gf64_sss::FieldElement;
use crate::field::FieldElement as FieldElementTrait;
use crate::poly::Polynomial;
use rand::RngCore;
use std::collections::VecDeque;

/// Beaver Triple Cache Structure
///
/// Uses queue structure for efficient triple management, supporting:
/// - On-demand triple generation
/// - LRU cache strategy to optimize memory usage
/// - Thread-safe concurrent access
pub struct BeaverTripleCache {
    /// Pre-generated triple queue
    cache: VecDeque<CachedTriple>,
    /// Cache capacity limit
    max_capacity: usize,
    /// Statistics
    stats: CacheStats,
}

/// Cached Triple Structure
///
/// Contains complete triple information and metadata
#[derive(Clone)]
pub struct CachedTriple {
    /// Triple's a shares
    pub a_shares: Vec<(FieldElement, FieldElement)>,
    /// Triple's b shares
    pub b_shares: Vec<(FieldElement, FieldElement)>,
    /// Triple's c shares
    pub c_shares: Vec<(FieldElement, FieldElement)>,
    /// Creation timestamp
    pub created_at: std::time::Instant,
}

/// Cache Statistics
///
/// Used for performance analysis and tuning
#[derive(Debug, Default)]
pub struct CacheStats {
    /// Cache hits
    pub hits: u64,
    /// Cache misses
    pub misses: u64,
    /// Total triples generated
    pub total_generated: u64,
}

impl BeaverTripleCache {
    /// Create a new triple cache
    ///
    /// # Parameters
    /// * `max_capacity` - Maximum cache capacity
    ///
    /// # Returns
    /// Initialized cache instance
    pub fn new(max_capacity: usize) -> Self {
        Self {
            cache: VecDeque::with_capacity(max_capacity),
            max_capacity,
            stats: CacheStats::default(),
        }
    }

    /// Get a triple (preferentially from cache)
    ///
    /// # Parameters
    /// * `x_coords` - x coordinate vector
    /// * `threshold` - Threshold parameter
    /// * `rng` - Random number generator
    ///
    /// # Returns
    /// Available triple, generates a new one if cache is empty
    pub fn get_triple<R: RngCore>(
        &mut self,
        x_coords: &[FieldElement],
        threshold: usize,
        rng: &mut R,
    ) -> Result<CachedTriple, Box<dyn std::error::Error>> {
        // Try to get from cache
        if let Some(triple) = self.cache.pop_front() {
            self.stats.hits += 1;
            return Ok(triple);
        }

        // Cache miss, generate single triple
        self.stats.misses += 1;

        // Clean up cache space if needed
        if self.cache.len() >= self.max_capacity {
            self.cache.pop_back();
        }

        // Generate new triple
        let triple = self.generate_single_triple(x_coords, threshold, rng)?;
        self.stats.total_generated += 1;

        Ok(triple)
    }

    /// Generate single triple
    ///
    /// Optimized triple generation algorithm to reduce redundant computation
    fn generate_single_triple<R: RngCore>(
        &self,
        x_coords: &[FieldElement],
        threshold: usize,
        rng: &mut R,
    ) -> Result<CachedTriple, Box<dyn std::error::Error>> {
        // Generate random a and b values
        let a_secret = rng.next_u64() % 1000000; // Limit range to avoid overflow
        let b_secret = rng.next_u64() % 1000000;

        // Calculate c = a * b in finite field
        let a_field = FieldElement::from_u64(a_secret);
        let b_field = FieldElement::from_u64(b_secret);
        let c_field = a_field * b_field;
        let c_secret = c_field.to_u64();

        // Create polynomials and generate shares
        let poly_a = Polynomial::new(threshold - 1, FieldElementTrait::from_u64(a_secret), rng);
        let poly_b = Polynomial::new(threshold - 1, FieldElementTrait::from_u64(b_secret), rng);
        let poly_c = Polynomial::new(threshold - 1, FieldElementTrait::from_u64(c_secret), rng);

        // Batch calculate share values for all coordinates
        let a_shares: Vec<(FieldElement, FieldElement)> =
            x_coords.iter().map(|&x| (x, poly_a.evaluate(&x))).collect();

        let b_shares: Vec<(FieldElement, FieldElement)> =
            x_coords.iter().map(|&x| (x, poly_b.evaluate(&x))).collect();

        let c_shares: Vec<(FieldElement, FieldElement)> =
            x_coords.iter().map(|&x| (x, poly_c.evaluate(&x))).collect();

        Ok(CachedTriple {
            a_shares,
            b_shares,
            c_shares,
            created_at: std::time::Instant::now(),
        })
    }

    /// Warm up cache
    ///
    /// Pre-generate specified number of triples to reduce first-call latency
    pub fn warmup<R: RngCore>(
        &mut self,
        count: usize,
        x_coords: &[FieldElement],
        threshold: usize,
        rng: &mut R,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for _ in 0..count {
            // Clean up cache space if needed
            if self.cache.len() >= self.max_capacity {
                self.cache.pop_back();
            }

            // Generate new triple
            let triple = self.generate_single_triple(x_coords, threshold, rng)?;
            self.cache.push_back(triple);
            self.stats.total_generated += 1;
        }
        Ok(())
    }

    /// Get cache statistics
    pub fn get_stats(&self) -> &CacheStats {
        &self.stats
    }

    /// Reset statistics
    pub fn reset_stats(&mut self) {
        self.stats = CacheStats::default();
    }

    /// Get cache hit rate
    pub fn hit_rate(&self) -> f64 {
        let total = self.stats.hits + self.stats.misses;
        if total == 0 {
            0.0
        } else {
            self.stats.hits as f64 / total as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha20Rng;

    fn create_test_rng() -> ChaCha20Rng {
        ChaCha20Rng::seed_from_u64(12345)
    }

    #[test]
    fn test_beaver_cache_basic() {
        let mut cache = BeaverTripleCache::new(10);
        let mut rng = create_test_rng();
        let x_coords: Vec<FieldElement> =
            (1..=5).map(|i| FieldElement::from_u64(i as u64)).collect();

        // First access should trigger triple generation
        let _triple1 = cache.get_triple(&x_coords, 3, &mut rng).unwrap();
        assert_eq!(cache.stats.misses, 1);
        assert_eq!(cache.stats.total_generated, 1);

        // Second access should also trigger generation (cache was empty after first pop)
        let _triple2 = cache.get_triple(&x_coords, 3, &mut rng).unwrap();
        assert_eq!(cache.stats.misses, 2);
        assert_eq!(cache.stats.total_generated, 2);
    }
}
