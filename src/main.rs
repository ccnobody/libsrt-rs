mod api;
use api::{srt_bind_socket, srt_create_socket, srt_getsockstate, str_to_socket_addr};

fn main() {
   match srt_create_socket() {
       Ok(fd) => {
        println!("fd: {}", fd);
        let socket_addr = str_to_socket_addr("127.0.0.1:9000").unwrap();
        println!("socket_addr: {:?}", socket_addr);
        match  srt_bind_socket(fd, &socket_addr) {
            Ok(_) => {
                let state = srt_getsockstate(fd);
                println!("state: {:?}", state);
            },
            Err(e) => eprintln!("Failed to bind SRT socket: {:?}", e),
        }
       },
       Err(e) => eprintln!("Failed to create SRT socket: {:?}", e),
   };

}
