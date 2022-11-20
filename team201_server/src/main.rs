// import needed modules
use std::io;
use std::time; // to set duration for server connection timeout
use std ::net::{TcpListener,TcpStream}; // server and client network functionality
use std::io::{Read,Write}; // access the input and output
use std::thread; // to native Rust threads

// enable access stream
// hold the stream’s state in a struct and perform input-output operations
fn handle_sender(mut stream: TcpStream) -> io::Result<()>{
    // cater to multiple access stream
    let mut buf = [0;512];
    for _ in 0..1000{
        // to enable receiver get sender's messages
        let bytes_read = stream.read(&mut buf)?;
        // sender stream as mutable variable
        if bytes_read == 0{
            return Ok(());
        }
        stream.write(&buf[..bytes_read])?;
        // acknowledge acceptance
        //render the sent message
        println!("from the sender:{}",String::from_utf8_lossy(&buf));
        // connection with a connected sender can be set to sleep
        thread::sleep(time::Duration::from_secs(1));  
    }
    // indicate success
    Ok(())
}

fn main() -> io::Result<()>{
    // get port 7878 binding enabled
    let receiver_listener = TcpListener::bind("127.0.0.1:7878").expect("Failed and bind with the sender");
    // Get a grip of the thread underlying
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
    // incoming connections, messages are listened to, and bound to a sever socket address.
    for stream in receiver_listener.incoming() {
        let stream = stream.expect("failed");
        // allow receiver and sender connect
        let handle = thread::spawn(move || {
            // should receiver be unable to read from the stream
            handle_sender(stream).unwrap_or_else(|error| eprintln!("{:?}",error))
        });
        
        // messages pushed across in same order they got sent
        thread_vec.push(handle);
    }

    for handle in thread_vec {
        // every single value Output in the heap gets returned
        handle.join().unwrap();
    }
    // indicate success
    Ok(())
}