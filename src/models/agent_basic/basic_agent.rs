use crate::models::general::llm::Message;

use super::basic_trait::BasicTraits;

#[derive(Debug, PartialEq)]
pub enum AgentState {
    Discovery,
    Working,
    UnitTesting,
    Finished,
}

#[derive(Debug)]
pub struct BasicAgent {
    pub objective: String,
    pub position: String,
    pub state: AgentState,
    pub memory: Option<Vec<Message>>,
}

impl BasicTraits for BasicAgent {
    fn new(objective: &str, position: &str) -> Self {
        Self {
            objective: objective.to_string(),
            position: position.to_string(),
            state: AgentState::Discovery,
            memory: Some(vec![]),
        }
    }

    fn update_state(&mut self, new_state: AgentState) {
        self.state = new_state
    }

    fn get_objective(&self) -> &str {
        &self.objective
    }

    fn get_position(&self) -> &str {
        &self.position
    }

    fn get_state(&self) -> &AgentState {
        &self.state
    }

    fn get_memory(&self) -> Option<&Vec<Message>> {
        self.memory.as_ref()
    }
}
