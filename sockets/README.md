# Sección sockets
--------------------------------------------------------------------------------
Para poder hacer la conexión entre los sockets, se necesita implementar las funciones primitivas, mandar y recibir mensajes.

* Función mandar mensajes:
  * listen(socket) -> String
     ``` rust
      pub fn listen(listen_on: net::SocketAddr)-> String {
       let socket = socket(listen_on);
       let mut data = read_message(socket);
       data
      }
      ```
