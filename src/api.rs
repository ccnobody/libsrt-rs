include!("error.rs");
use std::net::SocketAddr;
use std::str::FromStr;
use nix::libc;

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

pub fn srt_bind_socket(fd: i32, socket_addr: &std::net::SocketAddr) -> Result<i32, SrtError> {
    
    let (raw_addr, addr_len) = socket_addr_to_c_addr(&socket_addr);
    println!("raw_addr: {:?}, addr_len: {:?}", raw_addr, addr_len);
    let ret = unsafe { libsrt_sys::srt_bind(fd, raw_addr, addr_len) };
    if ret == -1 {
        return Err(srt_get_lasterror());
    }
    Ok(ret)
}

// 在文件的其他地方添加这个辅助函数
pub fn socket_addr_to_c_addr(addr: &std::net::SocketAddr) -> (*const libsrt_sys::sockaddr, i32) {
    match addr {
        std::net::SocketAddr::V4(addr) => (addr as *const _ as *const _, std::mem::size_of::<std::net::SocketAddrV4>() as i32),
        std::net::SocketAddr::V6(addr) => (addr as *const _ as *const _, std::mem::size_of::<std::net::SocketAddrV6>() as i32),
    }
}

// 将字符串转换为SocketAddr： ip:port => SocketAddr    
pub fn str_to_socket_addr(addr_str: &str) -> Result<SocketAddr, std::net::AddrParseError> {
    SocketAddr::from_str(addr_str)
}

// 绑定udp_fd到srt_fd, 成功则返回srt_fd, 失败返回错误
pub fn srt_bind_acquire(fd: i32, udp_fd: i32) -> Result<i32, SrtError> {    
    let ret = unsafe { libsrt_sys::srt_bind_acquire(fd, udp_fd) };
    if ret == -1 {
        return Err(srt_get_lasterror());
    }
    Ok(fd)
}

pub fn srt_getsockstate(fd: i32) -> libsrt_sys::SRT_SOCKSTATUS   {
    let ret = unsafe { libsrt_sys::srt_getsockstate(fd) };
    ret
}

pub fn srt_getsndbuffer(fd: i32, blocks: &mut usize, bytes: &mut usize) -> Result<i32, SrtError> {
    let ret = unsafe { libsrt_sys::srt_getsndbuffer(fd, blocks as *mut usize, bytes as *mut usize) };
    if ret == -1 {
        return Err(srt_get_lasterror());
    }
    Ok(ret)
}

pub fn srt_close(fd: i32) -> Result<i32, SrtError> {
    let ret = unsafe { libsrt_sys::srt_close(fd) };
    if ret == -1 {
        return Err(srt_get_lasterror());
    }
    Ok(fd)
}

pub fn srt_listen(fd: i32, backlog: i32) -> Result<i32, SrtError> {
    let ret = unsafe { libsrt_sys::srt_listen(fd, backlog) };
    if ret == -1 {
        return Err(srt_get_lasterror());
    }
    Ok(fd)
}   

pub fn srt_accept(fd: i32, addr: &mut SocketAddr) -> Result<i32, SrtError> {
    let mut storage: std::mem::MaybeUninit<libsrt_sys::sockaddr_storage> = std::mem::MaybeUninit::uninit();
    let mut addrlen = std::mem::size_of::<libsrt_sys::sockaddr_storage>() as i32;
    
    let ret = unsafe {
        libsrt_sys::srt_accept(fd, storage.as_mut_ptr() as *mut _, &mut addrlen)
    };
    
    if ret == -1 {
        return Err(srt_get_lasterror());
    }
    
    // 将接受到的地址转换为 SocketAddr
    unsafe {
        *addr = sockaddr_to_std_addr(&storage.assume_init(), addrlen as usize)
            .map_err(|_| SrtError::rust_err(RUST_ERR_CODE,"Failed to convert address"))?;
    }
    
    Ok(ret)
}

// 添加这个辅助函数来将 sockaddr 转换为 SocketAddr
fn sockaddr_to_std_addr(storage: &libsrt_sys::sockaddr_storage, len: usize) -> std::io::Result<SocketAddr> {
    use std::net::{SocketAddrV4, SocketAddrV6, Ipv4Addr, Ipv6Addr};
    
    match storage.ss_family as i32 {
        libc::AF_INET => {
            if len < std::mem::size_of::<libc::sockaddr_in>() {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "无效的地址长度"));
            }
            let addr_in: &libc::sockaddr_in = unsafe { &*(storage as *const _ as *const libc::sockaddr_in) };
            let ip = Ipv4Addr::from(u32::from_be(addr_in.sin_addr.s_addr));
            let port = u16::from_be(addr_in.sin_port);
            Ok(SocketAddr::V4(SocketAddrV4::new(ip, port)))
        },
        libc::AF_INET6 => {
            if len < std::mem::size_of::<libc::sockaddr_in6>() {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "无效的地址长度"));
            }
            let addr_in6: &libc::sockaddr_in6 = unsafe { &*(storage as *const _ as *const libc::sockaddr_in6) };
            let ip = Ipv6Addr::from(addr_in6.sin6_addr.s6_addr);
            let port = u16::from_be(addr_in6.sin6_port);
            let flowinfo = addr_in6.sin6_flowinfo;
            let scope_id = addr_in6.sin6_scope_id;
            Ok(SocketAddr::V6(SocketAddrV6::new(ip, port, flowinfo, scope_id)))
        },
        _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "不支持的地址族")),
    }
}

pub fn srt_connect(fd: i32, addr: &SocketAddr) -> Result<i32, SrtError> {
    let (raw_addr, addr_len) = socket_addr_to_c_addr(&addr);
    let ret = unsafe { libsrt_sys::srt_connect(fd, raw_addr, addr_len) };
    if ret == -1 {
        return Err(srt_get_lasterror());
    }
    Ok(fd)
}