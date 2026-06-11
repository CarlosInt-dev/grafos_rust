use grafos_rust::Grafo;

fn main() {
    println!("\n📦 EJEMPLO: ÁRBOL DE DEPENDENCIAS DE SOFTWARE");
    println!("════════════════════════════════════════════════\n");

    let mut deps = Grafo::new();

    // Crear módulos (nodos)
    let main = deps.agregar_nodo("main".to_string());
    let auth = deps.agregar_nodo("autenticacion".to_string());
    let db = deps.agregar_nodo("base_datos".to_string());
    let api = deps.agregar_nodo("api_rest".to_string());
    let security = deps.agregar_nodo("seguridad".to_string());
    let crypto = deps.agregar_nodo("criptografia".to_string());

    // Agregar dependencias
    println!("🔗 Mapeando dependencias de módulos...");
    deps.agregar_arista(main, auth, 1).unwrap();
    deps.agregar_arista(main, api, 1).unwrap();
    deps.agregar_arista(auth, db, 1).unwrap();
    deps.agregar_arista(auth, security, 1).unwrap();
    deps.agregar_arista(api, db, 1).unwrap();
    deps.agregar_arista(security, crypto, 1).unwrap();

    // Mostrar estructura
    println!("Módulos: {}", deps.nodos.join(", "));
    println!("Total de dependencias: {}\n", deps.contar_aristas());

    // ═════════════════════════════════════════
    // BFS: Encontrar todas las dependencias
    // ═════════════════════════════════════════
    println!("┌─────────────────────────────────────────┐");
    println!("│  🔍 BFS: CADENA DE DEPENDENCIAS        │");
    println!("└─────────────────────────────────────────┘\n");

    if let Some(ruta) = deps.bfs(main, crypto) {
        println!("Camino de dependencias (main → criptografia):");
        for (i, &idx) in ruta.iter().enumerate() {
            if i > 0 { print!(" → "); }
            print!("{}", deps.nodos[idx]);
        }
        println!("\nNivel de profundidad: {}", ruta.len() - 1);
    }

    // ═════════════════════════════════════════
    // DFS: Análisis completo de dependencias
    // ═════════════════════════════════════════
    println!("\n┌─────────────────────────────────────────┐");
    println!("│  🔍 DFS: ANÁLISIS COMPLETO             │");
    println!("└─────────────────────────────────────────┘\n");

    let deps_main = deps.dfs(main);
    println!("Módulos dependientes de main:");
    for (i, &idx) in deps_main.iter().enumerate() {
        if i > 0 { print!(", "); }
        print!("{}", deps.nodos[idx]);
    }
    println!();

    // ═════════════════════════════════════════
    // Verificar ciclos (importante en deps)
    // ═════════════════════════════════════════
    println!("\n┌─────────────────────────────────────────┐");
    println!("│  ✓ VALIDACIÓN: CICLOS CIRCULARES       │");
    println!("└─────────────────────────────────────────┘\n");

    if deps.tiene_ciclo() {
        println!("❌ ¡PROBLEMA! Hay una dependencia circular");
        println!("   Esto causaría un error de compilación");
    } else {
        println!("✓ Estructura válida: sin dependencias circulares");
    }

    // ═════════════════════════════════════════
    // Información de cada módulo
    // ═════════════════════════════════════════
    println!("\n┌─────────────────────────────────────────┐");
    println!("│  📊 DETALLES DE MÓDULOS                │");
    println!("└─────────────────────────────────────────┘\n");

    for i in 0..deps.nodos.len() {
        if let Some(vecinos) = deps.obtener_vecinos(i) {
            println!("{} depende de:", deps.nodos[i]);
            if vecinos.is_empty() {
                println!("  (sin dependencias)");
            } else {
                for (vecino, _) in vecinos {
                    println!("  → {}", deps.nodos[vecino]);
                }
            }
        }
    }

    println!("\n═══════════════════════════════════════════════════\n");
}
