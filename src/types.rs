use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Oncalls {
    pub oncalls: Vec<Oncall>,
}

#[derive(Deserialize, Debug)]
pub struct Oncall {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub user: User,
    pub schedule: Schedule,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct Schedule {
    pub summary: String,
}


