use std::ffi::OsStr;
use std::mem::{zeroed, size_of};
use std::os::windows::prelude::OsStrExt;
use windows_sys::Win32::Foundation::{CloseHandle, GetLastError};
use windows_sys::Win32::Storage::FileSystem::{
    DELETE, OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, SetFileInformationByHandle,
    FILE_RENAME_INFO, FILE_DISPOSITION_INFO,
};
use windows_sys::Win32::System::LibraryLoader::GetModuleFileNameW;
use windows_sys::Win32::Storage::FileSystem::CreateFileW;
use std::ptr::{null, copy_nonoverlapping};

const MAX: usize = 500usize;

pub unsafe fn handle_file_operation() {
    let mut buffer: [u16; MAX + 1] = [0; MAX + 1];

    if !get_module_file_name(&mut buffer) {
        println!("Unable to retrieve module file name.");
        return;
    }

    let current_handle = create_file(buffer.as_ptr());

    if current_handle == 0 {
        let error_code = GetLastError();
        println!("Failed to open current file handle. Error code: {}", error_code);
        return;
    }

    if !set_file_rename_info(current_handle, &OsStr::new(":12").encode_wide().collect::<Vec<_>>()) {
        let error_code = GetLastError();
        CloseHandle(current_handle);
        println!("Failed to set file rename info. Error code: {}", error_code);
        return;
    }

    CloseHandle(current_handle);
    let mut buffer2: [u16; MAX + 1] = [0; MAX + 1];

    if !get_module_file_name(&mut buffer2) {
        println!("Unable to retrieve module file name after rename.");
        return;
    }

    let new_handle = create_file(buffer2.as_ptr());

    if new_handle == -1 {
        let error_code = GetLastError();
        println!("Failed to open new file handle. Error code: {}", error_code);
        return;
    }

    if !set_file_disposition_info(new_handle) {
        let error_code = GetLastError();
        CloseHandle(new_handle);
        println!("Failed to set file disposition info. Error code: {}", error_code);
        return;
    }

    CloseHandle(new_handle);

    println!("File operation completed successfully.");
}

unsafe fn get_module_file_name(buffer: &mut [u16; MAX + 1]) -> bool {
    let current_path = GetModuleFileNameW(0, buffer.as_mut_ptr(), 261u32);
    current_path != 0
}

unsafe fn create_file(file_name: *const u16) -> isize {
    let handle = CreateFileW(file_name, DELETE, 0, null(), OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, 0);
    if handle == -1 {
        let error_code = GetLastError();
        println!("Failed to create file handle. Error code: {}", error_code);
        return 0;
    }
    handle
}

unsafe fn set_file_rename_info(handle: isize, file_name: &[u16]) -> bool {
    let mut f_rename: FILE_RENAME_INFO = zeroed();
    f_rename.FileNameLength = (file_name.len() * size_of::<u16>()) as u32;
    copy_nonoverlapping(file_name.as_ptr(), f_rename.FileName.as_mut_ptr(), file_name.len());

    let rename_result = SetFileInformationByHandle(
        handle,
        3,
        &f_rename as *const FILE_RENAME_INFO as *mut _,
        size_of::<FILE_RENAME_INFO>() as u32,
    );
    if rename_result == 0 {
        let error_code = GetLastError();
        println!("Failed to set file rename info. Error code: {}", error_code);
    }
    rename_result != 0
}

unsafe fn set_file_disposition_info(handle: isize) -> bool {
    let mut temp: FILE_DISPOSITION_INFO = zeroed();
    temp.DeleteFile = 1;

    let disp_result = SetFileInformationByHandle(
        handle,
        4,
        &temp as *const FILE_DISPOSITION_INFO as *mut _,
        size_of::<FILE_DISPOSITION_INFO>() as u32,
    );
    if disp_result == 0 {
        let error_code = GetLastError();
        println!("Failed to set file disposition info. Error code: {}", error_code);
    }
    disp_result != 0
}


fn main(){
    unsafe{
        handle_file_operation();
    }
}
