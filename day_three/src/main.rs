use std::fs;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("big.txt").expect("could not open file").replace('\n', " ");

    let disable_re = Regex::new(r"don't\(\).*?(?:do\(\)|$)").unwrap();

    let text = disable_re.replace_all(input.as_str(), " ").to_string();
    
    let mul_re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    
    let mut total:i32 = 0;
    for cap in mul_re.captures_iter(text.as_str()) {
        total += &cap[1].parse::<i32>().unwrap_or(0) * &cap[2].parse::<i32>().unwrap_or(0);
    }
    println!("{:?}", total);
    fs::write("text.txt", text).expect("unable to write");
}
