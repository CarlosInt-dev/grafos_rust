use std::collections::VecDeque;

/// Estructura para representar un grafo usando lista de adyacencia
/// basada en índices de vectores (soluciona problemas de borrowing de Rust)
#[derive(Debug, Clone)]
struct Grafo {
    nodos: Vec<String>,
    aristas: Vec<Vec<(usize, u32)>>, // Vec de (índice_destino, peso)
}

impl Grafo {
    /// Crea un nuevo grafo vacío
    fn new() -> Self {
        Grafo {
            nodos: Vec::new(),
            aristas: Vec::new(),
        }
    }

    /// Añade un nodo al grafo y retorna su índice
    fn agregar_nodo(&mut self, nombre: String) -> usize {
        let indice = self.nodos.len();
        self.nodos.push(nombre);
        self.aristas.push(Vec::new());
        indice
    }

    /// Añade una arista dirigida entre dos nodos con peso
    fn agregar_arista(&mut self, origen: usize, destino: usize, peso: u32) -> Result<(), String> {
        if origen >= self.nodos.len() || destino >= self.nodos.len() {
            return Err("Índice de nodo inválido".to_string());
        }
        self.aristas[origen].push((destino, peso));
        Ok(())
    }

    /// BFS: Búsqueda en Anchura - encuentra la ruta con menos conexiones
    /// Retorna el camino desde origen hasta destino
    fn bfs(&self, origen: usize, destino: usize) -> Option<Vec<usize>> {
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
                // Reconstruir el camino
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

    /// DFS: Búsqueda en Profundidad - detecta ciclos
    fn tiene_ciclo(&self) -> bool {
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

    /// Auxiliar para DFS que detecta ciclos usando recursión
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

    /// DFS simple: retorna lista de nodos en orden de visita
    fn dfs(&self, inicio: usize) -> Vec<usize> {
        if inicio >= self.nodos.len() {
            return Vec::new();
        }

        let mut visitados = vec![false; self.nodos.len()];
        let mut resultado = Vec::new();
        self.dfs_util(inicio, &mut visitados, &mut resultado);
        resultado
    }

    /// Auxiliar para DFS que recorre el grafo
    fn dfs_util(&self, nodo: usize, visitados: &mut Vec<bool>, resultado: &mut Vec<usize>) {
        visitados[nodo] = true;
        resultado.push(nodo);

        for &(vecino, _) in &self.aristas[nodo] {
            if !visitados[vecino] {
                self.dfs_util(vecino, visitados, resultado);
            }
        }
    }

    /// Imprime información del grafo de forma legible
    fn imprimir(&self) {
        println!("\n╔════════════════════════════════════════╗");
        println!("║          INFORMACIÓN DEL GRAFO         ║");
        println!("╚════════════════════════════════════════╝");
        println!("\nNodos ({}):", self.nodos.len());
        for (i, nodo) in self.nodos.iter().enumerate() {
            println!("  [{}] {}", i, nodo);
        }

        println!("\nAristas:");
        for (origen, destinos) in self.aristas.iter().enumerate() {
            for (destino, peso) in destinos {
                println!("  {} → {} (distancia: {} km)", 
                    self.nodos[origen], 
                    self.nodos[*destino], 
                    peso);
            }
        }
    }
}

fn main() {
    println!("\n🌍 SISTEMA DE RED DE TRANSPORTE - GRAFOS EN RUST");
    println!("═══════════════════════════════════════════════════\n");

    // Crear un grafo para una red de transporte entre ciudades
    let mut red_transporte = Grafo::new();

    // Agregar ciudades (nodos)
    println!("📍 Creando red de ciudades...");
    let madrid = red_transporte.agregar_nodo("Madrid".to_string());
    let barcelona = red_transporte.agregar_nodo("Barcelona".to_string());
    let valencia = red_transporte.agregar_nodo("Valencia".to_string());
    let bilbao = red_transporte.agregar_nodo("Bilbao".to_string());
    let sevilla = red_transporte.agregar_nodo("Sevilla".to_string());
    let malaga = red_transporte.agregar_nodo("Málaga".to_string());

    // Agregar conexiones (aristas) con distancias
    println!("🔗 Agregando rutas de transporte...\n");
    red_transporte.agregar_arista(madrid, barcelona, 620).unwrap();
    red_transporte.agregar_arista(madrid, valencia, 360).unwrap();
    red_transporte.agregar_arista(madrid, bilbao, 395).unwrap();
    red_transporte.agregar_arista(barcelona, valencia, 360).unwrap();
    red_transporte.agregar_arista(madrid, sevilla, 534).unwrap();
    red_transporte.agregar_arista(sevilla, malaga, 215).unwrap();
    red_transporte.agregar_arista(bilbao, madrid, 395).unwrap();

    // Mostrar información del grafo
    red_transporte.imprimir();

    // ═════════════════════════════════════════
    // DEMOSTRACIÓN BFS: Encontrar ruta más corta
    // ═════════════════════════════════════════
    println!("\n\n┌─────────────────────────────────────────┐");
    println!("│  🔍 BFS: RUTA CON MENOS CONEXIONES     │");
    println!("└─────────────────────────────────────────┘");
    
    let origen = madrid;
    let destino = sevilla;
    
    match red_transporte.bfs(origen, destino) {
        Some(camino) => {
            println!("\n✓ Ruta encontrada de {} a {}:", 
                red_transporte.nodos[origen], 
                red_transporte.nodos[destino]);
            print!("  Camino: ");
            for (i, &nodo) in camino.iter().enumerate() {
                if i > 0 { print!(" → "); }
                print!("{}", red_transporte.nodos[nodo]);
            }
            println!("\n  Conexiones utilizadas: {}", camino.len() - 1);
        }
        None => println!("✗ No hay ruta disponible"),
    }

    // ═════════════════════════════════════════
    // DEMOSTRACIÓN DFS: Recorrido
    // ═════════════════════════════════════════
    println!("\n\n┌─────────────────────────────────────────┐");
    println!("│  🔍 DFS: RECORRIDO EN PROFUNDIDAD      │");
    println!("└─────────────────────────────────────────┘");
    
    let inicio = madrid;
    let recorrido = red_transporte.dfs(inicio);
    
    println!("\n✓ Recorrido desde {}:", red_transporte.nodos[inicio]);
    print!("  Orden de visita: ");
    for (i, &nodo) in recorrido.iter().enumerate() {
        if i > 0 { print!(" → "); }
        print!("{}", red_transporte.nodos[nodo]);
    }
    println!();

    // ═════════════════════════════════════════
    // DETECCIÓN DE CICLOS
    // ═════════════════════════════════════════
    println!("\n\n┌─────────────────────────────────────────┐");
    println!("│  🔄 DETECCIÓN DE CICLOS                │");
    println!("└─────────────────────────────────────────┘\n");
    
    if red_transporte.tiene_ciclo() {
        println!("⚠️  El grafo contiene ciclos");
    } else {
        println!("✓ El grafo es acíclico (es un DAG - Grafo Acíclico Dirigido)");
    }

    // ═════════════════════════════════════════
    // EJEMPLO CON CICLO
    // ═════════════════════════════════════════
    println!("\n\n┌─────────────────────────────────────────┐");
    println!("│  📊 EJEMPLO CON CICLO                  │");
    println!("└─────────────────────────────────────────┘\n");

    let mut grafo_con_ciclo = Grafo::new();
    let n1 = grafo_con_ciclo.agregar_nodo("A".to_string());
    let n2 = grafo_con_ciclo.agregar_nodo("B".to_string());
    let n3 = grafo_con_ciclo.agregar_nodo("C".to_string());

    grafo_con_ciclo.agregar_arista(n1, n2, 1).unwrap();
    grafo_con_ciclo.agregar_arista(n2, n3, 1).unwrap();
    grafo_con_ciclo.agregar_arista(n3, n1, 1).unwrap(); // Crea ciclo: A→B→C→A

    grafo_con_ciclo.imprimir();
    
    println!("\nDetección de ciclos:");
    if grafo_con_ciclo.tiene_ciclo() {
        println!("⚠️  Este grafo contiene ciclos: A → B → C → A");
    } else {
        println!("✓ El grafo es acíclico");
    }

    println!("\n═══════════════════════════════════════════════════");
    println!("✅ Demostración completada exitosamente\n");
}