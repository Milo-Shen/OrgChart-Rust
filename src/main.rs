mod line;
mod mock_org_chart_data;
mod org_chart;

use crate::mock_org_chart_data::range;

fn main() {
    println!("Hello, world!,{}", range(0, 10));
}
