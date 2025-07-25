use std::fmt::Debug;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::models::agent_basic::basic_agent::BasicAgent;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FactSheet {
    pub project_description: String,
    pub project_scope: Option<ProjectScope>,
    pub external_urls: Option<Vec<String>>,
    pub backend_code: Option<String>,
    pub api_endpoint_schema: Option<Vec<RouteObject>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct ProjectScope {
    pub is_crud_required: bool,
    pub is_user_login_and_logout: bool,
    pub is_external_urls_required: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RouteObject {
    pub is_route_dynamic: String,
    pub method: String,
    pub request_body: serde_json::Value,
    pub response: serde_json::Value,
    pub route: String,
}

#[async_trait]
pub trait SpecialFunctions: Debug {
    fn get_attributes_from_agent(&self) -> &BasicAgent;

    async fn execute(
        &mut self,
        factsheet: &mut FactSheet,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
