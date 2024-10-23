fn main() {
    println!("Hello, world!");

    let p = pirate_share(100, 10);
    println!("{}", p);

    let p = pirate_share(100, 0);
    println!("{}", p);
}

fn pirate_share(total: u64, crew_size: usize) -> u64 {
    let half = total / 2;
    half / crew_size as u64
}
