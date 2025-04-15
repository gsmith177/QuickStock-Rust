
use chrono::{Local, Duration};

fn today() {
    let today = Local::now();
    let ten_days_ago = today - Duration::days(10);
    println!("Today: {}", today);
    println!("10 days ago: {}", ten_days_ago);
}

fn main() {
    today();
}
