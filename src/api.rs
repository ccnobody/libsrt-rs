

include!("error.rs");
// return true,success
// return false,failed
pub fn srt_startup() -> bool {
   let ret = unsafe { libsrt_sys::srt_startup() };
   ret == 0
}

pub fn srt_cleanup() -> bool {
    let ret = unsafe { libsrt_sys::srt_cleanup() };
    ret == 0
}

pub fn srt_create_socket()->Result<i32,SrtError>{
    let ret = unsafe { libsrt_sys::srt_create_socket() };
    if ret == -1 {
        return Err(srt_get_lasterror());
    }   
    Ok(ret)
}