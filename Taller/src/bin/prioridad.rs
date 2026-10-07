use std::io::{self, Write};
use std::time::Instant;

pub use taller::clear;

/// Ajusta la prioridad del proceso en sistemas Unix/macOS mediante nice.
fn establecer_prioridad_macos(es_alta: bool) {
    let valor_nice = if es_alta { -10 } else { 19 };

    unsafe {
        let _ = libc::nice(valor_nice);
    }
}

/// Tarea de cálculo intensivo para forzar el uso del núcleo de CPU
fn trabajo_pesado_cpu() -> u64 {
    let mut suma: u64 = 0;
    let total_iteraciones: u64 = 100_000_000;

    for i in 1..=total_iteraciones {
        suma = suma.wrapping_add((i).wrapping_mul(i).wrapping_rem(7));
    }
    suma
}

fn main() {
    loop {
        clear();

        println!("           SIMULADOR DE PRIORIDAD DE PROCESOS             ");
        println!("==========================================================");
        println!(" Demostración del Planificador de Procesos (Scheduler)");
        println!("==========================================================");
        println!(" Seleccione la prioridad para esta ejecución:");
        println!("  [1] Prioridad BAJA (Nice = 19 - Fondo)");
        println!("  [2] Prioridad ALTA (Nice = -10 - Requiere sudo)");
        println!("  (O presione [ENTER] sin escribir nada para salir)");
        print!("\nOpción (1 u 2): ");
        let _ = io::stdout().flush();

        let mut entrada = String::new();
        if io::stdin().read_line(&mut entrada).is_err() {
            break;
        }

        let seleccion = entrada.trim();

        if seleccion.is_empty() {
            break;
        }

        let es_alta = seleccion == "2";
        let modo_txt = if es_alta { "ALTA" } else { "BAJA" };

        println!("\n----------------------------------------------------------");
        println!(" Configurando prioridad: {}", modo_txt);
        establecer_prioridad_macos(es_alta);

        println!(" Ejecutando cálculo matemático pesado...");
        let inicio = Instant::now();
        let _resultado = trabajo_pesado_cpu();
        let duracion = inicio.elapsed();

        println!("----------------------------------------------------------");
        println!(" Cálculo finalizado exitosamente.");
        println!(" Tiempo total de ejecución: {:?}", duracion);
        println!("==========================================================");

        println!("\n Presione [ENTER] para continuar...");
        let _ = io::stdout().flush();
        let mut _pausa = String::new();
        let _ = io::stdin().read_line(&mut _pausa);
    }
}