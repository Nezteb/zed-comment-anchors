use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

#[derive(Clone, Debug)]
pub struct CommentAnchor {
    pub keyword: String,
    pub description: String,
}

// Default anchors as specified in README.md
pub fn default_anchors() -> HashMap<String, CommentAnchor> {
    let mut anchors = HashMap::new();

    anchors.insert("ANCHOR".to_string(), CommentAnchor {
        keyword: "ANCHOR".to_string(),
        description: "Used to indicate a section in your file".to_string(),
    });

    anchors.insert("TODO".to_string(), CommentAnchor {
        keyword: "TODO".to_string(),
        description: "An item that is awaiting completion".to_string(),
    });

    anchors.insert("FIXME".to_string(), CommentAnchor {
        keyword: "FIXME".to_string(),
        description: "An item that requires a bugfix".to_string(),
    });

    anchors.insert("STUB".to_string(), CommentAnchor {
        keyword: "STUB".to_string(),
        description: "Used for generated default snippets".to_string(),
    });

    anchors.insert("NOTE".to_string(), CommentAnchor {
        keyword: "NOTE".to_string(),
        description: "An important note for a specific code section".to_string(),
    });

    anchors.insert("REVIEW".to_string(), CommentAnchor {
        keyword: "REVIEW".to_string(),
        description: "An item that requires additional review".to_string(),
    });

    anchors
}

// In a real implementation, we would use Zed's storage API
// For now, we'll use a thread-safe static approach as a placeholder
static ANCHORS: OnceLock<Mutex<HashMap<String, CommentAnchor>>> = OnceLock::new();

fn get_anchors_instance() -> &'static Mutex<HashMap<String, CommentAnchor>> {
    ANCHORS.get_or_init(|| Mutex::new(default_anchors()))
}

pub fn get_anchors() -> HashMap<String, CommentAnchor> {
    match get_anchors_instance().lock() {
        Ok(guard) => guard.clone(),
        Err(_) => default_anchors(), // Fallback if mutex is poisoned
    }
}

pub fn set_anchors(anchors: HashMap<String, CommentAnchor>) {
    if let Ok(mut guard) = get_anchors_instance().lock() {
        *guard = anchors;
    }
}

// Represents an anchor found in a file
#[derive(Debug, Clone)]
pub struct AnchorOccurrence {
    pub anchor_type: String,
    pub file_path: String,
    pub line_number: usize,
    pub text: String,
}
