use grafos_rust::Grafo;

fn main() {
    println!("\n🤝 EJEMPLO: RED SOCIAL DE AMIGOS");
    println!("════════════════════════════════════════\n");

    let mut red_social = Grafo::new();

    // Crear nodos (usuarios)
    let alice = red_social.agregar_nodo("Alice".to_string());
    let bob = red_social.agregar_nodo("Bob".to_string());
    let charlie = red_social.agregar_nodo("Charlie".to_string());
    let diana = red_social.agregar_nodo("Diana".to_string());
    let eve = red_social.agregar_nodo("Eve".to_string());

    // Crear conexiones de amistad
    println!("📱 Creando red de amigos...");
    red_social.agregar_arista(alice, bob, 1).unwrap();
    red_social.agregar_arista(alice, charlie, 1).unwrap();
    red_social.agregar_arista(bob, diana, 1).unwrap();
    red_social.agregar_arista(charlie, diana, 1).unwrap();
    red_social.agregar_arista(diana, eve, 1).unwrap();
    red_social.agregar_arista(eve, bob, 1).unwrap(); // Crea ciclo

    // Mostrar estructura
    println!("Usuarios: {}", red_social.nodos.join(", "));
    println!("Total de conexiones: {}\n", red_social.contar_aristas());

    // ═════════════════════════════════════════
    // BFS: Encontrar grado de separación
    // ═════════════════════════════════════════
    println!("┌─────────────────────────────────────────┐");
    println!("│  🔍 BFS: GRADO DE SEPARACIÓN            │");
    println!("└─────────────────────────────────────────┘\n");

    if let Some(ruta) = red_social.bfs(alice, eve) {
        println!("Camino desde Alice a Eve:");
        for (i, &idx) in ruta.iter().enumerate() {
            if i > 0 { print!(" → "); }
            print!("{}", red_social.nodos[idx]);
        }
        println!("\nGrados de separación: {}", ruta.len() - 1);
    }

    // ═════════════════════════════════════════
    // DFS: Explorar conexiones
    // ═════════════════════════════════════════
    println!("\n┌─────────────────────────────────────────┐");
    println!("│  🔍 DFS: EXPLORACIÓN DE AMIGOS          │");
    println!("└─────────────────────────────────────────┘\n");

    let exploracion = red_social.dfs(alice);
    println!("Usuarios descubiertos desde Alice:");
    for (i, &idx) in exploracion.iter().enumerate() {
        if i > 0 { print!(", "); }
        print!("{}", red_social.nodos[idx]);
    }
    println!();

    // ═════════════════════════════════════════
    // Detección de ciclos
    // ═════════════════════════════════════════
    println!("\n┌─────────────────────────────────────────┐");
    println!("│  🔄 DETECCIÓN DE CICLOS                 │");
    println!("└─────────────────────────────────────────┘\n");

    if red_social.tiene_ciclo() {
        println!("⚠️  Se detectó un ciclo en la red social");
        println!("   (Existe un grupo de amigos mutuos)");
    } else {
        println!("✓ La red es acíclica");
    }

    println!("\n═══════════════════════════════════════════════════\n");
}
