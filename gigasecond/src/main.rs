use chrono::{DateTime, Utc, Duration};

fn main() {
    let utc: DateTime<Utc> = Utc::now();
    let future = after(utc);
    println!("{}", future);
}

fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    start + Duration::seconds(1_000_000_000)
}