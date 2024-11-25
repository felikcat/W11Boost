#define WIN32_LEAN_AND_MEAN
#include <windows.h> // Always first
#include <cstdio>
#include <string>
#include <Shlwapi.h>
#include <shellapi.h>
#include <KnownFolders.h>
#include <shlobj.h>
#include <stdio.h>
#include <curl/curl.h>
#include <iostream>
#include <fstream>
#include <UserEnv.h>
#include <fcntl.h>
#include <libloaderapi.h>
#include <wchar.h>
#include <time.h>
#include <sstream>
#include <AccCtrl.h>
#include <AclAPI.h>
#include <SrRestorePtApi.h>

inline LONG long_result;

typedef struct {
char *string;
bool return_code;
} utf8_result;

auto install_privacy_mode() -> int;
auto wide_string_to_utf8(const wchar_t *wide_string) -> utf8_result;
auto log_registry(const wchar_t *subKey, const wchar_t *valueName, const char *typeName) -> int;
auto set_dword(HKEY hKey, const wchar_t *subKey, const wchar_t *valueName, const DWORD value) -> LSTATUS;

auto set_string(HKEY hKey, const wchar_t *subKey, const wchar_t *valueName, const wchar_t *value) -> LSTATUS;
auto set_environment(HKEY hKey, const wchar_t *valueName, const wchar_t *value) -> LSTATUS;
auto remove_subkey(HKEY hKey, const wchar_t *subKey, const wchar_t *valueName) -> LSTATUS;
auto start_command_and_wait(wchar_t * cmdLine) -> int;
auto append_wchar_t(const wchar_t *original, const wchar_t *append, wchar_t *result) -> bool;
auto restorepoint_prep() -> void;
auto create_restore_point() -> int;
auto main_registry_edits() -> int;
auto get_log_directory() -> wchar_t*;
auto install_appx_support() -> int;
auto delete_directory_and_subfolders(LPCWSTR directory) -> long;
auto disable_sleep() -> int;
auto write_callback(char *ptr, size_t size, size_t nmemb, void *userdata) -> size_t;
auto get_windows_path(const GUID folderID) -> std::wstring;
