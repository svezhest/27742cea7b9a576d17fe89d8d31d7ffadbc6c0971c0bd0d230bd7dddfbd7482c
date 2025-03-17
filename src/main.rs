use plotters::prelude::*;
use plotters_bitmap::BitMapBackendError;
use std::result;
use plotters::prelude::DrawingAreaErrorKind;
use plotters::style::full_palette::RED_700;

fn highest_power(i: usize) -> (u32, usize) {
    if i == 0 {
        return (0, 0);
    }
    let m = (usize::BITS - i.leading_zeros() - 1) as u32;
    let power = 1usize << m;
    (m, power)
}

fn x_seq(i: usize) -> u64 {
    let mut sum = 0u64;
    let mut remaining = i;
    let mut multiplier = 1u64;
    
    while remaining > 0 {
        let (m, power) = highest_power(remaining);
        let five_pow_m = 5u64.pow(m);
        sum += multiplier * five_pow_m;
        remaining -= power;
        multiplier *= 4;
    }
    
    sum
}

fn index_to_x(i: usize, n: usize) -> f64 {
    let denominator = 5u64.pow((n - 1) as u32) as f64;
    x_seq(i) as f64 / denominator
}

fn analytical_c(n: usize, x: f64) -> f64 {
    let total_points = 1 + (1 << (n - 1));
    let mut low = 0;
    let mut high = total_points - 1;
    let mut found_index = 0;

    while low <= high {
        let mid = (low + high) / 2;
        let mid_x = index_to_x(mid, n);
        if mid_x <= x {
            found_index = mid;
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    let mirrored_index = total_points - found_index - 1;
    1.0 - index_to_x(mirrored_index, n)
}

fn main() -> result::Result<(), DrawingAreaErrorKind<BitMapBackendError>> {
    let n = 23;
    let total_points = 1 + (1 << (n - 1));
    let points: Vec<(f64, f64)> = (0..total_points)
        .map(|i| {
            let x = index_to_x(i, n);
            let y = analytical_c(n, x);
            (x * 100.0, y * 100.0)
        })
        .collect();

    let root = BitMapBackend::new("graph.png", (4000, 3000)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("20/80", ("sans-serif", 30).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0..100.0, 0.0..100.0)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(points, &RED_700))?;

    Ok(())
}


/*
fn main() {
    let n = 5;
    let total_points = 1 + (1 << (n - 1));

    for i in 0..total_points {
        let x = (i as f64) / (total_points as f64) / (50.0);
        let y = recursive_c(10, x);
        let ya = analytical_c(20, x);
       
        println!("x: {:.5}, y: {:.5}, (formula: {:.5})", x, y, ya);

    }
}
*/
