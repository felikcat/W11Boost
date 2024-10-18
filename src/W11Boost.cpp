#include "Resource.h"
#include <windows.h>
#include <dwmapi.h>
#include <wtypes.h>
#include <shlwapi.h>
#include <cstdio>
import Edits;
import RestorePoint;
import PrivacyMode;
import MSStore;

/* Type notes:
    void * = HINTERNET;
    const wchar_t * = LPCWSTR;
    unsigned long = DWORD;
    unsigned __int64 = ULONGLONG;
    WCHAR = wchar_t
    ATOM = WORD = unsigned short
    UINT = size_t
    TCHAR = WCHAR = wchar_t
*/

#define MAX_LOADSTRING 100

// Global Variables:
HINSTANCE hInst;                       // current instance
wchar_t szTitle[MAX_LOADSTRING];       // The title bar text
wchar_t szWindowClass[MAX_LOADSTRING]; // the main window class name

// Forward declarations of functions included in this code module:
unsigned short MyRegisterClass(HINSTANCE hInstance);
bool InitInstance(HINSTANCE, size_t);
LRESULT CALLBACK WndProc(HWND, UINT, WPARAM, LPARAM);

int APIENTRY wWinMain(_In_ HINSTANCE hInstance,
                      _In_opt_ HINSTANCE hPrevInstance, _In_ LPWSTR lpCmdLine,
                      _In_ int nCmdShow) {
  UNREFERENCED_PARAMETER(hPrevInstance);
  UNREFERENCED_PARAMETER(lpCmdLine);

  // Place code that should always be ran here:
  wchar_t currentPath[260]; // MAX_PATH = 260
  GetModuleFileNameW(NULL, currentPath, 260);

  wchar_t *removeExe = wcsrchr(currentPath, L'\\');
  if (removeExe != NULL) {
    *removeExe = L'\0';
  }

  const wchar_t *dirName = L"W11Boost Logs";
  wchar_t fullPath[260];
  swprintf_s(fullPath, 260, L"%s\\%s", currentPath, dirName);

  // Ensure double-null termination
  size_t len = wcslen(fullPath);
  if (len + 2 < MAX_PATH) {
    fullPath[len + 1] = L'\0';
  }

  if (PathFileExistsW(fullPath)) {
    SHFILEOPSTRUCTW dir = {0};
    dir.wFunc = FO_DELETE;
    dir.pFrom = fullPath;
    dir.fFlags = FOF_NO_UI | FOF_NOERRORUI;
    SHFileOperationW(&dir);
  }

  CreateDirectoryW(fullPath, NULL);

  // Initialize global strings
  LoadStringW(hInstance, IDS_APP_TITLE, szTitle, MAX_LOADSTRING);
  LoadStringW(hInstance, IDC_W11BOOST, szWindowClass, MAX_LOADSTRING);
  MyRegisterClass(hInstance);

  // Perform application initialization:
  if (!InitInstance(hInstance, nCmdShow)) {
    return FALSE;
  }

  HACCEL hAccelTable =
      LoadAcceleratorsW(hInstance, MAKEINTRESOURCE(IDC_W11BOOST));

  MSG msg;

  // Main message loop:
  while (GetMessageW(&msg, NULL, 0, 0)) {
    if (!TranslateAcceleratorW(msg.hwnd, hAccelTable, &msg)) {
      TranslateMessage(&msg);
      DispatchMessageW(&msg);
    }
  }

  return (size_t)msg.wParam;
}

//
//  FUNCTION: MyRegisterClass()
//
//  PURPOSE: Registers the window class.
//
unsigned short MyRegisterClass(HINSTANCE hInstance) {
  WNDCLASSEXW wcex = {
      .cbSize = sizeof(WNDCLASSEX),
      .style = CS_HREDRAW | CS_VREDRAW,
      .lpfnWndProc = WndProc,
      .cbClsExtra = 0,
      .cbWndExtra = 0,
      .hInstance = hInstance,
      .hCursor = LoadCursor(NULL, IDC_ARROW),
      .hbrBackground = (HBRUSH)(COLOR_WINDOW + 1),
      .lpszMenuName = MAKEINTRESOURCEW(IDC_W11BOOST),
      .lpszClassName = szWindowClass,
  };

  return RegisterClassExW(&wcex);
}

//
//   FUNCTION: InitInstance(HINSTANCE, int)
//
//   PURPOSE: Saves instance handle and creates main window
//
//   COMMENTS:
//
//        In this function, we save the instance handle in a global variable and
//        create and display the main program window.
//
bool InitInstance(HINSTANCE hInstance, size_t nCmdShow) {
  hInst = hInstance; // Store instance handle in our global variable

  size_t width = 640;
  size_t height = 480;

  HWND hWnd = CreateWindowExW(
      0, szWindowClass, szTitle, WS_OVERLAPPED | WS_MINIMIZEBOX | WS_SYSMENU,
      CW_USEDEFAULT, 0, width, height, NULL, NULL, hInstance, NULL);

  if (!hWnd) {
    return FALSE;
  }

  HMONITOR monitor = MonitorFromWindow(hWnd, MONITOR_DEFAULTTONEAREST);
  MONITORINFO mi = {sizeof(mi)};

  // Puts W11Boost's window in the center of the current monitor
  if (GetMonitorInfoW(monitor, &mi)) {
    RECT rcWork = mi.rcWork;
    size_t x = rcWork.left + (rcWork.right - rcWork.left - width) / 2;
    size_t y = rcWork.top + (rcWork.bottom - rcWork.top - height) / 2;

    SetWindowPos(hWnd, NULL, x, y, 0, 0, SWP_NOSIZE | SWP_NOZORDER);
  }

  ShowWindow(hWnd, nCmdShow);
  UpdateWindow(hWnd);

  return TRUE;
}

//
//  FUNCTION: WndProc(HWND, UINT, WPARAM, LPARAM)
//
//  PURPOSE: Processes messages for the main window.
//
//  WM_COMMAND  - process the application menu
//  WM_PAINT    - Paint the main window
//  WM_DESTROY  - post a quit message and return
//
//
LRESULT CALLBACK WndProc(HWND hWnd, UINT message, WPARAM wParam,
                         LPARAM lParam) {
  switch (message) {
  case WM_CREATE: {
    size_t fontSize = 30;
    size_t dpi = GetDpiForWindow(hWnd);
    size_t fontDpi = 96;
    HFONT hFont = CreateFontW(MulDiv(fontSize, dpi, fontDpi), 0, 0, 0, FW_LIGHT,
                              FALSE, FALSE, 0, ANSI_CHARSET, OUT_OUTLINE_PRECIS,
                              CLIP_DEFAULT_PRECIS, CLEARTYPE_NATURAL_QUALITY,
                              DEFAULT_PITCH | FF_DONTCARE, L"Segoe UI");

    RECT rcClient;
    GetClientRect(hWnd, &rcClient);

    size_t buttonWidth = rcClient.right / 2;
    size_t buttonHeight = rcClient.bottom / 2;

    HWND hButton1 = CreateWindowW(
        L"BUTTON", L"Apply W11Boost",
        WS_CHILD | WS_VISIBLE | BS_PUSHBUTTON | BS_FLAT,
        rcClient.left + buttonWidth, rcClient.top, buttonWidth, buttonHeight,
        hWnd, (HMENU)IDC_APPLY_W11BOOST, GetModuleHandle(NULL), NULL);

    HWND hButton2 = CreateWindowW(
        L"BUTTON", L"Privacy Mode",
        WS_CHILD | WS_VISIBLE | BS_PUSHBUTTON | BS_FLAT, rcClient.left,
        rcClient.top + buttonHeight, buttonWidth, buttonHeight, hWnd,
        (HMENU)IDC_PRIVACY_MODE, GetModuleHandle(NULL), NULL);

    HWND hButton3 = CreateWindowW(
        L"BUTTON", L"Create a backup",
        WS_CHILD | WS_VISIBLE | BS_PUSHBUTTON | BS_FLAT,
        rcClient.left + buttonWidth, rcClient.top + buttonHeight, buttonWidth,
        buttonHeight, hWnd, (HMENU)IDC_CREATE_RESTORE_POINT,
        GetModuleHandle(NULL), NULL);

    HWND hButton4 = CreateWindowW(
        L"BUTTON", L"Add Microsoft Store",
        WS_CHILD | WS_VISIBLE | BS_PUSHBUTTON | BS_FLAT, rcClient.left,
        rcClient.top, buttonWidth, buttonHeight, hWnd,
        (HMENU)IDC_INSTALL_MICROSOFT_STORE, GetModuleHandle(NULL), NULL);

    SendMessageW(hButton1, WM_SETFONT, (WPARAM)hFont, TRUE);
    SendMessageW(hButton2, WM_SETFONT, (WPARAM)hFont, TRUE);
    SendMessageW(hButton3, WM_SETFONT, (WPARAM)hFont, TRUE);
    SendMessageW(hButton4, WM_SETFONT, (WPARAM)hFont, TRUE);
  } break;
  case WM_COMMAND: {
    size_t wmId = LOWORD(wParam);
    // Parse the selections:
    switch (wmId) {
    case IDC_APPLY_W11BOOST:
      if (MessageBoxW(hWnd, L"Are you sure you want to apply W11Boost?",
                      L"W11Boost", MB_YESNO) == IDYES) {
        size_t gpResult = gp_edits();
        if (gpResult == 0) {
          MessageBoxW(hWnd, L"Complete. Manually reboot to apply all changes.",
                      L"W11Boost", MB_OK);
        } else {
          MessageBoxW(hWnd, L"Failed to install W11Boost!", L"W11Boost", MB_OK);
        }
      }
      break;
    case IDC_CREATE_RESTORE_POINT:
      if (MessageBoxW(
              hWnd, L"Are you sure you want to create a System Restore point?",
              L"W11Boost", MB_YESNO) == IDYES) {
        size_t srResult = create_restore_point();
        if (srResult == 0) {
          MessageBoxW(hWnd, L"System Restore point successfully created.",
                      L"W11Boost", MB_OK);
        } else {
          MessageBoxW(hWnd, L"System Restore point failed to be created!",
                      L"W11Boost", MB_OK);
        }
      }
      break;
    case IDC_PRIVACY_MODE:
      if (MessageBoxW(hWnd,
                      L"This will hide as much activity as possible on your "
                      L"PC, do you want this?",
                      L"W11Boost", MB_YESNO) == IDYES) {
        size_t pmResult = install_privacy_mode();

        if (pmResult == 0) {
          MessageBoxW(hWnd, L"Successfully installed Privacy Mode.",
                      L"W11Boost", MB_OK);
        } else {
          MessageBoxW(hWnd, L"Failed to install Privacy Mode!", L"W11Boost",
                      MB_OK);
        }
      }
      break;
    case IDC_INSTALL_MICROSOFT_STORE:
      if (MessageBoxW(hWnd,
                      L"Do you wish to install the Microsoft Store and .appx + "
                      L".appxbundle support?",
                      L"W11Boost", MB_YESNO) == IDYES) {
        size_t msResult = install_microsoft_store();

        if (msResult == 0) {
          MessageBoxW(hWnd,
                      L"Completed. The Microsoft Store may take a few "
                      L"additional minutes to show up.",
                      L"W11Boost", MB_OK);
        } else {
          MessageBoxW(hWnd, L"The App Installer likely failed to install!",
                      L"W11Boost", MB_OK);
        }
      }
    default:
      return DefWindowProcW(hWnd, message, wParam, lParam);
    }
  } break;
  case WM_PAINT: {
    PAINTSTRUCT ps;
    HDC hdc = BeginPaint(hWnd, &ps);
    EndPaint(hWnd, &ps);
  } break;
  case WM_DESTROY:
    PostQuitMessage(0);
    break;
  default:
    return DefWindowProcW(hWnd, message, wParam, lParam);
  }
  return 0;
}
