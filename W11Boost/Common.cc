#include "Common.h"

#define MAX_PATH_DOUBLE_NULL (MAX_PATH + 2) // accounts for double-null termination

auto get_log_directory() -> wchar_t*
{
    wchar_t currentPath[MAX_PATH_DOUBLE_NULL];
    GetModuleFileNameW(NULL, currentPath, MAX_PATH);

    wchar_t* removeExe = wcsrchr(currentPath, L'\\');
    if (removeExe != NULL)
    {
        *removeExe = L'\0';
    }

    const wchar_t* dirName = L"W11Boost Logs";
    wchar_t* fullPath = static_cast<wchar_t*>(malloc(MAX_PATH_DOUBLE_NULL * sizeof(wchar_t)));

    if (fullPath != NULL)
    {
        swprintf_s(fullPath, MAX_PATH, L"%s\\%s", currentPath, dirName);
        // Ensure double-null termination
        size_t len = wcslen(fullPath);
        if (len != NULL && len + 2 < MAX_PATH_DOUBLE_NULL)
        {
            fullPath[len + 1] = L'\0';
        }
    }

    return fullPath;
}

auto wide_string_to_utf8(const wchar_t* wide_string) -> utf8_result
{
    if (wide_string == NULL || *wide_string == U'\0')
        return {NULL, true};

    int wide_length = 0;
    while (wide_string[wide_length] != U'\0')
        wide_length++;

    int size_needed = WideCharToMultiByte(CP_UTF8, 0, wide_string, wide_length, NULL, 0, NULL, NULL);
    if (size_needed <= 0)
        return {NULL, true};

    char* result = static_cast<char*>(malloc(size_needed + 2));
    if (result == NULL)
        return {NULL, true};

    int converted_result =
        WideCharToMultiByte(CP_UTF8, 0, wide_string, wide_length, &result[0], size_needed, NULL, NULL);
    if (converted_result <= 0)
    {
        free(result);
        return {NULL, true};
    }
    result[size_needed] = '\0';
    return {result, false}; // if SUCCESS (0 / false), proceed
}

auto log_registry(const wchar_t* subkey, const wchar_t* value_name, const char* type_name) -> int
{
    wchar_t* current_dir = get_log_directory();
    wchar_t log_location[MAX_PATH_DOUBLE_NULL];
    FILE* log_file = NULL;
    utf8_result narrow_subkey{};
    utf8_result narrow_value_name{};

    __try
    {
        if (current_dir == NULL)
            __leave;

        swprintf_s(log_location, MAX_PATH, L"%s\\Registry.log", current_dir);

        if (current_dir == NULL)
            __leave;

        errno_t err = _wfopen_s(&log_file, log_location, L"a");
        if (err != 0 || log_file == NULL)
            __leave;

        if (log_file)
        {
            time_t now;
            time(&now);

            struct tm time_info;
            localtime_s(&time_info, &now);

            char time_string[21]; // 19 char + double null terminator
            strftime(time_string, sizeof(time_string), "%d-%m-%Y %H:%M:%S", &time_info);

            utf8_result narrow_subkey = wide_string_to_utf8(subkey);
            utf8_result narrow_value_name = wide_string_to_utf8(value_name);

            if (!narrow_subkey.return_code && !narrow_value_name.return_code)
            {
                fprintf_s(log_file, "%s %s %s\\%s\n", time_string, type_name, narrow_subkey.string,
                          narrow_value_name.string);
            }
        }
    }
    __finally
    {
        if (log_file)
            fclose(log_file);
        if (current_dir)
            free(current_dir);
        if (narrow_subkey.string)
            free(narrow_subkey.string);
        if (narrow_value_name.string)
            free(narrow_value_name.string);
    }
    return EXIT_SUCCESS;
}

auto delete_directory_and_subfolders(LPCWSTR directory) -> long
{
    wchar_t dir[MAX_PATH_DOUBLE_NULL];
    SHFILEOPSTRUCTW fos;

    wcscpy_s(dir, MAX_PATH, directory);

    fos.hwnd = NULL;
    fos.wFunc = FO_DELETE;
    fos.pFrom = dir;
    fos.pTo = NULL;
    fos.fFlags = FOF_NO_UI | FOF_NOERRORUI;

    return SHFileOperationW(&fos);
}

auto set_dword(HKEY hKey, const wchar_t* subkey, const wchar_t* value_name, const DWORD value) -> LSTATUS
{
    HKEY hSubKey;
    long_result = RegCreateKeyExW(hKey, subkey, 0, NULL, REG_OPTION_NON_VOLATILE, KEY_WRITE, NULL, &hSubKey, NULL);

    if (long_result == ERROR_SUCCESS)
        long_result =
            RegSetValueExW(hSubKey, value_name, 0, REG_DWORD, std::bit_cast<const BYTE*>(&value), sizeof(DWORD));

    const char* type_name = " - DWORD: ";
    log_registry(subkey, value_name, type_name);

    long_result = RegCloseKey(hSubKey);
    return long_result;
}

auto set_string(HKEY hKey, const wchar_t* subkey, const wchar_t* value_name, const wchar_t* value) -> LSTATUS
{
    HKEY hSubKey;
    if ((long_result = RegCreateKeyExW(hKey, subkey, 0, NULL, REG_OPTION_NON_VOLATILE, KEY_WRITE, NULL, &hSubKey,
                                       NULL)) != ERROR_SUCCESS ||
        (long_result = RegSetValueExW(hSubKey, value_name, 0, REG_SZ, std::bit_cast<const BYTE*>(value),
                                      static_cast<DWORD>(wcslen(value) * sizeof(wchar_t)))) != ERROR_SUCCESS ||
        (long_result = RegCloseKey(hSubKey)) != ERROR_SUCCESS)
        return EXCEPTION_EXECUTE_HANDLER;
    {
        const char* type_name = " - SZ: ";
        log_registry(subkey, value_name, type_name);
    }
    return long_result;
}

auto set_environment(HKEY hKey, const wchar_t* value_name, const wchar_t* value) -> LSTATUS
{
    HKEY hSubKey;
    const wchar_t* subkey = L"SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment";
    long_result = RegOpenKeyExW(hKey, subkey, 0, KEY_ALL_ACCESS, &hSubKey);

    if (long_result == ERROR_SUCCESS)
        long_result = RegSetValueExW(hSubKey, value_name, 0, REG_EXPAND_SZ, std::bit_cast<const BYTE*>(value),
                                     static_cast<DWORD>(wcslen(value) * sizeof(wchar_t)));

    const char* type_name = " - EXPAND_SZ: ";
    log_registry(subkey, value_name, type_name);

    long_result = RegCloseKey(hSubKey);
    return long_result;
}

auto remove_subkey(HKEY hKey, const wchar_t* subkey, const wchar_t* value_name) -> LSTATUS
{
    HKEY hSubKey;
    long_result = RegOpenKeyExW(hKey, subkey, 0, KEY_WRITE, &hSubKey);

    if (long_result == ERROR_SUCCESS)
        long_result = RegDeleteValueW(hSubKey, value_name);

    const char* type_name = " - Removed subkey: ";
    log_registry(subkey, value_name, type_name);

    long_result = RegCloseKey(hSubKey);
    return long_result;
}

auto start_command_and_wait(wchar_t* cmdLine) -> int
{
    STARTUPINFOW si{};
    PROCESS_INFORMATION pi{};

    SecureZeroMemory(&si, sizeof(si));
    si.cb = sizeof(si);
    SecureZeroMemory(&pi, sizeof(pi));

    if (!CreateProcessW(NULL, cmdLine, NULL, NULL, TRUE, CREATE_NO_WINDOW, NULL, NULL, &si, &pi))
        return EXIT_FAILURE;

    WaitForSingleObject(pi.hProcess, INFINITE);

    CloseHandle(pi.hProcess);
    CloseHandle(pi.hThread);
    return EXIT_SUCCESS;
}

auto append_wchar_t(const wchar_t* original, const wchar_t* append, wchar_t* _result) -> bool
{
    wcscpy_s(_result, MAX_PATH, original);
    if ((PathAppendW(_result, append)) == TRUE)
        return true;

    return false;
}

// This is for cURL
// "char *ptr" contains the contents
auto write_callback(char* ptr, size_t size, size_t nmemb, void* userdata) -> size_t
{
    FILE* stream = static_cast<FILE*>(userdata);
    return fwrite(ptr, size, nmemb, stream);
}

auto get_windows_path(const GUID folderID) -> std::wstring
{
    wchar_t* path = NULL;
    HRESULT hr = SHGetKnownFolderPath(folderID, 0, NULL, &path);

    if (FAILED(hr))
        CoTaskMemFree(path);

    std::wstring pathResult(path);
    CoTaskMemFree(path);

    return pathResult;
}
