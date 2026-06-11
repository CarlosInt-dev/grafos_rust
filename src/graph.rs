use std::collections::HashMap;

/// Estructura base del grafo con almacenamiento de ciudades y lista de adyacencia.
pub struct Graph {
    // Lista de ciudades (orden de inserción)
    pub ciudades: Vec<String>,
    // Lista de adyacencia: ciudad -> vecinos
    pub adjacency: HashMap<String, Vec<String>>,
}

impl Graph {
    /// Crea un grafo vacío.
    pub fn new() -> Self {
        Graph {
            ciudades: Vec::new(),
            adjacency: HashMap::new(),
        }
    }

    /// Agrega una ciudad si no existe.
    pub fn agregar_ciudad(&mut self, nombre: &str) {
        if !self.adjacency.contains_key(nombre) {
            self.ciudades.push(nombre.to_string());
            self.adjacency.insert(nombre.to_string(), Vec::new());
        }
    }

    /// Agrega una ruta entre `origen` y `destino`.
    /// El grafo se modela como no dirigido: agrega la arista en ambos sentidos.
    pub fn agregar_ruta(&mut self, origen: &str, destino: &str) {
        // Asegurar que ambas ciudades existan
        self.agregar_ciudad(origen);
        self.agregar_ciudad(destino);

        // Agregar destino a los vecinos de origen si no existe
        if let Some(vec) = self.adjacency.get_mut(origen) {
            if !vec.contains(&destino.to_string()) {
                vec.push(destino.to_string());
            }
        }

        // Agregar origen a los vecinos de destino si no existe (no dirigido)
        if let Some(vec) = self.adjacency.get_mut(destino) {
            if !vec.contains(&origen.to_string()) {
                vec.push(origen.to_string());
            }
        }
    }

    /// Devuelve una referencia a la lista de vecinos de `ciudad` o None si no existe.
    pub fn obtener_vecinos(&self, ciudad: &str) -> Option<&Vec<String>> {
        self.adjacency.get(ciudad)
    }
}

#[cfg(test)]
mod tests {
    use super::Graph;

    #[test]
    fn basic_graph_operations() {
        let mut g = Graph::new();
        g.agregar_ciudad("A");
        g.agregar_ciudad("B");
        g.agregar_ruta("A", "B");

        let vecinos_a = g.obtener_vecinos("A").unwrap();
        assert!(vecinos_a.contains(&"B".to_string()));

        let vecinos_b = g.obtener_vecinos("B").unwrap();
        assert!(vecinos_b.contains(&"A".to_string()));

        // agregar ruta duplicada no debe duplicar entradas
        g.agregar_ruta("A", "B");
        let vecinos_a = g.obtener_vecinos("A").unwrap();
        assert_eq!(vecinos_a.iter().filter(|&x| x == "B").count(), 1);
    }
}
