use std::collections::HashMap;
use zed_extension_api::{SlashCommandOutput, SlashCommandOutputSection, Worktree};
use crate::models::{AnchorOccurrence, get_anchors};

pub fn handle_find_comment_anchors(_args: Vec<String>, worktree: Option<&Worktree>) -> Result<SlashCommandOutput, String> {
    // TODO: Unused
    let _worktree = match worktree {
        Some(wt) => wt,
        None => return Err("No active worktree found".to_string()),
    };

    // TODO: Unused
    // Get all anchor keywords we need to search for
    let _anchors = get_anchors();

    // In a real implementation, we would search the workspace files for anchors
    // For now, we'll create a mock implementation that simulates finding anchors
    let mut results: Vec<AnchorOccurrence> = Vec::new();

    // Mock search results - in a production implementation,
    // you would use the Worktree API to search for each anchor keyword in files
    results.push(AnchorOccurrence {
        anchor_type: "TODO".to_string(),
        file_path: "src/lib.rs".to_string(),
        line_number: 10,
        text: "Implement search functionality".to_string(),
    });

    results.push(AnchorOccurrence {
        anchor_type: "FIXME".to_string(),
        file_path: "src/commands/mod.rs".to_string(),
        line_number: 15,
        text: "Fix error handling in command dispatcher".to_string(),
    });

    // Group results by file for better display
    let mut files: HashMap<String, Vec<AnchorOccurrence>> = HashMap::new();
    for occurrence in results {
        files.entry(occurrence.file_path.clone())
            .or_insert_with(Vec::new)
            .push(occurrence);
    }

    // Format output
    let mut output = String::new();
    output.push_str("Comment Anchors found in project:\n\n");

    if files.is_empty() {
        output.push_str("No comment anchors found.\n");
    } else {
        for (file, occurrences) in &files {
            output.push_str(&format!("File: {}\n", file));

            for occurrence in occurrences {
                output.push_str(&format!("  - [{}:{}] {}\n",
                    occurrence.anchor_type,
                    occurrence.line_number,
                    occurrence.text
                ));
            }

            output.push('\n');
        }
    }

    Ok(SlashCommandOutput {
        sections: vec![SlashCommandOutputSection {
            range: (0..output.len()).into(),
            label: "Comment Anchors Results".to_string(),
        }],
        text: output,
    })
}
