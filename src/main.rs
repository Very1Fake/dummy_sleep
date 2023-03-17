use std::{env::args, thread::sleep, time::Duration};

fn main() {
    let dur = args()
        .skip(1)
        .next()
        .map_or(Some(Duration::from_secs(32)), |val| {
            Some(Duration::from_secs(val.parse().unwrap()))
        })
        .unwrap();

    println!("Dummy is asleep for {dur:?}");

    sleep(dur);

    println!("Dummy is awake");
}
