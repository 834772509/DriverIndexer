@echo off
cd /d %~dp0

rem Find the path of the Microsoft Visual C++ 2017 or later
set VisualStudioInstallerFolder="%ProgramFiles(x86)%\Microsoft Visual Studio\Installer"
if %PROCESSOR_ARCHITECTURE%==x86 set VisualStudioInstallerFolder="%ProgramFiles%\Microsoft Visual Studio\Installer"
pushd %VisualStudioInstallerFolder%
for /f "usebackq tokens=*" %%i in (`vswhere -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath`) do (
  set VisualStudioInstallDir=%%i
)
popd

rem Load the Microsoft Visual C++ 2017 build environment
call "%VisualStudioInstallDir%\VC\Auxiliary\Build\vcvarsall.bat" x86_amd64

rem Load the VC-LTL configuration
for /f "tokens=1,2*" %%i in ('reg query "HKCU\Code\VC-LTL" /v Root ') do set VC_LTL_Root=%%k
if "%VC_LTL_Root%" == "" echo 未检测到VC-LTL，采用默认编译方式 && goto:build

rem Setting VC-LTL
rem 取消下方注释可以开启XP兼容模式，默认才用Vista兼容模式。
rem set SupportWinXP=true
rem 取消下方注释可以开启VC-LTL轻量模式，轻量模式更加注重体积控制，但是CRT规范将会维持在VS2008。如果你需要高度兼容微软UCRT，那么请不要打开此选项！！！
rem set DisableAdvancedSupport=true
call "%VC_LTL_Root%\config\config.cmd"

:build
rem We use static CRT because we don't want to depend on VC-LTL CRT redistribute
set RUSTFLAGS=--codegen target-feature=+crt-static

rem Build Rust project via cargo
cargo build --release

rem Add UPX
IF EXIST upx.exe upx "%cd%\target\release\DriverIndexer.exe" --best --compress-resources=0 --strip-relocs=0 --compress-icons=0 --compress-exports=0 --lzma
