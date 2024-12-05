use std::collections::HashMap;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;
    let (list1, list2) = parse_columns(&contents, &config.delimiter)?;
    println!("total distance: {}", calculate_distance(&list1, &list2));

    let counter = make_counter(&list2);
    println!("similarity score: {}", similarity_score(&list1, &counter));

    Ok(())
}

fn similarity_score(list1: &Vec<u32>, counter: &HashMap<u32, u32>) -> u32 {
    let mut total_score: u32 = 0;
    for num in list1 {
        total_score += num * counter.get(num).unwrap_or(&0)
    }

    total_score
}

fn make_counter(list: &Vec<u32>) -> HashMap<u32, u32> {
    let mut counter: HashMap<u32, u32> = HashMap::new();

    for num in list {
        let count = counter.entry(*num).or_insert(0);
        *count += 1;
    }

    counter
}

fn calculate_distance(list1: &Vec<u32>, list2: &Vec<u32>) -> u32 {
    let mut total_distance: u32 = 0;

    for (l1, l2) in list1.iter().zip(list2.iter()) {
        total_distance += (*l1).abs_diff(*l2);
    }
    total_distance
}

fn parse_columns(
    contents: &str,
    delimiter: &str,
) -> Result<(Vec<u32>, Vec<u32>), Box<dyn Error>> {
    let mut list1: Vec<u32> = Vec::new();
    let mut list2: Vec<u32> = Vec::new();

    for line in contents.lines() {
        let row: Vec<u32> = if delimiter == "s" {
            line.split_whitespace()
                .map(|elem| elem.trim().parse::<u32>())
                .collect::<Result<Vec<u32>, _>>()?
        } else {
            line.split(delimiter)
                .map(|elem| elem.trim().parse::<u32>())
                .collect::<Result<Vec<u32>, _>>()?
        };

        if row.len() == 2 {
            list1.push(row[0]);
            list2.push(row[1]);
        } else {
            return Err(format!("Expected exactly two values in row. Got {}.", line).into());
        }
    }

    list1.sort();
    list2.sort();

    Ok((list1, list2))
}

pub struct Config {
    pub file_path: String,
    pub delimiter: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a file path!"),
        };

        let delimiter =
            match args.next() {
                Some(arg) => match arg.to_lowercase().as_str() {
                    "comma" => ",",
                    "space" => "s",
                    "semicolon" => ";",
                    _ => return Err(
                        "Unknown delimiter. Allowed delimiters: ['comma', 'space', 'semicolon']",
                    ),
                },
                None => return Err("Did not get a delimiter!"),
            };

        Ok(Config {
            file_path: file_path,
            delimiter: delimiter.to_string(),
        })
    }
}
