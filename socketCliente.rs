use std::{thread, time::Duration};
use std::net;
use std::io;
//~ use std::io::*;
use std::net::UdpSocket;
use std::str;
//~ use std::str::from_utf8;
use serde::{Serialize, Deserialize};
use bincode;

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    id: u64,
    nombre: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Mensaje{
	contenido: Vec<Product>,
}


fn socket(listen_on: net::SocketAddr) -> net::UdpSocket {
  let attempt = net::UdpSocket::bind(listen_on);
  let mut socket;
  match attempt {
    Ok(sock) => {
      println!("conectado a {}", listen_on);
      socket = sock;
    },
    Err(err) => panic!("Could not bind: {}", err)
  }
  socket
}

//~ fn read_message(socket: net::UdpSocket) -> String {
fn read_message(socket: net::UdpSocket) -> Mensaje {
	let mut buf = [0; 2048];
	println!("Reading data");
	let result = socket.recv_from(&mut buf);
	drop(socket);
	//~ let mut req_msg ;
	let mut msg: Mensaje;
	match result {
		Ok((amt, src)) => {
			let buf = &mut buf[..amt];
			println!("Received data from {}", src);
			msg= bincode::deserialize(&buf).unwrap();
			println!("mensaje deserializado {:?}",msg);
			//~ req_msg = str::from_utf8(&buf).unwrap();
			//~ msg = String::from(req_msg);
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

pub fn listen(listen_on: net::SocketAddr)-> Mensaje {
	let socket = socket(listen_on);
	let mut data = read_message(socket);
	data
}

fn main(){
	println!("UDP");
	let ip = net::Ipv4Addr::new(127, 0, 0, 1);
	let listen_addr = net::SocketAddrV4::new(ip, 34254);
	let send_addr = net::SocketAddrV4::new(ip, 34255);
	println!("Mi direccion {} ",send_addr);
	println!("Escriba el mensaje:");
	let mut input = String::new();
	std::io::stdin().read_line(&mut input).unwrap();
	println!("{}",input);
	
	////Parte mensaje a enviar///////
	let data: Vec<u8> = input.as_bytes().to_vec();
	send_message(net::SocketAddr::V4(send_addr), net::SocketAddr::V4(listen_addr), data);
	
	////Parte para recibir el mensaje/////
	println!("{:}", "=".repeat(80));
	let msgRecibido = listen(net::SocketAddr::V4(send_addr));
	println!("mensaje recibido: {:?}", msgRecibido);
	let mut vecMensaje = msgRecibido.contenido; //Sacamos el contenido del mensaje, que es un vector de Product
	for i in vecMensaje.iter_mut() { //Realizamos iteraciones para sacar los items de la base de datos
		println!("id es {} y el nombre es {}", i.id, i.nombre);
	}
	println!("{:}", "=".repeat(80));
}
