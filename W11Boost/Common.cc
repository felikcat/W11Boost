#include "Common.h"
#include <UserEnv.h>
#include <Shlwapi.h>
#include <fcntl.h>
#include <Shlwapi.h>
#include <shellapi.h>
#include <libloaderapi.h>
#include <wchar.h>
#include <time.h>
#include <stdio.h>

wchar_t *get_log_directory() {
  wchar_t currentPath[MAX_PATH];
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

utf8_result wide_string_to_utf8(const wchar_t *wide_string) {
  if (wide_string == NULL || *wide_string == U'\0')
    return {NULL, false};

  int wide_length = 0;
  while (wide_string[wide_length] != U'\0')
    wide_length++;

  int size_needed = WideCharToMultiByte(CP_UTF8, 0, wide_string, wide_length,
                                           NULL, 0, NULL, NULL);
  if (size_needed <= 0)
    return {NULL, false};

  char *result = (char *)malloc(size_needed + 1);
  if (result == NULL)
    return {NULL, false};

  int converted_result =
      WideCharToMultiByte(CP_UTF8, 0, wide_string, wide_length, &result[0],
                          size_needed, NULL, NULL);
  if (converted_result <= 0) {
    free(result);
    return {NULL, true};
  }
  result[size_needed] = '\0';
  return {result, false}; // if SUCCESS (0), proceed
}

int log_registry(const wchar_t *subKey, const wchar_t *valueName,
                 const char *typeName) {
  wchar_t *currentDir = get_log_directory();
  wchar_t logLocation[MAX_PATH];

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
      fprintf_s(logFile, "%s %s %s\\%s\n", timeString, typeName,
                narrow_subKey.string, narrow_valueName.string);
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

LSTATUS set_dword(HKEY hKey, const wchar_t *subKey, const wchar_t *valueName,
                  const DWORD value) {
  result = RegCreateKeyExW(hKey, subKey, 0, NULL, REG_OPTION_NON_VOLATILE,
                           KEY_WRITE, NULL, &hSubKey, NULL);

  if (result == ERROR_SUCCESS)
    result = RegSetValueExW(hSubKey, valueName, 0, REG_DWORD,
                            (const BYTE *)&value, sizeof(DWORD));

  const char *typeName = " - DWORD: ";
  log_registry(subKey, valueName, typeName);

  result = RegCloseKey(hSubKey);
  return result;
}

LSTATUS set_string(HKEY hKey, const wchar_t *subKey, const wchar_t *valueName,
                   const wchar_t *value) {
  result = RegCreateKeyExW(hKey, subKey, 0, NULL, REG_OPTION_NON_VOLATILE,
                           KEY_WRITE, NULL, &hSubKey, NULL);

  if (result != ERROR_SUCCESS)
    return EXCEPTION_EXECUTE_HANDLER;

  result = RegSetValueExW(hSubKey, valueName, 0, REG_SZ, (const BYTE *)value,
                          sizeof(wchar_t));

  if (result != ERROR_SUCCESS)
    return EXCEPTION_EXECUTE_HANDLER;

  result = RegCloseKey(hSubKey);

  if (result != ERROR_SUCCESS)
    return EXCEPTION_EXECUTE_HANDLER;

  const char *typeName = " - SZ: ";
  log_registry(subKey, valueName, typeName);
  return EXIT_SUCCESS;
}

LSTATUS set_environment(HKEY hKey, const wchar_t *valueName,
                        const wchar_t *value) {
  const wchar_t *subKey =
      L"SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment";
  result = RegOpenKeyExW(hKey, subKey, 0, KEY_ALL_ACCESS, &hSubKey);

  if (result == ERROR_SUCCESS)
    result =
        RegSetValueExW(hSubKey, valueName, 0, REG_EXPAND_SZ,
                       (const BYTE *)value, (DWORD)(wcslen(value) * sizeof(wchar_t)));

  const char *typeName = " - EXPAND_SZ: ";
  log_registry(subKey, valueName, typeName);

  result = RegCloseKey(hSubKey);
  return result;
}

LSTATUS remove_subkey(HKEY hKey, const wchar_t *subKey,
                      const wchar_t *valueName) {
  HKEY hSubKey;
  result = RegOpenKeyExW(hKey, subKey, 0, KEY_WRITE, &hSubKey);

  if (result == ERROR_SUCCESS)
    result = RegDeleteValueW(hSubKey, valueName);

  const char *typeName = " - Removed subkey: ";
  log_registry(subKey, valueName, typeName);

  result = RegCloseKey(hSubKey);
  return result;
}

int gp_cleanup(HRESULT hr) {
  RegCloseKey(hKey);

  if (SUCCEEDED(hr))
    hr = pGPO->Save(TRUE, TRUE, &RegistryID, (GUID *)&_CLSID_GPESnapIn);

  if (FAILED(hr))
    return EXIT_FAILURE;

  // Apply new policy objects to the registry
  RefreshPolicyEx(TRUE, RP_FORCE);

  if (pGPO)
    pGPO->Release();

  return EXIT_SUCCESS;
}

int start_command_and_wait(wchar_t *cmdLine) {
  STARTUPINFOW si;
  PROCESS_INFORMATION pi;

  SecureZeroMemory(&si, sizeof(si));
  si.cb = sizeof(si);
  SecureZeroMemory(&pi, sizeof(pi));

  if (!CreateProcessW(NULL, cmdLine, NULL, NULL, TRUE, CREATE_NO_WINDOW, NULL,
                      NULL, &si, &pi))
    return EXIT_FAILURE;

  WaitForSingleObject(pi.hProcess, INFINITE);

  CloseHandle(pi.hProcess);
  CloseHandle(pi.hThread);
  return EXIT_SUCCESS;
}

bool append_wchar_t(const wchar_t *original, const wchar_t *append,
                    wchar_t *result) {
  wcscpy_s(result, MAX_PATH, original);
  if ((PathAppendW(result, append)) == TRUE)
    return true;

  return false;
};
