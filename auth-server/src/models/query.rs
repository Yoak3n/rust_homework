use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
pub struct QueryCount{
    pub time :Option<NaiveDateTime>,
    pub record :QueryModel,
}

#[derive(Debug, Deserialize,Serialize)]
pub struct QueryModel {
    pub time: Option<NaiveDateTime>,
    pub code: String,
    pub state: String,
}
