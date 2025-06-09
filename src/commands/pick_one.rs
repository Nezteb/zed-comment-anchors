use zed_extension_api::{SlashCommandOutput, SlashCommandOutputSection};

pub fn handle_pick_one(args: Vec<String>) -> Result<SlashCommandOutput, String> {
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
