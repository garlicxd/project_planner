use self::{task::get_tasks, team::get_teams};

pub mod task;
pub mod team;

const TASKS_PATH: &str = "tasks.csv";
const TEAMS_PATH: &str = "teams.csv";

pub struct Data {
    tasks: Vec<task::Task>,
    teams: Vec<team::Team>,
}

pub fn get_data() -> Data {
    Data {
        tasks: get_tasks(TASKS_PATH.to_string()).unwrap(),
        teams: get_teams(TEAMS_PATH.to_string()).unwrap(),
    }
}
