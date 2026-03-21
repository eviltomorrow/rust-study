use fern_sim::Fern;
use fern_sim::run_simulation;

pub fn display_fern_sim() {
    println!("P{}", r"c");

    let mut fern = Fern {
        size: 1.0,
        growth_rate: 0.001,
    };
    run_simulation(&mut fern, 1000);
    println!("final fern size: {}", fern.size);
}
