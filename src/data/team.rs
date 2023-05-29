use std::io;

use csv::StringRecord;
use serde::ser::Error;

#[derive(serde::Deserialize, Debug, Default)]
pub struct Team {
    pub id: u32,
    pub name: String,
    pub skill_tokens: u32,
}

pub fn get_tasks(path: String) -> Result<Vec<Team>, io::Error> {
    let mut reader = csv::Reader::from_path(path).unwrap(); /* csv::Reader::from_path(path).unwrap(); */
    let mut iter: csv::StringRecordsIter<std::fs::File> = reader.records();
    Ok(get_records(iter).unwrap())
}

fn get_records(iter: csv::StringRecordsIter<std::fs::File>) -> Result<Vec<Team>, io::Error> {
    let mut data = Vec::new();
    for result in iter {
        let mut record = Team {
            id: result
                .as_ref()
                .unwrap()
                .get(0)
                .unwrap()
                .to_string()
                .parse()
                .unwrap(),
            name: result
                .as_ref()
                .unwrap()
                .get(1)
                .unwrap()
                .to_string()
                .parse()
                .unwrap(),
            skill_tokens: result
                .as_ref()
                .unwrap()
                .get(2)
                .unwrap()
                .to_string()
                .parse()
                .unwrap(),
        };
        println!("{record:?}");
        data.push(record);
    }
    Ok(data)
}
