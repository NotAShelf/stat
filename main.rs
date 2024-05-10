use std::error::Error;
use std::fmt::Write;
use std::io::{self};

fn main() -> Result<(), Box<dyn Error>> {
    let mut numbers = Vec::new();
    let mut input = String::new();

    while io::stdin().read_line(&mut input).unwrap() > 0 {
        for word in input.split_whitespace() {
            match word.parse::<f64>() {
                Ok(num) => numbers.push(num),
                Err(_) => eprintln!("error parsing float: {}", word),
            }
        }
        input.clear();
    }

    println!("n {}", numbers.len());
    println!("min {:.4}", min(&numbers));
    println!("max {:.4}", max(&numbers));
    println!("sum {:.4}", sum(&numbers));
    println!("mean {:.4}", mean(&numbers));
    println!("median {:.4}", median(&numbers));
    println!("modes {:?}", modes(&numbers));
    println!("stdev {:.4}", stdev(&numbers));
    println!("{}", histogram(&numbers));

    Ok(())
}

fn min(a: &[f64]) -> f64 {
    a.iter().cloned().fold(f64::INFINITY, |a, b| a.min(b))
}

fn max(a: &[f64]) -> f64 {
    a.iter().cloned().fold(f64::NEG_INFINITY, |a, b| a.max(b))
}

fn sum(a: &[f64]) -> f64 {
    a.iter().sum()
}

fn mean(a: &[f64]) -> f64 {
    sum(a) / (a.len() as f64)
}

fn median(a: &[f64]) -> f64 {
    let mut a = a.to_vec();
    a.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mid = a.len() / 2;
    if a.len() % 2 == 0 {
        (a[mid - 1] + a[mid]) / 2.0
    } else {
        a[mid]
    }
}

fn modes(a: &[f64]) -> Vec<f64> {
    let mut counts = Vec::new();
    for &x in a {
        let mut found = false;
        for (value, count) in &mut counts {
            if *value == x {
                *count += 1;
                found = true;
                break;
            }
        }
        if !found {
            counts.push((x, 1));
        }
    }

    counts.sort_by(|a, b| b.1.cmp(&a.1)); // sort by count in descending order

    let highest_frequency = counts[0].1;
    let mut modes = Vec::new();
    for (value, count) in counts {
        if count == highest_frequency {
            modes.push(value);
        } else {
            break; // stop once we've found all modes with the highest frequency
        }
    }

    if highest_frequency == 1 || modes.len() == a.len() {
        modes.clear();
    }

    modes.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    modes
}

fn stdev(a: &[f64]) -> f64 {
    let mean = mean(a);
    let variance = a.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / (a.len() - 1) as f64;
    variance.sqrt()
}

fn histogram(a: &[f64]) -> String {
    let mut buckets = [0; 10];
    let min = min(a);
    let max = max(a);
    let delta = (max - min) / 10.0;

    for &v in a {
        let index = ((v - min) / delta) as usize;
        if index < 10 {
            buckets[index] += 1;
        }
    }

    let total = a.len();
    let mut output = String::new();
    for (i, &count) in buckets.iter().enumerate() {
        let range_start = min + (i as f64 * delta);
        let range_end = min + ((i + 1) as f64 * delta);
        let percentage = (count as f64 / total as f64) * 100.0;
        writeln!(
            output,
            "{:.4}-{:.4}\t{}\t{:.*}",
            range_start,
            range_end,
            count,
            2,
            "*".repeat((percentage * 2.0) as usize)
        )
        .unwrap();
    }

    output
}
