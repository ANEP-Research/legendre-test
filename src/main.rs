extern crate bitset_core;
extern crate charts;
extern crate rand;

mod legendre;
mod sieve;

use charts::{BarLabelPosition, Chart, ScaleBand, ScaleLinear, VerticalBarView};
use legendre::*;
use rand::prelude::*;
use sieve::prime_sieve;
use std::collections::HashSet;

const N: u32 = 1_000_000;
const MAX_SIZE_P: usize = 15;
const MAX_K: u32 = 10;
const SIZE_P_FILENAME: &'static str = "legendre-test-size-p.svg";
const P_FILENAME: &'static str = "legendre-test-p.svg";

// See 'charts' library example
fn show_charts(data: Vec<(String, f32)>, filename: &str) {
    // Define chart related sizes.
    let width = 800;
    let height = 600;
    let (top, right, bottom, left) = (90, 40, 50, 60);

    let mut domain = Vec::new();
    for (a, x) in data.clone() {
        domain.push(a);
    }
    let x = ScaleBand::new()
        .set_domain(domain)
        .set_range(vec![0, width - left - right]);

    // Create a linear scale that will interpolate values in [0, 100] range to corresponding
    // values in [availableHeight, 0] range (the height of the chart without the margins).
    // The [availableHeight, 0] range is inverted because SVGs coordinate system's origin is
    // in the top left corner, while chart's origin is in bottom left corner, hence we need to
    // invert the range on Y axis for the chart to display as though its origin is at bottom left.
    let y = ScaleLinear::new()
        .set_domain(vec![0.0, 1.0])
        .set_range(vec![height - top - bottom, 0]);

    // Create VerticalBar view that is going to represent the data as vertical bars.
    let view = VerticalBarView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        // .set_label_visibility(false)  // <-- uncomment this line to hide bar value labels
        .set_label_position(BarLabelPosition::Center)
        .load_data(&data)
        .unwrap();

    // Generate and save the chart.
    Chart::new()
        .set_width(width)
        .set_height(height)
        .set_margins(top, right, bottom, left)
        .add_title(String::from("Legendre's symbol square-number test"))
        .add_view(&view)
        .add_axis_bottom(&x)
        .add_axis_left(&y)
        .add_left_axis_label("Probability")
        .add_bottom_axis_label("Primes")
        .save(filename)
        .unwrap();
}

fn size_p_generate(n: u32, max_size_p: usize, max_k: u32) {
    let sqrt_n = (n as f32).sqrt() as u32;
    let mut is_sqnum: HashSet<u32> = HashSet::new();
    let mut rng = rand::thread_rng();
    for i in 1..=sqrt_n {
        is_sqnum.insert(i * i);
    }
    let primes = prime_sieve(n);
    let mut ans: Vec<(String, f32)> = Vec::new();
    for num in 1..=max_size_p {
        let mut sum_of_prob = 0.0;
        for _k in 0..max_k {
            let mut s_cnt = 0;
            let test_primes: Vec<u32> = primes.choose_multiple(&mut rng, num).cloned().collect();
            for a in 1..=n {
                let mut res = true;
                for p in test_primes.clone() {
                    if a % p != 0 && !legendre(p as u64, a as u64) {
                        res &= false;
                    }
                }
                if res && is_sqnum.get(&a).is_none() {
                    s_cnt += 1;
                }
            }
            let probability = 1.0 - ((s_cnt as f32) / ((n - sqrt_n) as f32));
            sum_of_prob += probability;
        }
        let avg = sum_of_prob / (max_k as f32);
        ans.push((format!("{}", num), avg));
        println!(
            "Numbers in [1,{}] passed test in the {:.8} probability",
            n, avg
        );
    }
    show_charts(ans, SIZE_P_FILENAME);
}

fn array_to_string(arr: Vec<u32>) -> String {
    let mut res = String::new();
    res.push_str("[");
    for i in 0..arr.len() {
        res.push_str(&format!("{}", arr[i]));
        if i != arr.len() - 1 {
            res.push_str(",");
        }
    }
    res.push_str("]");
    res
}

fn p_generate(n: u32, size_p: usize, max_k: u32) {
    let sqrt_n = (n as f32).sqrt() as u32;
    let mut is_sqnum: HashSet<u32> = HashSet::new();
    let mut rng = rand::thread_rng();
    for i in 1..=sqrt_n {
        is_sqnum.insert(i * i);
    }
    let primes = prime_sieve(n);
    let mut ans: Vec<(String, f32)> = Vec::new();
    for _k in 0..max_k {
        let mut s_cnt = 0;
        let test_primes: Vec<u32> = primes.choose_multiple(&mut rng, size_p).cloned().collect();
        for a in 1..=n {
            let mut res = true;
            for p in test_primes.clone() {
                if a % p != 0 && !legendre(p as u64, a as u64) {
                    res &= false;
                }
            }
            if res && is_sqnum.get(&a).is_none() {
                s_cnt += 1;
            }
        }
        let probability = 1.0 - ((s_cnt as f32) / ((n - sqrt_n) as f32));
        ans.push((array_to_string(test_primes.clone()), probability));
    }
    dbg!(ans.clone());
    show_charts(ans, P_FILENAME)
}

fn main() {
    //size_p_generate(N, MAX_SIZE_P, MAX_K);
    p_generate(N, 5, MAX_K);
}
