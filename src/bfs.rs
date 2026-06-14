use std::collections::{HashMap, VecDeque};

use crate::graph::Graph;

/// Busca la ruta con menos conexiones entre `origen` y `destino`
/// usando Búsqueda en Anchura (BFS).
///
/// Devuelve `Some(Vec<String>)` con las ciudades del camino en orden,
/// o `None` si no existe ruta entre ambas ciudades.
pub fn bfs(graph: &Graph, origen: &str, destino: &str) -> Option<Vec<String>> {
    // Validar que ambas ciudades existan en el grafo
    if graph.obtener_vecinos(origen).is_none() || graph.obtener_vecinos(destino).is_none() {
        return None;
    }
    
    // Caso trivial: origen == destino
    if origen == destino {
        return Some(vec![origen.to_string()]);
    }

    // Cola de ciudades por visitar
    let mut cola: VecDeque<String> = VecDeque::new();

    // Registro de ciudades ya visitadas
    let mut visitados: HashMap<String, bool> = HashMap::new();

    // Mapa de padres: ciudad -> ciudad desde la que llegamos
    // Esto nos permite reconstruir el camino al final
    let mut padres: HashMap<String, String> = HashMap::new();

    // Inicializar BFS desde el origen
    cola.push_back(origen.to_string());
    visitados.insert(origen.to_string(), true);

    while let Some(ciudad_actual) = cola.pop_front() {
        // Explorar cada vecino de la ciudad actual
        if let Some(vecinos) = graph.obtener_vecinos(&ciudad_actual) {
            for vecino in vecinos {
                if !visitados.contains_key(vecino) {
                    // Marcar como visitado y registrar su padre
                    visitados.insert(vecino.clone(), true);
                    padres.insert(vecino.clone(), ciudad_actual.clone());

                    // Si llegamos al destino, reconstruir y devolver el camino
                    if vecino == destino {
                        return Some(reconstruir_camino(&padres, origen, destino));
                    }

                    cola.push_back(vecino.clone());
                }
            }
        }
    }

    // Si la cola se vació sin encontrar el destino, no hay ruta
    None
}

/// Reconstruye el camino desde `origen` hasta `destino`
/// usando el mapa de padres generado por BFS.
fn reconstruir_camino(
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