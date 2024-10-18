module;
#include <windows.h>
#include <UserEnv.h>
#include <combaseapi.h>
#include <comdef.h>
#include <gpedit.h>
#include <time.h>
export module Common;
import <filesystem>;
import <fstream>;
import <chrono>;
import <iomanip>;
import <ctime>;
import <codecvt>;

export constexpr CLSID _CLSID_GroupPolicyObject = {
    0xea502722, 0xa23d, 0x11d1, 0xa7, 0xd3, 0x0, 0x0, 0xf8, 0x75, 0x71, 0xe3};
export constexpr IID _IID_IGroupPolicyObject = {
    0xea502723, 0xa23d, 0x11d1, 0xa7, 0xd3, 0x0, 0x0, 0xf8, 0x75, 0x71, 0xe3};
export constexpr CLSID _CLSID_GPESnapIn = {
    0x8fc0b734, 0xa0e1, 0x11d1, 0xa7, 0xd3, 0x0, 0x0, 0xf8, 0x75, 0x71, 0xe3};
export GUID RegistryID = REGISTRY_EXTENSION_GUID;
export IGroupPolicyObject *pGPO;
export HKEY hKey;
export HKEY hSubKey;
export LONG result;

export std::string wide_string_to_utf8(const std::wstring &wideString) {
  if (wideString.empty())
    return std::string();

  size_t size_needed =
      WideCharToMultiByte(CP_UTF8, 0, wideString.c_str(),
                          (size_t)wideString.size(), NULL, 0, NULL, NULL);
  if (size_needed <= 0)
    return {std::string(), true};

  std::string result(size_needed, 0);
  WideCharToMultiByte(CP_UTF8, 0, wideString.c_str(), (size_t)wideString.size(),
                      &result[0], size_needed, NULL, NULL);
  return {result, false}; // if 0, proceed
}

auto log_registry(const wchar_t *subKey, const wchar_t *valueName,
                  std::string typeName) {
  std::ofstream logFile(R"(W11Boost Logs\Registry.log)", std::ios::app);

  if (logFile.is_open()) {
    auto now = std::chrono::system_clock::now();
    auto convert = std::chrono::system_clock::to_time_t(now);
    tm timeInfo;
    localtime_s(&timeInfo, &convert);

    std::string narrow_subKey = wide_string_to_utf8(subKey);
    std::string narrow_valueName = wide_string_to_utf8(valueName);

    logFile << std::put_time(&timeInfo, "%d-%m-%Y %H:%M:%S") << typeName
            << narrow_subKey << "\\" << narrow_valueName << "\n";
  }
}

export LSTATUS set_dword(HKEY hKey, const wchar_t *subKey,
                         const wchar_t *valueName, const DWORD value) {
  result = RegCreateKeyExW(hKey, subKey, 0, NULL, REG_OPTION_NON_VOLATILE,
                           KEY_WRITE, NULL, &hSubKey, NULL);

  if (result == ERROR_SUCCESS)
    result =
        RegSetValueExW(hSubKey, valueName, 0, REG_DWORD,
                       reinterpret_cast<const BYTE *>(value), sizeof(DWORD));

  std::string typeName = " - DWORD: ";
  log_registry(subKey, valueName, typeName);

  result = RegCloseKey(hSubKey);
  return result;
}

export LSTATUS set_string(HKEY hKey, const wchar_t *subKey,
                          const wchar_t *valueName, const wchar_t *value) {
  using namespace std;
  result = RegCreateKeyExW(hKey, subKey, 0, NULL, REG_OPTION_NON_VOLATILE,
                           KEY_WRITE, NULL, &hSubKey, NULL);

  if (result == ERROR_SUCCESS)
    result =
        RegSetValueExW(hSubKey, valueName, 0, REG_SZ,
                       reinterpret_cast<const BYTE *>(value), sizeof(wchar_t));

  std::string typeName = " - SZ: ";
  log_registry(subKey, valueName, typeName);

  result = RegCloseKey(hSubKey);
  return result;
}

export LSTATUS set_environment(HKEY hKey, const wchar_t *valueName,
                               const wchar_t *value) {
  const wchar_t *subKey =
      LR"(SYSTEM\CurrentControlSet\Control\Session Manager\Environment)";
  result = RegOpenKeyExW(hKey, subKey, 0, KEY_ALL_ACCESS, &hSubKey);

  if (result == ERROR_SUCCESS)
    result = RegSetValueExW(hSubKey, valueName, 0, REG_EXPAND_SZ,
                            reinterpret_cast<const BYTE *>(value),
                            (wcslen(value) * sizeof(wchar_t)));

  std::string typeName = " - EXPAND_SZ: ";
  log_registry(subKey, valueName, typeName);

  result = RegCloseKey(hSubKey);
  return result;
}

export LSTATUS remove_key(HKEY hKey, const wchar_t *subKey) {
  result = RegDeleteKeyW(hKey, subKey);

  std::ofstream logFile(R"(W11Boost Logs\Registry.log)", std::ios::app);

  if (logFile.is_open()) {
    auto now = std::chrono::system_clock::now();
    auto convert = std::chrono::system_clock::to_time_t(now);
    tm timeInfo;
    localtime_s(&timeInfo, &convert);

    std::string narrow_subKey = wide_string_to_utf8(subKey);

    logFile << std::put_time(&timeInfo, "%d-%m-%Y %H:%M:%S")
            << " - Removed key: " << narrow_subKey << "\n";
  }
  return result;
}

export LSTATUS remove_subkey(HKEY hKey, const wchar_t *subKey,
                             const wchar_t *valueName) {
  HKEY hSubKey;
  result = RegOpenKeyExW(hKey, subKey, 0, KEY_WRITE, &hSubKey);

  if (result == ERROR_SUCCESS)
    result = RegDeleteValueW(hSubKey, valueName);

  std::string typeName = " - Removed subkey: ";
  log_registry(subKey, valueName, typeName);

  result = RegCloseKey(hSubKey);
  return result;
}

export void gp_cleanup(HRESULT hr) {
  RegCloseKey(hKey);

  if (SUCCEEDED(hr))
    hr = pGPO->Save(TRUE, TRUE, &RegistryID, (GUID *)&_CLSID_GPESnapIn);

  // Apply new policy objects to the registry
  RefreshPolicyEx(TRUE, RP_FORCE);

  if (pGPO)
    pGPO->Release();
}

export size_t start_command_and_wait(wchar_t *cmdLine) {
  STARTUPINFOW si;
  PROCESS_INFORMATION pi;

  SecureZeroMemory(&si, sizeof(si));
  si.cb = sizeof(si);
  SecureZeroMemory(&pi, sizeof(pi));

  if (!CreateProcessW(NULL, cmdLine, NULL, NULL, TRUE, CREATE_NO_WINDOW, NULL,
                      NULL, &si, &pi))
    return 1;

  WaitForSingleObject(pi.hProcess, INFINITE);

  CloseHandle(pi.hProcess);
  CloseHandle(pi.hThread);
  return 0;
}

export bool os_64bit_check() {
  SYSTEM_INFO sysInfo;
  GetNativeSystemInfo(&sysInfo);

  return (sysInfo.wProcessorArchitecture == PROCESSOR_ARCHITECTURE_AMD64 ||
          sysInfo.wProcessorArchitecture == PROCESSOR_ARCHITECTURE_ARM64);
}
