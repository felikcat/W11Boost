//! W11Boost library - exports common functions for integration testing.

pub mod common;

pub use common::{
	check_dword, delete_value, get_windows_path, registry_backup, remove_subkey, restore_from_backup,
	run_system_command, set_dword, set_string, CREATE_NO_WINDOW,
};
