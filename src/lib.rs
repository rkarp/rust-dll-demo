extern crate winapi;
extern crate kernel32;

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
pub extern "system" fn DllMain(
    dll_module: winapi::HINSTANCE,
    call_reason: winapi::DWORD,
    reserved: winapi::LPVOID)
{
    // Declare these constants locally since they are only used in this function
    const DLL_PROCESS_ATTACH: winapi::DWORD = 1;
    const DLL_PROCESS_DETACH: winapi::DWORD = 0;

    match call_reason {
        DLL_PROCESS_ATTACH => demo_init(),
        DLL_PROCESS_DETACH => (),
        _ => ()
    }
}

fn demo_init() {
    unsafe { kernel32::AllocConsole() };
    println!("Hello, world!");
}
