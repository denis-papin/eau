use std::fs::File;
use std::io::{BufRead, BufReader};
use anyhow::anyhow;
use chrono::{FixedOffset, NaiveDate, ParseResult};

#[derive(Debug)]
struct Data {
    date: NaiveDate,
    heure: String,
    ext: Option<f32>,
    int: Option<f32>,
}

use chrono::{DateTime, NaiveTime, ParseError, Utc};

fn parse_time(s: &str) -> ParseResult<NaiveTime> {
    NaiveTime::parse_from_str(s, "%H:%M")

    // dbg!(&time);
    // let date = Utc::today().naive_utc();
    // dbg!(&date);
    // DateTime::parse_from_str(&format!("{} {}", date, time), "%Y-%m-%d %H:%M:%S %Z")
}


fn difference_in_hours(start: &str, end: &str) -> anyhow::Result<f64> {
    let start_time = parse_time(start)?;
    dbg!(&start_time);
    let end_time = parse_time(end)?;
    dbg!(&end_time);
    let duration = end_time - start_time;
    Ok(duration.num_seconds() as f64 / 3600.0)
}


fn read_data(filename: &str) -> anyhow::Result<Vec<Data>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // skip first line
    let lines = reader.lines().skip(1);

    let mut all_data = Vec::new();

    for line in lines {
        let line = line.unwrap();
        println!("{}", &line);
        let items: Vec<&str> = line
            .split(';')
            .map(|s| s.trim())
            .enumerate()
            //.filter(|&(i, _)| i != 2)
            .map(|(_, s)| s)
            .collect();

        println!("{:#?}", &items);

        if items.len() < 4 {
            //return Err(anyhow!(""));
            break;
        }

        let date = NaiveDate::parse_from_str(items[0], "%d/%m/%Y").unwrap();
        let heure = items[1].to_string();
        let ext = items[2].replace(",", ".").parse::<f32>().ok();
        let int = items[3].replace(",", ".").parse::<f32>().ok();

        let data = Data {
            date/*: Default::default()*/,
            heure/*: "".to_string()*/,
            ext,
            int,
        };

        all_data.push(data);
    }

    Ok(all_data)
}


use chrono::{Duration};

fn calculate_flow_rate(data: &Vec<Data>) -> Vec<(NaiveDate,f64)> {
    // Initialize variables

    let first_date = data.get(0).unwrap().date;
    let first_time = &data.get(0).unwrap().heure;
    let mut first_value = data.get(0).unwrap().int.unwrap();
    let mut flow_rates = vec![(first_date,0.0_f64); data.len()];

    // Iterate over the data and calculate flow rates
    for (i, d) in data.iter().enumerate() {

        let offset = difference_in_hours(&d.heure, first_time);

        // Parse the date from the string
        let date = d.date; // NaiveDate::parse_from_str(&d.date, "%d/%m/%Y").unwrap();
        // Calculate the time interval since the first measurement
        let time_interval = date.signed_duration_since(first_date) * 24;
        // Calculate the flow rate, if possible
        if let Some(value) = d.int {
            let volume = value - first_value;
            let flow_rate = volume as f64 / time_interval.num_days() as f64 * 365.0 * 24.0;
            flow_rates[i] = (date, flow_rate);
        }
    }

    flow_rates
}

fn main() {
    let ds = read_data(r#"C:\Users\denis\Dropbox\Personnel\Tableau_Conso_Eau.txt"#).unwrap();
    let flow = calculate_flow_rate(&ds);
    println!("Hello, world! {:#?}", &ds);
    println!("Flow {:#?}", &flow);

    let a = difference_in_hours("13:30", "14:00");

    println!("{}", a.unwrap());
}
