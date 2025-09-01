use std::{
    io::{self, Write},
    ops::Range,
};

type GenericError = Box<dyn std::error::Error>;

fn main() -> Result<(), GenericError> {
    println!("Hello, world!");

    use std::fs::File;

    let mut local_file = File::create("hello.txt")?;
    say_hello(&mut local_file)?;

    let mut bytes = vec![];
    say_hello(&mut bytes)?;
    println!("{:?}", bytes);

    let s = min(10, 20);
    println!("{}", s);

    let mut buf: Vec<u8> = vec![];
    buf.write_all(b"Hello")?;

    let mut buf = vec![];
    buf.push(10);

    let writer: &mut dyn Write = &mut buf;
    let _ = writer;

    dot_product::<3>([0.2, 0.3, 0.4], [1f64, 2f64, 3.0]);

    let mut out = Sink;
    out.write(b"Hello world")?;

    assert_eq!('s'.is_emoji(), false);

    let mut c: Vec<u8> = vec![1u8, 2u8, 3u8];
    if let Ok(_) = c.write_html(&HtmlDocument {}) {}

    let set1 = SortedStringSet::new();
    let mut set2 = SortedStringSet::from_slice(&["a"]);

    set1.contains("c");
    set2.add("c");

    let s: Vec<String> = vec![String::from("hi")];
    unknown_words(&s, &SortedStringSet {});

    "hello".to_string();
    str::to_string("hello");

    SortedStringSet::contains(&StringSet::new(), "");

    Ok(())
}

fn say_hello(out: &mut dyn Write) -> io::Result<()> {
    out.write(b"hello world\n")?;
    Ok(())
}

fn min<T: Ord>(value1: T, value2: T) -> T {
    if value1 < value2 { value1 } else { value2 }
}

trait Reduce {}

trait Mapper {}

trait Serialize {}

#[allow(dead_code)]
fn run_query<M: Mapper + Serialize, R: Reduce + Serialize>(
    map: M,
    reduce: R,
) -> Result<(), io::Error> {
    let _ = map;
    let _ = reduce;
    Ok(())
}

#[allow(dead_code)]
fn run_query_2<M, R>(map: M, reduce: R)
where
    M: Mapper + Serialize,
    R: Reduce + Serialize,
{
    let _ = map;
    let _ = reduce;
}

trait IP {}

#[allow(dead_code)]
fn nearest<P>(target: &P, candidates: &[P])
where
    P: IP,
{
    let _ = target;
    let _ = candidates;
}

#[allow(dead_code)]
fn dot_product<const N: usize>(a: [f64; N], b: [f64; N]) -> f64 {
    let mut sum = 0f64;
    for i in 0..N {
        sum = a[i] + b[i];
    }
    sum
}
#[allow(dead_code)]
struct Canvas {}

#[allow(dead_code)]
impl Canvas {
    fn write_at(&mut self, x: i32, y: i32, c: char) {
        println!("{}, {}, {}", x, y, c);
    }
}
#[allow(dead_code)]
trait Visible {
    fn draw(&self, canvas: &mut Canvas);
    fn hit_test(&self, x: i32, y: i32) -> bool;
}
#[allow(dead_code)]
struct Broom {
    x: i32,
    y: i32,
    height: i32,
}

impl Visible for Broom {
    fn draw(&self, canvas: &mut Canvas) {
        for y in self.broomstick_range() {
            canvas.write_at(self.x, y, '|');
        }
        canvas.write_at(self.x, self.y, 'M');
    }

    fn hit_test(&self, x: i32, y: i32) -> bool {
        self.x == x && self.y - self.height - 1 <= y && y <= self.y
    }
}

#[allow(dead_code)]
impl Broom {
    fn broomstick_range(&self) -> Range<i32> {
        self.y - self.height - 1..self.y
    }
}

pub struct Sink;

impl Write for Sink {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        let _ = buf;
        Ok(())
    }
}

trait IsEmoji {
    fn is_emoji(&self) -> bool;
}

impl IsEmoji for char {
    fn is_emoji(&self) -> bool {
        //
        false
    }
}

struct HtmlDocument {}

trait WriteHtml {
    fn write_html(&mut self, html: &HtmlDocument) -> io::Result<()>;
}

impl<W: Write> WriteHtml for W {
    fn write_html(&mut self, html: &HtmlDocument) -> io::Result<()> {
        let _ = html;
        Ok(())
    }
}

#[allow(dead_code)]
trait Creature
where
    Self: Visible + Write,
{
}

trait StringSet {
    fn new() -> Self;
    fn from_slice(strings: &[&str]) -> Self;
    fn contains(&self, string: &str) -> bool;
    fn add(&mut self, string: &str);
}

struct SortedStringSet {}

impl StringSet for SortedStringSet {
    fn new() -> Self {
        SortedStringSet {}
    }

    fn from_slice(strings: &[&str]) -> Self {
        let _ = strings;
        SortedStringSet {}
    }

    fn contains(&self, _: &str) -> bool {
        true
    }

    fn add(&mut self, _: &str) {}
}

fn unknown_words<S: StringSet>(document: &[String], wordlist: &S) -> S {
    let mut unknowns = S::new();
    for word in document {
        if !wordlist.contains(word) {
            unknowns.add(word);
        }
    }
    unknowns
}
