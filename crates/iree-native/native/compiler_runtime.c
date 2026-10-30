#include <stdint.h>
#include <windows.h>

#pragma comment(lib, "delayimp.lib")
#pragma comment(linker, "/DELAYLOAD:IREECompiler.dll")

static INIT_ONCE compiler_once = INIT_ONCE_STATIC_INIT;
static HMODULE compiler_module;
static DWORD compiler_load_error;

static BOOL CALLBACK iree_native_load_compiler_once(
    PINIT_ONCE once, PVOID parameter, PVOID *context) {
  (void)once;
  (void)context;
  compiler_module = LoadLibraryExW((LPCWSTR)parameter, NULL,
                                   LOAD_WITH_ALTERED_SEARCH_PATH);
  if (!compiler_module) {
    compiler_load_error = GetLastError();
    if (!compiler_load_error) compiler_load_error = ERROR_MOD_NOT_FOUND;
  }
  return TRUE;
}

uint32_t iree_native_load_compiler(const uint16_t *path) {
  if (!InitOnceExecuteOnce(&compiler_once, iree_native_load_compiler_once,
                           (PVOID)path, NULL)) {
    DWORD error = GetLastError();
    return error ? (uint32_t)error : ERROR_DLL_INIT_FAILED;
  }
  return (uint32_t)compiler_load_error;
}
