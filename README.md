# 📦 Optimizador de Costos de Envío en Rust

Este proyecto es un programa en Rust que simula un optimizador de costos para servicios de mensajería, similar a las funcionalidades de plataformas como Rappi o Uber. El objetivo es encontrar la opción de envío más económica para un paquete con características específicas (peso y dimensiones), basándose en un conjunto de tarifas de diferentes servicios.

## 🚀 Características

* **Cálculo de Costos:** Calcula el costo total de envío para un paquete en cada servicio de mensajería.
* **Optimización:** Identifica y muestra la opción de envío más barata entre todas las disponibles.
* **Estructuras de Datos:** Modela la información de paquetes, tarifas y servicios de forma clara y organizada.
* **Pruebas:** Incluye pruebas unitarias para validar la lógica de cálculo y pruebas de integración para el algoritmo de optimización.

## ⚙️ Cómo Ejecutar el Proyecto

1.  Asegúrate de tener **Rust** y **Cargo** instalados en tu sistema.
2.  Clona el repositorio:
    ```bash
    git clone https://github.com/santiagourdaneta/optimizador-costos-envio-rust/
    cd optimizador-costos-envio-rust
    ```
3.  Ejecuta el programa:
    ```bash
    cargo run
    ```
    Cargo se encargará de compilar y ejecutar el código.

## ✅ Pruebas

El proyecto está diseñado para ser robusto y fácil de probar.

* Para ejecutar todas las pruebas unitarias y de integración:
    ```bash
    cargo test
    ```

## 📈 Pruebas de Estrés

Para evaluar el rendimiento del optimizador bajo una carga pesada, se ha incluido un bucle de estrés que simula el procesamiento de miles de paquetes.

* Para ejecutar la prueba de estrés, simplemente ejecuta el programa principal:
    ```bash
    cargo run
    ```

## 🛠️ Estructura del Código

* `main.rs`: Contiene toda la lógica del programa, incluyendo la definición de estructuras (`Paquete`, `Tarifa`, etc.), las funciones de cálculo y la función `main` que orquesta la simulación.
* `Cargo.toml`: Define el proyecto y sus dependencias, como el crate `rand` necesario para las pruebas de estrés.

## ✍️ Contribuir

¡Las contribuciones son bienvenidas! Si tienes ideas para mejorar la lógica de optimización, añadir nuevos servicios o cualquier otra mejora, no dudes en abrir un *issue* o enviar un *pull request*.

Labels & Tags
rust cargo optimizacion logistica costos simulación economia

Hashtags
#rust #rustlang #optimizacion #logistica #costos #cargo 
