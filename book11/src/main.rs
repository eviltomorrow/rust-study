use std::{fmt::Debug, fs::File, hash::Hash, io::Write};

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;

fn main() -> GenericResult<()> {
    println!("Hello, world!");

    let mut local_file = File::create("hello.txt")?;
    say_hello1(&mut local_file)?;

    let mut bytes = vec![];
    say_hello2(&mut bytes)?;
    println!("{:?}", String::from_utf8(bytes));

    let a: [f64; 3] = [0., 0., 0.];
    let b = [1f64; 3];
    let sum = dot_product(a, b);
    println!("{:?}", sum);

    Ok(())
}

fn say_hello1(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(br"hello world\n")?;
    out.flush()
}

fn say_hello2<T: Write>(out: &mut T) -> std::io::Result<()> {
    out.write_all(br"hello world\n")?;
    out.flush()
}

fn dot_product<const N: usize>(a: [f64; N], b: [f64; N]) -> f64 {
    let mut sum = 0.;
    for i in 0..N {
        sum += a[i] + b[i];
    }
    sum
}

#[allow(dead_code)]
fn top_ten<T: Debug>(_values: &Vec<T>) {}

#[allow(dead_code)]
fn top_ten_1<T: Debug + Hash + Eq>(_values: &Vec<T>) {}

#[allow(dead_code)]
fn run_query<M: Mapper + Serialize, R: Reducer + Serialize>(_data_1: M, _data_2: R) {}

#[allow(dead_code)]
fn run_query_1<M, R>()
where
    M: Mapper + Serialize,
    R: Reducer + Serialize,
{
}

#[allow(dead_code)]
trait Mapper {}
#[allow(dead_code)]
trait Reducer {}
#[allow(dead_code)]
trait Serialize {}

#[allow(dead_code)]
fn nearest<'t, 'c, P>(_target: &'t P, candidates: &'c [P]) -> &'c P
where
    P: Debug,
{
    &candidates[0]
}

#[allow(dead_code)]
enum C<T>
where
    T: Debug,
{
    NUM3(T),
}

#[allow(dead_code)]
trait Vegetable {}

#[allow(dead_code)]
struct Salad<V: Vegetable> {
    veggies: Vec<V>,
}

#[allow(dead_code)]
struct Salad2<'a, V: Vegetable> {
    veggies: Vec<&'a V>,
}

#[allow(dead_code)]
struct Salad3<'a> {
    veggies: Vec<&'a dyn Vegetable>,
}

#[allow(dead_code)]
struct Salad4 {
    veggies: Vec<Box<dyn Vegetable>>,
}
