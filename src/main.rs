mod data;
use crate::data::task::*;
use crate::data::*;

fn main() {
    println!("Project Planner");
    get_tasks("tasks.csv".to_string());
}
