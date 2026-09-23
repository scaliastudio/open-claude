; Open Claude installer. Per-user install: no admin prompt, nothing outside the user's profile.
#ifndef AppVersion
  #define AppVersion "0.0.0-dev"
#endif

[Setup]
AppId={{6B1E2C4A-3F7D-4E8B-9A51-0C2D7E4F9B13}
AppName=Open Claude
AppVersion={#AppVersion}
AppVerName=Open Claude {#AppVersion}
AppPublisher=Scalia Studio
AppPublisherURL=https://scaliastudio.dev/products/openclaude
AppSupportURL=https://github.com/scaliastudio/open-claude/issues
AppUpdatesURL=https://github.com/scaliastudio/open-claude/releases
PrivilegesRequired=lowest
DefaultDirName={autopf}\Open Claude
DisableDirPage=yes
DisableProgramGroupPage=yes
InfoBeforeFile=notice.txt
OutputDir=..\dist
OutputBaseFilename=OpenClaude-Setup
SetupIconFile=..\assets\open-claude.ico
UninstallDisplayIcon={app}\open-claude.ico
UninstallDisplayName=Open Claude
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
Compression=lzma2/max
SolidCompression=yes
WizardStyle=modern

[Tasks]
Name: "desktopicon"; Description: "Put Open Claude on the desktop"

[Files]
Source: "..\target\release\OpenClaude.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\assets\open-claude.ico"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{autoprograms}\Open Claude"; Filename: "{app}\OpenClaude.exe"; IconFilename: "{app}\open-claude.ico"
Name: "{autodesktop}\Open Claude"; Filename: "{app}\OpenClaude.exe"; IconFilename: "{app}\open-claude.ico"; Tasks: desktopicon

[Run]
Filename: "{app}\OpenClaude.exe"; Description: "Open Claude"; Flags: nowait postinstall skipifsilent
