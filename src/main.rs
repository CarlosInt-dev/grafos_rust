<<<<<<< HEAD

=======
use grafos_rust::Grafo;
>>>>>>> 93029acfaee85c2fa2591dea1fb5f4d75067fece

fn main() {
    println!("\n🌍 SISTEMA DE RED DE TRANSPORTE - GRAFOS EN RUST");
    println!("═══════════════════════════════════════════════════\n");

    let mut red_transporte = Grafo::new();

    println!("📍 Creando red de ciudades...");
    let madrid = red_transporte.agregar_nodo("Madrid".to_string());
    let barcelona = red_transporte.agregar_nodo("Barcelona".to_string());
    let valencia = red_transporte.agregar_nodo("Valencia".to_string());
    let bilbao = red_transporte.agregar_nodo("Bilbao".to_string());
    let sevilla = red_transporte.agregar_nodo("Sevilla".to_string());
    let malaga = red_transporte.agregar_nodo("Málaga".to_string());

    println!("🔗 Agregando rutas de transporte...\n");
    red_transporte.agregar_arista(madrid, barcelona, 620).unwrap();
    red_transporte.agregar_arista(madrid, valencia, 360).unwrap();
    red_transporte.agregar_arista(madrid, bilbao, 395).unwrap();
    red_transporte.agregar_arista(barcelona, valencia, 360).unwrap();
    red_transporte.agregar_arista(madrid, sevilla, 534).unwrap();
    red_transporte.agregar_arista(sevilla, malaga, 215).unwrap();
    red_transporte.agregar_arista(bilbao, madrid, 395).unwrap();

    println!("Ciudades: {}", red_transporte.nodos.join(", "));
    println!("Total de rutas: {}\n", red_transporte.contar_aristas());

    // ═════════════════════════════════════════
    // BFS
    // ═════════════════════════════════════════
    println!("┌─────────────────────────────────────────┐");
    println!("│  🔍 BFS: RUTA CON MENOS CONEXIONES     │");
    println!("└─────────────────────────────────────────┘\n");
    
    if let Some(camino) = red_transporte.bfs(madrid, sevilla) {
        println!("✓ Ruta de Madrid a Sevilla:");
        print!("  Camino: ");
        for (i, &nodo) in camino.iter().enumerate() {
            if i > 0 { print!(" → "); }
            print!("{}", red_transporte.nodos[nodo]);
        }
        println!("\n  Conexiones: {}", camino.len() - 1);
    }

    // ═════════════════════════════════════════
    // DFS
    // ═════════════════════════════════════════
    println!("\n┌─────────────────────────────────────────┐");
    println!("│  🔍 DFS: RECORRIDO EN PROFUNDIDAD      │");
    println!("└─────────────────────────────────────────┘\n");
    
    let recorrido = red_transporte.dfs(madrid);
    println!("✓ Recorrido desde Madrid:");
    print!("  Orden de visita: ");
    for (i, &nodo) in recorrido.iter().enumerate() {
        if i > 0 { print!(" → "); }
        print!("{}", red_transporte.nodos[nodo]);
    }
    println!();

    // ═════════════════════════════════════════
    // CICLOS
    // ═════════════════════════════════════════
    println!("\n┌─────────────────────────────────────────┐");
    println!("│  🔄 DETECCIÓN DE CICLOS                │");
    println!("└─────────────────────────────────────────┘\n");
    
    if red_transporte.tiene_ciclo() {
        println!("⚠️  El grafo contiene ciclos");
    } else {
        println!("✓ El grafo es acíclico");
    }

    println!("\n═══════════════════════════════════════════════════");
    println!("✅ Para más ejemplos, ejecuta:");
    println!("   cargo run --example red_social");
    println!("   cargo run --example dependencias\n");
}