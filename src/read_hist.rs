use std::fs::File;
use std::io::{ self, BufRead };
use std::path::Path;
use serde::{ Serialize, Deserialize };
use serde_json;
use std::io::prelude::*;

// This file contains helper functions that extracts data from two sample .dat files containing
// option data from AAPL (2013) and SPY (2024)

#[derive(Serialize, Deserialize)]
struct HistoricalData {
    call_prices: Vec<f32>,
    call_strikes: Vec<f32>,
    names: Vec<String>,
    put_prices: Vec<f32>,
    put_strikes: Vec<f32>,
    spot: f32,
    years_to_expiry: Vec<f32>,
    option_name: String,
    time: String,
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
// Source: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn write_lines(filename: &str, data: String) {
    let path = Path::new(filename);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the string to `file`, returns `io::Result<()>`
    match file.write_all(data.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

fn get_days_from_jan(month: &str) -> f32 {
    match month {
        "Jan" => 31.0,
        "Feb" => 57.0,
        "Mar" => 90.0,
        "Apr" => 120.0,
        "May" => 151.0,
        "Jun" => 181.0,
        "Jul" => 212.0,
        "Aug" => 243.0,
        "Sep" => 273.0,
        "Oct" => 304.0,
        "Nov" => 334.0,
        "Dec" => 365.0,
        _ => 0.0,
    }
}

pub fn get_appl_data() -> (f32, Vec<f32>, Vec<f32>, Vec<f32>, Vec<f32>, Vec<f32>, Vec<String>) {
    let mut spot = 0.0;
    let mut today: f32 = 0.0;
    let mut current_year = 2000;
    let mut call_prices: Vec<f32> = Vec::new();
    let mut call_strikes: Vec<f32> = Vec::new();
    let mut put_prices: Vec<f32> = Vec::new();
    let mut put_strikes: Vec<f32> = Vec::new();
    let mut years_to_expiry: Vec<f32> = Vec::new();
    let mut names: Vec<String> = Vec::new();

    let mut line_counter = 0;

    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("data/AAPLQuoteData.dat") {
        // Extract option strike, prices, and current spot
        for line in lines.flatten() {
            line_counter += 1;

            if line_counter == 1 {
                let split_line = line.split(",").collect::<Vec<&str>>();
                spot = split_line[1].parse().unwrap();
            } else if line_counter == 2 {
                let split_line = line.split(",").collect::<Vec<&str>>();
                let date_line = split_line[0].split(" ").collect::<Vec<&str>>();
                current_year = date_line[2].parse().unwrap();
                let current_day: f32 = date_line[1].parse().unwrap();

                today = get_days_from_jan(date_line[0]) + current_day - 30.0;
            } else if line_counter >= 4 {
                let split_line = line.split(",").collect::<Vec<&str>>();

                if split_line.len() != 15 {
                    continue;
                }

                // Parse name
                let date_price = split_line[0].split(" ").collect::<Vec<&str>>();
                let name = date_price[3];

                names.push(name.to_string());

                // Parse time to expiry and spot
                let strike: f32 = date_price[2].parse().unwrap();

                if strike <= 0.0 {
                    continue;
                }

                call_strikes.push(strike);
                put_strikes.push(strike);

                let year: i32 = date_price[0].parse().unwrap();
                let days: f32 = get_days_from_jan(date_price[1]);
                let expiration = ((year - (current_year % 2000)) as f32) * 365.0 + days;

                years_to_expiry.push((expiration - today) / 365.0);

                // Parse put and call prices
                let call_bid: f32 = split_line[3].parse().unwrap();
                let call_ask: f32 = split_line[4].parse().unwrap();
                let put_bid: f32 = split_line[10].parse().unwrap();
                let put_ask: f32 = split_line[11].parse().unwrap();

                call_prices.push((call_bid + call_ask) / 2.0);
                put_prices.push((put_bid + put_ask) / 2.0);
            }
        }
    }

    return (spot, call_prices, call_strikes, put_prices, put_strikes, years_to_expiry, names);
}

pub fn get_spy_data() -> (
    f32,
    Vec<f32>,
    Vec<f32>,
    Vec<f32>,
    Vec<f32>,
    Vec<f32>,
    Vec<String>,
    String,
    String,
) {
    let mut spot = 0.0;
    let mut today: f32 = 0.0;
    let mut current_year = 2000;
    let mut call_prices: Vec<f32> = Vec::new();
    let mut call_strikes: Vec<f32> = Vec::new();
    let mut put_prices: Vec<f32> = Vec::new();
    let mut put_strikes: Vec<f32> = Vec::new();
    let mut years_to_expiry: Vec<f32> = Vec::new();
    let mut names: Vec<String> = Vec::new();
    let mut option_name: String = String::from("");
    let mut time: String = String::from("");

    let mut line_counter = 0;

    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("data/SPYQuoteData.dat") {
        // Extract option strike, prices, and current spot
        for line in lines.flatten() {
            line_counter += 1;

            if line_counter == 1 {
                let split_line = line.split(",").collect::<Vec<&str>>();
                option_name = split_line[0].to_string();
            } else if line_counter == 2 {
                let split_line = line.split(",").collect::<Vec<&str>>();

                let bid: f32 = split_line[1].parse().unwrap();
                let ask: f32 = split_line[2].parse().unwrap();

                spot = (bid + ask) / 2.0;

                time = split_line[0].to_string();

                let date_line = split_line[0].split(" ").collect::<Vec<&str>>();
                current_year = date_line[2].parse().unwrap();
                let current_day: f32 = date_line[1].parse().unwrap();

                today = get_days_from_jan(date_line[0]) + current_day;
            } else if line_counter >= 4 {
                let split_line = line.split(",").collect::<Vec<&str>>();

                if split_line.len() != 22 {
                    continue;
                }

                // Parse strike
                let strike: f32 = split_line[11].parse().unwrap();

                if strike <= 0.0 {
                    continue;
                }

                call_strikes.push(strike);
                put_strikes.push(strike);

                // Parse name
                names.push(split_line[1].to_string());

                // Parse time to expiry
                let date_price = split_line[0].split(" ").collect::<Vec<&str>>();

                let year: i32 = date_price[3].parse().unwrap();
                let days: f32 = date_price[2].parse().unwrap();
                let month_days: f32 = get_days_from_jan(date_price[1]);

                // Add one for end of day
                let expiration = ((year - current_year) as f32) * 365.0 + month_days + days + 1.0;

                years_to_expiry.push((expiration - today) / 365.0);

                // Parse put and call prices
                let call_bid: f32 = split_line[4].parse().unwrap();
                let call_ask: f32 = split_line[5].parse().unwrap();
                let put_bid: f32 = split_line[15].parse().unwrap();
                let put_ask: f32 = split_line[16].parse().unwrap();

                call_prices.push((call_bid + call_ask) / 2.0);
                put_prices.push((put_bid + put_ask) / 2.0);
            }
        }
    }

    (
        spot,
        call_prices,
        call_strikes,
        put_prices,
        put_strikes,
        years_to_expiry,
        names,
        option_name,
        time,
    )
}

pub fn print_appl_data() {
    let (spot, call_prices, call_strikes, put_prices, put_strikes, years_to_expiry, names) =
        get_appl_data();

    let data = HistoricalData {
        call_prices: call_prices,
        call_strikes: call_strikes,
        names: names,
        put_prices: put_prices,
        put_strikes: put_strikes,
        spot: spot,
        years_to_expiry: years_to_expiry,
        option_name: String::from("AAPL (APPLE INC)"),
        time: String::from("Dec 19 2013 @ 15:02 ET"),
    };

    let json_string = serde_json::to_string(&data).expect("Failed to serialize to JSON");

    // Print the JSON string
    println!("{}", json_string);

    write_lines("data/aapl.json", json_string);
}

pub fn print_spy_data() {
    let (
        spot,
        call_prices,
        call_strikes,
        put_prices,
        put_strikes,
        years_to_expiry,
        names,
        option_name,
        time,
    ) = get_spy_data();

    let data = HistoricalData {
        call_prices: call_prices,
        call_strikes: call_strikes,
        names: names,
        put_prices: put_prices,
        put_strikes: put_strikes,
        spot: spot,
        years_to_expiry: years_to_expiry,
        option_name: option_name,
        time,
    };

    let json_string = serde_json::to_string(&data).expect("Failed to serialize to JSON");

    // Print the JSON string
    println!("{}", json_string);

    write_lines("data/spy.json", json_string);
}
