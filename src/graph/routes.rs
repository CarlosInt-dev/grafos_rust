use std::collections::{HashMap};
use crate::graph::graph::Graph;
use crate::graph::bfs;

pub fn reconstruir_camino(
    padres: &HashMap<String, String>,
    origen: &str,
    destino: &str,
) -> Vec<String> {
    let mut camino = Vec::new();
    let mut actual = destino.to_string();

    // Seguir los padres desde destino hasta origen
    while actual != origen {
        camino.push(actual.clone());
        actual = padres[&actual].clone();
    }

    camino.push(origen.to_string());
    camino.reverse(); // El camino se construyó al revés
    camino
}
pub fn mostrar_ruta(graph: &Graph, origen: &str, destino: &str) {
    println!("\n🔍 Buscando ruta más corta entre '{}' y '{}':", origen, destino);
    match bfs::ruta_mas_corta(graph, origen, destino) {
        Some(ruta) => {
            println!("Ruta encontrada: {}", ruta.join(" -> "));
        }
        None => {
            println!("No se encontró una ruta entre '{}' y '{}'", origen, destino);
        }
    }
}