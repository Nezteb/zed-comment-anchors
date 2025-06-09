use zed_extension_api::{SlashCommandOutput, SlashCommandOutputSection};

pub fn handle_echo(args: Vec<String>) -> Result<SlashCommandOutput, String> {
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
