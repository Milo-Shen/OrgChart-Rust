use std::time::{Duration, Instant};

mod line;
mod mock_org_chart_data;
mod org_chart;
mod utils;

fn main() {
    let start_time = Instant::now();
    let test = mock_org_chart_data::mock_org_chart_data(300000, 20, false);
    let duration = start_time.elapsed();
    println!("{:?}", duration);
    println!("Hello, world!");
}
