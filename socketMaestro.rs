use mysql::*;
use mysql::prelude::*;
use chrono::prelude::*; //For date and time
use std::net;
use std::io;
use std::io::*;
use std::net::UdpSocket;
use std::str;
use std::str::from_utf8;
use std::{thread, time::Duration};

pub struct Product {
    id: u64,
    nombre: String,
}
pub fn encontrarArchivos(conn: &mut PooledConn, id: i32) -> std::result::Result<Vec<Product>, mysql::error::Error> {
    conn.exec_map("select id, nombre from prueba1 where id =:id_busqueda",
        params! {
            "id_busqueda" => id,
        },
        |(id,nombre)| Product {
            id: id,
            nombre: nombre
        }
    )
}

pub fn insertarDatos(conn: &mut PooledConn, id: i32, nombre: String) -> std::result::Result<u64, mysql::error::Error>{
	conn.exec_drop("insert into prueba1 (id,nombre) values (:id, :nombre);",
		params! {
			"id" => id,
			"nombre" => nombre,
		},
	).and_then(|_| Ok(conn.last_insert_id()))
}

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
	let url = "mysql://admin:xd@localhost:3306/prueba";
	let pool = Pool::new(url).unwrap();
	//~ let mut conn = pool.get_conn().unwrap();
	
	//~ Seccion conexion udp
	println!("UDP");
	let ip = net::Ipv4Addr::new(127, 0, 0, 1);
	let my_dir = net::SocketAddrV4::new(ip, 34254);
	let send_dir = net::SocketAddrV4::new(ip, 34255);
    println!("Mi direccion {} ",my_dir);
    
    //~ conn.query_iter("select id, nombre from prueba1").unwrap().for_each(|row| {
		//~ let r:(i32, String) = from_row(row.unwrap());
		//~ println!("{}, {}", r.0, r.1);
	//~ });
	//~ println!("Escriba el id:");
	//~ let mut id = String::new();
	//~ let mut idUser: i32;
	//~ std::io::stdin().read_line(&mut id).unwrap();
	//~ let idUser: i32 = id.trim().parse().expect("invalid input");
	//~ println!("Escriba el nombre:");
	//~ let mut nombreUser = String::new();
	
	//~ match std::io::stdin().read_line(&mut nombreUser) {
        //~ Ok(_) => {
            //~ let nombreUserClean = nombreUser.trim();
            //~ nombreUser = String::from(nombreUserClean);
        //~ }
        //~ Err(e) => {
            //~ println!("{:?}", e);
        //~ }
    //~ }
	
	//~ println!("id es {}",idUser);
	//~ print!("nombre es {}",nombreUser);
	loop{
		let mut conn = pool.get_conn().unwrap();
		println!("{:}", "=".repeat(80));
		let msg = listen(net::SocketAddr::V4(my_dir));
		println!("mensaje recibido: {:?}", msg);
		let idMsg: i32 = msg.trim().parse().expect("invalid input");
		let handle = thread::spawn(move || {
			//~ match insertarDatos(&mut conn, idUser, nombreUser) {
				//~ Ok(last_id) => println!("Inserted product with ID {}", last_id),
				//~ Err(e) => println!("Error: {:?}", e),
			//~ }
			let _ = encontrarArchivos(&mut conn, idMsg).map(|list| {
				for p in list {
					println!("Found product {}, {}", p.id,p.nombre);
				}
			});
		});
		println!("{:}", "=".repeat(80));
		//~ join() se encarga de espera a que el hilo termine su ejecución, unwrap() desenvuelve el resultado que regresa join()
		handle.join().unwrap();
	}
        
        
	
            
}
