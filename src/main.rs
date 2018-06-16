//Llamamos a prelude para acceder a funciones que nos dejarán leer
//y escribir en el "stream".
use std::io::prelude::*;
//Tcp listener nos deja "escuchar" conexiones TCP.
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;

fn main() {
    //Aquí elegimos "escuchar" la dirección 127.0.0.2 en el puerto 7878"
    //Además de que 7878 es RUST escrito en teléfono los no-administradores
    //solo pueden escuchar en puertos mayores al 1024
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    //La función bind en éste escenario es como la función "new" que
    //regresará un nuevo "TcpListener", la función se llama "Bind" porque
    //en redes conectarse a un puerto es conocido como "enlazamiento"


    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }

}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let mut file = File::open("index.html").unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
    let status_line = "HTTP/1.1 404 ERROR A LA VERGA!\r\n\r\n";

    let mut file = File::open("404.html").unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    }
}
