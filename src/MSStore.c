#include "Common.h"
#include <wininet.h>
#include <ShlObj_core.h>
#include <malloc.h>
#include <KnownFolders.h>

int download_file(const wchar_t *url, const wchar_t *destination) {
  const wchar_t *userAgent =
      L"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, "
      L"like Gecko) Chrome/129.0.0.0 Safari/537.3";
  void *hInternet =
      InternetOpenW(userAgent, INTERNET_OPEN_TYPE_PRECONFIG, NULL, NULL, 0);
  if (!hInternet)
    return EXIT_FAILURE;

  void *hUrl = InternetOpenUrlW(hInternet, url, NULL, 0,
                                INTERNET_FLAG_RELOAD | INTERNET_FLAG_SECURE, 0);
  if (!hUrl) {
    InternetCloseHandle(hInternet);
    return EXIT_FAILURE;
  }

  void *hFile = CreateFileW(destination, GENERIC_WRITE, 0, NULL, CREATE_ALWAYS,
                            FILE_ATTRIBUTE_NORMAL, NULL);

  if (hFile == INVALID_HANDLE_VALUE) {
    InternetCloseHandle(hUrl);
    InternetCloseHandle(hInternet);
    return EXIT_FAILURE;
  }

  const int length = 4096;
  char *buffer = (char *)malloc(length);
  if (buffer == NULL) {
    InternetCloseHandle(hUrl);
    InternetCloseHandle(hInternet);
    return EXIT_FAILURE;
  }

  unsigned long bytesRead, bytesWritten;

  while (InternetReadFile(hUrl, buffer, (size_t)buffer, &bytesRead) &&
         bytesRead > 0) {

    if (!WriteFile(hFile, buffer, bytesRead, &bytesWritten, NULL) ||
        bytesWritten != bytesRead) {
      CloseHandle(hFile);
      InternetCloseHandle(hUrl);
      InternetCloseHandle(hInternet);
      return EXIT_FAILURE;
    }
  }

  CloseHandle(hFile);
  InternetCloseHandle(hUrl);
  InternetCloseHandle(hInternet);
  return EXIT_SUCCESS;
}

int install_microsoft_store() {

  wchar_t cmdLine1[] = L"wsreset.exe -i";
  start_command_and_wait(cmdLine1);

  // .appx and winget support
  const wchar_t *url =
      L"https://github.com/microsoft/winget-cli/releases/latest/download/"
      L"Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle";

  PWSTR desktopPath;
  SHGetKnownFolderPath(&FOLDERID_Desktop, 0, NULL, &desktopPath);
  const wchar_t *appName =
      L"Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle";
  wchar_t destination[MAX_PATH];
  append_wchar_t(desktopPath, appName, destination);

  if (download_file(url, destination))
    return EXIT_FAILURE;

  wchar_t cmdLine2[] = L"powershell.exe Add -AppxPackage -Path '${home}\\Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle'";
           start_command_and_wait(cmdLine2);

  return EXIT_SUCCESS;
};
