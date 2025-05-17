@echo off
setlocal enabledelayedexpansion

:: Configure MT builds
cmake -B build_MT -G "Visual Studio 17 2022" -A x64 -DJNIHOOK_MT_RUNTIME=ON
if %errorlevel% neq 0 exit /b %errorlevel%

:: Build MT configurations
cmake --build build_MT --config Debug --target ALL_BUILD
if %errorlevel% neq 0 exit /b %errorlevel%
echo Finished building MTd successfully

cmake --build build_MT --config Release --target ALL_BUILD
if %errorlevel% neq 0 exit /b %errorlevel%
echo Finished building MT successfully

:: Configure MD builds
cmake -B build_MD -G "Visual Studio 17 2022" -A x64 -DJNIHOOK_MT_RUNTIME=OFF
if %errorlevel% neq 0 exit /b %errorlevel%

:: Build MD configurations
cmake --build build_MD --config Debug --target ALL_BUILD
if %errorlevel% neq 0 exit /b %errorlevel%
echo Finished building MDd successfully

cmake --build build_MD --config Release --target ALL_BUILD
if %errorlevel% neq 0 exit /b %errorlevel%
echo Finished building MD successfully

echo All builds completed successfully
