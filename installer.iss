; 客戶關係管理系統 - Inno Setup 繁體中文安裝腳本

[Setup]
; AppId 的左大括號必須雙寫 {{ 來轉義
AppId={{C7A9E1B2-D3F4-4E5F-A6B7-C8D9E0A1B2C3}
AppName=客戶關係管理系統
AppVersion=1.0.0
AppPublisher=Nikuswork
DefaultDirName={autopf}\CustomerManager
DefaultGroupName=客戶關係管理系統
AllowNoIcons=yes
OutputBaseFilename=CustomerManager_Setup
Compression=lzma
SolidCompression=yes
WizardStyle=modern

[Languages]
; 啟用繁體中文介面
Name: "chinesetraditional"; MessagesFile: "compiler:Languages\ChineseTraditional.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
Source: "D:\Nikuswork\customer-manager-rust\target\release\customer-manager-rust.exe"; DestDir: "{app}"; Flags: ignoreversion
; 包含說明文件與操作手冊
Source: "D:\Nikuswork\customer-manager-rust\SOFTWARE_SPEC.md"; DestDir: "{app}"; Flags: ignoreversion
Source: "D:\Nikuswork\customer-manager-rust\USER_GUIDE.md"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\客戶關係管理系統"; Filename: "{app}\customer-manager-rust.exe"
Name: "{autodesktop}\客戶關係管理系統"; Filename: "{app}\customer-manager-rust.exe"; Tasks: desktopicon

[Run]
Filename: "{app}\customer-manager-rust.exe"; Description: "{cm:LaunchProgram,客戶關係管理系統}"; Flags: nowait postinstall skipifsilent

[UninstallDelete]
Type: files; Name: "{app}\customer.db"
