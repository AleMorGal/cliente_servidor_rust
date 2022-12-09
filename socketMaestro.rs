use mysql::*;
use mysql::prelude::*;
use chrono::prelude::*;
use std::fs::File;
use std::io::Read;
use std::net;
use std::io;
//~ use std::io::*;
use std::net::UdpSocket;
use std::str;
//~ use std::str::from_utf8;
use std::{thread, time::Duration};
use serde::{Serialize, Deserialize};
use bincode;

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    id: i32,
    filename: String,
    filePath: String,
    extension: String,
}


#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Mensaje{
	confirmacion: Option<bool>,
  palabra_busqueda: Option<String>,
	contenido: Option<Vec<Product>>,
  archivo: Option<Vec<u8>>,
}

#[allow(dead_code)]
impl Mensaje{
  fn new(confirmacion: bool, palabra_busqueda: String, contenido: Vec<Product>, archivo: Vec<u8>) -> Mensaje {
    Mensaje{
      confirmacion: Some(confirmacion),
      palabra_busqueda: Some(palabra_busqueda),
      contenido: Some(contenido),
      archivo: Some(archivo),
    }
  }
}

pub fn encontrarArchivos(conn: &mut PooledConn, id: i32) -> std::result::Result<Vec<Product>, mysql::error::Error> {
    conn.exec_map("select id, filename, filePath, extension from files where id =:id_busqueda",
        params! {
            "id_busqueda" => id,
        },
        |(id,filename, filePath, extension)| Product {
            id: id,
            filename: filename,
            filePath: filePath,
            extension: extension,
        }
    )
}

/*pub fn insertarDatos(conn: &mut PooledConn, id: i32, nombre: String) -> std::result::Result<u64, mysql::error::Error>{
	conn.exec_drop("insert into prueba1 (id,nombre) values (:id, :nombre);",
		params! {
			"id" => id,
			"nombre" => nombre,
		},
	).and_then(|_| Ok(conn.last_insert_id()))
}*/

fn socket(listen_on: net::SocketAddr) -> net::UdpSocket {
  let attempt = net::UdpSocket::bind(listen_on);
  let mut socket;
  match attempt {
    Ok(sock) => {
      println!("Conectado a {}", listen_on);
      socket = sock;
    },
    Err(err) => panic!("Could not bind: {}", err)
  }
  socket
}

fn read_message(socket: net::UdpSocket) -> String {
  let mut buf = [0; 2048];
  println!("Reading data");
  let result = socket.recv_from(&mut buf);
  drop(socket);
  //~ let mut data;
  let mut req_msg ;
  let mut msg;
  match result {
    Ok((amt, src)) => {
		let buf = &mut buf[..amt];
      println!("Received data from {}", src);
      req_msg = str::from_utf8(&buf).unwrap();
      //~ req_msg = from_utf8(&buf).expect("utf-8 convert failed");
      //~ println!("mensaje recivido: {:?}", req_msg);
      //~ data = Vec::from(&buf[0..amt]);
      //~ println!("mensaje en bytes: {:?}", data);
      msg = String::from(req_msg);
    },
    Err(err) => panic!("Read error: {}", err)
  }
  msg
}

pub fn send_message(send_addr: net::SocketAddr, target: net::SocketAddr, data: Vec<u8>) {
  let socket = socket(send_addr);
  println!("Mensaje enviado");
  let result = socket.send_to(&data, target);
  drop(socket);
  match result {
    Ok(amt) => println!("Se enviaron {} bytes", amt),
    Err(err) => panic!("Write error: {}", err)
  }
}

pub fn listen(listen_on: net::SocketAddr) -> String {
	let socket = socket(listen_on);
	let mut data = read_message(socket);
	data
}


fn main() {
	//~ Seccion de base de datos
	let url = "mysql://root:root@localhost:3306/udp_server_client";
	let pool = Pool::new(url).unwrap();
	//~ let mut conn = pool.get_conn().unwrap();
	
	//~ Seccion conexion udp
	println!("UDP");
	let ip = net::Ipv4Addr::new(127, 0, 0, 1);
	let my_dir = net::SocketAddrV4::new(ip, 34254);
	let send_dir = net::SocketAddrV4::new(ip, 34255);
    println!("Mi direccion {} ",my_dir);
    
	loop{
		let mut conn = pool.get_conn().unwrap();
		println!("{:}", "=".repeat(80));
		let msg = listen(net::SocketAddr::V4(my_dir));
		println!("mensaje recibido: {:?}", msg);
		let idMsg: i32 = msg.trim().parse().expect("invalid input");
		let handle = thread::spawn(move || {
			
			//Ete pedazo de código se queda comentado por si se quiere agregar datos a la BD
			
			//~ match insertarDatos(&mut conn, idUser, nombreUser) {
				//~ Ok(last_id) => println!("Inserted product with ID {}", last_id),
				//~ Err(e) => println!("Error: {:?}", e),
			//~ }
			
			let resultadosBD = encontrarArchivos(&mut conn, idMsg);
			
			let mut vecResultados = Vec::new();
			resultadosBD.map(|lista| {
				for elemento in lista {
					println!("Found product {}, {}, {}, {}", elemento.id,elemento.filename, elemento.filePath, elemento.extension);
					vecResultados.push(elemento);
				}
			});
      let mut data_encoded= Vec::new();
      /*for i in vecResultados.iter_mut(){
        let mut f = File::open(i.filePath.clone()).expect("No se puede abrir el archivo");
        f.read_to_end(&mut data_encoded).expect("No se puede convertir el archivo");
      }
      //f.read_to_end(&mut data_encoded).expect("No se puede convertir el archivo");*/
			let m = Mensaje{confirmacion: Some(true), palabra_busqueda: None, contenido: Some(vecResultados), archivo: Some(data_encoded)};
			let codigo_serializado = bincode::serialize(&m).unwrap();
			send_message(net::SocketAddr::V4(my_dir), net::SocketAddr::V4(send_dir), codigo_serializado);

			
		});
		println!("{:}", "=".repeat(80));
		
		//~ join() se encarga de espera a que el hilo termine su ejecución, unwrap() desenvuelve el resultado que regresa join()
		handle.join().unwrap();
	}
            
}
