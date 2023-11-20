mod line;
mod mock_org_chart_data;
mod org_chart;
mod utils;

fn main() {
    let test = mock_org_chart_data::mock_org_chart_data(10, 4, false);
    println!("{:?}", test);
    println!("Hello, world!");
}
