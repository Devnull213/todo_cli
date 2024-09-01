extern crate chrono;
use chrono::prelude::DateTime;
use chrono::{Utc};


#[derive(Debug)]
pub struct Task {
    //Add status and optionally a priority
    pub name: String,
    pub description: String,
    pub creation_date: DateTime<Utc>, 
}

