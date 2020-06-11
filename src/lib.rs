#![cfg(windows)]

use winapi::shared::minwindef;
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID};
use winapi::um::consoleapi;

/// Entry point which will be called by the system once the DLL has been loaded
/// in the target process. Declaring this function is optional.
///
/// # Safety
///
/// What you can safely do inside here is very limited, see the Microsoft documentation
/// about "DllMain". Rust also doesn't officially support a "life before main()",
/// though it is unclear what that that means exactly for DllMain.
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(
    dll_module: HINSTANCE,
    call_reason: DWORD,
    reserved: LPVOID)
    -> BOOL
{
    const DLL_PROCESS_ATTACH: DWORD = 1;
    const DLL_PROCESS_DETACH: DWORD = 0;

    match call_reason {
        DLL_PROCESS_ATTACH => demo_init(),
        DLL_PROCESS_DETACH => (),
        _ => ()
    }
    minwindef::TRUE
}

fn demo_init() {
    unsafe { consoleapi::AllocConsole() };
    println!("Hello, world!");
}