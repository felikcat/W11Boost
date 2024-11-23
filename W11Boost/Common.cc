import Common;
#define WIN32_LEAN_AND_MEAN
#define NTDDI_VERSION NTDDI_WIN10_RS4
#include <windows.h> // Always first
#include <UserEnv.h>
#include <Shlwapi.h>
#include <fcntl.h>
#include <shellapi.h>
#include <libloaderapi.h>
#include <wchar.h>
#include <time.h>
#include <stdio.h>
#include <string>
#include <iostream>
#include <sstream>
#include <ShlObj.h>
#include <ostream>

auto get_log_directory() -> wchar_t* {
  wchar_t currentPath[MAX_PATH + 1];
  GetModuleFileNameW(NULL, currentPath, MAX_PATH);

  wchar_t *removeExe = wcsrchr(currentPath, L'\\');
  if (removeExe != NULL) {
    *removeExe = L'\0';
  }

  const wchar_t *dirName = L"W11Boost Logs";
  wchar_t *fullPath = (wchar_t *)malloc(MAX_PATH * sizeof(wchar_t));

  if (fullPath != NULL) {
    swprintf_s(fullPath, MAX_PATH, L"%s\\%s", currentPath, dirName);
    // Ensure double-null termination
    size_t len = wcslen(fullPath);
    if (len != NULL && len + 2 < MAX_PATH) {
      fullPath[len + 1] = L'\0';
    }
  }

  return fullPath;
}

auto wide_string_to_utf8(const wchar_t *wide_string) -> utf8_result {
  if (wide_string == NULL || *wide_string == U'\0')
    return {NULL, false};

  int wide_length = 0;
  while (wide_string[wide_length] != U'\0')
    wide_length++;

  int size_needed = WideCharToMultiByte(CP_UTF8, 0, wide_string, wide_length, NULL, 0, NULL, NULL);
  if (size_needed <= 0)
    return {NULL, false};

  char *result = (char *)malloc(size_needed + 1);
  if (result == NULL)
    return {NULL, false};

  int converted_result = WideCharToMultiByte(CP_UTF8, 0, wide_string, wide_length, &result[0], size_needed, NULL, NULL);
  if (converted_result <= 0) {
    free(result);
    return {NULL, true};
  }
  result[size_needed] = '\0';
  return {result, false}; // if SUCCESS (0), proceed
}

auto log_registry(const wchar_t *subKey, const wchar_t *valueName, const char *typeName) -> int {
  wchar_t *currentDir = get_log_directory();
  wchar_t logLocation[MAX_PATH + 1];

  if (currentDir == NULL)
    return EXIT_FAILURE;

  swprintf_s(logLocation, MAX_PATH, L"%s\\Registry.log", currentDir);

  if (currentDir == NULL)
    return EXIT_FAILURE;

  FILE *logFile = NULL;
  errno_t err = _wfopen_s(&logFile, logLocation, L"a");
  if (err != 0 || logFile == NULL) {
    free(currentDir);
    return EXIT_FAILURE;
  }

  if (logFile) {
    time_t now;
    time(&now);

    struct tm timeInfo;
    localtime_s(&timeInfo, &now);

    char timeString[20]; // 19 char + 1 null terminator
    strftime(timeString, sizeof(timeString), "%d-%m-%Y %H:%M:%S", &timeInfo);

    utf8_result narrow_subKey = wide_string_to_utf8(subKey);
    utf8_result narrow_valueName = wide_string_to_utf8(valueName);

    if (!narrow_subKey.error && !narrow_valueName.error) {
      fprintf_s(logFile, "%s %s %s\\%s\n", timeString, typeName, narrow_subKey.string, narrow_valueName.string);
    }

    if (narrow_subKey.string)
      free(narrow_subKey.string);
    if (narrow_valueName.string)
      free(narrow_valueName.string);
  }

  fclose(logFile);
  free(currentDir);
  return EXIT_SUCCESS;
}

auto delete_directory_and_subfolders(LPCWSTR directory) -> long {
  wchar_t dir[MAX_PATH + 1];
  SHFILEOPSTRUCTW fos;

  wcscpy_s(dir, MAX_PATH, directory);

  fos.hwnd = NULL;
  fos.wFunc = FO_DELETE;
  fos.pFrom = dir;
  fos.pTo = NULL;
  fos.fFlags = FOF_NO_UI | FOF_NOERRORUI;

  return SHFileOperationW(&fos);
}

auto set_dword(HKEY hKey, const wchar_t *subKey, const wchar_t *valueName, const DWORD value) -> LSTATUS {
  result = RegCreateKeyExW(hKey, subKey, 0, NULL, REG_OPTION_NON_VOLATILE, KEY_WRITE, NULL, &hSubKey, NULL);

  if (result == ERROR_SUCCESS)
    result = RegSetValueExW(hSubKey, valueName, 0, REG_DWORD, (const BYTE *)&value, sizeof(DWORD));

  const char *typeName = " - DWORD: ";
  log_registry(subKey, valueName, typeName);

  result = RegCloseKey(hSubKey);
  return result;
}

auto set_string(HKEY hKey, const wchar_t *subKey, const wchar_t *valueName, const wchar_t *value) -> LSTATUS {
  if ((result = RegCreateKeyExW(hKey, subKey, 0, NULL, REG_OPTION_NON_VOLATILE, KEY_WRITE, NULL, &hSubKey, NULL)) !=
          ERROR_SUCCESS ||
      (result = RegSetValueExW(hSubKey, valueName, 0, REG_SZ, (const BYTE *)value,
                               (DWORD)wcslen(value) * sizeof(wchar_t))) != ERROR_SUCCESS ||
      (result = RegCloseKey(hSubKey)) != ERROR_SUCCESS)
    return EXCEPTION_EXECUTE_HANDLER;
  {
    const char *typeName = " - SZ: ";
    log_registry(subKey, valueName, typeName);
  }
  return result;
}

auto set_environment(HKEY hKey, const wchar_t *valueName, const wchar_t *value) -> LSTATUS {
  const wchar_t *subKey = L"SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment";
  result = RegOpenKeyExW(hKey, subKey, 0, KEY_ALL_ACCESS, &hSubKey);

  if (result == ERROR_SUCCESS)
    result = RegSetValueExW(hSubKey, valueName, 0, REG_EXPAND_SZ, (const BYTE *)value,
                            (DWORD)(wcslen(value) * sizeof(wchar_t)));

  const char *typeName = " - EXPAND_SZ: ";
  log_registry(subKey, valueName, typeName);

  result = RegCloseKey(hSubKey);
  return result;
}

auto remove_subkey(HKEY hKey, const wchar_t *subKey, const wchar_t *valueName) -> LSTATUS {
  result = RegOpenKeyExW(hKey, subKey, 0, KEY_WRITE, &hSubKey);

  if (result == ERROR_SUCCESS)
    result = RegDeleteValueW(hSubKey, valueName);

  const char *typeName = " - Removed subkey: ";
  log_registry(subKey, valueName, typeName);

  result = RegCloseKey(hSubKey);
  return result;
}

auto start_command_and_wait(wchar_t *cmdLine) -> int {
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

auto append_wchar_t(const wchar_t *original, const wchar_t *append, wchar_t *_result) -> bool {
  wcscpy_s(_result, MAX_PATH, original);
  if ((PathAppendW(_result, append)) == TRUE)
    return true;

  return false;
}

// This is for cURL
// "char *ptr" contains the contents
auto write_callback(char *ptr, size_t size, size_t nmemb, void *userdata) -> size_t {
  FILE *stream = (FILE *)userdata;
  return fwrite(ptr, size, nmemb, stream);
}

auto get_windows_path(const GUID folderID) -> std::wstring {
  wchar_t *path = NULL;
  HRESULT hr = SHGetKnownFolderPath(folderID, 0, NULL, &path);

  if (FAILED(hr))
    CoTaskMemFree(path);

  std::wstring pathResult(path);
  CoTaskMemFree(path);

  return pathResult;
}
