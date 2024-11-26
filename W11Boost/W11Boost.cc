#include "Resource.h"
#include "Common.h"

/* Type notes:
    HINTERNET = void *
    LPCWSTR = const wchar_t *
    DWORD = unsigned long
    ULONGLONG = unsigned __int64
    WCHAR = wchar_t
    ATOM = WORD = unsigned short
    UINT = unsigned int
    TCHAR = WCHAR = wchar_t
*/

#define MAX_LOADSTRING 100

// Global Variables:
HINSTANCE hInst;                       // Current instance
wchar_t szTitle[MAX_LOADSTRING];       // The title bar text
wchar_t szWindowClass[MAX_LOADSTRING]; // The main window class name
struct
{
    UINT restorePoint, localPrivacy, microsoftStore, appxSupport, disableSleep;
} checkboxStatus;
int int_result;

// Forward declarations of functions included in this code module:
auto MyRegisterClass(HINSTANCE hInstance) -> unsigned short;
auto InitInstance(HINSTANCE, int) -> bool;
auto CALLBACK WndProc(HWND, unsigned int, WPARAM, LPARAM) -> LRESULT;

auto APIENTRY wWinMain(_In_ HINSTANCE hInstance, _In_opt_ HINSTANCE hPrevInstance, _In_ LPWSTR lpCmdLine,
                       _In_ int nCmdShow) -> int
{
    UNREFERENCED_PARAMETER(hPrevInstance);
    UNREFERENCED_PARAMETER(lpCmdLine);

    // Place code that should always be ran here:
    wchar_t* fullPath = get_log_directory();
    if (PathFileExistsW(fullPath))
    {
        SHFILEOPSTRUCTW dir = {};
        dir.wFunc = FO_DELETE;
        dir.pFrom = fullPath;
        dir.fFlags = FOF_NO_UI | FOF_NOERRORUI;
        SHFileOperationW(&dir);
    }
    CreateDirectoryW(fullPath, NULL);
    free(fullPath);

    // Initialize global strings
    LoadStringW(hInstance, IDS_APP_TITLE, szTitle, MAX_LOADSTRING);
    LoadStringW(hInstance, IDC_W11BOOST, szWindowClass, MAX_LOADSTRING);
    MyRegisterClass(hInstance);

    // Perform application initialization:
    if (!InitInstance(hInstance, nCmdShow))
    {
        return FALSE;
    }

    HACCEL hAccelTable = LoadAcceleratorsW(hInstance, MAKEINTRESOURCE(IDC_W11BOOST));

    MSG msg;

    // Main message loop:
    while (GetMessageW(&msg, NULL, 0, 0))
    {
        if (!TranslateAcceleratorW(msg.hwnd, hAccelTable, &msg))
        {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }

    return static_cast<int>(msg.wParam);
}

//
//  FUNCTION: MyRegisterClass()
//
//  PURPOSE: Registers the window class.
//
auto MyRegisterClass(HINSTANCE hInstance) -> unsigned short
{
    WNDCLASSEXW wcex = {
        .cbSize = sizeof(WNDCLASSEX),
        .style = CS_HREDRAW | CS_VREDRAW,
        .lpfnWndProc = WndProc,
        .cbClsExtra = 0,
        .cbWndExtra = 0,
        .hInstance = hInstance,
        .hIcon = NULL,
        .hCursor = LoadCursor(NULL, IDC_ARROW),
        .hbrBackground = (HBRUSH)(COLOR_WINDOW + 1),
        .lpszMenuName = MAKEINTRESOURCEW(IDC_W11BOOST),
        .lpszClassName = szWindowClass,
        .hIconSm = NULL,
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
auto InitInstance(HINSTANCE hInstance, int nCmdShow) -> bool
{
    hInst = hInstance; // Store instance handle in our global variable

    auto dpi = GetDpiForSystem();
    int width = MulDiv(640, dpi, 100);
    int height = MulDiv(480, dpi, 100);

    HWND hWnd = CreateWindowExW(0, szWindowClass, szTitle, WS_OVERLAPPED | WS_MINIMIZEBOX | WS_SYSMENU, CW_USEDEFAULT,
                                0, width, height, NULL, NULL, hInstance, NULL);

    if (!hWnd)
    {
        return FALSE;
    }

    HMONITOR monitor = MonitorFromWindow(hWnd, MONITOR_DEFAULTTONEAREST);
    MONITORINFO mi = {.cbSize = sizeof(mi), .rcMonitor = NULL, .rcWork = NULL, .dwFlags = NULL};

    // Puts W11Boost's window in the center of the current monitor
    if (GetMonitorInfoW(monitor, &mi))
    {
        RECT rcWork = mi.rcWork;
        long x = rcWork.left + (rcWork.right - rcWork.left - width) / 2;
        long y = rcWork.top + (rcWork.bottom - rcWork.top - height) / 2;

        SetWindowPos(hWnd, NULL, x, y, 0, 0, SWP_NOSIZE | SWP_NOZORDER);
    }

    ShowWindow(hWnd, nCmdShow);
    UpdateWindow(hWnd);

    return TRUE;
}

auto appx_void(HWND hWnd) -> void
{
    int_result = install_appx_support();

    if (int_result != 0)
        MessageBoxW(hWnd, L"Failed to install .appx/.appxbundle and WinGet support!", L"W11Boost -> Error", MB_OK);
}

//
//  FUNCTION: WndProc(HWND, UINT, WPARAM, LPARAM)
//
//  PURPOSE: Processes messages for the main window.
//
//  WM_COMMAND  - Process the application menu
//  WM_PAINT    - Paint the main window
//  WM_DESTROY  - Post a quit message and return
//
//
auto CALLBACK WndProc(HWND hWnd, unsigned int message, WPARAM wParam, LPARAM lParam) -> LRESULT
{
    switch (message)
    {
    case WM_CREATE: {
        struct s_font
        {
            int size, dpi;
        } font;

        font.size = 24;
        font.dpi = USER_DEFAULT_SCREEN_DPI;

        UINT screen_dpi = GetDpiForWindow(hWnd);

        int scale_font = MulDiv(font.size, screen_dpi, font.dpi);

        HFONT hFont =
            CreateFontW(scale_font, 0, 0, 0, FW_LIGHT, FALSE, FALSE, 0, ANSI_CHARSET, OUT_OUTLINE_PRECIS,
                        CLIP_DEFAULT_PRECIS, CLEARTYPE_NATURAL_QUALITY, DEFAULT_PITCH | FF_DONTCARE, L"Segoe UI");

        RECT rcClient;
        GetClientRect(hWnd, &rcClient);

        struct s_common
        {
            int left, right, top, bottom, centerWidth, centerHeight;
        } common;

        struct s_button
        {
            int width, height;
            HWND apply;
        } button;

        struct s_checkbox
        {
            int width, height;
            HWND localPrivacy, backup, store, appx, sleep;
        } checkbox;

        common.left = rcClient.left + 4;
        common.right = rcClient.right - 8;

        common.top = rcClient.top;
        common.bottom = rcClient.bottom - 4;

        common.centerWidth = common.right / 2;
        common.centerHeight = common.bottom / 2;

        button.width = common.right;
        button.height = (common.centerHeight * 4) / 10; // 40%

        checkbox.width = rcClient.right;
        checkbox.height = (common.centerHeight * 2) / 10; // 20%

        struct s_button const aButton = {
            .width = {},
            .height = {},
            .apply = CreateWindowW(L"BUTTON", L"Apply W11Boost", WS_CHILD | WS_VISIBLE | BS_PUSHBUTTON | BS_FLAT,
                                   common.left, common.bottom - button.height, button.width, button.height, hWnd,
                                   reinterpret_cast<HMENU>(IDC_APPLY_W11BOOST), GetModuleHandle(NULL), NULL),

        };
        struct s_checkbox const aCheckbox = {
            .width = {},
            .height = {},
            .localPrivacy =
                CreateWindowW(L"BUTTON", L"Reduce local data collection", WS_CHILD | WS_VISIBLE | BS_CHECKBOX | BS_FLAT,
                              common.left, common.top + (checkbox.height), checkbox.width, checkbox.height, hWnd,
                              reinterpret_cast<HMENU>(IDC_LOCAL_PRIVACY), GetModuleHandle(NULL), NULL),
            .backup = CreateWindowW(L"BUTTON", L"Create a system restore point",
                                    WS_CHILD | WS_VISIBLE | BS_CHECKBOX | BS_FLAT, common.left,
                                    common.top + (checkbox.height * 2), checkbox.width, checkbox.height, hWnd,
                                    reinterpret_cast<HMENU>(IDC_CREATE_RESTORE_POINT), GetModuleHandle(NULL), NULL),
            .store =
                CreateWindowW(L"BUTTON", L"Install the Microsoft Store", WS_CHILD | WS_VISIBLE | BS_CHECKBOX | BS_FLAT,
                              common.left, common.top + (checkbox.height * 3), checkbox.width, checkbox.height, hWnd,
                              reinterpret_cast<HMENU>(IDC_INSTALL_MICROSOFT_STORE), GetModuleHandle(NULL), NULL),
            .appx = CreateWindowW(L"BUTTON", L"Install .appx/.appxbundle support and WinGet",
                                  WS_CHILD | WS_VISIBLE | BS_CHECKBOX | BS_FLAT, common.left,
                                  common.top + (checkbox.height * 4), checkbox.width, checkbox.height, hWnd,
                                  reinterpret_cast<HMENU>(IDC_INSTALL_APPX_SUPPORT), (GetModuleHandle(NULL)), NULL),
            .sleep =
                CreateWindowW(L"BUTTON", L"Disable sleep and hibernate", WS_CHILD | WS_VISIBLE | BS_CHECKBOX | BS_FLAT,
                              common.left, common.top + (checkbox.height * 5), checkbox.width, checkbox.height, hWnd,
                              reinterpret_cast<HMENU>(IDC_DISABLE_SLEEP), GetModuleHandle(NULL), NULL),
        };

        HWND hButton[] = {aButton.apply};
        HWND hCheckbox[] = {aCheckbox.localPrivacy, aCheckbox.backup, aCheckbox.store, aCheckbox.appx, aCheckbox.sleep};

        HWND optionalText =
            CreateWindowW(L"STATIC", L"Optional:", WS_CHILD | WS_VISIBLE | BS_FLAT | BS_CENTER, common.left, common.top,
                          checkbox.width, checkbox.height, hWnd, NULL, ((GetModuleHandle(NULL))), NULL);
        SendMessageW(optionalText, WM_SETFONT, std::bit_cast<WPARAM>(hFont), TRUE);

        for (size_t i = 0; i < sizeof hButton / sizeof aButton.apply; ++i)
        {
            SendMessageW(hButton[i], WM_SETFONT, std::bit_cast<WPARAM>(hFont), TRUE);
        }

        for (size_t i = 0; i < sizeof hCheckbox / sizeof aCheckbox.localPrivacy; ++i)
        {
            SendMessageW(hCheckbox[i], WM_SETFONT, std::bit_cast<WPARAM>(hFont), TRUE);
        }

        CheckDlgButton(hWnd, IDC_LOCAL_PRIVACY, BST_CHECKED);
        CheckDlgButton(hWnd, IDC_CREATE_RESTORE_POINT, BST_CHECKED);
    }
    break;

    case WM_COMMAND: {
        int wmId = LOWORD(wParam);

        // Parse the selections:
        switch (wmId)
        {
        case IDC_APPLY_W11BOOST:
            if (MessageBoxW(hWnd, L"Do you want to apply W11Boost?", L"W11Boost", MB_YESNO) == IDYES)
            {
                checkboxStatus.restorePoint = IsDlgButtonChecked(hWnd, IDC_CREATE_RESTORE_POINT);
                checkboxStatus.localPrivacy = IsDlgButtonChecked(hWnd, IDC_LOCAL_PRIVACY);
                checkboxStatus.microsoftStore = IsDlgButtonChecked(hWnd, IDC_INSTALL_MICROSOFT_STORE);
                checkboxStatus.appxSupport = IsDlgButtonChecked(hWnd, IDC_INSTALL_APPX_SUPPORT);
                checkboxStatus.disableSleep = IsDlgButtonChecked(hWnd, IDC_DISABLE_SLEEP);

                if (checkboxStatus.restorePoint)
                {
                    int_result = create_restore_point();

                    if (int_result != 0)
                    {
                        MessageBoxW(hWnd, L"System Restore point failed to be created!", L"W11Boost -> Error", MB_OK);
                        return EXIT_FAILURE;
                    }
                }

                if (checkboxStatus.localPrivacy)
                {
                    int_result = install_privacy_mode();

                    if (int_result != 0)
                    {
                        MessageBoxW(hWnd, L"Failed to install 'Reduce local data collection'!", L"W11Boost -> Error",
                                    MB_OK);
                        return EXIT_FAILURE;
                    }
                }

                if (checkboxStatus.microsoftStore)
                {
                    wchar_t cmdLine[] = L"wsreset.exe -i";
                    int_result = start_command_and_wait(cmdLine);

                    if (int_result != 0)
                    {
                        MessageBoxW(hWnd, L"Failed to install the Microsoft Store!", L"W11Boost -> Error", MB_OK);
                        return EXIT_FAILURE;
                    }
                }

                if (checkboxStatus.microsoftStore != TRUE && checkboxStatus.appxSupport)
                {
                    if (MessageBoxW(hWnd,
                                    L"Are you certain the Microsoft Store is already installed? It is required for "
                                    L".appx/.appxbundle and WinGet support.",
                                    L"W11Boost", MB_YESNO) == IDYES)
                    {
                        appx_void(hWnd);
                    }
                }
                else if (checkboxStatus.microsoftStore && checkboxStatus.appxSupport)
                {
                    appx_void(hWnd);
                }

                if (checkboxStatus.disableSleep)
                {
                    int_result = disable_sleep();

                    if (int_result != 0)
                    {
                        MessageBoxW(hWnd, L"Disabling sleep and hibernation failed!", L"W11Boost -> Error", MB_OK);
                        return EXIT_FAILURE;
                    }
                }

                if (checkboxStatus.appxSupport)
                {
                    int_result = install_appx_support();

                    if (int_result != 0)
                    {
                        MessageBoxW(hWnd, L"Installing .appx support failed!", L"W11Boost -> Error", MB_OK);
                        return EXIT_FAILURE;
                    }
                }

                int_result = main_registry_edits();

                if (int_result == 0)
                {
                    MessageBoxW(hWnd, L"Complete. Manually reboot to apply all changes.", L"W11Boost", MB_OK);
                }
                else
                {
                    MessageBoxW(hWnd, L"Failed to install W11Boost!", L"W11Boost -> Error", MB_OK);
                }
            }
            break;

        case IDC_CREATE_RESTORE_POINT:
            checkboxStatus.restorePoint = IsDlgButtonChecked(hWnd, IDC_CREATE_RESTORE_POINT);

            if (checkboxStatus.restorePoint)
            {
                CheckDlgButton(hWnd, IDC_CREATE_RESTORE_POINT, BST_UNCHECKED);
            }
            else
            {
                CheckDlgButton(hWnd, IDC_CREATE_RESTORE_POINT, BST_CHECKED);
            }
            break;

        case IDC_LOCAL_PRIVACY:
            checkboxStatus.localPrivacy = IsDlgButtonChecked(hWnd, IDC_LOCAL_PRIVACY);

            if (checkboxStatus.localPrivacy)
            {
                CheckDlgButton(hWnd, IDC_LOCAL_PRIVACY, BST_UNCHECKED);
            }
            else
            {
                CheckDlgButton(hWnd, IDC_LOCAL_PRIVACY, BST_CHECKED);
            }
            break;

        case IDC_INSTALL_MICROSOFT_STORE:
            checkboxStatus.microsoftStore = IsDlgButtonChecked(hWnd, IDC_INSTALL_MICROSOFT_STORE);

            if (checkboxStatus.microsoftStore)
            {
                CheckDlgButton(hWnd, IDC_INSTALL_MICROSOFT_STORE, BST_UNCHECKED);
            }
            else
            {
                CheckDlgButton(hWnd, IDC_INSTALL_MICROSOFT_STORE, BST_CHECKED);
            }
            break;

        case IDC_INSTALL_APPX_SUPPORT:
            checkboxStatus.appxSupport = IsDlgButtonChecked(hWnd, IDC_INSTALL_APPX_SUPPORT);

            if (checkboxStatus.appxSupport)
            {
                CheckDlgButton(hWnd, IDC_INSTALL_APPX_SUPPORT, BST_UNCHECKED);
            }
            else
            {
                CheckDlgButton(hWnd, IDC_INSTALL_APPX_SUPPORT, BST_CHECKED);
            }
            break;

        case IDC_DISABLE_SLEEP:
            checkboxStatus.disableSleep = IsDlgButtonChecked(hWnd, IDC_DISABLE_SLEEP);

            if (checkboxStatus.disableSleep)
            {
                CheckDlgButton(hWnd, IDC_DISABLE_SLEEP, BST_UNCHECKED);
            }
            else
            {
                CheckDlgButton(hWnd, IDC_DISABLE_SLEEP, BST_CHECKED);
            }
            break;

        default:
            return DefWindowProcW(hWnd, message, wParam, lParam);
        }
        break;
    }

    case WM_PAINT: {
        PAINTSTRUCT ps;
        HDC hdc = BeginPaint(hWnd, &ps);

        if (hdc == NULL)
        {
            MessageBoxW(hWnd, L"BeginPaint failed", L"W11Boost -> Error", MB_OK | MB_ICONERROR);
            return EXIT_FAILURE;
        }

        FillRect(hdc, &ps.rcPaint, reinterpret_cast<HBRUSH>(COLOR_MENU + 1));
        EndPaint(hWnd, &ps);
    }
    break;
    case WM_DESTROY:
        PostQuitMessage(0);
        break;
    default:
        return DefWindowProcW(hWnd, message, wParam, lParam);
    }
    return 0;
}
