# Sección de hilos
El objetivo de esta sección es explicar como funciona la concurrencia en el lenguaje de programación de Rust mediante el uso de hilos.

## Creación de Hilos
Para hacer uso de los hilos en Rust es necesario importar "thread" de la libreria estandar de Rust.

```rust
//~ Importar thread de la biblioteca estandar de rust, esto permitira el uso de los hilos.
use std::thread;
};
```

Para poder crear un hilo se utiliza la función `thread::spawn`, una vez que se crea el hilo, este encargará de ejecutar las instrucciones que se encuentren dentro de las llaves {}.
```rust
//~ Función que se encarga de crear un hilo.
thread::spawn(move || {
  //~ Código que ejecutara el hilo
});
```
Hay dos puntos importantes a destacar de esta función:
- `JoinHandle`: La función una vez que es ejecutada, devuelve un objeto `JoinHandle`, este objeto se desprende del hilo cuando este termina, esto quiere decir que toma el resultado que obtuvo el hilo y lo regresa, sin embargo, al desprenderse el hilo se asegura que ya no se pueda volver a acceder a el hilo del que se desprendio.
- `move`: La palabra clave 'move' se encarga de tomar o 'mover' el valor y propiedad que tiene la refencia usada dentro del hilo y asi poder usar este valor como una variable dentro del hilo. Cuando se 'mueve' este valor, la variable que era propietaria en un inicio, ya no lo sera, por lo que, no podra volver a usarse esa variable fuera del hilo. 

Para asegurar que cada uno de los hilos termina su ejecución, se guardar el resultado del hilo en una variable manejadora (Handle) y se una la función `join()` la cual se encargar de esperar a que el hilo termine su ejecución, join() devuelve un resultado (resultado del hilo o un error) por lo que, se usa `unwrap()` para obtener ese resultado.

### Ejemplo
```rust
//~ Importar thread de la biblioteca estandar de rust, esto permitira el uso de los hilos.
use std::thread;

fn main() {
    for i in 1..10 {
        //~ Handle se usa para manejar el JoinHandle que recibe de la función thread::spawn
        //~ thread::spawn se encarga de crear un hilo
        let handle = thread::spawn(move || {
            //~ la refencia que "mueve" en esta caso move es (i)
            println!("Hola desde el hilo {}", i);
        });
        //~ join() se encarga de espera a que el hilo termine su ejecución, unwrap() desenvuelve el resultado que regresa join()
        handle.join().unwrap();
    }
}
```

## Paso de mensajes entre hilos

La principal caracteristica por el cual se usan los hilos es para realizar tareas de manera más eficiente repartiendo el trabajo, para lograr esto se realiza un paso de mensajes entre hilos, por medio de estos mensajes es como se logra establecer una comunicación entre los hilos y se vuelve posible divir una tarea en multiples tareas más pequeñas, asignarselas a los hilos y despues juntar los resultados.

Para poder pasa mensajes entre los hilos en Rust se usan "Canales" por los cuales se mueve la información. Para poder hacer uso de estos es necesario importar mpsc (multi-producer, single-consumer) de la libreria estandar de Rust.
```rust
//~ Importar mpsc de la biblioteca estandar de rust, esto permitira el paso de mensajes entre canales
use std::sync::mpsc;
```

Para crear un canal se utiliza la función `mpsc::channel()`, esta función nos devolvera una tupla que contiene:
- Sender<T>: Quien envia el mensaje.
- Receiver<T>: Quien recibe el mensaje.
  
Comunmente en Rust a Sender se le asigna la variable `tx` por ser el transmisor y a Receiver se le asigna la variable `rx` por ser el receptor. Tanto `send()` como `recv()` devuelven un objeto resultado, por lo que, se usa `unwrap()` para desenvolverlo.
  
### Ejemplos
```rust
//~ Importar thread de la biblioteca estandar de rust, esto permitira el uso de los hilos.
use std::thread;
//~ Importar mpsc de la biblioteca estandar de rust, esto permitira el paso de mensajes entre canales
use std::sync::mpsc;

fn main() {
    //~ Se crea el canal y declaramos nuestro transmisor y nuestro transmisor
    let (tx, rx) = mpsc::channel();

    //~ Se crea un hilo
    thread::spawn(move || {
        //~ Se crea un mensaje simple
        let msg = String::from("Hola desde el hilo");

        //~ El transmisor se encarga de enviar el mensaje simple
        tx.send(msg).unwrap();
    });

    //~ El receptor recibe le mensaje y lo imprime
    let received = rx.recv().unwrap();
    println!("Obtuve el mensaje: {}", received); 
}
```
En este ejemplo se usa `recv()`, sin embargo, no es la unica forma de recibir el mensaje, en realidad hay dos formas de recibir el mensaje:
  - `recv()`: Es una función **bloqueante**, bloquea a los hilos mientras espera por un mensaje, devuelve un resultado (mensaje o erro), en cuanto recibe el mensaje o cuando el canal se cierra.
  - `try_recv()`: Es una función **NO bloqueante**, no bloquea a los hilos esperando por el mensaje, devuelve un resultado de manera inmediata (mensaje o error).


