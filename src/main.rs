use std::ffi::OsStr;
use std::mem::{zeroed, size_of};
use std::os::windows::prelude::OsStrExt;
use std::time::Duration;
use windows_sys::Win32::Foundation::{GetLastError, CloseHandle};
use windows_sys::Win32::Storage::FileSystem::{
    DELETE, OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, SetFileInformationByHandle,
    FILE_RENAME_INFO, FILE_DISPOSITION_INFO,
};
use windows_sys::Win32::System::LibraryLoader::GetModuleFileNameW;
use windows_sys::Win32::Storage::FileSystem::CreateFileW;
use std::ptr::{null, copy_nonoverlapping};

const MAX: usize = 500usize;

unsafe fn get_module_file_name(buffer: &mut [u16; MAX + 1]) -> bool {
    let current_path = GetModuleFileNameW(0, buffer.as_mut_ptr(), 261u32);
    if current_path == 0 {
        println!("Failed to get module file name: {:?}", GetLastError());
        return false;
    }
    true
}

unsafe fn create_file(file_name: *const u16) -> isize {
    let handle = CreateFileW(file_name, DELETE, 0, null(), OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, 0);
    if handle == 0 {
        println!("Failed to create file: {:?}", GetLastError());
    }
    handle
}

unsafe fn set_file_rename_info(handle: isize, file_name: &Vec<u16>) -> bool {
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
        println!("Failed to set file rename info: {:?}", GetLastError());
        return false;
    }
    true
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
        println!("Failed to set file disposition info: {:?}", GetLastError());
        return false;
    }
    true
}

fn main() {
    unsafe {
        let mut buffer: [u16; MAX + 1] = [0; MAX + 1];

        if !get_module_file_name(&mut buffer) {
            return;
        }

        let current_handle = create_file(buffer.as_ptr());
        println!("handle1: {}", current_handle);

        if current_handle == 0 {
            return;
        }

        if !set_file_rename_info(current_handle, &OsStr::new(":12").encode_wide().collect()) {
            CloseHandle(current_handle);
            return;
        }

        CloseHandle(current_handle);
        let mut buffer2: [u16; MAX + 1] = [0; MAX + 1];

        if !get_module_file_name(&mut buffer2) {
            return;
        }

        let new_handle = create_file(buffer2.as_ptr());
        println!("handle2: {}", new_handle);

        if new_handle == -1 {
            return;
        }

        if !set_file_disposition_info(new_handle) {
            CloseHandle(new_handle);
            return;
        }

        CloseHandle(new_handle);

        std::thread::sleep(Duration::from_secs(10));
    }
}
