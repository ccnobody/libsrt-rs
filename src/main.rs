mod api;
use api::srt_create_socket;

fn main() {
   match srt_create_socket() {
       Ok(fd) => println!("epoll: {}", fd),
       Err(e) => eprintln!("Failed to create SRT socket: {:?}", e),
   };
}
