use std::fs;
pub struct Configuracion{
    pub file_name: String,
    pub consulta: String,

}

impl Configuracion {
    pub fn new(args:&[String]) -> Configuracion {
        let file_name = args[1].clone();
        let consulta=args[2].clone();
        Configuracion { file_name, consulta}
    }
    
}

pub fn run(configuracion:Configuracion) {
    let content=fs::read_to_string(configuracion.file_name).expect("No se puede leer el archivo");
    let found = search(&configuracion.consulta, &content);
    for line in found  {
        println!("{}",line);
    }
}


fn search<'a>(query:&str, content:&'a str)->Vec<&'a str> {
    let mut result = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}