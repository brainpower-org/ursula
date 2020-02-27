use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use regex::Regex;

use svg::Document;
use svg::node::element::Circle;

pub fn parse_from_file(mut input_file: &File) {
    dbg!(input_file);
    println!("Start reading chart file");
    let mut chart_content = String::new();
    input_file.read_to_string(&mut chart_content);

    println!("Start parsing file");
    let chart_match: Regex = Regex::new("^(?P<chart>pie)").unwrap();

    match chart_match.captures(&chart_content) {
        Some(capture) => {
            match capture.name("chart").unwrap().as_str() {
                "pie" => parse_pi_chart(&chart_content),
                _=>  println!("No supported chart found")
            }
        },
        None => println!("No supported chart found")
    };
}

#[derive(Debug)]
struct PiChart {
    title: Option<String>,
    slices: HashMap<String, f64>,
}

fn parse_pi_chart(chart_content: &String) {
    println!("Parsing pie chart");
    let chart_regex = Regex::new("(?m)^\\s*\"(?P<slice_name>[^\"]+)\"\\s*:\\s*(?P<slice_value>[^\\s]+)").unwrap();
    let slices = chart_regex.captures_iter(chart_content).fold(HashMap::new(), |mut slices, capture| {
        slices.entry(String::from(capture.name("slice_name").unwrap().as_str())).or_insert(capture.name("slice_value").unwrap().as_str().parse().unwrap());
        slices
    });

    println!("Parsed pi chart");
    let pi_chart = dbg!(PiChart {
        title: None,
        slices
    });

    let circle = Circle::new()
        .set("cx", 50)
        .set("cy", 50)
        .set("r", 40)
        .set("stroke", "black")
        .set("stroke-width", 1)
        .set("fill", "white");

    let document = dbg!(Document::new()
        .set("viewBox", (0, 0, 100, 100))
        .add(circle));

    svg::save("image.svg", &document).unwrap();
}