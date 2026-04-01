use std::cell::{Cell, RefCell};

fn main() {
    println!("Hello, world!");

    let width = 1024;
    let height = 576;

    let image = GrayscaleMap {
        pixels: vec![0; width * height],
        size: (width, height),
    };
    println!("{:?}", image.size);

    let g = new_grayscale_map(vec![0; width * height], (width, height));
    println!("{:?}", g.size);

    let hokey = Broom {
        name: "Hokey".to_string(),
        height: 60,
        health: 100,
        position: (100.0, 200f32, 0.0),
        intent: BroomIntent::DumpWater,
    };

    let (hokey1, hokey2) = chop(hokey);
    println!("{:?}, {:?}", hokey1, hokey2);

    let image_bounds = Bounds(1024, 768);
    println!("{:?}", image_bounds);

    let o = OneSuch;
    println!("{:?}", o);

    let mut q = Queue {
        older: Vec::<char>::new(),
        younger: Vec::<char>::new(),
    };
    q.push('C');
    q.push('D');

    if let Some(data) = q.pop() {
        println!("{}", data);
    }

    match q.pop() {
        Some(d) => {
            println!("{}", d);
        }
        None => {
            println!("None");
        }
    }
    let d = if let Some(d) = q.pop() {
        String::from(d)
    } else {
        String::from("None")
    };

    println!("{:?}", q);
    println!("{:?}, {}", d, q.is_empty());

    let (older, younger) = q.split();
    println!("{:?}, {:?}", older, younger);

    let mut bq = Box::new(Queue::new());
    bq.push('C');
    println!("{:?}", bq);

    bq.append();

    let c = Vector2::ZERO.scaled_by(10.0);
    println!("{:?}", c);

    let d = Vector2::UNIT.scaled_by(20.0);
    println!("{:?}", d);

    let mut q = QueueGeneric::<String>::new();
    q.push(String::from("Hello".to_string()));
    println!("{:?}, {:?}", q.is_empty(), q);

    let mut q = QueueGeneric::new();
    q.push(10.0f64);
    q.push(20.0f64);
    println!("{:?}", q.sum());

    let c = vec![1, 2, 3, 4, 5, 6, 7];
    let e = find_extrema(&c);
    println!("{:?}", e);

    let c = find_extrema_1(&c);
    println!("{:?}", c);

    let coefficients = [1f64; 5];
    let p = Polynomial::new(coefficients);
    let sum = p.eval(10.0);
    println!("{}", sum);

    let s = SpiderBoot1 {
        hardware_err_count: Cell::new(0),
    };
    s.hardware_err_count.set(10);

    let v = s.hardware_err_count.get();
    println!("{:?}, {}", s, v);

    let s = SpiderBoot2 {
        hardware_err_count: RefCell::new(10),
    };
    let v = s.hardware_err_count.borrow();
    println!("{:?}", v);

    let ref_cell = RefCell::new("hello".to_string());

    let r = ref_cell.borrow();
    let count = r.len();
    println!("{:?}", count);
    drop(r);

    let mut w = ref_cell.borrow_mut();
    w.push_str(" world!");
    println!("{:?}", w);

    let mut account = Account {
        name: String::from("Hello"),
        age: 10,
    };
    account.name.push_str("world");
    account.age = 20;
}

#[derive(Debug)]
#[allow(dead_code)]
struct Bounds(usize, usize);

pub struct Bounds2(pub usize, pub usize);

pub struct Ascii(pub Vec<u8>);

#[derive(Debug)]
pub struct OneSuch;

#[derive(Debug)]
#[allow(dead_code)]
struct Broom {
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent,
}

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
enum BroomIntent {
    FetchWater,
    DumpWater,
}

#[allow(dead_code)]
#[derive(Debug)]
struct GrayscaleMap {
    pixels: Vec<u8>,
    size: (usize, usize),
}

fn new_grayscale_map(pixels: Vec<u8>, size: (usize, usize)) -> GrayscaleMap {
    GrayscaleMap { pixels, size }
}

fn chop(b: Broom) -> (Broom, Broom) {
    let mut broom1 = Broom {
        height: b.height / 2,
        ..b
    };

    let mut broom2 = Broom {
        name: broom1.name.clone(),
        ..broom1
    };

    broom1.name.push_str(" I");
    broom2.name.push_str(" II");

    (broom1, broom2)
}

#[derive(Debug)]
struct Queue {
    older: Vec<char>,
    younger: Vec<char>,
}

impl Queue {
    fn new() -> Queue {
        Queue {
            older: Vec::<char>::with_capacity(20),
            younger: Vec::<char>::with_capacity(20),
        }
    }

    fn push(&mut self, c: char) {
        self.younger.push(c);
    }

    fn pop(&mut self) -> Option<char> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }
            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }
        self.older.pop()
    }

    fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    fn split(self) -> (Vec<char>, Vec<char>) {
        (self.older, self.younger)
    }

    fn append(self: Box<Self>) {}
}

struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector2 {
    const ZERO: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    const UNIT: Vector2 = Vector2 { x: 1.0, y: 0.0 };

    fn scaled_by(&self, val: f32) -> f32 {
        self.x + self.y + val
    }
}

#[derive(Debug)]
struct QueueGeneric<T> {
    older: Vec<T>,
    younger: Vec<T>,
}

impl<T> QueueGeneric<T> {
    pub fn new() -> QueueGeneric<T> {
        Self::new_inner()
    }

    fn new_inner() -> QueueGeneric<T> {
        Self {
            older: Vec::new(),
            younger: Vec::new(),
        }
    }

    pub fn push(&mut self, t: T) {
        self.younger.push(t);
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }
}

impl QueueGeneric<f64> {
    fn sum(&self) -> f64 {
        let mut sum: f64 = 0.0;
        for i in &self.older {
            sum += i;
        }
        for i in &self.younger {
            sum += i;
        }
        sum
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Extrema<'elt> {
    greatest: &'elt i32,
    least: &'elt i32,
}

fn find_extrema_1(slice: &'_ [i32]) -> Extrema<'_> {
    Extrema {
        greatest: &slice[0],
        least: &slice[1],
    }
}

fn find_extrema<'s>(slice: &'s [i32]) -> Extrema<'s> {
    let mut greatest = &slice[0];
    let mut least = &slice[0];

    for i in 1..slice.len() {
        if slice[i] < *least {
            least = &slice[0];
        }
        if slice[i] > *greatest {
            greatest = &slice[i];
        }
    }
    Extrema { greatest, least }
}

#[derive(Debug)]
struct Polynomial<const N: usize> {
    coefficients: [f64; N],
}

impl<const N: usize> Polynomial<N> {
    fn new(coefficients: [f64; N]) -> Polynomial<N> {
        Polynomial { coefficients }
    }

    fn eval(&self, x: f64) -> f64 {
        let mut sum = 0.0;
        for i in (0..N).rev() {
            sum = self.coefficients[i] + x * sum;
        }
        sum
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[allow(dead_code)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct SpiderBoot1 {
    hardware_err_count: Cell<u32>,
}

#[derive(Debug)]
struct SpiderBoot2 {
    hardware_err_count: RefCell<u32>,
}

#[derive(Debug)]
struct Account {
    name: String,
    age: i32,
}
