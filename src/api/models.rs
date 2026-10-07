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

#[derive(Debug, Clone, Deserialize)]
pub struct Module {
    pub id: u64,
    pub workflow_state: Option<String>,
    pub position: i32,
    pub name: Option<String>,
    pub unlock_at: Option<String>,
    pub require_sequential_progress: Option<bool>,
    pub requirement_type: Option<String>,
    #[serde(default)]
    pub prerequisite_module_ids: Vec<u64>,
    pub items_count: Option<u32>,
    pub items_url: Option<String>,
    pub state: Option<String>,
    pub completed_at: Option<String>,
    pub publish_final_grade: Option<bool>,
    pub published: Option<bool>,
}
