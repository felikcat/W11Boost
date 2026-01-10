use super::tweaks::defaults::RECOMMENDED_TWEAKS;
use std::collections::HashMap;

/// GUI view mode
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ViewMode
{
        #[default]
        Tweaks,
        ConfirmApply,

        ConfirmUnsetAll,
        ConfirmRestorePoint,
        SelectedTweaks,
}

/// Selection state for tree navigation
#[derive(Clone, Default)]
pub struct SelectionState
{
        pub selected_category: Option<String>,
        pub selected_tweak: Option<String>,
        pub expanded_categories: HashMap<String, bool>,
}

/// Checkbox states for all tweaks
pub struct TweakStates
{
        pub states: HashMap<String, bool>,
        pub input_values: HashMap<String, String>,
}

impl Default for TweakStates
{
        fn default() -> Self
        {
                let mut states = HashMap::new();
                for &id in RECOMMENDED_TWEAKS {
                        states.insert(id.to_string(), true);
                }
                Self {
                        states,
                        input_values: HashMap::new(),
                }
        }
}

/// Snapshot of navigation state for history
#[derive(Clone, PartialEq)]
pub struct NavigationEntry
{
        pub mode: ViewMode,
        pub selected_category: Option<String>,
        pub selected_tweak: Option<String>,
        pub search_query: String,
}
