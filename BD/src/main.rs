use sqlx::mysql::MySqlPoolOptions;
use dotenv::dotenv;

//datos en base de datos deben de ser NOT NULL para evitar usar Option
#[allow(dead_code)]
#[derive(Debug)]
struct File{
    id:i32,
    filename:String,
}

#[async_std::main]
async fn main() -> Result<(), sqlx::Error>
{
    //Vector en el que se guarda cada entrada de la base de datos
    //let mut x: Vec<File> = Vec::new();
    let pool = MySqlPoolOptions::new()
        .max_connections(2)
        .connect("mysql://root:root@localhost/udp_test")
        .await?;
    /*sqlx::query("INSERT INTO files VALUES (?,?,?)")
        .bind(8)
        .bind("archivo10")
        .bind("/run/media/cardcathouse/Main/Documentos/SistemasDistribuidos/Prácticas/")
        .execute(&pool)
        .await?;*/
    
    /*Pasos siguientes:
        -Guardar ID y nombre de archivo en arreglo/vector 
        -Se muestra al usuario lista de archivos que hay disponibles
        -Usuario inserta ID de archivo que quiere consultar
        -Cliente manda ID de archivo a servidor
        -Servidor obtiene ruta de archivo a partir de ID
        -Servidor obtiene archivo y manda a cliente a través de UDP
        -Fin*/
    
    /*Guardando lista de archivos en struct. 
    ¿Arreglo de structs para mostrar todos? 
    ¿Comprobar que ID existe en lista?
    */

    /*Mostrando entradas en base de datos
    Para usar macro query_as! se requiere:
    -usar crate dotenv
    -crear archivo .env en raíz de carpeta de poryecto
    -dentro de archivo, poner
        DATABASE_URL = misma url que se usa en connect
        feature = "macros"
    -en programa, use dotenv::dotenv;
    -en Cargo.toml también se debe declarar uso de dotenv
    -Se puede usar cargo use dotenv en raíz de proyecto
    */

    /*Todas las entradas se guardan en un vector que guarda el struct File
    File contiene ID de archivo y su nombre
    ¿cómo hacer para que solo se imprima datos sin some? */
    let files: Vec<File> = 
        sqlx::query_as!(File, r"select id, filename from files")
        .fetch_all(&pool)
        .await?;
    println!("{:?}", files);
    Ok(())

    
}
