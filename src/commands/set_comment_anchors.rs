use zed_extension_api::{SlashCommandOutput, SlashCommandOutputSection};
use crate::models::{default_anchors, set_anchors, get_anchors};

pub fn handle_set_comment_anchors(_args: Vec<String>) -> Result<SlashCommandOutput, String> {
    // Reset to default anchors
    set_anchors(default_anchors());

    let anchors = get_anchors();
    let mut anchor_text = String::new();

    anchor_text.push_str("Comment anchors have been set to:\n\n");

    for (key, anchor) in anchors {
        anchor_text.push_str(&format!("- {}: {}\n", key, anchor.description));
    }

    Ok(SlashCommandOutput {
        sections: vec![SlashCommandOutputSection {
            range: (0..anchor_text.len()).into(),
            label: "Comment Anchors".to_string(),
        }],
        text: anchor_text,
    })
}
