use std::fs::OpenOptions;
use std::io::{self, BufRead, Write};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use sysinfo::System;

pub use taller::clear;

/// Guarda un registro de alerta en alertas_ram.txt si la RAM supera el umbral
fn registrar_alerta_ram(porcentaje: f64, usada: f64, total: f64) {
    // Abre el archivo en modo 'append' (si no existe, lo crea)
    if let Ok(mut archivo) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("alertas_ram.txt")
    {
        // Obtener timestamp UNIX en segundos
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let registro = format!(
            "[{}] ALERTA: Consumo de RAM crítico ({:.1}%). Usada: {:.2} GB / Total: {:.2} GB\n",
            timestamp, porcentaje, usada, total
        );

        let _ = archivo.write_all(registro.as_bytes());
    }
}

fn main() {
    let mut sys = System::new_all();

    // espera y recibe enter
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let stdin = io::stdin();
        let mut iterator = stdin.lock().lines();
        if let Some(Ok(_)) = iterator.next() {
            let _ = tx.send(());
        }
    });

    loop {
        clear();

        // Refresca la memoria y el uso de CPU
        sys.refresh_memory();
        sys.refresh_cpu_usage();

        // --- Datos de RAM ---
        let total_gb = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
        let used_gb = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
        let free_gb = sys.free_memory() as f64 / 1024.0 / 1024.0 / 1024.0;

        let porcentaje_uso = if total_gb > 0.0 {
            (used_gb / total_gb) * 100.0
        } else {
            0.0
        };

        // --- Datos de SWAP ---
        let total_swap_gb = sys.total_swap() as f64 / 1024.0 / 1024.0 / 1024.0;
        let used_swap_gb = sys.used_swap() as f64 / 1024.0 / 1024.0 / 1024.0;
        let free_swap_gb = sys.free_swap() as f64 / 1024.0 / 1024.0 / 1024.0;

        let porcentaje_swap = if total_swap_gb > 0.0 {
            (used_swap_gb / total_swap_gb) * 100.0
        } else {
            0.0
        };

        // --- Verificación y guardado de log (> 80%) ---
        if porcentaje_uso > 80.0 {
            registrar_alerta_ram(porcentaje_uso, used_gb, total_gb);
        }

        // --- Datos de CPU ---
        let porcentaje_cpu = sys.global_cpu_usage();
        let cpus = sys.cpus();
        let nucleos = cpus.len();
        let modelo_cpu = cpus
            .first()
            .map(|cpu| cpu.brand().trim())
            .unwrap_or("Desconocido");

        println!("            HERRAMIENTA DE MONITOREO              ");
        println!("==================================================");
        println!("                       RAM                        ");
        println!("==================================================");
        println!(" Porcentaje Uso: {:.1}%", porcentaje_uso);
        if porcentaje_uso > 80.0 {
            println!(" [!] ADVERTENCIA: Alto consumo (>80%) - Log guardado");
        }
        println!("--------------------------------------------------");
        println!(" Total:          {:.2} GB", total_gb);
        println!(" Usada:          {:.2} GB", used_gb);
        println!(" Libre:          {:.2} GB", free_gb);
        println!("--------------------------------------------------");
        println!(" SWAP Total:     {:.2} GB", total_swap_gb);
        println!(" SWAP Usada:     {:.2} GB ({:.1}%)", used_swap_gb, porcentaje_swap);
        println!(" SWAP Libre:     {:.2} GB", free_swap_gb);
        println!();
        println!("==================================================");
        println!("                       CPU                        ");
        println!("==================================================");
        println!(" Porcentaje Uso: {:.1}%", porcentaje_cpu);
        println!("--------------------------------------------------");
        println!(" CPU:            {}", modelo_cpu);
        println!(" Núcleos:        {}", nucleos);
        println!();
        println!("==================================================");
        println!("Presione [ENTER] para regresar al menú principal");

        // Comprueba si se recibió la señal de Enter (sin bloquear)
        if rx.try_recv().is_ok() {
            break;
        }

        // Refresco cada 1 segundo
        thread::sleep(Duration::from_secs(1));
    }
}