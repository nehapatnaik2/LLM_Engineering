use std::time::Instant;

#[inline(always)]
struct Lcg {
    value: u32,
}

impl Lcg {
    #[inline(always)]
    fn new(seed: u32) -> Self {
        Self { value: seed }
    }

    #[inline(always)]
    fn next(&mut self) -> u32 {
        const A: u32 = 1_664_525;
        const C: u32 = 1_013_904_223;
        self.value = self.value.wrapping_mul(A).wrapping_add(C);
        self.value
    }
}

#[inline(always)]
fn max_subarray_sum(n: usize, seed: u32, min_val: i128, max_val: i128) -> i128 {
    let mut lcg = Lcg::new(seed);
    let range_i128 = max_val - min_val + 1;
    let range_u128 = range_i128 as u128;

    // Kadane's algorithm (O(n)) with streaming random generation
    let mut max_so_far: i128 = i128::MIN;
    let mut max_ending_here: i128 = 0;

    if range_u128 <= (u32::MAX as u128) && range_u128 != 0 {
        let range_u32 = range_u128 as u32;
        for _ in 0..n {
            let r = lcg.next();
            let val = (r % range_u32) as i128 + min_val;
            if max_ending_here <= 0 {
                max_ending_here = val;
            } else {
                max_ending_here += val;
            }
            if max_ending_here > max_so_far {
                max_so_far = max_ending_here;
            }
        }
    } else if range_u128 != 0 {
        for _ in 0..n {
            let r = lcg.next() as u128;
            let val = (r % range_u128) as i128 + min_val;
            if max_ending_here <= 0 {
                max_ending_here = val;
            } else {
                max_ending_here += val;
            }
            if max_ending_here > max_so_far {
                max_so_far = max_ending_here;
            }
        }
    } else {
        // Degenerate case: range == 0 (shouldn't occur with valid inputs)
        let val = min_val;
        for _ in 0..n {
            if max_ending_here <= 0 {
                max_ending_here = val;
            } else {
                max_ending_here += val;
            }
            if max_ending_here > max_so_far {
                max_so_far = max_ending_here;
            }
        }
    }

    max_so_far
}

#[inline(always)]
fn total_max_subarray_sum(n: usize, initial_seed: u32, min_val: i128, max_val: i128) -> i128 {
    let mut total_sum: i128 = 0;
    let mut lcg_gen = Lcg::new(initial_seed);
    for _ in 0..20 {
        let seed = lcg_gen.next();
        total_sum += max_subarray_sum(n, seed, min_val, max_val);
    }
    total_sum
}

fn main() {
    // Parameters
    let n: usize = 10000;
    let initial_seed: u32 = 42;
    let min_val: i128 = -10;
    let max_val: i128 = 10;

    // Timing the function
    let start_time = Instant::now();
    let result = total_max_subarray_sum(n, initial_seed, min_val, max_val);
    let elapsed = start_time.elapsed().as_secs_f64();

    println!("Total Maximum Subarray Sum (20 runs): {}", result);
    println!("Execution Time: {:.6} seconds", elapsed);
}