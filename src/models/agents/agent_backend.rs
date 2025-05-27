use std::io::Read;

use crate::{
    ai_functions::aifunc_backend::{
        print_backend_webserver_code, print_fixed_code, print_improved_webserver_code,
        print_rest_api_endpoints,
    },
    helpers::general::{
        perfom_ai_call, read_code_template_content, read_exec_main_contents, save_backend_code,
    },
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

        save_backend_code(&response).expect("Failed to save backend code");
        factsheet.backend_code = Some(response);
    }

    async fn call_improved_backend_code(&mut self, factsheet: &mut FactSheet) {
        let msg_context = format!(
            "CODE_TEMPLATE : {:?} \n PROJECT_DESCRIPTION: {:?} \n",
            factsheet.backend_code, factsheet
        );

        let response = perfom_ai_call(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_improved_webserver_code),
            print_improved_webserver_code,
        )
        .await;

        save_backend_code(&response).expect("Failed to save backend code");
        factsheet.backend_code = Some(response);
    }

    async fn call_fix_code_bugs(&mut self, factsheet: &mut FactSheet) {
        let msg_context = format!(
            "BROKEN_CODE : {:?} \n ERROR_BUGS: {:?} \n
            THIS FUNCTION ONLY OUTPUTS CODE. JUST OUTPUT THE CODE",
            factsheet.backend_code, self.bug_errors
        );

        let response = perfom_ai_call(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_fixed_code),
            print_fixed_code,
        )
        .await;

        save_backend_code(&response).expect("Failed to save backend code");
        factsheet.backend_code = Some(response);
    }

    async fn call_extract_rest_api_endpoints(&self) -> String {
        let mut exec_content = String::new();
        read_exec_main_contents()
            .expect("Failed to open exec main contents")
            .read_to_string(&mut exec_content)
            .expect("Failed to read exec main contents");

        let msg_context = format!("CODE INPUT: {}", exec_content);

        let response = perfom_ai_call(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_rest_api_endpoints),
            print_rest_api_endpoints,
        )
        .await;

        response
    }
}
