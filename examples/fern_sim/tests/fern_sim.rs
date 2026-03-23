use fern_sim::{self, Fern};

#[test]
fn test_run() {
    let mut fern = Fern {
        size: 0.0,
        growth_rate: 0.0,
    };
    fern_sim::run_simulation(&mut fern, 10);
}
