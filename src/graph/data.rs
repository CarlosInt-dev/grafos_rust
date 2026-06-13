use crate::graph::graph::Graph;

pub fn create_transport_network() -> Graph {
    let mut red = Graph::new();

    // === CIUDADES ===
    red.agregar_ciudad("San Salvador");
    red.agregar_ciudad("Santa Ana");
    red.agregar_ciudad("Sonsonate");
    red.agregar_ciudad("Ahuachapan");
    red.agregar_ciudad("La Libertad");
    red.agregar_ciudad("San Miguel");
    red.agregar_ciudad("Usulutan");
    red.agregar_ciudad("San Vicente");
    red.agregar_ciudad("Cojutepeque");
    red.agregar_ciudad("Zacatecoluca");

    // === RUTAS (bidireccionales) ===
    red.agregar_ruta("San Salvador", "Santa Ana");
    red.agregar_ruta("San Salvador", "Sonsonate");
    red.agregar_ruta("San Salvador", "La Libertad");
    red.agregar_ruta("San Salvador", "Cojutepeque");
    red.agregar_ruta("San Salvador", "San Vicente");

    red.agregar_ruta("Santa Ana", "Ahuachapan");
    red.agregar_ruta("Santa Ana", "Sonsonate");

    red.agregar_ruta("Sonsonate", "Ahuachapan");

    red.agregar_ruta("San Vicente", "Zacatecoluca");
    red.agregar_ruta("San Vicente", "Usulutan");

    red.agregar_ruta("Zacatecoluca", "Usulutan");

    red.agregar_ruta("Usulutan", "San Miguel");

    red.agregar_ruta("Cojutepeque", "San Vicente");

    return red;
}