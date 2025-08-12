use std::ops::Range;

use crate::spores::Sporangium;

mod plant_structures2;
mod proteins;
mod spores2;

#[doc(alias = "plus")]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 如果两个范围重叠，返回
///
/// ```
/// assert_eq!(book08::overlap(0..7,3..10), true);
/// ```

///
pub fn overlap(r1: Range<usize>, r2: Range<usize>) -> bool {
    r1.start < r1.end && r2.start < r2.end && r1.start < r2.end && r2.start < r1.end
}

pub fn main() {
    println!("Hello, world!");

    let mut x = 1;
    let mut y = 2;
    std::mem::swap(&mut x, &mut y);

    spores::genes(&spores::Spore {});
    let _ = spores::Sporangium {};
    spores::produce_spore(&mut Sporangium {});

    crate::spores2::Spore2 {};

    spores2::produce_spore(&mut spores::Sporangium {});

    plant_structures2::say();

    use plant_structures::{self, roots};
    plant_structures::go();
    roots::hello();

    let r = spores::save_spore(&spores::Spore {});
    match r {
        Ok(data) => {
            println!("{:?}", data)
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }

    proteins::say();
}

mod spores {
    use cells::{Cell, Gene};

    pub struct Spore {}

    pub struct Sporangium {}

    pub fn produce_spore(factory: &mut Sporangium) -> Spore {
        _ = factory;
        recombine(&mut Cell {});
        Spore {}
    }

    fn recombine(_: &mut Cell) {}

    pub(crate) fn genes(_: &Spore) -> Vec<Gene> {
        let v: Vec<Gene> = Vec::new();
        v
    }

    mod cells {
        pub struct Cell {}

        pub struct Gene {}
    }

    use std::io::Result as IOResult;
    pub fn save_spore(_: &Spore) -> IOResult<()> {
        Err(std::io::Error::new(
            std::io::ErrorKind::AddrInUse,
            "already in use!",
        ))
    }
}

pub mod plant_structures {
    #[derive(Debug)]
    pub struct Fern {
        pub roots: roots::RootSet,
    }

    pub const ROOT_TEMPERATURE: f64 = 20.0;

    pub mod roots {
        use super::Fern;
        #[derive(Debug)]
        pub struct RootSet {}
        pub mod products {
            #[derive(Debug)]
            pub(in crate::plant_structures) struct Cytokinin {}
        }
        pub fn hello() {
            let s = Fern { roots: RootSet {} };
            println!("{:?}", s);
        }
    }

    use roots::products::Cytokinin;
    fn new() -> Cytokinin {
        Cytokinin {}
    }

    pub fn go() {
        let c = new();
        println!("{:?}", c);
    }

    pub mod stems {}

    pub mod leaves {}
}
