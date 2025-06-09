pub mod echo;
pub mod pick_one;
pub mod set_comment_anchors;
pub mod find_comment_anchors;

use zed_extension_api::{SlashCommand, SlashCommandOutput, Worktree};

// A central function to dispatch commands to their respective handlers
pub fn handle_command(
    command: SlashCommand,
    args: Vec<String>,
    worktree: Option<&Worktree>,
) -> Result<SlashCommandOutput, String> {
    match command.name.as_str() {
        "echo" => echo::handle_echo(args),
        "pick-one" => pick_one::handle_pick_one(args),
        "set-comment-anchors" => set_comment_anchors::handle_set_comment_anchors(args),
        "find-comment-anchors" => find_comment_anchors::handle_find_comment_anchors(args, worktree),
        unknown => Err(format!("unknown slash command: \"{unknown}\"")),
    }
}
