//Tcp listener nos deja "escuchar" conexiones TCP.
use std::net::TcpListener

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

        println!("Se estableció una conexión");
    }

}
