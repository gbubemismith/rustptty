use std::io::Read;

use crate::{
    ai_functions::aifunc_backend::print_backend_webserver_code,
    helpers::general::{perfom_ai_call, read_code_template_content},
    models::agent_basic::basic_agent::{AgentState, BasicAgent},
};

use super::agent_traits::FactSheet;

#[derive(Debug)]
pub struct AgentBackendDeveloper {
    attributes: BasicAgent,
    bug_errors: Option<String>,
    bug_count: u8,
}

impl AgentBackendDeveloper {
    pub fn new() -> Self {
        let attributes = BasicAgent {
            objective: "Develops backend code for webserver and json database".to_string(),
            position: "Backend developer".to_string(),
            state: AgentState::Discovery,
            memory: Some(vec![]),
        };

        Self {
            attributes,
            bug_errors: None,
            bug_count: 0,
        }
    }

    async fn call_initial_backend_code(&mut self, factsheet: &mut FactSheet) {
        let mut code_template_content = String::new();
        read_code_template_content()
            .expect("Failed to open code template")
            .read_to_string(&mut code_template_content)
            .expect("Failed to read code template content");

        let msg_context = format!(
            "CODE_TEMPLATE : {} \n PROJECT_DESCRIPTION: {} \n",
            code_template_content, factsheet.project_description
        );

        let response = perfom_ai_call(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_backend_webserver_code),
            print_backend_webserver_code,
        )
        .await;
    }
}
