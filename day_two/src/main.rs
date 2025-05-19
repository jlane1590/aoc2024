fn main() {
    let rdr = csv::ReaderBuilder::new().has_headers(false).flexible(true).delimiter(b' ').from_path("big-levels.csv");

    let mut safe_report_cnt: u32 = 0;
    for result in rdr.expect("bad reader").deserialize() {
        let report: Vec<i32> = result.expect("bad record");
        if report.is_sorted_by(|a, b| (b-a)>=1 && (b-a)<=3) ||
           report.is_sorted_by(|a, b| (a-b)>=1 && (a-b)<=3) {
            safe_report_cnt += 1;
            continue;
        }
        for skip_index in 0..report.len() {
            let mut dampened_report: Vec<i32> = report.clone();
            dampened_report.remove(skip_index);
            if dampened_report.is_sorted_by(|a, b| (b-a)>=1 && (b-a)<=3) ||
               dampened_report.is_sorted_by(|a, b| (a-b)>=1 && (a-b)<=3) {
                safe_report_cnt += 1;
                break;
            }
        }
    }
    println!("{:?}", safe_report_cnt);
}
