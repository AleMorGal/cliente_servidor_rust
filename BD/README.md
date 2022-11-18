# Sección de conexión con base de datos

El programa [main.rs](/BD/src/main.rs) es un ejemplo de las implementación de las funciones que se usaran para usar la base de datos en el sistema cliente servidor.

Se asume que el sistema en el que se ejecuta tiene las siguientes características:
- MariaDB instalado
- Base de datos creada después de ejecutar el archivo [createDB.sql](/BD/createDB.sql)

Este código utiliza el *crate* `sqlx` que permite la conectividad con la base de datos. Se debe de incluir en el archivo `Cargo.toml` del proyecto, como se ve [aquí](Cargo.toml). Además de incluirlo allí, se incluye en el programa con `use sqlx::mysql::MySqlPoolOptions;`.

Cabe mencionar que `sqlx` permite hacer uso de las funciones de procesamiento asíncronas de Rust para admitir varias conexiones a la base de datos a la vez. Esto se configura al hacer de la función `main` del programa `async`. También, al crear la conexión con la base de datos se establece el número máximo de conexiones usando `max_connections(n)`.

Adicionalmente, se requiere crear un archivo llamado `.env`, como el que se ve [aquí](.env). Este tiene la siguiente estructura:

```
feature = "macros"
DATABASE_URL = URL_DE_BASE_DE_DATOS
```

Este archivo es requerido para alguno de los macros que utiliza `sqlex`. Para que el programa pueda leer el archivo, se incluye el crate `dotenv` usando la instrucción `use dotenv::dotenv;`. 

A grandes rasgos, se le pasa a la base de datos un *query*, y los resultados se guardan en un `struct` con los campos que regrese el *query*. Si se desea guardar varias filas, estas se guardan en un vector que almacena los `struct` que contienen cada una de las filas.

Primero, el programa muestra el identificador y el nombre de todos los archivos que están en la base de datos. Será tarea del servidor mandar el vector que contiene esta información al cliente.

Después, se le pide al usuario que ingrese el identificador del archivo que desea recuperar. El cliente manda al servidor este da. El identificador se usa para mandar un *query*  la base de datos que regresa el nombre del archivo, su extensión y su ruta. Esto sucedería del lado del servidor.

Teniendo la ruta, se busca el archivo, se abre como un *stream* de datos y se convierte en un vector de bytes. Se le pide al usuario que ingrese la ruta en la que se va a guardar el archivo, y el vector se escribe en un nuevo archivo. El servidor enviaría el vector de bytes al cliente, el cual se encargaría de escribir el nuevo archivo.

Para mayores detalles, leer los comentarios dentro de [main.rs](/BD/src/main.rs).
