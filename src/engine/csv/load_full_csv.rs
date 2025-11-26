use crate::engine::model::daily_quote::DailyQuote;
use csv::Reader;
use csv::ReaderBuilder;
use std::path::Path;

pub fn load_full_csv() -> Vec<DailyQuote>
{
    let path = "data/QQQ.csv";

    //Load csv file into here, we will do line by line, passing each into an object and then store the object into a data structure.
    let mut data_vec = Vec::new();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(Path::new(path))
        .expect("Cannot open CSV file");


    for result in rdr.deserialize() {
        match result
            {
                Ok(record) => data_vec.push(record),
                Err(e) => {
                    eprintln!("Skipping bad csv row {}", e);
                    continue;
                }
            }
    }
    return data_vec;
}
