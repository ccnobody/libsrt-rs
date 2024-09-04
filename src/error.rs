use std::fmt;
use std::ffi::CStr;
use std::ffi::c_int;

#[cfg(unix)]
use nix::errno::Errno;

#[cfg(windows)]
use windows_sys::Win32::Foundation::WIN32_ERROR;

pub struct SrtError {
    code: i32,
    sys_error_code: i32,
    message: Option<String>,
    #[cfg(unix)]
    unix_error: Option<Errno>,
    #[cfg(windows)]
    windows_error: Option<WIN32_ERROR>,
}

impl SrtError {
    pub fn new(code: i32,sys_error_code: i32) -> Self {
        SrtError {
            code,
            sys_error_code,
            message: None,
            #[cfg(unix)]
            unix_error: Some(Errno::from_i32(sys_error_code)),
            #[cfg(windows)]
            windows_error: Some(sys_error_code as WIN32_ERROR),
        }
    }

    pub fn with_message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }

    pub fn code(&self) -> i32 {
        self.code
    }

    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    #[cfg(unix)]
    pub fn sys_error(&self) -> Option<Errno> {
        self.unix_error
    }

    #[cfg(windows)]
    pub fn sys_error(&self) -> Option<WIN32_ERROR> {
        self.windows_error
    }
}

impl fmt::Display for SrtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "srt_error:{} sys_erron:{} ", self.code,self.sys_error_code)?;
        if let Some(ref msg) = self.message {
            write!(f, ": {}", msg)?;
        }
        Ok(())
    }
}

impl fmt::Debug for SrtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("SrtError");
        debug_struct.field("code", &self.code);
        debug_struct.field("sys_error_code", &self.sys_error_code);
        debug_struct.field("message", &self.message);
        #[cfg(unix)]
        debug_struct.field("unix_error", &self.unix_error);
        #[cfg(windows)]
        debug_struct.field("windows_error", &self.windows_error);
        debug_struct.finish()
    }
}

pub fn srt_get_lasterror() -> SrtError {
    unsafe {
        let mut sys_error_no : c_int = 0;
        let srt_error_no = libsrt_sys::srt_getlasterror(&mut sys_error_no as *mut c_int);
        let c_str = libsrt_sys::srt_getlasterror_str();
        let mut error = SrtError::new(srt_error_no,sys_error_no);
        if !c_str.is_null() {
            let error_message = CStr::from_ptr(c_str).to_string_lossy().into_owned();
            if !error_message.is_empty() {
                error = error.with_message(error_message);
            }
        }
        error
    }
}

pub fn srt_clearlasterror(){
    unsafe{
        libsrt_sys::srt_clearlasterror();
    }
}