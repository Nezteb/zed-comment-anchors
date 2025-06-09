// https://zed.dev/docs/extensions/developing-extensions#webassembly
use zed_extension_api::{self as zed, SlashCommand, SlashCommandOutput, Worktree};

mod commands;
mod models;

struct CommentAnchors;

impl zed::Extension for CommentAnchors {
    fn new() -> Self {
        CommentAnchors
    }

    fn run_slash_command(
        &self,
        command: SlashCommand,
        args: Vec<String>,
        worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        commands::handle_command(command, args, worktree)
    }
}

zed::register_extension!(CommentAnchors);
