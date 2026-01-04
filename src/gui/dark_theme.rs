//! Dark theme support for Windows GUI

use windows::Win32::Foundation::{COLORREF, HWND as WinHWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Dwm::{DWMWA_USE_IMMERSIVE_DARK_MODE, DwmSetWindowAttribute};
use windows::Win32::Graphics::Gdi::{
	CreatePen, CreateSolidBrush, DT_CENTER, DT_SINGLELINE, DT_VCENTER, DeleteObject, DrawTextW, FillRect,
	GetStockObject, HBRUSH as WinHBRUSH, HDC as WinHDC, HGDIOBJ, NULL_BRUSH, OPAQUE, PS_SOLID, Rectangle,
	SelectObject, SetBkColor, SetBkMode, SetTextColor, TRANSPARENT,
};
use windows::Win32::UI::Controls::{DRAWITEMSTRUCT, ODS_SELECTED, ODT_BUTTON, SetWindowTheme};
use windows::Win32::UI::WindowsAndMessaging::{
	CallWindowProcW, EnumChildWindows, GWL_STYLE, GWLP_WNDPROC, GetClassNameW, GetWindowLongW, GetWindowTextW,
	SendMessageW, SetWindowLongPtrW, WM_DRAWITEM, WM_THEMECHANGED, WNDPROC,
};
use windows::core::PCWSTR;
use winsafe::gui::Brush;
use winsafe::{COLORREF as WinsafeCOLORREF, HBRUSH, HDC, HWND};

#[link(name = "kernel32")]
unsafe extern "system" {
	fn LoadLibraryW(lp_lib_file_name: *const u16) -> isize;
	fn GetProcAddress(h_module: isize, lp_proc_name: *const u8) -> *const ();
}

// Dark background color
const DARK_BG_COLOR: WinsafeCOLORREF = WinsafeCOLORREF::from_rgb(32, 32, 32);
const DARK_BG_COLORREF: COLORREF = COLORREF(0x0020_2020); // BGR format
const LIGHT_TEXT_COLORREF: COLORREF = COLORREF(0x00FF_FFFF);

// Button colors
const BTN_FACE_COLORREF: COLORREF = COLORREF(0x003D3D3D);
const BTN_BORDER_COLORREF: COLORREF = COLORREF(0x00808080);
const BTN_BORDER_FOCUSED_COLORREF: COLORREF = COLORREF(0x00D77800); // Windows accent blue (BGR)
const BTN_TEXT_COLORREF: COLORREF = COLORREF(0x00FFFFFF);

static mut DARK_BRUSH: WinHBRUSH = WinHBRUSH(std::ptr::null_mut());
static mut ORIGINAL_WNDPROC: WNDPROC = None;

unsafe extern "system" fn dark_button_subclass_proc(hwnd: WinHWND, msg: u32, wparam: WPARAM, lparam: LPARAM)
-> LRESULT
{
	if msg == WM_DRAWITEM {
		let dis = unsafe { &*(lparam.0 as *const DRAWITEMSTRUCT) };
		if dis.CtlType == ODT_BUTTON {
			draw_dark_button(dis);
			return LRESULT(1);
		}
	}
	unsafe { CallWindowProcW(ORIGINAL_WNDPROC, hwnd, msg, wparam, lparam) }
}

fn install_draw_item_hook(hwnd: WinHWND)
{
	unsafe {
		ORIGINAL_WNDPROC = Some(std::mem::transmute(SetWindowLongPtrW(
			hwnd,
			GWLP_WNDPROC,
			dark_button_subclass_proc as *const () as isize,
		)));
	}
}

const BS_TYPEMASK: i32 = 0x0000_000F;
const BS_OWNERDRAW: i32 = 0x0000_000B;

fn make_button_owner_drawn(hwnd: WinHWND)
{
	unsafe {
		let style = GetWindowLongW(hwnd, GWL_STYLE);
		let new_style = (style & !BS_TYPEMASK) | BS_OWNERDRAW;
		SetWindowLongPtrW(hwnd, GWL_STYLE, new_style as isize);
	}
}

fn draw_dark_button(dis: &DRAWITEMSTRUCT)
{
	let is_pressed = (dis.itemState.0 & ODS_SELECTED.0) != 0;
	let border_color = if is_pressed {
		BTN_BORDER_FOCUSED_COLORREF
	} else {
		BTN_BORDER_COLORREF
	};

	unsafe {
		let hdc = WinHDC(dis.hDC.0);
		if hdc.is_invalid() {
			return;
		}

		let face_brush = CreateSolidBrush(BTN_FACE_COLORREF);
		if face_brush.is_invalid() {
			return;
		}

		let border_pen = CreatePen(PS_SOLID, if is_pressed { 2 } else { 1 }, border_color);
		if border_pen.is_invalid() {
			let _ = DeleteObject(HGDIOBJ(face_brush.0));
			return;
		}

		FillRect(hdc, &dis.rcItem, face_brush);

		let old_pen = SelectObject(hdc, HGDIOBJ(border_pen.0));
		let null_brush = GetStockObject(NULL_BRUSH);
		let old_brush = SelectObject(hdc, null_brush);
		let _ = Rectangle(
			hdc,
			dis.rcItem.left,
			dis.rcItem.top,
			dis.rcItem.right,
			dis.rcItem.bottom,
		);
		SelectObject(hdc, old_pen);
		SelectObject(hdc, old_brush);

		let mut text: [u16; 64] = [0; 64];
		let len = GetWindowTextW(dis.hwndItem, &mut text);
		if len > 0 {
			SetTextColor(hdc, BTN_TEXT_COLORREF);
			SetBkMode(hdc, TRANSPARENT);

			let mut text_rect = dis.rcItem;
			DrawTextW(
				hdc,
				&mut text[..len as usize],
				&mut text_rect,
				DT_CENTER | DT_VCENTER | DT_SINGLELINE,
			);
		}

		let _ = DeleteObject(HGDIOBJ(face_brush.0));
		let _ = DeleteObject(HGDIOBJ(border_pen.0));
	}
}

pub fn set_dark_colors_for_static(hdc: &HDC) -> HBRUSH
{
	unsafe {
		let win_hdc = WinHDC(hdc.ptr() as *mut _);
		SetTextColor(win_hdc, LIGHT_TEXT_COLORREF);
		SetBkMode(win_hdc, OPAQUE);
		SetBkColor(win_hdc, DARK_BG_COLORREF);
		HBRUSH::from_ptr(DARK_BRUSH.0 as *mut _)
	}
}

pub fn create_dark_brush() -> Brush
{
	let brush = HBRUSH::CreateSolidBrush(DARK_BG_COLOR)
		.expect("Failed to create dark brush")
		.leak();
	unsafe { DARK_BRUSH = CreateSolidBrush(DARK_BG_COLORREF) };
	Brush::Handle(brush)
}

type FnAllowDarkModeForWindow = unsafe extern "system" fn(isize, bool) -> bool;
type FnSetPreferredAppMode = unsafe extern "system" fn(i32) -> i32;

static mut ALLOW_DARK_MODE_FOR_WINDOW: Option<FnAllowDarkModeForWindow> = None;

pub fn init_dark_mode_apis() -> bool
{
	unsafe {
		let uxtheme: [u16; 12] = [
			'u' as u16, 'x' as u16, 't' as u16, 'h' as u16, 'e' as u16, 'm' as u16, 'e' as u16, '.' as u16,
			'd' as u16, 'l' as u16, 'l' as u16, 0,
		];
		let h_uxtheme = LoadLibraryW(uxtheme.as_ptr());
		if h_uxtheme == 0 {
			return false;
		}

		let set_preferred_ptr = GetProcAddress(h_uxtheme, 135 as *const u8); // ordinal 135
		if set_preferred_ptr.is_null() {
			return false;
		}
		let set_preferred_app_mode: FnSetPreferredAppMode = std::mem::transmute(set_preferred_ptr);

		let allow_dark_ptr = GetProcAddress(h_uxtheme, 133 as *const u8); // ordinal 133
		if allow_dark_ptr.is_null() {
			return false;
		}
		ALLOW_DARK_MODE_FOR_WINDOW = Some(std::mem::transmute(allow_dark_ptr));
		set_preferred_app_mode(1);
		true
	}
}

fn apply_dark_mode_to_window(hwnd: WinHWND, control_type: ControlType)
{
	unsafe {
		if let Some(allow_dark) = ALLOW_DARK_MODE_FOR_WINDOW {
			allow_dark(hwnd.0 as isize, true);
		}
		match control_type {
			ControlType::Checkbox => {
				// No visual styles = WM_CTLCOLOR* works
				let _ = SetWindowTheme(hwnd, PCWSTR::null(), PCWSTR::null());
			}
			ControlType::Button => {
				let _ = SetWindowTheme(hwnd, windows::core::w!("DarkMode_Explorer"), PCWSTR::null());
			}
			ControlType::Other => {
				let _ = SetWindowTheme(hwnd, windows::core::w!("Explorer"), PCWSTR::null());
			}
		}
		SendMessageW(hwnd, WM_THEMECHANGED, Some(WPARAM(0)), Some(LPARAM(0)));
	}
}

#[derive(Clone, Copy)]
enum ControlType
{
	Checkbox,
	Button,
	Other,
}

// Button style constants
const BS_PUSHBUTTON: i32 = 0x0000_0000;
const BS_DEFPUSHBUTTON: i32 = 0x0000_0001;
const BS_CHECKBOX: i32 = 0x0000_0002;
const BS_AUTOCHECKBOX: i32 = 0x0000_0003;
const BS_RADIOBUTTON: i32 = 0x0000_0004;
const BS_3STATE: i32 = 0x0000_0005;
const BS_AUTO3STATE: i32 = 0x0000_0006;
const BS_AUTORADIOBUTTON: i32 = 0x0000_0009;

fn get_control_type(hwnd: WinHWND) -> ControlType
{
	let mut class_name: [u16; 16] = [0; 16];
	unsafe {
		let len = GetClassNameW(hwnd, &mut class_name);
		if len > 0 {
			let button: [u16; 7] = [
				'B' as u16, 'u' as u16, 't' as u16, 't' as u16, 'o' as u16, 'n' as u16, 0,
			];
			if class_name[..7] == button {
				let style = GetWindowLongW(hwnd, GWL_STYLE) & BS_TYPEMASK;
				return match style {
					BS_CHECKBOX | BS_AUTOCHECKBOX | BS_RADIOBUTTON | BS_3STATE | BS_AUTO3STATE
					| BS_AUTORADIOBUTTON => ControlType::Checkbox,
					BS_PUSHBUTTON | BS_DEFPUSHBUTTON => ControlType::Button,
					_ => ControlType::Other,
				};
			}
		}
	}
	ControlType::Other
}

unsafe extern "system" fn enum_child_proc(hwnd: WinHWND, _lparam: LPARAM) -> windows::core::BOOL
{
	let control_type = get_control_type(hwnd);
	match control_type {
		ControlType::Button => make_button_owner_drawn(hwnd),
		ControlType::Checkbox => apply_dark_mode_to_window(hwnd, ControlType::Checkbox),
		ControlType::Other => apply_dark_mode_to_window(hwnd, ControlType::Other),
	}
	windows::core::BOOL(1)
}

pub fn enable_dark_mode(hwnd: &HWND)
{
	let win_hwnd = WinHWND(hwnd.ptr() as *mut _);
	let value: u32 = 1;
	unsafe {
		// Dark title bar
		let _ = DwmSetWindowAttribute(
			win_hwnd,
			DWMWA_USE_IMMERSIVE_DARK_MODE,
			&value as *const u32 as *const std::ffi::c_void,
			std::mem::size_of::<u32>() as u32,
		);
	}

	install_draw_item_hook(win_hwnd);
	apply_dark_mode_to_window(win_hwnd, ControlType::Other);
	unsafe {
		let _ = EnumChildWindows(Some(win_hwnd), Some(enum_child_proc), LPARAM(0));
	}
}
