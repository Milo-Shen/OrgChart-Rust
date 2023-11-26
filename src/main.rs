use std::time::Instant;

mod line;
mod mock_org_chart_data;
mod org_chart;
mod utils;

fn main() {
    let start_time = Instant::now();
    let mock_data = mock_org_chart_data::mock_org_chart_data(300000, 20, false);
    let duration = start_time.elapsed();
    println!("mock data: {:?}", duration);
    let start_time = Instant::now();
    let mut chart = org_chart::OrgChart::new(true, 200.0, 100.0, 100.0, 50.0, 10.0, 41.0, 2.0, 2);
    let duration = start_time.elapsed();
    println!("init org chart: {:?}", duration);
    let start_time = Instant::now();
    chart.initialization(mock_data);
    let duration = start_time.elapsed();
    println!("build org chart time {:?}", duration);
    let data = chart.get_render_data();
    // println!("{:#?}", data.0);
    println!("Hello, world!");
}
