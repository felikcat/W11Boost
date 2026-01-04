// Feature modules
mod dark_theme;
mod disable_copilot;
mod disable_recall;
mod disable_sleep;
mod disable_telemetry;
mod minimize_forensics;
mod minimize_online_data_collection;
mod non_intrusive_tweaks;
mod remove_w11boost;
mod reset_windows_store;

use std::cell::RefCell;
use std::rc::Rc;

use anyhow::{Result, anyhow};
use windows::Win32::Foundation::{HWND as WinHWND, LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{SendMessageW, WM_UPDATEUISTATE};
use winsafe::co::{SW, WS};
use winsafe::gui::{Button, ButtonOpts, CheckBox, CheckBoxOpts, Label, LabelOpts, WindowMain, WindowMainOpts};
use winsafe::prelude::*;

use dark_theme::{create_dark_brush, enable_dark_mode, init_dark_mode_apis, set_dark_colors_for_static};

const UIS_SET: u32 = 1;
const UISF_HIDEFOCUS: u32 = 0x1;

const WINDOW_WIDTH: i32 = 480;
const WINDOW_HEIGHT: i32 = 400;

#[derive(Clone, Copy, PartialEq)]
enum UiMode
{
	Normal,
	ConfirmApply,
	ConfirmRemove,
}

struct App
{
	mode: UiMode,
	// Status
	status: Label,
	// Main buttons
	btn_apply: Button,
	btn_remove: Button,
	// Confirmation buttons
	btn_yes: Button,
	btn_no: Button,
	// Feature checkboxes
	cb_minimize_forensics: CheckBox,
	cb_minimize_online: CheckBox,
	cb_disable_recall: CheckBox,
	cb_disable_copilot: CheckBox,
	cb_disable_telemetry: CheckBox,
	cb_disable_sleep: CheckBox,
	cb_install_store: CheckBox,
}

impl App
{
	fn set_mode(&mut self, mode: UiMode, message: &str)
	{
		self.mode = mode;
		let _ = self.status.hwnd().SetWindowText(message);

		let (main_vis, confirm_vis) = match mode {
			UiMode::Normal => (SW::SHOW, SW::HIDE),
			_ => (SW::HIDE, SW::SHOW),
		};

		self.btn_apply.hwnd().ShowWindow(main_vis);
		self.btn_remove.hwnd().ShowWindow(main_vis);
		self.btn_yes.hwnd().ShowWindow(confirm_vis);
		self.btn_no.hwnd().ShowWindow(confirm_vis);
	}

	fn apply_features(&self) -> Result<()>
	{
		non_intrusive_tweaks::run()?;

		if self.cb_minimize_forensics.is_checked() {
			minimize_forensics::run()?;
		}
		if self.cb_minimize_online.is_checked() {
			minimize_online_data_collection::run()?;
		}
		if self.cb_disable_recall.is_checked() {
			disable_recall::run()?;
		}
		if self.cb_disable_copilot.is_checked() {
			disable_copilot::run()?;
		}
		if self.cb_disable_telemetry.is_checked() {
			disable_telemetry::run()?;
		}
		if self.cb_disable_sleep.is_checked() {
			disable_sleep::run()?;
		}
		if self.cb_install_store.is_checked() {
			reset_windows_store::run()?;
		}

		Ok(())
	}

	fn handle_confirm(&mut self)
	{
		let mode = self.mode;
		let result = match mode {
			UiMode::ConfirmApply => self.apply_features(),
			UiMode::ConfirmRemove => remove_w11boost::run(),
			UiMode::Normal => return,
		};

		let message = match result {
			Ok(()) => match mode {
				UiMode::ConfirmApply => "Done! Reboot for changes to take full effect.",
				UiMode::ConfirmRemove => "Removed! Reboot for changes to take full effect.",
				UiMode::Normal => "",
			},
			Err(ref e) => {
				self.set_mode(UiMode::Normal, &format!("Error: {e}"));
				return;
			}
		};

		self.set_mode(UiMode::Normal, message);
	}
}

pub fn draw_gui() -> Result<()>
{
	let _ = init_dark_mode_apis();

	let wnd = WindowMain::new(WindowMainOpts {
		title: "W11Boost",
		size: (WINDOW_WIDTH, WINDOW_HEIGHT),
		class_bg_brush: create_dark_brush(),
		style: WS::CAPTION | WS::SYSMENU | WS::MINIMIZEBOX,
		..Default::default()
	});

	// Layout constants
	let y_start = 20;
	let spacing = 32;
	let x = 20;
	let cb_width = 350;
	let cb_height = 24;
	let button_y = WINDOW_HEIGHT - 60;
	let button_width = 180;
	let button_height = 36;
	let confirm_btn_width = 100;

	// Create app state with all controls
	let app = Rc::new(RefCell::new(App {
		mode: UiMode::Normal,
		status: Label::new(
			&wnd,
			LabelOpts {
				text: "",
				position: (20, WINDOW_HEIGHT - 110),
				size: (WINDOW_WIDTH - 40, 40),
				..Default::default()
			},
		),
		btn_apply: Button::new(
			&wnd,
			ButtonOpts {
				text: "Apply W11Boost",
				position: (40, button_y),
				width: button_width,
				height: button_height,
				..Default::default()
			},
		),
		btn_remove: Button::new(
			&wnd,
			ButtonOpts {
				text: "Remove W11Boost",
				position: (WINDOW_WIDTH - button_width - 40, button_y),
				width: button_width,
				height: button_height,
				..Default::default()
			},
		),
		btn_yes: Button::new(
			&wnd,
			ButtonOpts {
				text: "Yes",
				position: (WINDOW_WIDTH / 2 - confirm_btn_width - 20, button_y),
				width: confirm_btn_width,
				height: button_height,
				..Default::default()
			},
		),
		btn_no: Button::new(
			&wnd,
			ButtonOpts {
				text: "No",
				position: (WINDOW_WIDTH / 2 + 20, button_y),
				width: confirm_btn_width,
				height: button_height,
				..Default::default()
			},
		),
		cb_minimize_forensics: CheckBox::new(
			&wnd,
			CheckBoxOpts {
				text: "Minimize forensics / local data",
				position: (x, y_start),
				size: (cb_width, cb_height),
				..Default::default()
			},
		),
		cb_minimize_online: CheckBox::new(
			&wnd,
			CheckBoxOpts {
				text: "Minimize Microsoft online data",
				position: (x, y_start + spacing),
				size: (cb_width, cb_height),
				..Default::default()
			},
		),
		cb_disable_recall: CheckBox::new(
			&wnd,
			CheckBoxOpts {
				text: "Disable Windows Recall",
				position: (x, y_start + spacing * 2),
				size: (cb_width, cb_height),
				..Default::default()
			},
		),
		cb_disable_copilot: CheckBox::new(
			&wnd,
			CheckBoxOpts {
				text: "Disable Windows Copilot",
				position: (x, y_start + spacing * 3),
				size: (cb_width, cb_height),
				..Default::default()
			},
		),
		cb_disable_telemetry: CheckBox::new(
			&wnd,
			CheckBoxOpts {
				text: "Disable telemetry in various programs",
				position: (x, y_start + spacing * 4),
				size: (cb_width, cb_height),
				..Default::default()
			},
		),
		cb_disable_sleep: CheckBox::new(
			&wnd,
			CheckBoxOpts {
				text: "Disable sleep and hibernate",
				position: (x, y_start + spacing * 5),
				size: (cb_width, cb_height),
				..Default::default()
			},
		),
		cb_install_store: CheckBox::new(
			&wnd,
			CheckBoxOpts {
				text: "Install Microsoft Store",
				position: (x, y_start + spacing * 6),
				size: (cb_width, cb_height),
				..Default::default()
			},
		),
	}));

	// Window creation handler
	let app_init = app.clone();
	let wnd_init = wnd.clone();
	wnd.on().wm_create(move |_| {
		let a = app_init.borrow();
		enable_dark_mode(&wnd_init.hwnd());
		a.cb_disable_telemetry.set_check(true);
		a.btn_yes.hwnd().ShowWindow(SW::HIDE);
		a.btn_no.hwnd().ShowWindow(SW::HIDE);

		let win_hwnd = WinHWND(wnd_init.hwnd().ptr() as *mut _);
		unsafe {
			SendMessageW(
				win_hwnd,
				WM_UPDATEUISTATE,
				Some(WPARAM((UIS_SET | (UISF_HIDEFOCUS << 16)) as usize)),
				Some(LPARAM(0)),
			);
		}
		Ok(0)
	});

	// Color handlers
	wnd.on().wm_ctl_color_static(move |msg| Ok(set_dark_colors_for_static(&msg.hdc)));
	wnd.on().wm_ctl_color_btn(move |msg| Ok(set_dark_colors_for_static(&msg.hdc)));

	// Button handlers - each only needs one clone of app
	let app_apply = app.clone();
	app.borrow().btn_apply.on().bn_clicked(move || {
		app_apply.borrow_mut().set_mode(UiMode::ConfirmApply, "Apply W11Boost changes?");
		Ok(())
	});

	let app_remove = app.clone();
	app.borrow().btn_remove.on().bn_clicked(move || {
		app_remove.borrow_mut().set_mode(UiMode::ConfirmRemove, "Remove W11Boost and restore settings?");
		Ok(())
	});

	let app_yes = app.clone();
	app.borrow().btn_yes.on().bn_clicked(move || {
		app_yes.borrow_mut().handle_confirm();
		Ok(())
	});

	let app_no = app.clone();
	app.borrow().btn_no.on().bn_clicked(move || {
		app_no.borrow_mut().set_mode(UiMode::Normal, "");
		Ok(())
	});

	wnd.run_main(None).map(|_| ()).map_err(|e| anyhow!("Window error: {e}"))
}
