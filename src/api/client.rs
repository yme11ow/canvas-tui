use reqwest::blocking::Client;
use reqwest::header::LINK;
use serde::de::DeserializeOwned;

use crate::api::models::{Module, Tabs};

use super::models::Course;

pub struct CanvasClient {
    http: Client,
    base_url: String,
    token: String,
}

impl CanvasClient {
    pub fn new(base_url: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            http: Client::builder()
                .user_agent(concat!(
                    env!("CARGO_PKG_NAME"),
                    "/",
                    env!("CARGO_PKG_VERSION")
                ))
                .build()
                .expect("failed to build HTTP client"),
            base_url: base_url.into().trim_end_matches('/').to_string(),
            token: token.into(),
        }
    }

    pub fn from_env() -> Result<Self, std::env::VarError> {
        Ok(Self::new(
            std::env::var("CANVAS_URL")?,
            std::env::var("CANVAS_TOKEN")?,
        ))
    }

    pub fn courses(&self) -> reqwest::Result<Vec<Course>> {
        self.get_paginated("/api/v1/courses?enrollment_state=active&per_page=100")
    }

    pub fn tabs(&self, course_id: u64) -> reqwest::Result<Vec<Tabs>> {
        self.get_paginated(&format!("/api/v1/courses/{}/tabs", course_id))
    }

    pub fn modules(&self, course_id: u64) -> reqwest::Result<Vec<Module>> {
        self.get_paginated(&format!("/api/v1/courses/{}/modules", course_id))
    }

    /// Canvas paginates results and puts the next page's URL in the `Link` header.
    fn get_paginated<T: DeserializeOwned>(&self, path: &str) -> reqwest::Result<Vec<T>> {
        let mut url = Some(format!("{}{}", self.base_url, path));
        let mut items = Vec::new();

        while let Some(current) = url {
            let resp = self
                .http
                .get(&current)
                .bearer_auth(&self.token)
                .send()?
                .error_for_status()?;

            url = resp
                .headers()
                .get(LINK)
                .and_then(|v| v.to_str().ok())
                .and_then(next_link);
            items.extend(resp.json::<Vec<T>>()?);
        }
        Ok(items)
    }
}

/// Parses `<https://...>; rel="next", <https://...>; rel="last"`
fn next_link(header: &str) -> Option<String> {
    header.split(',').find_map(|part| {
        let (url, rel) = part.split_once(';')?;
        rel.contains(r#"rel="next""#).then(|| {
            url.trim()
                .trim_start_matches('<')
                .trim_end_matches('>')
                .to_string()
        })
    })
}
