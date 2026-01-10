//! W11Boost library - exports common functions for integration testing.

pub mod common;
pub mod gui;
pub mod ipc;
pub mod service;
pub mod service_client;
pub mod trusted_installer;

pub use common::{
        CREATE_NO_WINDOW, check_dword, delete_value, get_windows_path, init_registry_gpo, remove_subkey,
        run_powershell_command, run_system_command, run_system_command_output, save_registry_gpo, set_binary,
        set_dword, set_dword_gpo, set_expand_sz, set_string, set_string_gpo,
};
