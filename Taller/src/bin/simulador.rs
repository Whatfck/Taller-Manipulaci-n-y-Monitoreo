use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::time::Instant;

pub use taller::clear;

fn main() {
    // 1. Inicializar la Caché en RAM
    let mut cache: HashMap<String, String> = HashMap::new();

    loop {
        clear();

        println!("            SIMULADOR DE MEMORIA CACHÉ            ");
        println!("==================================================");
        println!(" Archivos en RAM: {}", cache.len());
        println!("==================================================");
        println!(" Ingrese la ruta de un archivo (ej: Cargo.toml)");
        println!(" (O presione [ENTER] sin escribir nada para salir)");
        print!("> ");
        let _ = io::stdout().flush();

        // 2. Capturar la ruta del archivo
        let mut entrada = String::new();
        if io::stdin().read_line(&mut entrada).is_err() {
            break;
        }

        let ruta = entrada.trim();

        // Si el usuario presiona ENTER sin escribir nada, sale al menú principal
        if ruta.is_empty() {
            break;
        }

        println!("\n--------------------------------------------------");

        // 3. Verificar si el archivo ya está en la Caché en RAM
        if cache.contains_key(ruta) {
            // --- HIT DE CACHÉ ---
            let inicio = Instant::now();
            let contenido = cache.get(ruta).unwrap();
            let duracion = inicio.elapsed();

            println!(" [HIT DE CACHÉ] Archivo leído directamente desde la RAM");
            println!(" Tamaño:            {} bytes", contenido.len());
            println!(" Tiempo de acceso:  {:?}", duracion);
        } else {
            // --- MISS DE CACHÉ ---
            println!(" [MISS DE CACHÉ] El archivo no está en RAM.");
            println!(" Leyendo desde el disco físico (Memoria Secundaria)...");

            // 4. Leer desde el disco físico y guardar en la RAM
            let inicio = Instant::now();
            match fs::read_to_string(ruta) {
                Ok(contenido) => {
                    let duracion = inicio.elapsed();

                    println!(" Leído desde DISCO exitosamente.");
                    println!(" Tamaño:            {} bytes", contenido.len());
                    println!(" Tiempo de acceso:  {:?}", duracion);

                    // Guardamos la copia en la RAM
                    cache.insert(ruta.to_string(), contenido);
                    println!(" -> Archivo cargado en la memoria RAM para futuras consultas.");
                }
                Err(e) => {
                    println!(" Error al leer el archivo desde el disco: {}", e);
                }
            }
        }

        println!("==================================================");
        println!("\nPresione [ENTER] para continuar...");
        let mut _pausa = String::new();
        let _ = io::stdin().read_line(&mut _pausa);
    }
}