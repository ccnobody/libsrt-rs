pub mod data;
use data::SrtTraceBStats;
use libsrt_sys::{*,SRT_INVLID_SOCKET};
use std::io::{Error, Result};
use std::ffi::c_void;
use std::os::raw::{c_int, c_char};

// 接受连接相关函数
pub fn accept_srt(u: SRTSOCKET, addr: *mut sockaddr, addrlen: *mut c_int) -> Result<SRTSOCKET> {
    let result = unsafe { srt_accept(u, addr, addrlen) };
    if result == SRT_INVLID_SOCKET {
        Err(Error::last_os_error())
    } else {
        Ok(result)
    }
}

pub fn accept_bond_srt(listeners: *const SRTSOCKET, lsize: c_int, ms_time_out: i64) -> Result<SRTSOCKET> {
    let result = unsafe { srt_accept_bond(listeners, lsize, ms_time_out) };
    if result == SRT_INVLID_SOCKET {
        Err(Error::last_os_error())
    } else {
        Ok(result)
    }
}

// 绑定相关函数
pub fn bind_srt(u: SRTSOCKET, name: *const sockaddr, namelen: c_int) -> Result<()> {
    let result = unsafe { srt_bind(u, name, namelen) };
    if result == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn bind_acquire_srt(u: SRTSOCKET, sys_udp_sock: UDPSOCKET) -> Result<()> {
    let result = unsafe { srt_bind_acquire(u, sys_udp_sock) };
    if result == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

// 统计相关函数
pub fn bistats_srt(u: SRTSOCKET, clear: bool, instantaneous: bool) -> Result<SrtTraceBStats> {
    let mut stats = SrtTraceBStats::new();
    let result = unsafe {
        srt_bistats(u, &mut stats as *mut SrtTraceBStats as *mut SRT_TRACEBSTATS, clear as c_int, instantaneous as c_int)
    };
    if result == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(stats)
    }
}

pub fn bstats_srt(u: SRTSOCKET, perf: *mut SRT_TRACEBSTATS, clear: bool) -> Result<()> {
    let result = unsafe { srt_bstats(u, perf, clear as c_int) };
    if result == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn clearlasterror_srt() {
    unsafe { srt_clearlasterror() };
}

pub fn connect_srt(u: SRTSOCKET, name: *const sockaddr, namelen: c_int) -> Result<()> {
    let result = unsafe { srt_connect(u, name, namelen) };
    if result == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn clock_type_srt() -> c_int {
    unsafe { srt_clock_type() }
}

pub fn cleanup_srt() {
    unsafe { srt_cleanup() };
}

pub fn close_srt(u: SRTSOCKET) -> Result<()> {
    let result = unsafe { srt_close(u) };
    if result == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

// Epoll 相关函数
pub fn epoll_create_srt() -> Result<c_int> {
    let result = unsafe { srt_epoll_create() };
    if result == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(result)
    }
}

pub fn epoll_release_srt(eid: c_int) -> Result<()> {
    let result = unsafe { srt_epoll_release(eid) };
    if result == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn epoll_remove_ssock_srt(eid: c_int, s: SYSSOCKET) -> Result<()> {
    let result = unsafe { srt_epoll_remove_ssock(eid, s) };
    if result == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn epoll_update_usock_srt(eid: c_int, u: SRTSOCKET, events: c_int) -> Result<()> {
    let result = unsafe { srt_epoll_update_usock(eid, u, &events as *const c_int) };
    if result == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

// 获取信息相关函数
pub fn getlasterror_srt(errno_loc: *mut c_int) -> c_int {
    unsafe { srt_getlasterror(errno_loc) }
}

pub fn getlasterror_str_srt() -> &'static str {
    unsafe {
        let cstr = std::ffi::CStr::from_ptr(srt_getlasterror_str());
        cstr.to_str().unwrap_or("Invalid error string")
    }
}

pub fn getpeername_srt(u: SRTSOCKET, name: *mut sockaddr, namelen: *mut c_int) -> Result<()> {
    let result = unsafe { srt_getpeername(u, name, namelen) };
    if result == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn getrejectreason_srt(sock: SRTSOCKET) -> c_int {
    unsafe { srt_getrejectreason(sock) }
}

pub fn getsndbuffer_srt(sock: SRTSOCKET, blocks: *mut usize, bytes: *mut usize) -> Result<()> {
    let result = unsafe { srt_getsndbuffer(sock, blocks, bytes) };
    if result == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn getsockflag_srt(u: SRTSOCKET, opt: SRT_SOCKOPT, optval: *mut c_void, optlen: *mut c_int) -> Result<()> {
    let result = unsafe { srt_getsockflag(u, opt, optval, optlen) };
    if result == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn getsockstate_srt(u: SRTSOCKET) -> SRT_SOCKSTATUS {
    unsafe { srt_getsockstate(u) }
}

pub fn getversion_srt() -> u32 {
    unsafe { srt_getversion() }
}

// 监听相关函数
pub fn listen_srt(u: SRTSOCKET, backlog: c_int) -> Result<()> {
    let result = unsafe { srt_listen(u, backlog) };
    if result == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

// 接收相关函数
pub fn recvfile_srt(u: SRTSOCKET, path: *const c_char, offset: *mut i64, size: i64, block: c_int) -> Result<i64> {
    let result = unsafe { srt_recvfile(u, path, offset, size, block) };
    if result == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(result)
    }
}

pub fn rejectreason_str_srt(id: c_int) -> &'static str {
    unsafe {
        let cstr = std::ffi::CStr::from_ptr(srt_rejectreason_str(id));
        cstr.to_str().unwrap_or("Invalid reject reason")
    }
}

pub fn rendezvous_srt(u: SRTSOCKET, local_name: *const sockaddr, local_namelen: c_int, remote_name: *const sockaddr, remote_namelen: c_int) -> Result<()> {
    let result = unsafe { srt_rendezvous(u, local_name, local_namelen, remote_name, remote_namelen) };
    if result == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn recvmsg_srt(u: SRTSOCKET, buf: *mut c_char, len: c_int) -> Result<c_int> {
    let result = unsafe { srt_recvmsg(u, buf, len) };
    if result == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(result)
    }
}

pub fn resetlogfa_srt(fara: *const c_int, fara_size: usize) {
    unsafe { srt_resetlogfa(fara, fara_size) };
}

pub fn recv_srt(u: SRTSOCKET, buf: *mut c_char, len: c_int) -> Result<c_int> {
    let result = unsafe { srt_recv(u, buf, len) };
    if result == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(result)
    }
}

pub fn recvmsg2_srt(u: SRTSOCKET, buf: *mut c_char, len: c_int, mctrl: *mut SRT_MSGCTRL) -> Result<c_int> {
    let result = unsafe { srt_recvmsg2(u, buf, len, mctrl) };
    if result == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(result)
    }
}

// 发送相关函数
pub fn send_srt(u: SRTSOCKET, buf: *const c_char, len: c_int) -> Result<c_int> {
    let result = unsafe { srt_send(u, buf, len) };
    if result == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(result)
    }
}

pub fn sendfile_srt(u: SRTSOCKET, path: *const c_char, offset: *mut i64, size: i64, block: c_int) -> Result<i64> {
    let result = unsafe { srt_sendfile(u, path, offset, size, block) };
    if result == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(result)
    }
}

pub fn sendmsg_srt(u: SRTSOCKET, buf: *const c_char, len: c_int, ttl: c_int, inorder: bool) -> Result<c_int> {
    let result = unsafe { srt_sendmsg(u, buf, len, ttl, inorder as c_int) };
    if result == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(result)
    }
}

pub fn sendmsg2_srt(u: SRTSOCKET, buf: *const c_char, len: c_int, mctrl: *mut SRT_MSGCTRL) -> Result<c_int> {
    let result = unsafe { srt_sendmsg2(u, buf, len, mctrl) };
    if result == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(result)
    }
}

pub fn socket_srt(arg1: c_int, arg2: c_int, arg3: c_int) -> Result<SRTSOCKET> {
    let result = unsafe { srt_socket(arg1, arg2, arg3) };
    if result == SRT_INVLID_SOCKET {
        Err(Error::last_os_error())
    } else {
        Ok(result)
    }
}

// 设置相关函数
pub fn setloghandler_srt(opaque: *mut c_void, handler: SRT_LOG_HANDLER_FN) {
    unsafe { srt_setloghandler(opaque, handler) };
}

pub fn setlogflags_srt(flags: c_int) {
    unsafe { srt_setlogflags(flags) };
}

pub fn setloglevel_srt(ll: c_int) {
    unsafe { srt_setloglevel(ll) };
}

pub fn setrejectreason_srt(sock: SRTSOCKET, value: c_int) {
    unsafe { srt_setrejectreason(sock, value) };
}

pub fn setsockflag_srt(u: SRTSOCKET, opt: SRT_SOCKOPT, optval: *const c_void, optlen: c_int) -> Result<()> {
    let result = unsafe { srt_setsockflag(u, opt, optval, optlen) };
    if result == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn setsockopt_srt(u: SRTSOCKET, level: c_int, optname: SRT_SOCKOPT, optval: *const c_void, optlen: c_int) -> Result<()> {
    let result = unsafe { srt_setsockopt(u, level, optname, optval, optlen) };
    if result == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn time_now_srt() -> i64 {
    unsafe { srt_time_now() }
}
