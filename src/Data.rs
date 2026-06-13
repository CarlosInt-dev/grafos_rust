
use grafos_rust::Grafo;

/// Red de transporte de El Salvador
/// Integrante 2 - Julio Calderon (CU24003)
pub fn create_transport_network() -> Grafo {
    let mut red = Grafo::new();

    // === CIUDADES ===
    let san_salvador  = red.agregar_nodo("San Salvador".to_string());
    let santa_ana     = red.agregar_nodo("Santa Ana".to_string());
    let sonsonate     = red.agregar_nodo("Sonsonate".to_string());
    let ahuachapan    = red.agregar_nodo("Ahuachapan".to_string());
    let la_libertad   = red.agregar_nodo("La Libertad".to_string());
    let san_miguel    = red.agregar_nodo("San Miguel".to_string());
    let usulutan      = red.agregar_nodo("Usulutan".to_string());
    let san_vicente   = red.agregar_nodo("San Vicente".to_string());
    let cojutepeque   = red.agregar_nodo("Cojutepeque".to_string());
    let zacatecoluca  = red.agregar_nodo("Zacatecoluca".to_string());

    // === RUTAS (bidireccionales, peso = km aproximados) ===
    red.agregar_arista(san_salvador, santa_ana,    64).unwrap();
    red.agregar_arista(santa_ana,    san_salvador, 64).unwrap();

    red.agregar_arista(san_salvador, sonsonate,    74).unwrap();
    red.agregar_arista(sonsonate,    san_salvador, 74).unwrap();

    red.agregar_arista(san_salvador, la_libertad,  34).unwrap();
    red.agregar_arista(la_libertad,  san_salvador, 34).unwrap();

    red.agregar_arista(san_salvador, cojutepeque,  33).unwrap();
    red.agregar_arista(cojutepeque,  san_salvador, 33).unwrap();

    red.agregar_arista(san_salvador, san_vicente,  59).unwrap();
    red.agregar_arista(san_vicente,  san_salvador, 59).unwrap();

    red.agregar_arista(santa_ana,    ahuachapan,   40).unwrap();
    red.agregar_arista(ahuachapan,   santa_ana,    40).unwrap();

    red.agregar_arista(santa_ana,    sonsonate,    40).unwrap();
    red.agregar_arista(sonsonate,    santa_ana,    40).unwrap();

    red.agregar_arista(sonsonate,    ahuachapan,   55).unwrap();
    red.agregar_arista(ahuachapan,   sonsonate,    55).unwrap();

    red.agregar_arista(san_vicente,  zacatecoluca, 30).unwrap();
    red.agregar_arista(zacatecoluca, san_vicente,  30).unwrap();

    red.agregar_arista(san_vicente,  usulutan,     60).unwrap();
    red.agregar_arista(usulutan,     san_vicente,  60).unwrap();

    red.agregar_arista(zacatecoluca, usulutan,     45).unwrap();
    red.agregar_arista(usulutan,     zacatecoluca, 45).unwrap();

    red.agregar_arista(usulutan,     san_miguel,   74).unwrap();
    red.agregar_arista(san_miguel,   usulutan,     74).unwrap();

    red.agregar_arista(cojutepeque,  san_vicente,  35).unwrap();
    red.agregar_arista(san_vicente,  cojutepeque,  35).unwrap();

    red
}