use std::ptr;
use std::mem;
use windows::Win32::Foundation::{CloseHandle, PWSTR};
use windows::Win32::Storage::FileSystem::{CreateFileW, ReadDirectoryChangesW, FILE_NOTIFY_CHANGE, FILE_NOTIFY_INFORMATION};
use windows::Win32::System::Threading::{GetCurrentProcess, GetCurrentProcessId, GetCurrentThread, GetCurrentThreadId};
use windows::Win32::System::SystemServices::{FILE_LIST_DIRECTORY, FILE_SHARE_READ, FILE_SHARE_WRITE, FILE_SHARE_DELETE, OPEN_EXISTING};

fn main() -> windows::Result<()> {
    let path = std::env::current_dir().unwrap();
    let path_str: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();

    unsafe {
        let handle = CreateFileW(
            PWSTR(path_str.as_ptr() as _),
            FILE_LIST_DIRECTORY,
            FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
            ptr::null_mut(),
            OPEN_EXISTING,
            0,
            None,
        )
        .unwrap();

        loop {
            let mut buffer = [0u8; 1024];
            let mut bytes_returned = 0u32;

            ReadDirectoryChangesW(
                handle,
                buffer.as_mut_ptr() as _,
                buffer.len() as u32,
                true.into(),
                FILE_NOTIFY_CHANGE::FILE_NOTIFY_CHANGE_FILE_NAME.0,
                &mut bytes_returned,
                ptr::null_mut(),
                None,
            )
            .unwrap();

            let mut offset = 0isize;
            loop {
                let info = &*(buffer.as_ptr().offset(offset) as *const FILE_NOTIFY_INFORMATION);
                let action = info.Action;
                let file_name_len = info.FileNameLength as usize / 2;
                let file_name_ptr = info.FileName.as_ptr();
                let file_name_slice = std::slice::from_raw_parts(file_name_ptr, file_name_len);
                let file_name = String::from_utf16_lossy(file_name_slice);

                match action {
                    1 => println!("Added: {}", file_name),
                    2 => println!("Removed: {}", file_name),
                    3 => println!("Modified: {}", file_name),
                    4 => println!("Renamed old name: {}", file_name),
                    5 => println!("Renamed new name: {}", file_name),
                    _ => (),
                }

                if info.NextEntryOffset == 0 {
                    break;
                }
                offset += info.NextEntryOffset as isize;
            }
        }
    }
}
