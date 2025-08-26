use std::{
    error::Error,
    fs,
    io::{self, Write, stderr},
    path::Path,
    thread,
    time::Duration,
};

fn main() {
    println!("Hello, world!");

    thread::spawn(|| {
        pirate_share(10, 0);
    });

    thread::sleep(Duration::from_secs(1));

    const THE_USUAL: WeatherReport = WeatherReport::Sunny;
    let los_angles = LatLng::LosAngles;
    let report = get_weather(los_angles).unwrap_or(THE_USUAL);
    display_weather(LatLng::LosAngles, &report);

    let result = get_weather(LatLng::LosAngles).is_err();
    println!("{}", result);

    let src = Path::new("/tmp/c");
    let dst = Path::new("/tmp/d");
    let result = move_all(src, dst);

    match result {
        Ok(_) => println!("OK"),
        Err(err) => {
            println!("wrong: {:?}", err);
            print_error(&err);
        }
    }

    let read_numbers_result = read_numbers2(10);
    match read_numbers_result {
        Ok(data) => println!("{:?}", data),
        Err(err) => {
            println!("{}", err);
            print_error(err.as_ref());
        }
    }
}

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;

fn read_numbers2(i: i32) -> GenericResult<Vec<i16>> {
    if i < 0 {
        let io_error = io::Error::new(io::ErrorKind::Other, "time out");
        Err(GenericError::from(io_error))
    } else {
        Ok(vec![1, 2])
    }
}
// fn read_numbers(file: &mut dyn BufRead) -> GenericResult<Vec<i64>> {
//     let mut numbers = vec![];
//     for line_result in file.lines() {
//         let line = line_result?;
//         numbers.push(line.parse()?);
//     }
//     Ok(numbers)
// }

fn display_weather(location: LatLng, report: &WeatherReport) {
    println!("{:?}->{:?}", location, report)
}

fn get_weather(location: LatLng) -> Result<WeatherReport, io::Error> {
    println!("{:?}", location);
    Ok(WeatherReport::Rain)
}

// fn print_error(mut err: &dyn Error) {
//     let _ = writeln!(stderr(), "error: {}", err);
//     while let Some(source) = err.source() {
//         let _ = writeln!(stderr(), "caused by: {}", source);
//         err = source;
//     }
// }

fn move_all(src: &Path, dst: &Path) -> io::Result<()> {
    for entry_result in src.read_dir()? {
        let entry = entry_result?;
        let dst_file = dst.join(entry.file_name());
        fs::rename(entry.path(), dst_file)?;
    }
    Ok(())
}

#[derive(Debug)]
enum WeatherReport {
    Sunny,
    Rain,
}

#[derive(Debug)]
enum LatLng {
    LosAngles,
}

fn pirate_share(total: u64, crew_size: usize) -> u64 {
    let half = total / 2;
    half / crew_size as u64
}

fn print_error(mut err: &dyn Error) {
    let _ = writeln!(stderr(), "error: {}", err);
    while let Some(source) = err.source() {
        let _ = writeln!(stderr(), "caused by: {}", source);
        err = source;
    }
}
