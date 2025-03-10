#[cfg(target_os = "windows")]
pub fn win_get_long_path_name(path: &str) -> anyhow::Result<String> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use winapi::um::fileapi::GetLongPathNameW;

    let mut buf: Vec<u16> = vec![0; 1024];
    let path_wide: Vec<u16> = OsStr::new(path).encode_wide().chain(Some(0)).collect();
    let res = unsafe { GetLongPathNameW(path_wide.as_ptr(), buf.as_mut_ptr(), buf.len() as u32) };
    if res == 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    Ok(String::from_utf16_lossy(&buf[..res as usize]))
}

#[cfg(not(target_os = "windows"))]
pub fn win_get_long_path_name(_path: &str) -> anyhow::Result<String> {
    unimplemented!();
}
