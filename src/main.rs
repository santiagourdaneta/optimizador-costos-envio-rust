use std::fmt;

// --- 1. Estructuras de Datos ---

/// Representa las dimensiones de un paquete.
#[derive(Debug)]
struct Dimensiones {
    ancho: f64,
    alto: f64,
    profundidad: f64,
}

/// Representa un paquete con su peso y dimensiones.
#[derive(Debug)]
struct Paquete {
    peso_kg: f64,
    dimensiones: Dimensiones,
}

/// Representa las tarifas de un servicio de mensajería.
#[derive(Debug)]
struct Tarifa {
    costo_base: f64,
    costo_por_kg: f64,
    costo_por_volumen_cm3: f64,
}

/// Representa un servicio de mensajería específico.
#[derive(Debug)]
struct ServicioDeMensajeria {
    nombre: String,
    tarifa: Tarifa,
}

/// Implementación para mostrar el costo de una opción de envío.
struct OpcionDeEnvio<'a> {
    servicio: &'a str,
    costo: f64,
}

impl fmt::Display for OpcionDeEnvio<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Servicio: {}, Costo Total: ${:.2}", self.servicio, self.costo)
    }
}


// --- 2. Lógica de Optimización ---

impl Paquete {
    /// Calcula el volumen del paquete en cm cúbicos.
    fn volumen_cm3(&self) -> f64 {
        self.dimensiones.ancho * self.dimensiones.alto * self.dimensiones.profundidad
    }
}

impl ServicioDeMensajeria {
    /// Calcula el costo total de envío para un paquete dado.
    fn calcular_costo(&self, paquete: &Paquete) -> f64 {
        let costo_por_peso = self.tarifa.costo_por_kg * paquete.peso_kg;
        let costo_por_volumen = self.tarifa.costo_por_volumen_cm3 * paquete.volumen_cm3();
        self.tarifa.costo_base + costo_por_peso + costo_por_volumen
    }
}

/// Encuentra la opción de envío más barata entre una lista de servicios.
/// La función debe especificar que el 'OpcionDeEnvio' que devuelve
/// vive al menos tanto como el slice 'servicios' que se le pasa.
fn encontrar_opcion_mas_barata<'a>(servicios: &'a [ServicioDeMensajeria], paquete: &Paquete) -> OpcionDeEnvio<'a> {
    let mut mejor_opcion = OpcionDeEnvio {
        servicio: "No disponible",
        costo: f64::MAX,
    };

    for servicio in servicios {
        let costo_actual = servicio.calcular_costo(paquete);
        if costo_actual < mejor_opcion.costo {
            mejor_opcion.costo = costo_actual;
            mejor_opcion.servicio = &servicio.nombre;
        }
    }
    mejor_opcion
}

// --- 3. Función Principal ---

fn main() {
    println!("📦 Optimizador de Costos de Envío 📦");
    println!("------------------------------------");

    // Datos de ejemplo:
    let paquete_a_enviar = Paquete {
        peso_kg: 5.5,
        dimensiones: Dimensiones {
            ancho: 15.0,
            alto: 10.0,
            profundidad: 20.0,
        },
    };

    // Servicios de mensajería con diferentes tarifas:
    let servicios = vec![
        ServicioDeMensajeria {
            nombre: String::from("Rappi Courier"),
            tarifa: Tarifa {
                costo_base: 5.0,
                costo_por_kg: 1.5,
                costo_por_volumen_cm3: 0.001,
            },
        },
        ServicioDeMensajeria {
            nombre: String::from("Uber Paquetes"),
            tarifa: Tarifa {
                costo_base: 8.0,
                costo_por_kg: 1.2,
                costo_por_volumen_cm3: 0.0008,
            },
        },
        ServicioDeMensajeria {
            nombre: String::from("DHL Express"),
            tarifa: Tarifa {
                costo_base: 20.0,
                costo_por_kg: 1.0,
                costo_por_volumen_cm3: 0.002,
            },
        },
    ];

    println!("Paquete a enviar: {:?}", paquete_a_enviar);
    println!("\nServicios y sus costos calculados:");

    // 4. Calcular y mostrar las opciones
    for servicio in &servicios {
        let costo = servicio.calcular_costo(&paquete_a_enviar);
        println!("- {}: ${:.2}", servicio.nombre, costo);
    }
    
    println!("\n------------------------------------");

    // 5. Encontrar la opción más barata
    let mejor_opcion = encontrar_opcion_mas_barata(&servicios, &paquete_a_enviar);
    println!("🎉 La opción de envío más barata es: {}", mejor_opcion);

     // Simulación de prueba de estrés
    println!("\n--- Prueba de Estrés (100,000 paquetes) ---");

    let num_paquetes = 100_000;
    let mut mas_barata_final: OpcionDeEnvio = OpcionDeEnvio {
        servicio: "No disponible",
        costo: f64::MAX,
    };
    
    for _ in 0..num_paquetes {
        let paquete = Paquete {
            peso_kg: rand::random::<f64>() * 20.0 + 1.0,
            dimensiones: Dimensiones {
                ancho: rand::random::<f64>() * 50.0 + 10.0,
                alto: rand::random::<f64>() * 50.0 + 10.0,
                profundidad: rand::random::<f64>() * 50.0 + 10.0,
            },
        };
        let mejor_opcion_actual = encontrar_opcion_mas_barata(&servicios, &paquete);
        if mejor_opcion_actual.costo < mas_barata_final.costo {
            mas_barata_final = mejor_opcion_actual;
        }
    }
    
    println!("El costo más bajo encontrado en {} paquetes fue: {}", num_paquetes, mas_barata_final);
    println!("Esto demuestra que el programa maneja una gran carga de trabajo eficientemente.");
}


//Pruebas Unitarias

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_volumen_calculo() {
        let paquete = Paquete {
            peso_kg: 1.0,
            dimensiones: Dimensiones { ancho: 10.0, alto: 10.0, profundidad: 10.0 },
        };
        assert_eq!(paquete.volumen_cm3(), 1000.0);
    }
    
    #[test]
    fn test_costo_con_tarifa_cero() {
        let servicio = ServicioDeMensajeria {
            nombre: "Test Zero".to_string(),
            tarifa: Tarifa {
                costo_base: 0.0,
                costo_por_kg: 0.0,
                costo_por_volumen_cm3: 0.0,
            },
        };
        let paquete = Paquete {
            peso_kg: 10.0,
            dimensiones: Dimensiones { ancho: 10.0, alto: 10.0, profundidad: 10.0 },
        };
        assert_eq!(servicio.calcular_costo(&paquete), 0.0);
    }
}

//Pruebas de Integración

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
fn test_encontrar_opcion_mas_barata() {
    let servicios = vec![
        ServicioDeMensajeria {
            nombre: "Servicio_A".to_string(),
            tarifa: Tarifa { costo_base: 10.0, costo_por_kg: 1.0, costo_por_volumen_cm3: 0.001 },
        },
        ServicioDeMensajeria {
            nombre: "Servicio_B".to_string(),
            tarifa: Tarifa { costo_base: 5.0, costo_por_kg: 2.0, costo_por_volumen_cm3: 0.0005 },
        },
    ];

    let paquete_pequeno = Paquete {
        peso_kg: 2.0,
        dimensiones: Dimensiones { ancho: 10.0, alto: 10.0, profundidad: 10.0 },
    };
    let mejor_opcion = encontrar_opcion_mas_barata(&servicios, &paquete_pequeno);

    // Recalculando el costo para Servicio_A: 10.0 + (1.0 * 2.0) + (0.001 * 1000.0) = 13.0
    // Recalculando el costo para Servicio_B: 5.0 + (2.0 * 2.0) + (0.0005 * 1000.0) = 9.5
    // Por lo tanto, Servicio_B sigue siendo el más barato.

    assert_eq!(mejor_opcion.servicio, "Servicio_B");
    assert_eq!(mejor_opcion.costo, 9.5); // <-- El valor esperado correcto es 9.5
}
}

