use std::io;

use csv::StringRecord;
use serde::ser::Error;

#[derive(serde::Deserialize, Debug, Default)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub approx_skill: u32,
    pub approx_length: u32,
    pub true_length: u32,
    pub dependencies: Vec<u32>, // vagy Vec<Task>
    pub earliest_start: Option<u32>,
    pub earliest_finish: Option<u32>,
    pub latest_start: Option<u32>,
    pub latest_finish: Option<u32>,
    pub float: Option<u32>,
    pub done: bool,
}

pub fn get_tasks(path: String) -> Result<Vec<Task>, io::Error> {
    let mut reader = csv::Reader::from_path(path).unwrap(); /* csv::Reader::from_path(path).unwrap(); */
    let mut iter: csv::StringRecordsIter<std::fs::File> = reader.records();
    Ok(get_records(iter).unwrap())
}

fn get_records(iter: csv::StringRecordsIter<std::fs::File>) -> Result<Vec<Task>, io::Error> {
    let mut data = Vec::new();
    for result in iter {
        let mut record = Task {
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
            dependencies: result
                .as_ref()
                .unwrap()
                .get(2)
                .unwrap()
                .to_string()
                .split(' ')
                .flat_map(|x| x.parse())
                .collect(),
            approx_skill: result
                .as_ref()
                .unwrap()
                .get(3)
                .unwrap()
                .to_string()
                .parse()
                .unwrap(),
            approx_length: result
                .as_ref()
                .unwrap()
                .get(4)
                .unwrap()
                .to_string()
                .parse()
                .unwrap(),
            ..Default::default()
        };
        println!("{record:?}");
        data.push(record);
    }
    Ok(data)
}
