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
if "%VC_LTL_Root%" == "" echo δ��⵽VC-LTL������Ĭ�ϱ��뷽ʽ && goto:build

rem Setting VC-LTL
rem ȡ���·�ע�Ϳ��Կ���XP����ģʽ��Ĭ�ϲ���Vista����ģʽ��
rem set SupportWinXP=true
rem ȡ���·�ע�Ϳ��Կ���VC-LTL����ģʽ������ģʽ����ע��������ƣ�����CRT�淶����ά����VS2008���������Ҫ�߶ȼ���΢��UCRT����ô�벻Ҫ�򿪴�ѡ�����
rem set DisableAdvancedSupport=true
call "%VC_LTL_Root%\config\config.cmd"

:build
rem We use static CRT because we don't want to depend on VC-LTL CRT redistribute
set RUSTFLAGS=--codegen target-feature=+crt-static

rem Build Rust project via cargo
cargo build --release

rem Add UPX
IF EXIST upx.exe upx "%cd%\target\release\DriverIndexer.exe" --best --compress-resources=0 --strip-relocs=0 --compress-icons=0 --compress-exports=0 --lzma
