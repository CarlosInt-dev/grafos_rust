mod graph;

use std::io::{self, Write};

fn leer_texto(mensaje: &str) -> String {
    print!("{}", mensaje);
    io::stdout().flush().unwrap();

    let mut entrada = String::new();

    io::stdin()
        .read_line(&mut entrada)
        .expect("Error al leer la entrada");

    entrada.trim().to_string()
}

fn mostrar_ciudades(red: &graph::graph::Graph) {
    println!("\nCiudades disponibles:");

    for (i, ciudad) in red.ciudades.iter().enumerate() {
        println!("{}. {}", i + 1, ciudad);
    }
}

fn obtener_ciudad(red: &graph::graph::Graph, entrada: &str) -> Option<String> {
    if entrada.is_empty() {
        return None;
    }

    if let Ok(numero) = entrada.parse::<usize>() {
        if numero >= 1 && numero <= red.ciudades.len() {
            return Some(red.ciudades[numero - 1].clone());
        }

        return None;
    }

    if red.obtener_vecinos(entrada).is_some() {
        return Some(entrada.to_string());
    }

    None
}

fn main() {
    println!("\n🌍 SISTEMA DE RED DE TRANSPORTE - GRAFOS EN RUST");

    let red = graph::data::create_transport_network();

    loop {
        println!("\n==============================");
        println!("MENÚ PRINCIPAL");
        println!("==============================");
        println!("1. Buscar ruta");
        println!("2. Ver ciudades disponibles");
        println!("3. Salir");

        let opcion = leer_texto("\nSeleccione una opción: ");

        match opcion.as_str() {
            "1" => {
                println!("\n--- Buscar ruta más corta ---");

                mostrar_ciudades(&red);

                println!("\nPuede escribir el nombre de la ciudad o el número correspondiente.");

                let entrada_origen = leer_texto("\nIngrese ciudad de origen: ");
                let entrada_destino = leer_texto("Ingrese ciudad de destino: ");

                let origen = match obtener_ciudad(&red, &entrada_origen) {
                    Some(ciudad) => ciudad,
                    None => {
                        println!("\n❌ Ciudad de origen inválida.");
                        println!("Escriba el nombre de la ciudad correctamente o puede escribir el número correspondiente.");
                        continue;
                    }
                };

                let destino = match obtener_ciudad(&red, &entrada_destino) {
                    Some(ciudad) => ciudad,
                    None => {
                        println!("\n❌ Ciudad de destino inválida.");
                        println!("Escriba el nombre de la ciudad correctamente o puede escribir el número correspondiente.");
                        continue;
                    }
                };

                match graph::bfs::ruta_mas_corta(&red, &origen, &destino) {
                    Some(ruta) => {
                        println!("\n✅ Ruta encontrada:");
                        println!("{}", ruta.join(" -> "));

                        let conexiones = ruta.len().saturating_sub(1);
                        println!("Conexiones: {}", conexiones);
                    }
                    None => {
                        println!("\n❌ No se encontró una ruta entre '{}' y '{}'.", origen, destino);
                    }
                }
            }

            "2" => {
                mostrar_ciudades(&red);
            }

            "3" => {
                println!("\nPrograma finalizado.");
                break;
            }

            _ => {
                println!("\n❌ Opción inválida. Ingrese una opción válida: 1, 2 o 3.");
            }
        }
    }
}