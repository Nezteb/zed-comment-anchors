// https://zed.dev/docs/extensions/developing-extensions#webassembly
use zed_extension_api::{
    self as zed, SlashCommand, SlashCommandOutput, SlashCommandOutputSection, Worktree,
};

struct CommentAnchors;

impl zed::Extension for CommentAnchors {
    fn new() -> Self {
        CommentAnchors
    }

    fn run_slash_command(
        &self,
        command: SlashCommand,
        args: Vec<String>,
        _worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        match command.name.as_str() {
            "echo" => {
                if args.is_empty() {
                    return Err("nothing to echo".to_string());
                }

                let text = args.join(" ");

                Ok(SlashCommandOutput {
                    sections: vec![SlashCommandOutputSection {
                        range: (0..text.len()).into(),
                        label: "Echo".to_string(),
                    }],
                    text,
                })
            }
            "pick-one" => {
                let Some(selection) = args.first() else {
                    return Err("no option selected".to_string());
                };

                match selection.as_str() {
                    "option-1" | "option-2" | "option-3" => {}
                    invalid_option => {
                        return Err(format!("{invalid_option} is not a valid option"));
                    }
                }

                let text = format!("You chose {selection}.");

                Ok(SlashCommandOutput {
                    sections: vec![SlashCommandOutputSection {
                        range: (0..text.len()).into(),
                        label: format!("Pick One: {selection}"),
                    }],
                    text,
                })
            }
            command => Err(format!("unknown slash command: \"{command}\"")),
        }
    }
}

zed::register_extension!(CommentAnchors);
