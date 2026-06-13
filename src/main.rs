mod data;

fn main() {
    println!("\n=== SISTEMA DE RED DE TRANSPORTE - EL SALVADOR ===\n");

    let red = data::create_transport_network();

    println!("Ciudades registradas: {}", red.nodos.join(", "));
    println!("Total de rutas: {}\n", red.contar_aristas());

    // BFS: San Salvador -> Ahuachapan
    let origen  = 0; // San Salvador
    let destino = 3; // Ahuachapan

    println!("Buscando ruta de {} a {}...\n", red.nodos[origen], red.nodos[destino]);

    match red.bfs(origen, destino) {
        Some(camino) => {
            println!("Ruta encontrada:");
            for (i, &nodo) in camino.iter().enumerate() {
                if i > 0 { print!(" -> "); }
                print!("{}", red.nodos[nodo]);
            }
            println!("\nConexiones: {}", camino.len() - 1);
        }
        None => println!("No se encontro ruta."),
    }

    // DFS desde San Salvador
    println!("\nRecorrido DFS desde San Salvador:");
    let recorrido = red.dfs(origen);
    for (i, &nodo) in recorrido.iter().enumerate() {
        if i > 0 { print!(" -> "); }
        print!("{}", red.nodos[nodo]);
    }
    println!();

    // Ciclos
    if red.tiene_ciclo() {
        println!("\nEl grafo contiene ciclos.");
    } else {
        println!("\nEl grafo es aciclico.");
    }
}
