use std::process::Command;
use inquire::{Select};
pub use taller::clear;

fn ejecutar(binario: &str) {
    clear();
    println!("Ejecutando {}...", binario);

    let status = Command::new("cargo")
        .args(["run","-q","--bin", binario])
        .status();

    match status {
        Ok(s) if s.success() => println!("Regresando al menú principal..."),
        Ok(s) => eprintln!("El subprograma finalizó con código: {:?}", s.code()),
        Err(e) => eprintln!("No se pudo lanzar el subprograma: {}", e),
    }
}

fn main(){
    let opciones = vec![
        "Monitoreo de procesos",
        "Simulador de Memoria Caché",
        "Estrés de Memoria y Salto a la Virtual",
        "Prioridad de Procesos",
        "Salir"];

    loop {
        let seleccion = Select::new("Seleccione una opción:", opciones.clone()).prompt();
    
        match seleccion {
            Ok("Monitoreo de procesos") => {
                ejecutar("monitoreo");
            }

            Ok("Simulador de Memoria Caché") => {
                ejecutar("simulador");
            }

            Ok("Estrés de Memoria y Salto a la Virtual") => {
                ejecutar("estres");
            }

            Ok("Prioridad de Procesos") => {
                ejecutar("prioridad");
            }

            Ok("Salir") => {
                println!("Saliendo...");
                break;
            }
            
            Err(_) => {
                println!("Error, el preceso se a cancelado.");
                break;
            }

            Ok(_) => unreachable!()
            
        }
    }
}