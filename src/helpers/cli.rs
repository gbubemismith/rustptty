use std::io::{stdin, stdout};

use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};

#[derive(PartialEq, Debug)]
pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue,
}

impl PrintCommand {
    pub fn print_agent_msg(&self, agent_position: &str, agent_statement: &str) {
        let mut stdout = stdout();

        let statement_color = match self {
            Self::AICall => Color::Green,
            Self::UnitTest => Color::DarkMagenta,
            Self::Issue => Color::Red,
        };

        stdout.execute(SetForegroundColor(Color::Yellow)).unwrap();
        print!("Agent: {}: ", agent_position);

        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        println!("{}", agent_statement);

        stdout.execute(ResetColor).unwrap();
    }
}

pub fn get_user_response(question: &str) -> String {
    let mut stdout = stdout();

    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("");
    println!("{}", question);

    stdout.execute(ResetColor).unwrap();

    let mut user_response = String::new();
    stdin()
        .read_line(&mut user_response)
        .expect("Failed to read user response");

    return user_response.trim().to_string();
}

#[cfg(test)]
mod tests {
    use super::PrintCommand;

    #[test]
    fn test_prints_agent_msg() {
        PrintCommand::AICall.print_agent_msg(
            "Managing agent",
            "This is a test message, processing something!",
        );
    }
}
