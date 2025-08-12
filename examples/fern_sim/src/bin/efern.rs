use ::fern_sim;
use fern_sim::{Fern, run_simulation};

/// 这是 efern main 方法
fn main() {
    let mut fern = Fern {
        size: 1.0,
        growth_rate: 0.001,
    };
    run_simulation(&mut fern, 1000);
    println!("finale fern size: {}", fern.size);

    use crate::run_simulation;
    run_simulation(&mut fern, 10);
    println!("finale fern size: {}", fern.size);

    fern_sim::run_simulation(&mut fern, 1);
    println!("finale fern size: {}", fern.size);
}
