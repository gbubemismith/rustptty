use std::{
    fs::File,
    io::{self, BufRead, BufWriter, Write},
};

use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::{apis::call_request::call_gpt, models::general::llm::Message};

use super::cli::PrintCommand;

const CODE_TEMPLATE_PATH: &str =
    "/Users/gbubemismith/Documents/Rust/rustptty/assets/code_template.rs";
const EXEC_MAIN_PATH: &str = "/Users/gbubemismith/Documents/Rust/rustptty/assets/main.rs";
const SCHEMA_PATH: &str = "/Users/gbubemismith/Documents/Rust/rustptty/schemas/api_schema.json";

pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {
    let ai_function_str = ai_func(func_input);

    let msg = format!("FUNCTION {}
    INSTRUCTION: You are a function printer. You ONLY print results of functions. Nothing else, no commentary. Here is the input of the function {}.", ai_function_str, func_input);

    Message {
        role: "system".to_string(),
        content: msg,
    }
}

pub async fn perfom_ai_call(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    func: for<'a> fn(&'a str) -> &'static str,
) -> String {
    let extended_msg = extend_ai_function(func, &msg_context);

    PrintCommand::AICall.print_agent_msg(agent_position, agent_operation);

    let llm_response_res = call_gpt(vec![extended_msg.clone()]).await;

    match llm_response_res {
        Ok(resp) => resp,
        // Retry
        Err(_) => call_gpt(vec![extended_msg])
            .await
            .expect("Retry call to llm failed!"),
    }
}

pub async fn perfom_ai_call_decoded<T: DeserializeOwned>(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    func: for<'a> fn(&'a str) -> &'static str,
) -> T {
    let llm_resposne = perfom_ai_call(msg_context, agent_position, agent_operation, func).await;

    let json_string = extract_json_from_code_block(&llm_resposne);

    let decoded_reponse =
        serde_json::from_str::<T>(json_string).expect("Failed to decode response");

    decoded_reponse
}

pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
    let response = client.get(url).send().await?;

    Ok(response.status().as_u16())
}

pub fn read_code_template_content() -> Result<impl BufRead, std::io::Error> {
    let file = File::open(CODE_TEMPLATE_PATH)?;

    Ok(io::BufReader::new(file))
}

pub fn save_backend_code(mut reader: impl BufRead) -> Result<(), std::io::Error> {
    let file = File::create(EXEC_MAIN_PATH)?;
    let mut writer = BufWriter::new(file);

    io::copy(&mut reader, &mut writer)?;
    writer.flush()?;

    Ok(())
}

pub fn save_api_endpoints(mut reader: impl BufRead) -> Result<(), std::io::Error> {
    let file = File::create(SCHEMA_PATH)?;
    let mut writer = BufWriter::new(file);

    io::copy(&mut reader, &mut writer)?;
    writer.flush()?;

    Ok(())
}

fn extract_json_from_code_block(s: &str) -> &str {
    let s = s.trim();
    if s.starts_with("```") {
        if let Some(start) = s.find('{').or_else(|| s.find('[')) {
            if let Some(end) = s.rfind('}') {
                return &s[start..=end];
            }
            if let Some(end) = s.rfind(']') {
                return &s[start..=end];
            }
        }
    }

    s
}

#[cfg(test)]
mod tests {
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    use super::*;

    #[test]
    fn tests_extending_ai_function() {
        let extended_msg = extend_ai_function(convert_user_input_to_goal, "dummy input");
        assert_eq!(extended_msg.role, "system".to_string());
    }

    #[tokio::test]
    async fn tests_perform_ai_call() {
        let msg_context = "Build me a todo application".to_string();

        let result = perfom_ai_call(
            msg_context,
            "Managing agent",
            "Defining user requirements",
            convert_user_input_to_goal,
        )
        .await;

        assert!(result.len() > 20)
    }
}
