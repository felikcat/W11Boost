module; // Use global module fragments
#define WIN32_LEAN_AND_MEAN
#include <windows.h> // Always first
export module Common;

export {
  inline HKEY hKey;
  inline HKEY hSubKey;
  inline LONG result;

  typedef struct {
    char *string;
    bool error;
  } utf8_result;

  int install_privacy_mode();
  utf8_result wide_string_to_utf8(const wchar_t *wide_string);
  int log_registry(const wchar_t *subKey, const wchar_t *valueName, const char *typeName);
  LSTATUS set_dword(HKEY hKey, const wchar_t *subKey, const wchar_t *valueName, const DWORD value);

  LSTATUS set_string(HKEY hKey, const wchar_t *subKey, const wchar_t *valueName, const wchar_t *value);
  LSTATUS set_environment(HKEY hKey, const wchar_t *valueName, const wchar_t *value);
  LSTATUS remove_subkey(HKEY hKey, const wchar_t *subKey, const wchar_t *valueName);
  int start_command_and_wait(wchar_t * cmdLine);
  bool append_wchar_t(const wchar_t *original, const wchar_t *append, wchar_t *result);
  void restorepoint_prep();
  int create_restore_point();
  int main_registry_edits();
  wchar_t *get_log_directory();
  int rw_w11boost();
  int install_appx_support();
  long delete_directory_and_subfolders(LPCWSTR directory);
  int disable_sleep();
}
