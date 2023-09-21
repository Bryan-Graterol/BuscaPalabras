use std::env;
use app1_rust::Configuracion;


fn main() {
    let args:Vec<String> = env::args().collect();
    // Variables: archivo y consulta

    let config = Configuracion::new(&args);
    
    println!("archivo: {}",config.file_name);
    println!("query: {}",config.consulta);
    
    app1_rust::run(config);
}



