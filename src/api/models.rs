use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Course {
    pub id: u64,
    pub name: Option<String>,
    pub course_code: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Tabs {
    pub id: Option<String>,
    pub label: Option<String>,
    pub position: i32,
    pub visibility: Option<String>,
    #[serde(rename = "type")]
    pub type_of_tab: Option<String>,
}
