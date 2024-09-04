use libsrt_sys::{ srt_epoll_create};

fn main() {
   let epoll = unsafe { srt_epoll_create() };
   println!("epoll: {}", epoll);
}
