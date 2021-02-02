use postgres::{Client, NoTls, Error};
use std::fs::File;
use std::io::Read;


struct Persona {
    identificacion: String,
    nombre: String,
    genero: String,
    estado_civil: String,
    fecha_nac: String,
    telefono: String,
    direccion: String,
    email: String,
    estado: i32,
    observacion: String

}

fn main() -> Result<(), Error> {


    let mut client = Client::connect("postgresql://postgres:admin1234@localhost:5432/datosPersona", NoTls)?;
    


    let filename = "src/archivo.csv";
    // Open the file in read-only mode.
    match File::open(filename) {
        // The file is open (no error).
        Ok(mut file) => {
            let mut content = String::new();

            // Read all the file content into a variable (ignoring the result of the operation).
            file.read_to_string(&mut content).unwrap();

            let data=content;


let split = data.lines();
let vec = split.collect::<Vec<&str>>();

for i in &vec {
    let split1 = i.split(";");
    let vecsv = split1.collect::<Vec<&str>>();
    
    
    let persona = Persona {

        identificacion: vecsv[0].to_string(),
        nombre: vecsv[1].to_string(),
        genero:vecsv[2].to_string(),
        estado_civil: vecsv[3].to_string(),
        fecha_nac: vecsv[4].to_string(),
        telefono: vecsv[5].to_string(),
        direccion:vecsv[6].to_string(),
        email: vecsv[7].to_string(),
        estado:1,
        observacion: String::from("ninguna")
    };

    

    client.execute(
        "INSERT INTO personas (identificacion, nombre, genero, estado_civil, fecha_nac, telefono, direccion, email, estado, observacion) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)",
        &[&persona.identificacion,&persona.nombre,&persona.genero,&persona.estado_civil,&persona.fecha_nac,&persona.telefono,&persona.direccion,&persona.email,&persona.estado,&persona.observacion],
)?;
    

   }//Cierre de for vec
    
               },
        // Error handling.
        Err(error) => {
            println!("Error opening file {}: {}", filename, error);
        },
    }
    
    Ok(())
    
     
}
