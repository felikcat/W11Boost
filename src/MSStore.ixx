module;
#include <windows.h>
#include <wininet.h>
export module MSStore;
import Common;
import <filesystem>;
import <optional>;
import <span>;
import <string>;
import <vector>;


std::optional<bool> download_file(const std::wstring &url,
                                  const std::filesystem::path &destination,
                                  const std::wstring &userAgent) {;
  void *hInternet = InternetOpenW(
      userAgent.c_str(),
                                 INTERNET_OPEN_TYPE_PRECONFIG, NULL, NULL, 0);
  if (!hInternet)
    return std::nullopt;

  void *hUrl =
      InternetOpenUrlW(hInternet, url.c_str(), NULL, 0,
                               INTERNET_FLAG_RELOAD | INTERNET_FLAG_SECURE, 0);
  if (!hUrl) {
    InternetCloseHandle(hInternet);
    return std::nullopt;
  }

  std::filesystem::create_directories(destination.parent_path());
  void *hFile = CreateFileW(destination.c_str(), GENERIC_WRITE, 0, NULL,
                           CREATE_ALWAYS, FILE_ATTRIBUTE_NORMAL, NULL);

  if (hFile == INVALID_HANDLE_VALUE) {
    InternetCloseHandle(hUrl);
    InternetCloseHandle(hInternet);
    return std::nullopt;
  }

  std::vector<char> buffer(4096);
  unsigned long bytesRead, bytesWritten;

  while (InternetReadFile(hUrl, buffer.data(),
                          static_cast<unsigned long>(buffer.size()),
                          &bytesRead) &&
         bytesRead > 0) {
    std::span writeSpan = std::span(buffer.data(), bytesRead);
    if (!WriteFile(hFile, writeSpan.data(),
                   static_cast<unsigned long>(writeSpan.size()), &bytesWritten,
                   NULL) ||
        bytesWritten != bytesRead) {
      CloseHandle(hFile);
      InternetCloseHandle(hUrl);
      InternetCloseHandle(hInternet);
      return std::nullopt;
    }
  }

  CloseHandle(hFile);
  InternetCloseHandle(hUrl);
  InternetCloseHandle(hInternet);
  return 0;
}

export size_t install_microsoft_store() {
  wchar_t cmdLine1[] = LR"(wsreset.exe -i)";
  start_command_and_wait(cmdLine1);

  // .appx and winget support
  const wchar_t *url =
      L"https://github.com/microsoft/winget-cli/releases/latest/download/"
      L"Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle";
  const wchar_t *destination =
      LR"(C:\Windows\Temp\Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle)";
  const wchar_t *userAgent =
      LR"(Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/129.0.0.0 Safari/537.3)";

  download_file(url, destination, userAgent);

  wchar_t cmdLine2[] =
      LR"(powershell.exe Add-AppxPackage -Path 'C:\Windows\Temp\Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle')";

  start_command_and_wait(cmdLine2);

  return 0;
};
