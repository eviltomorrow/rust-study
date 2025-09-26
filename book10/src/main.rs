fn main() {
    let m = 10;
    let n = 20;

    let result = compare(n, m);
    println!("{:?}", result);

    assert_eq!(size_of::<Ordering>(), 1);
    assert_eq!(Ordering::Equal as i8, 1);

    let n = 20;
    let http_status_result = http_status_from_u32(n);
    let http_status = if let Some(data) = http_status_result {
        data
    } else {
        HttpStatus::Ok
    };
    println!("{:?}", http_status);

    let t = new_timeunit("Hours");
    let r1 = t.plural();
    let r2 = t.singular();
    println!("{}, {}", r1, r2);
}

#[derive(Debug)]
enum Ordering {
    Less,
    Equal,
    Greater,
}

fn compare(n: i32, m: i32) -> Ordering {
    if n < m {
        return Ordering::Less;
    }
    if n > m {
        return Ordering::Greater;
    }
    Ordering::Equal
}

#[derive(Debug)]
enum HttpStatus {
    Ok = 200,
    NotModified = 304,
    NotFound = 404,
}

fn http_status_from_u32(n: u32) -> Option<HttpStatus> {
    match n {
        200 => Some(HttpStatus::Ok),
        304 => Some(HttpStatus::NotModified),
        404 => Some(HttpStatus::NotFound),
        _ => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

impl TimeUnit {
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }

    fn singular(self) -> &'static str {
        self.plural().trim_end_matches("s")
    }
}

fn new_timeunit(c: &'static str) -> TimeUnit {
    match c {
        "Seconds" => TimeUnit::Seconds,
        "Minutes" => TimeUnit::Minutes,
        "Hours" => TimeUnit::Hours,
        "Days" => TimeUnit::Days,
        "Months" => TimeUnit::Months,
        "Years" => TimeUnit::Years,
        _ => TimeUnit::Days,
    }
}

enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}
