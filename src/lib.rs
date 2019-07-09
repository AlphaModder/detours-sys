#![allow(non_camel_case_types)]

#[cfg(test)]
mod tests;

use winapi::{
    shared::{
        minwindef::{BOOL, HMODULE, LPCVOID, PDWORD, DWORD, LPVOID, HINSTANCE},
        windef::HWND,
        ntdef::{LONG, PVOID, HANDLE, LPCSTR, ULONG, VOID, LPSTR, LPCWSTR, LPWSTR, INT},
        guiddef::{REFGUID, GUID},
    },
    um::{
        minwinbase::LPSECURITY_ATTRIBUTES,
        processthreadsapi::{LPSTARTUPINFOA, LPPROCESS_INFORMATION, LPSTARTUPINFOW},
    },
};


#[repr(C)] pub struct DETOUR_TRAMPOLINE { _private: [u8; 0] }

pub type PDETOUR_TRAMPOLINE = *mut DETOUR_TRAMPOLINE;

pub type PF_DETOUR_ENUMERATE_EXPORT_CALLBACK = Option<unsafe extern "system" fn(
    pContext: PVOID,
    nOrdinal: ULONG,
    pszName: LPCSTR,
    pCode: PVOID,
) -> BOOL>;

pub type PF_DETOUR_IMPORT_FILE_CALLBACK = Option<unsafe extern "system" fn(
    pContext: PVOID,
    hModule: HMODULE,
    pszFile: LPCSTR,
) -> BOOL>;

pub type PF_DETOUR_IMPORT_FUNC_CALLBACK = Option<unsafe extern "system" fn(
    pContext: PVOID,
    nOrdinal: ULONG,
    pszFunc: LPCSTR,
    pvFunc: PVOID,
) -> BOOL>;

pub type PF_DETOUR_IMPORT_FUNC_CALLBACK_EX = Option<unsafe extern "system" fn(
    pContext: PVOID,
    nOrdinal: ULONG,
    pszFunc: LPCSTR,
    ppvFunc: *mut PVOID,
) -> BOOL>;

pub type PDETOUR_BINARY = *mut VOID;

pub type PDETOUR_LOADED_BINARY = *mut VOID;

pub type PF_DETOUR_BINARY_BYWAY_CALLBACK = Option<unsafe extern "system" fn(
    pContext: PVOID,
    pszFile: LPCSTR,
    ppszOutFile: *mut LPCSTR,
) -> BOOL>;

pub type PF_DETOUR_BINARY_FILE_CALLBACK = Option<unsafe extern "system" fn(
    pContext: PVOID,
    pszOrigFile: LPCSTR,
    pszFile: LPCSTR,
    ppszOutFile: *mut LPCSTR,
) -> BOOL>;

pub type PF_DETOUR_BINARY_SYMBOL_CALLBACK = Option<unsafe extern "system" fn(
    pContext: PVOID,
    nOrigOrdinal: ULONG,
    nOrdinal: ULONG,
    pnOutOrdinal: *mut ULONG,
    pszOrigSymbol: LPCSTR,
    pszSymbol: LPCSTR,
    ppszOutSymbol: *mut LPCSTR,
) -> BOOL>;

pub type PF_DETOUR_BINARY_COMMIT_CALLBACK = Option<unsafe extern "system" fn(
    pContext: PVOID,
) -> BOOL>;

pub type PDETOUR_CREATE_PROCESS_ROUTINEA = Option<unsafe extern "system" fn(
    lpApplicationName: LPCSTR,
    lpCommandLine: LPSTR,
    lpProcessAttributes: LPSECURITY_ATTRIBUTES,
    lpThreadAttributes: LPSECURITY_ATTRIBUTES,
    bInheritHandles: BOOL,
    dwCreationFlags: DWORD,
    lpEnvironment: LPVOID,
    lpCurrentDirectory: LPCSTR,
    lpStartupInfo: LPSTARTUPINFOA,
    lpProcessInformation: LPPROCESS_INFORMATION,
) -> BOOL>;

pub type PDETOUR_CREATE_PROCESS_ROUTINEW = Option<unsafe extern "system" fn(
    lpApplicationName: LPCWSTR,
    lpCommandLine: LPWSTR,
    lpProcessAttributes: LPSECURITY_ATTRIBUTES,
    lpThreadAttributes: LPSECURITY_ATTRIBUTES,
    bInheritHandles: BOOL,
    dwCreationFlags: DWORD,
    lpEnvironment: LPVOID,
    lpCurrentDirectory: LPCSTR,
    lpStartupInfo: LPSTARTUPINFOW,
    lpProcessInformation: LPPROCESS_INFORMATION,
) -> BOOL>;

extern "system" {
    pub fn DetourTransactionBegin() -> LONG;

    pub fn DetourTransactionAbort() -> LONG;

    pub fn DetourTransactionCommit() -> LONG;

    pub fn DetourTransactionCommitEx(
        pppFailedPointer: *mut *mut PVOID,
    ) -> LONG;

    pub fn DetourUpdateThread(
        hThread: HANDLE,
    ) -> LONG;

    pub fn DetourAttach(
        ppPointer: *mut PVOID,
        pDetour: PVOID,
    ) -> LONG;

    pub fn DetourAttachEx(
        ppPointer: *mut PVOID,
        pDetour: PVOID,
        ppRealTrampoline: *mut PDETOUR_TRAMPOLINE,
        ppRealTarget: *mut PVOID,
        ppRealDetour: *mut PVOID,
    ) -> LONG;
    
    pub fn DetourDetach(
        ppPointer: *mut PVOID,
        pDetour: PVOID,
    ) -> LONG;

    pub fn DetourSetIgnoreTooSmall(
        fIgnore: BOOL,
    ) -> BOOL;

    pub fn DetourSetRetainRegions(
        fRetain: BOOL,
    ) -> BOOL;

    pub fn DetourSetSystemRegionLowerBound(
        pSystemRegionLowerBound: PVOID,
    ) -> PVOID;

    pub fn DetourSetSystemRegionUpperBound(
        pSystemRegionUpperBound: PVOID,
    ) -> PVOID;

    pub fn DetourFindFunction(
        pszModule: LPCSTR,
        pszFunction: LPCSTR,
    ) -> PVOID;

    pub fn DetourCodeFromPointer(
        pPointer: PVOID,
        ppGlobals: *mut PVOID,
    ) -> PVOID;

    pub fn DetourCopyInstruction(
        pDst: PVOID,
        ppDstPool: *mut PVOID,
        pSrc: PVOID,
        ppTarget: *mut PVOID,
        plExtra: *mut LONG,
    ) -> PVOID;

    pub fn DetourSetCodeModule(
        hModule: HMODULE,
        fLimitReferencesToModule: BOOL,
    ) -> BOOL;

    pub fn DetourAllocateRegionWithinJumpBounds(
        pbTarget: LPCVOID,
        pcbAllocatedSize: PDWORD,
    ) -> PVOID;

    pub fn DetourGetContainingModule(
        pvAddr: PVOID
    ) -> HMODULE;

    pub fn DetourEnumerateModules(
        hModuleLast: HMODULE
    ) -> HMODULE;

    pub fn DetourGetEntryPoint(
        hModule: HMODULE,
    ) -> PVOID;

    pub fn DetourGetModuleSize(
        hModule: HMODULE,
    ) -> ULONG;

    pub fn DetourEnumerateExports(
        hModule: HMODULE,
        pContext: PVOID,
        pfExport: PF_DETOUR_ENUMERATE_EXPORT_CALLBACK,
    ) -> BOOL;

    pub fn DetourEnumerateImports(
        hModule: HMODULE,
        pContext: PVOID,
        pfImportFile: PF_DETOUR_IMPORT_FILE_CALLBACK,
        pfImportFunc: PF_DETOUR_IMPORT_FUNC_CALLBACK,
    ) -> BOOL;

    pub fn DetourEnumerateImportsEx(
        hModule: HMODULE,
        pContext: PVOID,
        pfImportFile: PF_DETOUR_IMPORT_FILE_CALLBACK,
        pfImportFuncEx: PF_DETOUR_IMPORT_FUNC_CALLBACK_EX,
    ) -> BOOL;

    pub fn DetourFindPayload(
        hModule: HMODULE,
        rguid: REFGUID,
        pcbData: *mut DWORD,
    ) -> PVOID;

    pub fn DetourFindPayloadEx(
        rguid: REFGUID,
        pcbData: *mut DWORD,
    ) -> PVOID;

    pub fn DetourGetSizeOfPayloads(
        hModule: HMODULE,
    ) -> DWORD;

    pub fn DetourBinaryOpen(
        hFile: HANDLE,
    ) -> PDETOUR_BINARY;

    pub fn DetourBinaryEnumeratePayloads(
        pBinary: PDETOUR_BINARY,
        pGuid: *mut GUID,
        pcbData: *mut DWORD,
        pnIterator: *mut DWORD,
    ) -> PVOID;

    pub fn DetourBinaryFindPayload(
        pBinary: PDETOUR_BINARY,
        rguid: REFGUID,
        pcbData: *mut DWORD,
    ) -> PVOID;

    pub fn DetourBinarySetPayload(
        pBinary: PDETOUR_BINARY,
        rguid: REFGUID,
        pData: PVOID,
        cbData: DWORD,
    ) -> PVOID;

    pub fn DetourBinaryDeletePayload(
        pBinary: PDETOUR_BINARY,
        rguid: REFGUID
    ) -> BOOL;

    pub fn DetourBinaryPurgePayloads(
        pBinary: PDETOUR_BINARY,
    ) -> BOOL;

    pub fn DetourBinaryResetImports(
        pBinary: PDETOUR_BINARY,
    ) -> BOOL;

    pub fn DetourBinaryEditImports(
        pBinary: PDETOUR_BINARY,
        pContext: PVOID,
        pfByway: PF_DETOUR_BINARY_BYWAY_CALLBACK,
        pfFile: PF_DETOUR_BINARY_FILE_CALLBACK,
        pfSymbol: PF_DETOUR_BINARY_SYMBOL_CALLBACK,
        pfCommit: PF_DETOUR_BINARY_COMMIT_CALLBACK,
    ) -> BOOL;

    pub fn DetourBinaryWrite(
        pBinary: PDETOUR_BINARY,
        hFile: HANDLE,
    ) -> BOOL;

    pub fn DetourBinaryClose(
        pBinary: PDETOUR_BINARY,
    ) -> BOOL;

    pub fn DetourCreateProcessWithDllA(
        lpApplicationName: LPCSTR,
        lpCommandLine: LPSTR,
        lpProcessAttributes: LPSECURITY_ATTRIBUTES,
        lpThreadAttributes: LPSECURITY_ATTRIBUTES,
        bInheritHandles: BOOL,
        dwCreationFlags: DWORD,
        lpEnvironment: LPVOID,
        lpCurrentDirectory: LPCSTR,
        lpStartupInfo: LPSTARTUPINFOA,
        lpProcessInformation: LPPROCESS_INFORMATION,
        lpDllName: LPCSTR,
        pfCreateProcessA: PDETOUR_CREATE_PROCESS_ROUTINEA,
    ) -> BOOL;

    pub fn DetourCreateProcessWithDllW(
        lpApplicationName: LPCWSTR,
        lpCommandLine: LPWSTR,
        lpProcessAttributes: LPSECURITY_ATTRIBUTES,
        lpThreadAttributes: LPSECURITY_ATTRIBUTES,
        bInheritHandles: BOOL,
        dwCreationFlags: DWORD,
        lpEnvironment: LPVOID,
        lpCurrentDirectory: LPCWSTR,
        lpStartupInfo: LPSTARTUPINFOW,
        lpProcessInformation: LPPROCESS_INFORMATION,
        lpDllName: LPCSTR,
        pfCreateProcessW: PDETOUR_CREATE_PROCESS_ROUTINEW,
    ) -> BOOL;

    pub fn DetourProcessViaHelperA(
        dwTargetPid: DWORD,
        lpDllName: LPCSTR,
        pfCreateProcessA: PDETOUR_CREATE_PROCESS_ROUTINEA,
    ) -> BOOL;

    pub fn DetourProcessViaHelperW(
        dwTargetPid: DWORD,
        lpDllName: LPCSTR,
        pfCreateProcessA: PDETOUR_CREATE_PROCESS_ROUTINEW,
    ) -> BOOL;

    pub fn DetourProcessViaHelperDllsA(
        dwTargetPid: DWORD,
        nDlls: DWORD,
        rlpDlls: *const LPCSTR,
        lpDllName: LPCSTR,
        pfCreateProcessA: PDETOUR_CREATE_PROCESS_ROUTINEA,
    ) -> BOOL;

    pub fn DetourProcessViaHelperDllsW(
        dwTargetPid: DWORD,
        nDlls: DWORD,
        rlpDlls: *const LPCSTR,
        lpDllName: LPCSTR,
        pfCreateProcessA: PDETOUR_CREATE_PROCESS_ROUTINEW,
    ) -> BOOL;

    pub fn DetourUpdateProcessWithDll(
        hProcess: HANDLE,
        rlpDlls: *const LPCSTR,
        nDlls: DWORD,
    ) -> BOOL;

    pub fn DetourUpdateProcessWithDllEx(
        hProcess: HANDLE,
        hImage: HMODULE,
        bIs32Bit: BOOL,
        rlpDlls: *const LPCSTR,
        nDlls: DWORD,
    ) -> BOOL;

    pub fn DetourCopyPayloadToProcess(
        hProcess: HANDLE,
        rguid: REFGUID,
        pvData: PVOID,
        cbData: DWORD,
    ) -> BOOL;

    pub fn DetourRestoreAfterWith() -> BOOL;

    pub fn DetourRestoreAfterWithEx(
        pvData: PVOID,
        cbData: DWORD,
    ) -> BOOL;

    pub fn DetourIsHelperProcess() -> BOOL;

    pub fn DetourFinishHelperProcess(
        hwnd: HWND,
        hinst: HINSTANCE,
        lpszCmdLine: LPSTR,
        nCmdShow: INT,
    );
}