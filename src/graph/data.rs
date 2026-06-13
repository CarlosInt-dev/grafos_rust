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
    red.agregar_ciudad("Cabañas");
    red.agregar_ciudad("Chalatenango");
    red.agregar_ciudad("La Paz");
    red.agregar_ciudad("Morazan");
    red.agregar_ciudad("La Union");
    red.agregar_ciudad("Cuscatlan");


    // === RUTAS (bidireccionales) ===
    // Zona occidental
    red.agregar_ruta("Ahuachapan", "Santa Ana");
    red.agregar_ruta("Ahuachapan", "Sonsonate");

    red.agregar_ruta("Santa Ana", "Sonsonate");
    red.agregar_ruta("Santa Ana", "Chalatenango");

    // Zona central
    red.agregar_ruta("Sonsonate", "La Libertad");

    red.agregar_ruta("La Libertad", "San Salvador");

    red.agregar_ruta("San Salvador", "Chalatenango");
    red.agregar_ruta("San Salvador", "Cuscatlan");
    red.agregar_ruta("San Salvador", "La Paz");
    red.agregar_ruta("San Salvador", "La Libertad");

    red.agregar_ruta("Chalatenango", "Cuscatlan");
    red.agregar_ruta("Chalatenango", "Cabañas");

    red.agregar_ruta("Cuscatlan", "Cabañas");
    red.agregar_ruta("Cuscatlan", "San Vicente");
    red.agregar_ruta("Cuscatlan", "La Paz");

    red.agregar_ruta("La Paz", "San Vicente");

    // Zona paracentral
    red.agregar_ruta("San Vicente", "Cabañas");
    red.agregar_ruta("San Vicente", "Usulutan");

    // Zona oriental
    red.agregar_ruta("Cabañas", "San Miguel");
    red.agregar_ruta("Cabañas", "Morazan");

    red.agregar_ruta("Usulutan", "San Miguel");

    red.agregar_ruta("San Miguel", "Morazan");
    red.agregar_ruta("San Miguel", "La Union");

    red.agregar_ruta("Morazan", "La Union");

    return red;
}