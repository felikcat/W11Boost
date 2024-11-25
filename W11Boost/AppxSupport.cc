#include "Common.h"

auto install_appx_support() -> int {
  CURL *curl = curl_easy_init();

  auto desktopPath = get_windows_path(FOLDERID_Desktop);
  auto fullPath = desktopPath + L"\\Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle";

  FILE *file = NULL;
  errno_t err = _wfopen_s(&file, fullPath.c_str(), L"wb");
  if (err != 0 || file == NULL) {
    free(file);
    return EXIT_FAILURE;
  }

  if (curl) {
    curl_easy_setopt(curl, CURLOPT_URL,
                     "https://github.com/microsoft/winget-cli/releases/latest/download/Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle");

    curl_easy_setopt(curl, CURLOPT_USERAGENT,
                     "Mozilla/5.0 (Windows NT 10.0; WOW64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.6556.192 Safari/537.36");

    curl_easy_setopt(curl, CURLOPT_USE_SSL, (long)CURLUSESSL_ALL);
    curl_easy_setopt(curl, CURLOPT_FOLLOWLOCATION, 1L); // Required due to GitHub redirecting
    curl_easy_setopt(curl, CURLOPT_CONNECTTIMEOUT_MS, 10000L); // 10000 miliseconds -> 10 seconds

    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, write_callback);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, file);

    CURLcode res = curl_easy_perform(curl);

    curl_easy_cleanup(curl);
    fclose(file);

    if (res != CURLcode::CURLE_OK)
      return EXIT_FAILURE;
  }

  wchar_t installAppx[] = LR"(powershell.exe Add-AppxPackage ([Environment]::GetFolderPath("Desktop") + "\Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle"))";
  start_command_and_wait(installAppx);

  return EXIT_SUCCESS;
}
