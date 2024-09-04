extern crate chrono;
use chrono::prelude::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    //Add status and optionally a priority
    pub name: String,
    pub description: String,
    // pub creation_date: DateTime<Utc>,
    pub completed: bool,
}
