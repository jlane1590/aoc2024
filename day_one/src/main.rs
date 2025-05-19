use std::collections::HashMap;

type Record = (i64, i64);

fn main() {
    let rdr = csv::ReaderBuilder::new().has_headers(false).delimiter(b'\t').from_path("big-locations.csv");

    let mut loc0: Vec<i64> = Vec::new();
    let mut loc1: Vec<i64> = Vec::new();

    for result in rdr.expect("bad reader").deserialize() {
        let record: Record = result.expect("bad record");
        loc0.push(record.0);
        loc1.push(record.1);
    }

    loc0.sort();
    loc1.sort();

    let mut total_distance: i64 = 0;
    for entry in loc0.iter().zip(loc1.iter()) {
        total_distance += (entry.0 - entry.1).abs();
    }
    println!("{:?}", total_distance);

    let mut m:HashMap<i64, i64> = HashMap::new();
    for i in loc1 {
        *m.entry(i).or_default() += 1;
    }

    let mut similarity: i64 = 0;
    for l0 in loc0 {
        similarity += l0 * *m.entry(l0).or_default();
    }
    println!("{:?}", similarity);
}
