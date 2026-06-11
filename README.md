# Grafos en Rust 🦀

Implementación educativa de grafos en Rust, demostrando por qué el compilador de Rust cuida tan de cerca las estructuras de datos.

## 📚 ¿Por qué Rust cuida los Grafos?

En Rust, los grafos **tradicionales con punteros** son problemáticos porque:

- **Ownership Conflicts**: Cada nodo es dueño de sus vecinos, pero los vecinos también quieren ser dueños
- **Ciclos de Referencias**: Imposibles con `Box<T>` puro
- **Borrow Checker**: Rust previene use-after-free y data races

**Nuestra Solución**: Lista de adyacencia basada en **índices de vectores**. Esto permite:
- ✅ Ownership claro (el vector posee todos los nodos)
- ✅ Seguridad de memoria sin GC
- ✅ Flexibilidad para ciclos
- ✅ Rendimiento O(1) para acceso a nodos

## 🎯 Entregables

### ✅ Código Funcional
- **Estructura Grafo**: Lista de adyacencia con índices
- **BFS**: Búsqueda en anchura (ruta con menos conexiones)
- **DFS**: Búsqueda en profundidad (recorrido y detección de ciclos)
- **Tests Unitarios**: 4 tests validados

### 📦 Ejemplos Prácticos

#### 1. **Red de Transporte** (main.rs)
```bash
cargo run
```
- Red de ciudades españolas
- BFS: encuentra ruta directa más corta
- DFS: explora todas las ciudades reachables
- Detección de ciclos

#### 2. **Red Social** (examples/red_social.rs)
```bash
cargo run --example red_social
```
- Usuarios conectados por amistades
- BFS: calcula grado de separación (6 grados de separación)
- DFS: explora círculo de amigos
- Ciclos: detecta amigos mutuos

#### 3. **Árbol de Dependencias** (examples/dependencias.rs)
```bash
cargo run --example dependencias
```
- Módulos de software con dependencias
- BFS: cadena completa de dependencias
- DFS: análisis completo de dependencias
- **Validación crítica**: detecta dependencias circulares

## 🏗️ Estructura del Proyecto

```
grafos_rust/
├── src/
│   ├── main.rs           # Ejemplo: red de transporte
│   └── lib.rs            # Biblioteca Grafo + tests
├── examples/
│   ├── red_social.rs     # Ejemplo: red de amigos
│   └── dependencias.rs   # Ejemplo: árbol de dependencias
└── Cargo.toml
```

## 🚀 Compilación y Ejecución

### Compilar el proyecto
```bash
cargo build
```

### Ejecutar el programa principal
```bash
cargo run
```

### Ejecutar ejemplos
```bash
cargo run --example red_social
cargo run --example dependencias
```

### Ejecutar tests
```bash
cargo test
```

### Compilación optimizada
```bash
cargo build --release
cargo run --release
```

## 📋 API del Grafo

```rust
impl Grafo {
    // Creación
    fn new() -> Self
    fn agregar_nodo(&mut self, nombre: String) -> usize
    fn agregar_arista(&mut self, origen: usize, destino: usize, peso: u32) -> Result<(), String>
    
    // Búsqueda
    fn bfs(&self, origen: usize, destino: usize) -> Option<Vec<usize>>
    fn dfs(&self, inicio: usize) -> Vec<usize>
    
    // Análisis
    fn tiene_ciclo(&self) -> bool
    fn obtener_vecinos(&self, nodo: usize) -> Option<Vec<(usize, u32)>>
    fn contar_aristas(&self) -> usize
}
```

## 💡 Conceptos Clave

### BFS (Búsqueda en Anchura)
- **Algoritmo**: Cola FIFO
- **Uso**: Encontrar camino con menos conexiones
- **Complejidad**: O(V + E)
- **Aplicación**: Redes de transporte, redes sociales

### DFS (Búsqueda en Profundidad)
- **Algoritmo**: Recursión o Stack LIFO
- **Uso**: Detectar ciclos, recorrido completo
- **Complejidad**: O(V + E)
- **Aplicación**: Detección de ciclos, ordenamiento topológico

### Detección de Ciclos
- Usa DFS con pila de recursión
- Si encontramos un nodo en la pila de recursión actual → ciclo
- Crítico para validar dependencias

## 🧪 Tests

```bash
cargo test

# Salida esperada:
# running 4 tests
# test tests::test_agregar_nodos ... ok
# test tests::test_bfs_encuentra_camino ... ok
# test tests::test_ciclo_detectado ... ok
# test tests::test_sin_ciclo ... ok
```

## 🔍 Por qué esta aproximación es "Rusty"

1. **Sin GC**: Memory management explícito pero seguro
2. **Sin Punteros**: Indices son más seguros que `*mut T`
3. **Sin Data Races**: El borrow checker garantiza seguridad
4. **Sin Unsafe**: Código 100% safe Rust
5. **Eficiente**: Cache-friendly gracias a vectores contiguos

## 📊 Comparación de Enfoques

| Enfoque | Ventajas | Desventajas |
|---------|----------|-------------|
| **Punteros Crudos** | Tradicional | Unsafe, data races, memory leaks |
| **Box + Rc** | Seguro | Complejidad, overhead, no funciona bien con ciclos |
| **Índices** | ✅ Seguro, eficiente, sencillo | Menos "object-oriented" |

## 👥 Trabajo Colaborativo

Este proyecto demuestra cómo trabajar en equipo con Rust:
- ✅ `lib.rs`: Código reutilizable
- ✅ `main.rs`: Ejemplo principal
- ✅ `examples/`: Ejemplos modulares
- ✅ Tests integrados
- ✅ Git history limpio

Cada integrante puede:
1. Crear nuevos ejemplos en `examples/`
2. Extender la API en `lib.rs`
3. Escribir tests para nuevas funcionalidades
4. Mantener código limpio y documentado

## 📈 Extensiones Posibles

- Algoritmo de Dijkstra (camino más corto con pesos)
- Algoritmo de Floyd-Warshall (todos los caminos más cortos)
- Ordenamiento Topológico
- Componentes Fuertemente Conectadas (SCC)
- Spanning Tree Mínimo (Kruskal/Prim)

## 📝 Notas de Implementación

- **Índices vs Punteros**: La indirección de vectores es más segura
- **Generic Traits**: `Grafo` podría ser genérico sobre el tipo de nodo
- **Pesados vs No Pesados**: Soportamos grafos pesados (u32)
- **Dirigidos**: Los ejemplos usan grafos dirigidos (puede adaptarse)

---

**Entrega**: Lunes, 15 de junio de 2026  
**Presentación Presencial**: Mismo día  
**Tecnología**: Rust 🦀 | Git 📦 | CLI 🖥️
