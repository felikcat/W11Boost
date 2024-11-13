import Common;
#include <shlobj_core.h>
#include <curl/curl.h>

int install_appx_support() {
  CURL *curl = curl_easy_init();
  size_t tempDir = GetTempPathW(0, NULL);

  if (curl) {
    CURLcode res;
    curl_easy_setopt(curl, CURLOPT_URL,
                     "https://github.com/microsoft/winget-cli/releases/latest/download/"
                     "Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle");

    curl_easy_setopt(curl, CURLOPT_USERAGENT,
                     "Mozilla/5.0 (Windows NT 10.0; WOW64; x64) AppleWebKit/537.36 (KHTML, like Gecko) "
                     "Chrome/130.0.6556.192 Safari/537.36");

    curl_easy_setopt(curl, CURLOPT_USE_SSL, (long)CURLUSESSL_ALL);
    curl_easy_setopt(curl, CURLOPT_CONNECTTIMEOUT_MS, 10000L); // 10000 miliseconds -> 10 seconds

    res = curl_easy_perform(curl);
    if (res != CURLcode::CURLE_OK) {
      curl_easy_cleanup(curl);
      return EXIT_FAILURE;
    } else {
      curl_easy_cleanup(curl);
    }
  }
  return EXIT_SUCCESS;
}
