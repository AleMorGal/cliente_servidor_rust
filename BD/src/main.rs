use sqlx::mysql::MySqlPoolOptions;
use dotenv::dotenv;
use std::io::{self, Write};
use std::fs::File;
use std::io::Read;

//datos en base de datos deben de ser NOT NULL para evitar usar Option
#[allow(dead_code)]
#[derive(Debug)]
struct Archivo{
    id:i32,
    filename:String,
}

#[allow(dead_code)]
#[derive(Debug)]
struct file_p{
    fileName:String,
    filePath:String,
    extension:String,
}

#[async_std::main]
async fn main() -> Result<(), sqlx::Error>
{
    //Vector en el que se guarda cada entrada de la base de datos
    let pool = MySqlPoolOptions::new()
        .max_connections(2)
        .connect("mysql://root:root@localhost/udp_test")
        .await?;

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
    File contiene ID de archivo y su nombre*/
    println!("Mostrando archivos disponibles:");
    let files: Vec<Archivo> = 
        sqlx::query_as!(Archivo, r"select id, filename from files")
        .fetch_all(&pool)
        .await?;
    println!("{:#?}", files);

    //Pidiendo a usuario ID de documento
    println!("Inserta el ID del documento que quieres recuperar:");
    let mut id_select = String::new();
    let b1 = std::io::stdin().read_line(&mut id_select).unwrap();

    //Pasando ID de documento a BD, debería regresar ruta de documento
    println!("Recuperando documento...");
    let f_path = sqlx::query_as!(file_p,
        "select fileName, filePath, extension from files where id = ?", id_select)
        .fetch_one(&pool)
        .await?;
    
    //println!("Ruta del archivo: {}{}", f_path.filePath, f_path.extension);

    /*Una vez que tengamos ruta, abrimos documento y se convierte a bytes, que
    se almacenan como Vec<u8>. Ese vector de datos binarios es el que se envía
    por medio de UDP. Cuando el cliente recibe datos, los vuelve a guardar en un 
    documento.*/
    let mut data = Vec::new();
    let mut f = File::open(f_path.filePath).expect("No se puede abrir el archivo");
    f.read_to_end(&mut data).expect("No se puede leer el archivo.");
    let mut new_path = String::new();
    //Pedimos a usuario insertar nueva ubicación donde se va a guardar archivo
    println!("Inserta la ruta en donde se guardará el archivo:");
    let b2 = std::io::stdin().read_line(&mut new_path).unwrap();
    //Se le quita el newline a la ruta
    new_path.pop();
    //Ahora que ya tenemos nueva ruta, lo juntamos con nombre de archivo y extensión
    //Por ahora no copia la extensión. Intentar corregir.
    let mut v = vec![new_path,"/".to_string(),f_path.fileName,f_path.extension];
    //println!("{:#?}", v);
    let s: String = v.concat();
    //println!("{}", s);
    //Creamos el archivo en el disco del cliente
    let mut fw = File::create(s).expect("No se puede crear archivo");
    //Guardamos el vector de bytes en el archivo
    fw.write_all(&data).expect("No se puede crear archivo");
    println!("El archivo se guardó exitosamente");
    Ok(())
}
