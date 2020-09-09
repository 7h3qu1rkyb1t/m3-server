use std::{
    fs::File,
    io::prelude::*,
    net::{ TcpListener, TcpStream },
};

    // let device = rodio::default_output_device().unwrap();
    // let sink = Sink::new(&device);
    // sink.append(source);
    // thread::sleep(Duration::from_secs(10));
    // sink.pause();
    // thread::sleep(Duration::from_secs(2));
    // sink.play();
    // sink.sleep_until_end();
    // let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

fn main(){
    let listener = TcpListener::bind("127.0.0.1:5533").unwrap();

    for stream in listener.incoming(){
        handler(stream.unwrap());
    }
}

fn handler(mut stream: TcpStream){
    println!("Debug: client connected");
    let mut file = File::open("/storage/music/temp/you! .mp3").unwrap();
    let mut buffer = [0u8;4096];
    // rodio::source::Buffered::from(buffer);
    while let Ok(size) = file.read(&mut buffer){
        if size> 0 {
            stream.write(&buffer[..size]).unwrap();
        }else{
            break;
        }
    }
}
