use std::io::{self, BufRead, Write};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use sysinfo::System;

pub use taller::clear;

fn main() {
    let mut sys = System::new_all();

    // Vector principal que mantendrá los datos vivos en RAM / Swap
    let mut contenedor_masivo: Vec<Vec<u8>> = Vec::new();

    let mb = 1024 * 1024;
    let bloque_mb = 50; // 50 MB por iteración

    // Límite de seguridad: detener si la RAM libre es menor a 100 MB (0.10 GB)
    let limite_seguridad_ram_libre_gb = 0.10;

    // Hilo secundario para escuchar la tecla ENTER sin bloquear la asignación de memoria
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let stdin = io::stdin();
        let mut iterator = stdin.lock().lines();
        if let Some(Ok(_)) = iterator.next() {
            let _ = tx.send(());
        }
    });

    let mut iteracion = 0;

    loop {
        clear();

        sys.refresh_memory();

        let total_ram_gb = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
        let libre_ram_gb = sys.free_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
        let usada_ram_gb = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;

        let total_swap_gb = sys.total_swap() as f64 / 1024.0 / 1024.0 / 1024.0;
        let usada_swap_gb = sys.used_swap() as f64 / 1024.0 / 1024.0 / 1024.0;

        let asignado_total_mb = contenedor_masivo.len() * bloque_mb;

        iteracion += 1;

        println!("      ESTRÉS DE MEMORIA Y SALTO A LA VIRTUAL (SWAP)      ");
        println!("==========================================================");
        println!(" Retenido por el proceso: {} MB (~{:.2} GB)", asignado_total_mb, asignado_total_mb as f64 / 1024.0);
        println!(" Iteración actual:         {}", iteracion);
        println!("----------------------------------------------------------");
        println!(" RAM Usada:                {:.2} / {:.2} GB", usada_ram_gb, total_ram_gb);
        println!(" RAM Libre:                {:.2} GB", libre_ram_gb);
        println!(" SWAP Usada:               {:.2} / {:.2} GB", usada_swap_gb, total_swap_gb);
        println!("==========================================================");
        println!(" Presione [ENTER] en cualquier momento para detener la prueba.");

        // --- VERIFICACIÓN DE SEÑAL DE ENTER ---
        if rx.try_recv().is_ok() {
            println!("\n [!] Asignación detenida por el usuario.");
            break;
        }

        // --- VERIFICACIÓN DE SEGURIDAD ---
        if libre_ram_gb < limite_seguridad_ram_libre_gb {
            println!("\n [!] Límite de seguridad alcanzado (< {:.2} GB RAM libre).", limite_seguridad_ram_libre_gb);
            println!(" [!] Asignación detenida para prevenir congelamiento.");
            break;
        }

        // --- ASIGNACIÓN DE MEMORIA ---
        let mut nuevo_bloque = vec![0u8; bloque_mb * mb];
        for byte in nuevo_bloque.iter_mut().step_by(4096) {
            *byte = 255;
        }

        contenedor_masivo.push(nuevo_bloque);

        thread::sleep(Duration::from_millis(300));
    }

    println!("\n==========================================================");
    println!(" Memoria retenida. Verifique el uso en el Administrador de Tareas.");
    println!(" Presione [ENTER] para liberar memoria y regresar al menú...");
    let _ = io::stdout().flush();

    let mut _pausa = String::new();
    let _ = io::stdin().read_line(&mut _pausa);

    // Liberación explícita
    contenedor_masivo.clear();
}