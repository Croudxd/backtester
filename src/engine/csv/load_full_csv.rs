use crate::engine::model::DailyQuote;
use csv::Reader;

pub fn load_full_csv() -> Vec<DailyQuote>
{
    let path = "../../../data/QQQ.csv";
    //Load csv file into here, we will do line by line, passing each into an object and then store the object into a data structure.
    let mut rdr = Reader::from_path(path).expect("failed to read csv from path");
    let mut data_vec = Vec::new();

    for result in rdr.deserialize() {
        let record: DailyQuote = result.expect("Failed to deserialize csv row");
        data_vec.push(record);
    }
    return data_vec;

}
