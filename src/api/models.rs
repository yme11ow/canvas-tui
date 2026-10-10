use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Course {
    pub id: u64,
    pub name: Option<String>,
    // pub course_code: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Tabs {
    pub id: Option<String>,
    pub label: Option<String>,
    pub position: i32,
    //pub visibility: Option<String>,
    //#[serde(rename = "type")]
    //pub type_of_tab: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Module {
    //pub id: u64,
    //pub workflow_state: Option<String>,
    //pub position: i32,
    pub name: Option<String>,
    //pub unlock_at: Option<String>,
    //pub require_sequential_progress: Option<bool>,
    //pub requirement_type: Option<String>,
    //#[serde(default)]
    //pub prerequisite_module_ids: Vec<u64>,
    //pub items_count: Option<u32>,
    //pub items_url: Option<String>,
    //pub state: Option<String>,
    //pub completed_at: Option<String>,
    //pub publish_final_grade: Option<bool>,
    //pub published: Option<bool>,
}

// Fields typed as serde_json::Value need the serde_json crate before they can be uncommented.
#[derive(Debug, Clone, Deserialize)]
pub struct Assignment {
    //pub id: u64,
    pub name: Option<String>,
    //pub description: Option<String>,
    //pub created_at: Option<String>,
    //pub updated_at: Option<String>,
    //pub due_at: Option<String>,
    //pub lock_at: Option<String>,
    //pub unlock_at: Option<String>,
    //pub has_overrides: Option<bool>,
    //pub all_dates: Option<serde_json::Value>,
    //pub course_id: Option<u64>,
    //pub html_url: Option<String>,
    //pub submissions_download_url: Option<String>,
    //pub assignment_group_id: Option<u64>,
    //pub due_date_required: Option<bool>,
    //#[serde(default)]
    //pub allowed_extensions: Vec<String>,
    //pub max_name_length: Option<u32>,
    //pub turnitin_enabled: Option<bool>,
    //pub vericite_enabled: Option<bool>,
    //pub turnitin_settings: Option<serde_json::Value>,
    //pub grade_group_students_individually: Option<bool>,
    //pub external_tool_tag_attributes: Option<serde_json::Value>,
    //pub peer_reviews: Option<bool>,
    //pub automatic_peer_reviews: Option<bool>,
    //pub peer_review_count: Option<u32>,
    //pub peer_reviews_assign_at: Option<String>,
    //pub intra_group_peer_reviews: Option<bool>,
    //pub group_category_id: Option<u64>,
    //pub needs_grading_count: Option<u32>,
    //pub needs_grading_count_by_section: Option<serde_json::Value>,
    //pub position: Option<i32>,
    //pub post_to_sis: Option<bool>,
    //pub integration_id: Option<String>,
    //pub integration_data: Option<serde_json::Value>,
    //pub points_possible: Option<f64>,
    //#[serde(default)]
    //pub submission_types: Vec<String>,
    //pub has_submitted_submissions: Option<bool>,
    //pub grading_type: Option<String>,
    //pub grading_standard_id: Option<u64>,
    //pub published: Option<bool>,
    //pub unpublishable: Option<bool>,
    //pub only_visible_to_overrides: Option<bool>,
    //pub locked_for_user: Option<bool>,
    //pub lock_info: Option<serde_json::Value>,
    //pub lock_explanation: Option<String>,
    //pub quiz_id: Option<u64>,
    //pub anonymous_submissions: Option<bool>,
    //pub discussion_topic: Option<serde_json::Value>,
    //pub freeze_on_copy: Option<bool>,
    //pub frozen: Option<bool>,
    //#[serde(default)]
    //pub frozen_attributes: Vec<String>,
    //pub submission: Option<serde_json::Value>,
    //pub use_rubric_for_grading: Option<bool>,
    //pub rubric_settings: Option<serde_json::Value>,
    //pub rubric: Option<serde_json::Value>,
    //#[serde(default)]
    //pub assignment_visibility: Vec<u64>,
    //pub overrides: Option<serde_json::Value>,
    //pub omit_from_final_grade: Option<bool>,
    //pub hide_in_gradebook: Option<bool>,
    //pub moderated_grading: Option<bool>,
    //pub grader_count: Option<u32>,
    //pub final_grader_id: Option<u64>,
    //pub grader_comments_visible_to_graders: Option<bool>,
    //pub graders_anonymous_to_graders: Option<bool>,
    //pub grader_names_visible_to_final_grader: Option<bool>,
    //pub anonymous_grading: Option<bool>,
    //pub allowed_attempts: Option<i32>,
    //pub post_manually: Option<bool>,
    //pub score_statistics: Option<serde_json::Value>,
    //pub can_submit: Option<bool>,
    //#[serde(default)]
    //pub ab_guid: Vec<String>,
    //pub academic_integrity_pledge: Option<String>,
    //pub annotatable_attachment_id: Option<u64>,
    //pub anonymize_students: Option<bool>,
    //pub require_lockdown_browser: Option<bool>,
    //pub important_dates: Option<bool>,
    //pub muted: Option<bool>,
    //pub anonymous_peer_reviews: Option<bool>,
    //pub anonymous_instructor_annotations: Option<bool>,
    //pub graded_submissions_exist: Option<bool>,
    //pub is_quiz_assignment: Option<bool>,
    //pub in_closed_grading_period: Option<bool>,
    //pub can_duplicate: Option<bool>,
    //pub original_course_id: Option<u64>,
    //pub original_assignment_id: Option<u64>,
    //pub original_lti_resource_link_id: Option<u64>,
    //pub original_assignment_name: Option<String>,
    //pub original_quiz_id: Option<u64>,
    //pub workflow_state: Option<String>,
}

pub enum TabKind {
    Modules,
    Assignments,
    Announcements,
    Grades,
    Other,
}

impl Tabs {
    pub fn kind(&self) -> TabKind {
        match self.id.as_deref() {
            Some("modules") => TabKind::Modules,
            Some("assignments") => TabKind::Assignments,
            Some("announcements") => TabKind::Announcements,
            Some("grades") => TabKind::Grades,
            _ => TabKind::Other,
        }
    }
}
