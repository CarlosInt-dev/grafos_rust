use std::collections::VecDeque;

/// Grafo genérico con lista de adyacencia basada en índices
#[derive(Debug, Clone)]
pub struct Grafo {
    pub nodos: Vec<String>,
    pub aristas: Vec<Vec<(usize, u32)>>,
}

impl Grafo {
    pub fn new() -> Self {
        Grafo {
            nodos: Vec::new(),
            aristas: Vec::new(),
        }
    }

    pub fn agregar_nodo(&mut self, nombre: String) -> usize {
        let indice = self.nodos.len();
        self.nodos.push(nombre);
        self.aristas.push(Vec::new());
        indice
    }

    pub fn agregar_arista(&mut self, origen: usize, destino: usize, peso: u32) -> Result<(), String> {
        if origen >= self.nodos.len() || destino >= self.nodos.len() {
            return Err("Índice de nodo inválido".to_string());
        }
        self.aristas[origen].push((destino, peso));
        Ok(())
    }

    /// BFS: encuentra la ruta con menos conexiones
    pub fn bfs(&self, origen: usize, destino: usize) -> Option<Vec<usize>> {
        if origen >= self.nodos.len() || destino >= self.nodos.len() {
            return None;
        }

        let mut visitados = vec![false; self.nodos.len()];
        let mut cola = VecDeque::new();
        let mut padre = vec![None; self.nodos.len()];

        cola.push_back(origen);
        visitados[origen] = true;

        while let Some(nodo_actual) = cola.pop_front() {
            if nodo_actual == destino {
                let mut camino = Vec::new();
                let mut actual = destino;
                while let Some(p) = padre[actual] {
                    camino.push(actual);
                    actual = p;
                }
                camino.push(origen);
                camino.reverse();
                return Some(camino);
            }

            for &(vecino, _) in &self.aristas[nodo_actual] {
                if !visitados[vecino] {
                    visitados[vecino] = true;
                    padre[vecino] = Some(nodo_actual);
                    cola.push_back(vecino);
                }
            }
        }

        None
    }

    /// DFS: recorrido en profundidad
    pub fn dfs(&self, inicio: usize) -> Vec<usize> {
        if inicio >= self.nodos.len() {
            return Vec::new();
        }

        let mut visitados = vec![false; self.nodos.len()];
        let mut resultado = Vec::new();
        self.dfs_util(inicio, &mut visitados, &mut resultado);
        resultado
    }

    fn dfs_util(&self, nodo: usize, visitados: &mut Vec<bool>, resultado: &mut Vec<usize>) {
        visitados[nodo] = true;
        resultado.push(nodo);

        for &(vecino, _) in &self.aristas[nodo] {
            if !visitados[vecino] {
                self.dfs_util(vecino, visitados, resultado);
            }
        }
    }

    /// Detección de ciclos usando DFS
    pub fn tiene_ciclo(&self) -> bool {
        let mut visitados = vec![false; self.nodos.len()];
        let mut rec_stack = vec![false; self.nodos.len()];

        for i in 0..self.nodos.len() {
            if !visitados[i] {
                if self.dfs_ciclo(i, &mut visitados, &mut rec_stack) {
                    return true;
                }
            }
        }

        false
    }

    fn dfs_ciclo(&self, nodo: usize, visitados: &mut Vec<bool>, rec_stack: &mut Vec<bool>) -> bool {
        visitados[nodo] = true;
        rec_stack[nodo] = true;

        for &(vecino, _) in &self.aristas[nodo] {
            if !visitados[vecino] {
                if self.dfs_ciclo(vecino, visitados, rec_stack) {
                    return true;
                }
            } else if rec_stack[vecino] {
                return true;
            }
        }

        rec_stack[nodo] = false;
        false
    }

    /// Obtener vecinos de un nodo
    pub fn obtener_vecinos(&self, nodo: usize) -> Option<Vec<(usize, u32)>> {
        if nodo >= self.nodos.len() {
            return None;
        }
        Some(self.aristas[nodo].clone())
    }

    /// Contar número de aristas
    pub fn contar_aristas(&self) -> usize {
        self.aristas.iter().map(|v| v.len()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agregar_nodos() {
        let mut grafo = Grafo::new();
        let n1 = grafo.agregar_nodo("A".to_string());
        let n2 = grafo.agregar_nodo("B".to_string());
        
        assert_eq!(n1, 0);
        assert_eq!(n2, 1);
        assert_eq!(grafo.nodos.len(), 2);
    }

    #[test]
    fn test_bfs_encuentra_camino() {
        let mut grafo = Grafo::new();
        let a = grafo.agregar_nodo("A".to_string());
        let b = grafo.agregar_nodo("B".to_string());
        let c = grafo.agregar_nodo("C".to_string());
        
        grafo.agregar_arista(a, b, 1).unwrap();
        grafo.agregar_arista(b, c, 1).unwrap();
        
        let camino = grafo.bfs(a, c);
        assert_eq!(camino, Some(vec![a, b, c]));
    }

    #[test]
    fn test_ciclo_detectado() {
        let mut grafo = Grafo::new();
        let a = grafo.agregar_nodo("A".to_string());
        let b = grafo.agregar_nodo("B".to_string());
        let c = grafo.agregar_nodo("C".to_string());
        
        grafo.agregar_arista(a, b, 1).unwrap();
        grafo.agregar_arista(b, c, 1).unwrap();
        grafo.agregar_arista(c, a, 1).unwrap();
        
        assert!(grafo.tiene_ciclo());
    }

    #[test]
    fn test_sin_ciclo() {
        let mut grafo = Grafo::new();
        let a = grafo.agregar_nodo("A".to_string());
        let b = grafo.agregar_nodo("B".to_string());
        let c = grafo.agregar_nodo("C".to_string());
        
        grafo.agregar_arista(a, b, 1).unwrap();
        grafo.agregar_arista(b, c, 1).unwrap();
        
        assert!(!grafo.tiene_ciclo());
    }
}
