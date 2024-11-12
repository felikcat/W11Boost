#ifndef COMMON_H
#define COMMON_H

#define WIN32_LEAN_AND_MEAN
#include <windows.h> // Always first
#include <combaseapi.h>
#include <prsht.h>
#include <gpedit.h> // Requires prsht.h first; not unused
#include <stdbool.h>

constexpr inline CLSID _CLSID_GroupPolicyObject = {
    0xea502722, 0xa23d, 0x11d1, {0xa7, 0xd3, 0x0, 0x0, 0xf8, 0x75, 0x71, 0xe3}};
constexpr inline IID _IID_IGroupPolicyObject = {
    0xea502723, 0xa23d, 0x11d1, {0xa7, 0xd3, 0x0, 0x0, 0xf8, 0x75, 0x71, 0xe3}};
constexpr inline CLSID _CLSID_GPESnapIn = {
    0x8fc0b734, 0xa0e1, 0x11d1, {0xa7, 0xd3, 0x0, 0x0, 0xf8, 0x75, 0x71, 0xe3}};

inline GUID RegistryID = {0x35378EAC,
                   0x683F,
                   0x11D2,
                   {0xA8, 0x9A, 0x00, 0xC0, 0x4F, 0xBB, 0xCF, 0xA2}};
inline IGroupPolicyObject *pGPO;
inline HKEY hKey;
inline HKEY hSubKey;
inline LONG result;

typedef struct {
  char *string;
  bool error;
} utf8_result;

int install_privacy_mode();
utf8_result wide_string_to_utf8(const wchar_t *wide_string);
int log_registry(const wchar_t *subKey, const wchar_t *valueName,
                 const char *typeName);
LSTATUS set_dword(HKEY hKey, const wchar_t *subKey, const wchar_t *valueName,
                  const DWORD value);

LSTATUS set_string(HKEY hKey, const wchar_t *subKey, const wchar_t *valueName,
                   const wchar_t *value);
LSTATUS set_environment(HKEY hKey, const wchar_t *valueName,
                        const wchar_t *value);
LSTATUS remove_subkey(HKEY hKey, const wchar_t *subKey,
                      const wchar_t *valueName);
int gp_cleanup(HRESULT hr);
int start_command_and_wait(wchar_t *cmdLine);
bool append_wchar_t(const wchar_t *original, const wchar_t *append,
                    wchar_t *result);
int download_file(const wchar_t *url, const wchar_t *destination);
int install_microsoft_store();
void restorepoint_prep();
int create_restore_point();
int gp_edits();
wchar_t *get_log_directory();
int rw_w11boost();
int install_appx_support();

#endif // COMMON_H
