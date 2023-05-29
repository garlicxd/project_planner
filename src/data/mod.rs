pub mod task;
mod team;

use crate::data::task::Task;
use crate::data::team::Team;
use csv::Reader;
use std::error::Error;

pub struct Data {
    tasks: Vec<task::Task>,
    teams: Vec<team::Team>,
}
