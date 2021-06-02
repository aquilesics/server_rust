
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;


fn main() {
    let listener: TcpListener =  TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming(){
        //nao mutavel
        let stream = stream.unwrap();
        println!("conexao estabelecida");
        handle_connection(stream);
    }
    
}

fn handle_connection(mut stream:TcpStream){
    //ouvindo conexao
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();

    // respondendo conexao
    let get = b"GET / HTTP/1.1\r\n"; 

    if buffer.starts_with(get) {
        let contents = fs::read_to_string("Hello.html").unwrap();
    
        let status = "HTTP/1.1 200 OK\r\n";
        let header  = format!("Content-Length: {}\r\n",contents.len());
        let body = contents;

        let response = format!(
            "{}{}\r\n{}",
            status,
            header,
            body);

        let data = response.as_bytes();
        stream.write(data).unwrap();
        stream.flush().unwrap();    
    }

    else {
        let contents = fs::read_to_string("404.html").unwrap();

        let status = "HTTP/1.1 404 NOT FOUND\r\n";
        let body = contents;

        let response = format!("{}\r\n{}",status,body);

        let data = response.as_bytes();
        stream.write(data).unwrap();
        stream.flush().unwrap();  
    }
    
   
}

