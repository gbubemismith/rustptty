use crate::models::general::llm::Message;

use super::basic_agent::AgentState;

pub trait BasicTraits {
    fn new(objective: &str, position: &str) -> Self;
    fn update_state(&mut self, new_state: AgentState);
    fn get_objective(&self) -> &str;
    fn get_position(&self) -> &str;
    fn get_state(&self) -> &AgentState;
    fn get_memory(&self) -> Option<&Vec<Message>>;
}
