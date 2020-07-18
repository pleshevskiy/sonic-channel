use sonic_channel::*;


fn main() -> Result<(), SonicError> {
    // let mut stream = TcpStream::connect("localhost:1491")?;

    // let mut buffer = [0; 128];
    // stream.read(&mut buffer)?;

    // let res = String::from_utf8(buffer.to_vec()).expect("Cannot convert response buffer to utf8");
    // dbg!(res);

    // let command = "START search SecretPassword";

    // stream.write(command.as_bytes())?;

    // let mut buffer = [0; 1000];
    // loop {
    //     let mut line_buffer = [0; 128];
    //     let res_length = stream.read(&mut line_buffer)?;
        
    // }
    // stream.read(&mut buffer)?;

    // let res = String::from_utf8(buffer.to_vec()).expect("Cannot convert response buffer to utf8");

    // dbg!(res);

    // let mut buffer = [0; 128];
    // stream.read(&mut buffer)?;
    // let res = String::from_utf8(buffer.to_vec()).expect("Cannot convert response buffer to utf8");
    // dbg!(res);


    // let (tx, rx) = mpsc::channel();

    // thread::spawn(|| {
    //     let listener = TcpListener::bind("localhost:7777").expect("Should open connection for tests");

    //     for stream in listener.incoming() {
    //         let mut stream = stream.expect("Should connect socket successfully");
    //         stream.write("CONNECTED\r\n".as_bytes()).unwrap();

    //         loop {
    //             let mut buffer = [0; 60];
    //             println!("wait from client");
    //             let n = stream.read(&mut buffer).expect("Cannot read stream from client");

    //             let message = String::from_utf8(buffer[0..n].to_vec()).expect("Should convert response buffer to utf8");
    //             dbg!(&message);

    //             if message.starts_with("START") {
    //                 stream.write("STARTED search protocol(1) buffer(20000)\r\n".as_bytes()).unwrap();
    //             } else if message.starts_with("PING") {
    //                 stream.write("PONG\r\n".as_bytes()).unwrap();
    //             }
    //         }
    //     }
    // });


    // let client = thread::spawn(|| {
    //     let mut stream = TcpStream::connect("localhost:7777").expect("Should open connection for tests");

    //     let message = "hello world".as_bytes();
    //     stream.write_all(message).expect("Should write to stream successfully");
    // });

    // client.join().unwrap();
    // server.join().unwrap();

    // let received = rx.recv().expect("Should recieve message from thread");

    // dbg!(received);

    let mut channel = SonicChannel::connect("localhost:1491")?;
    // std::thread::sleep(std::time::Duration::from_secs(5));
    // let mut channel = SonicChannel::connect("localhost:7777")?;
    // channel.start(ChannelMode::Ingest, "SecretPassword")?;
    // std::thread::sleep(std::time::Duration::from_secs(1));
    // let pong = channel.ping()?;
    // dbg!(pong);
    // let pushed = channel.push("collection", "bucket", "user:1", "my best recipe")?;
    // dbg!(pushed);

    channel.start(ChannelMode::Search, "SecretPassword")?;
    std::thread::sleep(std::time::Duration::from_secs(1));
    let pong = channel.ping()?;
    dbg!(pong);

    let objects = channel.query("collection", "bucket", "recipe")?;
    dbg!(objects);

    Ok(())
}