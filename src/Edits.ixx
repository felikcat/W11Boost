module;
#include <windows.h>
#include <UserEnv.h>
#include <combaseapi.h>
#include <comdef.h>
#include <gpedit.h>
export module Edits;
import Common;
import <string>;
import <filesystem>;
import <fstream>;

export size_t gp_edits() {
  // Apartment-threaded required for GPOs
  HRESULT hr = CoInitializeEx(NULL, COINIT_APARTMENTTHREADED);
  if (FAILED(hr))
    return 1;

  //---- HKEY_LOCAL_MACHINE ----//
  CoCreateInstance(_CLSID_GroupPolicyObject, NULL, CLSCTX_INPROC_SERVER,
                   _IID_IGroupPolicyObject, (LPVOID *)&pGPO);

  hr = pGPO->OpenLocalMachineGPO(GPO_OPEN_LOAD_REGISTRY);
  if (FAILED(hr))
    return 1;

  hKey = HKEY_LOCAL_MACHINE;
  pGPO->GetRegistryKey(GPO_SECTION_MACHINE, &hKey);

  // If allowed (1): unused apps would be uninstalled with their user data left
  // intact, then reinstalled if launched afterwards at any point in time.
  set_dword(hKey, LR"(SOFTWARE\Policies\Microsoft\Windows\Appx)",
            L"AllowAutomaticAppArchiving", 0);

  // Shows what's slowing down bootups and shutdowns.
  set_dword(hKey,
            LR"(SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System)",
            L"verbosestatus", 1);

  // Ask to not allow execution of experiments by Microsoft.
  set_dword(hKey, LR"(SOFTWARE\Microsoft\PolicyManager\current\device\System)",
            L"AllowExperimentation", 0);

  // Power Throttling causes severe performance reduction for VMWare
  // Workstation 17.
  set_dword(hKey, LR"(SYSTEM\CurrentControlSet\Control\Power\PowerThrottling)",
            L"PowerThrottlingOff", 1);

  // https://docs.microsoft.com/en-us/windows/desktop/win7appqual/fault-tolerant-heap
  // FTH being enabled causes issues with specific apps such as Assetto Corsa.
  set_dword(hKey, LR"(SOFTWARE\Microsoft\FTH)", L"Enabled", 0);

  // [P1] Automated file cleanup without user interaction is a bad idea, even if
  // ran only on low-disk space events.
  set_dword(hKey, LR"(SOFTWARE\Policies\Microsoft\Windows\Appx)",
            L"AllowStorageSenseGlobal", 0);
  set_dword(hKey, LR"(SOFTWARE\Policies\Microsoft\Windows\StorageSense)",
            L"AllowStorageSenseGlobal", 0);

  // Allocate more RAM to NTFS' paged pool.
  set_dword(hKey, LR"(SYSTEM\CurrentControlSet\Policies)",
            L"NtfsForceNonPagedPoolAllocation", 0);
  wchar_t raisePoolLimit[] = L"fsutil.exe behavior set memoryusage 2";
  start_command_and_wait(raisePoolLimit);

  // Disable automatic repair to instead ask for a repair. Does not disable
  // Windows' Recovery environment thankfully.
  wchar_t disableAutoRepair[] =
      LR"(bcdedit.exe /set "{default}" recoveryenabled no)";
  start_command_and_wait(disableAutoRepair);

  // Do not page drivers and other system code to a disk, keep it in memory.
  set_dword(
      hKey,
      LR"(SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management)",
      L"DisablePagingExecutive", 1);

  //---- Disables "Fast startup".
  set_dword(hKey, LR"(SYSTEM\CurrentControlSet\Control\Session Manager\Power)",
            L"HiberbootEnabled", 0);

  std::wstring filePath =
      LR"(C:\Windows\System32\SleepStudy\UserNotPresentSession.etl)";

  // Incase the file doesn't exist.
  std::ofstream file(filePath);
  file.close();

  DWORD attributes = GetFileAttributesW(filePath.c_str());
  attributes |= FILE_ATTRIBUTE_ARCHIVE | FILE_ATTRIBUTE_READONLY;

  if (SetFileAttributesW(filePath.c_str(), attributes))
    return 1;
  //-----

  //---- Visual Studio 2017 up to 2022: PerfWatson2 (VSCEIP; telemetry) is
  // intensive on resources, disable it.
  if (!os_64bit_check()) {
    set_dword(hKey, LR"(Software\Microsoft\VSCommon\15.0\SQM)", L"OptIn", 0);
    set_dword(hKey, LR"(Software\Microsoft\VSCommon\16.0\SQM)", L"OptIn", 0);
    set_dword(hKey, LR"(Software\Microsoft\VSCommon\17.0\SQM)", L"OptIn", 0);
  }
  // 64-bit / x86_64
  set_dword(hKey, LR"(SOFTWARE\Wow6432Node\Microsoft\VSCommon\15.0\SQM)",
            L"OptIn", 0);
  set_dword(hKey, LR"(SOFTWARE\Wow6432Node\Microsoft\VSCommon\16.0\SQM)",
            L"OptIn", 0);
  set_dword(hKey, LR"(SOFTWARE\Wow6432Node\Microsoft\VSCommon\17.0\SQM)",
            L"OptIn", 0);
  //----

  //---- [P1] Disable various forms of telemetry.
  set_dword(
      hKey,
      LR"(SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection)",
      L"AllowTelemetry", 0);

  set_dword(
      hKey,
      LR"(SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection)",
      L"MaxTelemetryAllowed", 0);

  set_dword(hKey, LR"(SOFTWARE\Policies\Microsoft\Windows\AdvertisingInfo)",
            L"DisabledByGroupPolicy", 1);

  set_dword(hKey, LR"(SOFTWARE\Policies\Microsoft\Windows\DataCollection)",
            L"AllowDeviceNameInTelemetry", 0);

  set_dword(hKey, LR"(SOFTWARE\Policies\Microsoft\Windows\DataCollection)",
            L"AllowTelemetry", 0);

  // https://learn.microsoft.com/en-us/windows/privacy/manage-connections-from-windows-operating-system-components-to-microsoft-services#31-services-configuration
  set_dword(hKey, LR"(SOFTWARE\Policies\Microsoft\Windows\DataCollection)",
            L"DisableOneSettingsDownloads", 1);

  set_dword(hKey, LR"(SOFTWARE\Policies\Microsoft\Windows\DataCollection)",
            L"DisableTelemetryOptInChangeNotification", 1);

  // Disable "Connected User Experiences and Telemetry" service
  set_dword(
      hKey,
      LR"(SOFTWARE\Microsoft\Windows\CurrentVersion\Diagnostics\DiagTrack)",
      L"ShowedToastAtLevel", 1);
  set_dword(hKey, LR"(SYSTEM\CurrentControlSet\Services\DiagTrack)", L"Start",
            4);

  // Disable "Diagnostic Policy Service".
  // Logs tons of information to be sent off and analyzed by Microsoft, and in
  // some cases caused noticeable performance slowdown.
  set_dword(hKey, LR"(SYSTEM\CurrentControlSet\Services\DPS)", L"Start", 4);

  // Disable "Customer Experience Improvement Program"; also implies turning off
  // the Inventory Collector.
  set_dword(hKey, LR"(SOFTWARE\Policies\Microsoft\AppV\CEIP)", L"CEIPEnable",
            0);
  set_dword(hKey, LR"(SOFTWARE\Microsoft\SQMClient\Windows)", L"CEIPEnable", 0);

  //-- [P1] Disable "Windows Error Reporting" service.
  set_dword(hKey, LR"(SOFTWARE\Microsoft\Windows\Windows Error Reporting)",
            L"Disabled", 1);

  set_dword(hKey, LR"(SOFTWARE\Microsoft\Windows\Windows Error Reporting)",
            L"AutoApproveOSDumps", 0);

  set_dword(hKey, LR"(SOFTWARE\Microsoft\Windows\Windows Error Reporting)",
            L"DefaultConsent", 1);

  set_dword(hKey, LR"(SOFTWARE\Microsoft\Windows\Windows Error Reporting)",
            L"DefaultOverrideBehavior", 0);

  set_dword(hKey, LR"(SOFTWARE\Microsoft\Windows\Windows Error Reporting)",
            L"DontSendAdditionalData", 1);

  set_dword(hKey, LR"(SOFTWARE\Microsoft\Windows\Windows Error Reporting)",
            L"LoggingDisabled", 1);

  set_dword(hKey, LR"(SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting)",
            L"AllOrNone", 0);

  set_dword(hKey, LR"(SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting)",
            L"IncludeKernelFaults", 0);

  set_dword(hKey, LR"(SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting)",
            L"IncludeMicrosoftApps", 0);

  set_dword(hKey, LR"(SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting)",
            L"IncludeShutdownErrs", 0);

  set_dword(hKey, LR"(SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting)",
            L"IncludeWindowsApps", 0);

  set_dword(hKey, LR"(SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting)",
            L"DoReport", 0);

  set_dword(hKey,
            LR"(SOFTWARE\Policies\Microsoft\Windows\DeviceInstall\Settings)",
            L"DisableSendGenericDriverNotFoundToWER", 1);

  set_dword(hKey,
            LR"(SOFTWARE\Policies\Microsoft\Windows\DeviceInstall\Settings)",
            L"DisableSendRequestAdditionalSoftwareToWER", 1);

  set_dword(hKey,
            LR"(SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting)",
            L"Disabled", 1);

  set_string(
      hKey,
      LR"(SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting\ExcludedApplications)",
      L"*", L"*");

  set_string(
      hKey,
      LR"(SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting\ExclusionList)",
      L"*", L"*");

  set_dword(hKey, LR"(SYSTEM\CurrentControlSet\Services\WerSvc)", L"Start", 4);
  //--

  // Disable telemetry for Tablet PC's handwriting recognition.
  set_dword(hKey, LR"(SOFTWARE\Policies\Microsoft\Windows\TabletPC)",
            L"PreventHandwritingDataSharing", 1);
  //----

  // [P1] Disable feedback reminders.
  set_dword(hKey, LR"(Software\Policies\Microsoft\Windows\DataCollection)",
            L"DoNotShowFeedbackNotifications", 1);

  // Ask OneDrive to only generate network traffic if signed in to OneDrive.
  set_dword(hKey, LR"(SOFTWARE\Microsoft\OneDrive)",
            L"PreventNetworkTrafficPreUserSignIn", 1);

  // Don't ask to change the current privacy settings after applying a major
  // Windows update.
  set_dword(hKey, LR"(SOFTWARE\Policies\Microsoft\Windows\OOBE)",
            L"DisablePrivacyExperience", 1);

  //---- HKEY_CURRENT_USER ----//
  gp_cleanup(hr);
  CoCreateInstance(_CLSID_GroupPolicyObject, NULL, CLSCTX_INPROC_SERVER,
                   _IID_IGroupPolicyObject, (LPVOID *)&pGPO);

  hr = pGPO->OpenLocalMachineGPO(GPO_OPEN_LOAD_REGISTRY);
  if (FAILED(hr))
    return 1;

  hKey = HKEY_CURRENT_USER;
  pGPO->GetRegistryKey(GPO_SECTION_USER, &hKey);

  set_dword(hKey,
            LR"(Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo)",
            L"Enabled", 0);
  set_dword(
      hKey,
      LR"(Software\Microsoft\Speech_OneCore\Settings\OnlineSpeechPrivacy)",
      L"HasAccepted", 0);
  set_dword(hKey, LR"(Software\Microsoft\InputPersonalization)",
            L"RestrictImplicitInkCollection", 1);
  set_dword(hKey, LR"(Software\Microsoft\InputPersonalization)",
            L"RestrictImplicitTextCollection", 1);

  // Disable the language list fingerprinting method; useful for bypassing
  // geolocation restrictions.
  set_dword(hKey, LR"(Control Panel\International\User Profile)",
            L"HttpAcceptLanguageOptOut", 1);

  // [P2] Disable feedback reminders.
  set_dword(hKey, LR"(SOFTWARE\Microsoft\Siuf\Rules)", L"NumberOfSIUFInPeriod",
            0);
  set_dword(hKey, LR"(SOFTWARE\Microsoft\Siuf\Rules)", L"PeriodInNanoSeconds",
            0);

  // Enable multiple processes for explorer.exe for increased stability and
  // performance.
  set_dword(hKey,
            LR"(Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced)",
            L"SeparateProcess", 1);

  // Hidden file extensions are abused to hide the real file format, example:
  // an executable (.exe, .scr, .com) pretending to be a PDF.
  set_dword(hKey,
            LR"(Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced)",
            L"HideFileExt", 0);

  // Visual Studio 2022: disable other telemetry.
  set_dword(hKey, LR"(Software\Microsoft\VisualStudio\Telemetry)",
            L"TurnOffSwitch", 1);

  gp_cleanup(hr);

  //---- Outside of GPO ----//

  // [P2] Disable "Windows Error Reporting" service.
  remove_key(
      hKey,
      LR"(SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting\InclusionList)");
  remove_key(hKey,
             LR"(SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting\Consent)");

  // [P2] Automated file cleanup without user interaction is a bad idea, even if
  // ran only on low-disk space events.
  remove_key(hKey,
             LR"(Software\Microsoft\Windows\CurrentVersion\StorageSense)");

  // Random disconnection fix for specific network adapters, such as Intel's
  // I225-V.
  wchar_t setWFL[] =
      LR"(powershell.exe Set-NetAdapterAdvancedProperty -Name '*' -DisplayName 'Wait for Link' -RegistryValue 0)";
  start_command_and_wait(setWFL);

  return 0;
}
