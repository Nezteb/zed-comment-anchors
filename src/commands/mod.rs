pub mod echo;
pub mod pick_one;

use zed_extension_api::{SlashCommand, SlashCommandOutput, Worktree};

// A central function to dispatch commands to their respective handlers
pub fn handle_command(
    command: SlashCommand,
    args: Vec<String>,
    _worktree: Option<&Worktree>,
) -> Result<SlashCommandOutput, String> {
    match command.name.as_str() {
        "echo" => echo::handle_echo(args),
        "pick-one" => pick_one::handle_pick_one(args),
        unknown => Err(format!("unknown slash command: \"{unknown}\"")),
    }
}
