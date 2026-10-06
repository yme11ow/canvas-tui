use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Course {
    pub id: u64,
    pub name: Option<String>,
    pub course_code: Option<String>,
}
