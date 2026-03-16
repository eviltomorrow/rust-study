use std::error::Error;
use std::fs;
use std::io::{self, Write, stderr};
use std::path::Path;

fn main() {
    println!("Hello, world!");

    if let Ok(data) = get_result() {
        println!("{:?}", data);
    }

    match get_weather(LatLng::L) {
        Ok(report) => {
            println!("{:?}", report);
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
    const THE_USUAL: WeatherReport = WeatherReport::Sunny(20);

    let result = get_weather(LatLng::L);
    println!("{}", result.is_ok());

    let result = get_weather_err(LatLng::L);
    println!("{}", result.is_err());

    println!("{:?}", result);

    let result = get_weather_err(LatLng::L);
    let val = result.unwrap_or(THE_USUAL);
    println!("{:?}", val,);

    if let WeatherReport::Sunny(v) = val {
        println!("{}", v);
    }

    let result = get_weather_err(LatLng::L);
    let val = result.unwrap_or_else(|_err| get_weather(LatLng::L).unwrap());
    println!("{:?}", val);

    let result = get_weather(LatLng::L);
    let val = result.expect("panic: error");
    println!("{:?}", val);

    let result = get_weather_err(LatLng::L);
    let val = result.unwrap_or_else(|_err| WeatherReport::Rain);
    println!("{:?}", val);

    let result = get_weather(LatLng::L);
    let result_ref = result.as_ref();
    let val = result_ref.unwrap();
    println!("{:?}", val);
    println!("{:?}", result);
    let val = result.ok().unwrap();
    println!("{:?}", val);
    // println!("{}", result);
    //
    let err = get_weather_err(LatLng::L).err().unwrap();
    println!("{:?}", err);
    print_error(&err);
}

fn get_result() -> Result<(), io::Error> {
    Ok(())
}

fn get_weather(location: LatLng) -> Result<WeatherReport, io::Error> {
    let _ = location;
    Ok(WeatherReport::Rain)
}

fn get_weather_err(location: LatLng) -> Result<WeatherReport, io::Error> {
    let _ = location;
    Err(io::Error::new(io::ErrorKind::NotFound, "panic: not found"))
}

fn print_error(mut err: &dyn Error) {
    let _ = writeln!(stderr(), "error: {}", err);
    while let Some(source) = err.source() {
        let _ = writeln!(stderr(), "caused by: {}", source);
        err = source;
    }
}

enum LatLng {
    L,
}

#[derive(Debug)]
enum WeatherReport {
    Sunny(i32),
    Rain,
}

fn _move_all(src: &Path, dst: &Path) -> io::Result<()> {
    for entry_result in src.read_dir()? {
        let entry = entry_result?;
        let dst_file = dst.join(entry.file_name());
        fs::rename(entry.path(), dst_file)?;
    }
    Ok(())
}

fn _get_option() -> Option<WeatherReport> {
    let value = get_weather(LatLng::L).ok()?;
    Some(value)
}
