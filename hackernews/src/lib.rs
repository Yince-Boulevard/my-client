use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};
pub(crate) mod api;
mod ui;
pub use ui::App;
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct StoryItem {
    pub id: i64,
    pub title: String,
    pub url: Option<String>,
    pub text: Option<String>,
    #[serde(default)]
    pub by: String,
    #[serde(default)]
    pub score: i64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    #[serde(default)]
    pub kids: Vec<i64>,
    pub r#type: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Comment {
    pub id: i64,
    #[serde(default)]
    pub by: String,
    #[serde(default)]
    pub text: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    #[serde(default)]
    pub kids: Vec<i64>,
    #[serde(default)]
    pub sub_comments: Vec<Comment>,
    pub r#type: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct StoryData {
    #[serde(flatten)]
    pub items: Vec<StoryItem>,
    #[serde(default)]
    pub comments: Vec<Comment>,
}
