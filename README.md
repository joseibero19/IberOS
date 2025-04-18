# IberOS ✨

**Un sistema operativo de nueva generación enfocado en seguridad, rendimiento e inteligencia artificial, construido sobre un microkernel moderno en Rust.**

![IberOS Logo](docs/logo.png)

## Visión

IberOS aspira a ser una plataforma robusta, segura y adaptable para usuarios generales, profesionales e investigadores. Creemos en la necesidad de un sistema operativo construido desde cero con los principios de seguridad por diseño, modularidad y la integración inteligente de IA, aprovechando el poder y la seguridad de Rust y una arquitectura de microkernel. Queremos construir una comunidad abierta y colaborativa en torno a este proyecto.

## Estado Actual del Proyecto ⚠️

**¡Atención! IberOS está en una fase MUY TEMPRANA de desarrollo (Pre-Alfa / Prueba de Concepto).**

Actualmente estamos trabajando en la **Prueba de Concepto (PoC) inicial:**

* Un microkernel **minimalista** escrito en **Rust**.
* Capaz de **arrancar** en el emulador **QEMU** (target x86_64).
* Inicialización básica de hardware (consola VGA para imprimir mensajes).
* Demostración simple de Comunicación Entre Procesos (IPC) entre procesos simulados.
* Logo y mensaje de bienvenida personalizados.

**ESTE PROYECTO NO ES USABLE PARA TAREAS DIARIAS Y ES ALTAMENTE EXPERIMENTAL.**

## Objetivos del PoC

* Validar la arquitectura básica del microkernel.
* Demostrar la capacidad de arrancar código Rust en bare-metal (virtualizado).
* Establecer el entorno de compilación y ejecución (`no_std`, QEMU).
* Implementar una demostración básica de IPC.
* Servir como base para atraer colaboradores e interés futuro.

## Pila Tecnológica (Actual y Planeada)

* **Lenguaje Principal (Kernel & Servicios Críticos):** Rust
* **Arquitectura del Núcleo:** Microkernel
* **Interfaz de Usuario (Futuro):** C++ con Qt (Planeado, sujeto a cambios)
* **Otros Lenguajes (Servicios, IA, Apps):** C++, Python, Go (Potencialmente)
* **Plataforma Inicial de Desarrollo/Pruebas:** QEMU (x86_64)

## Características Implementadas

* Arranque básico en QEMU
* Salida de texto en pantalla VGA
* Logo personalizado de IberOS
* Sistema básico de manejo de interrupciones
* Simulación de IPC (Comunicación Entre Procesos)
* Implementación básica de memoria

## Cómo Compilar y Ejecutar

### Requisitos Previos

* Rust (nightly) - `rustup default nightly`
* QEMU - Para Windows, descarga desde [qemu.org](https://www.qemu.org/download/) o instala con `choco install qemu`
* cargo-binutils - `cargo install cargo-binutils`
* bootimage - `cargo install bootimage`
* llvm-tools-preview - `rustup component add llvm-tools-preview`

### Pasos para Compilar y Ejecutar

```bash
# 1. Clonar el repositorio
git clone https://github.com/joseibero19/IberOS.git
cd IberOS

# 2. Compilar y ejecutar (usando el script incluido)
.\build.ps1  # En Windows
# o
./build.sh   # En Linux/macOS (cuando esté disponible)
```

Alternativamente, puedes ejecutar los comandos manualmente:

```bash
# Compilar el kernel
cargo build

# Crear la imagen de arranque
cargo bootimage

# Ejecutar en QEMU
qemu-system-x86_64 -drive format=raw,file=target/x86_64-iberos/debug/bootimage-iberos.bin
```

## Estructura del Proyecto

```
IberOS/
├── src/                    # Código fuente del kernel
│   ├── main.rs             # Punto de entrada del kernel
│   ├── vga_buffer.rs       # Controlador de pantalla VGA
│   ├── ipc.rs              # Implementación de IPC
│   ├── allocator.rs        # Asignador de memoria
│   └── string.rs           # Implementación básica de cadenas
├── Cargo.toml              # Configuración del proyecto Rust
├── Cargo.lock              # Versiones bloqueadas de dependencias
├── build.ps1               # Script de compilación para Windows
├── .cargo/                 # Configuración específica de Cargo
│   └── config.toml         # Configuración para compilación cruzada
└── README.md               # Este archivo
```

## Roadmap

### Fase 1 (Actual - PoC)
- [x] Kernel básico que arranca en QEMU
- [x] Salida de texto básica
- [x] Simulación de IPC

### Fase 2 (Próxima)
- [ ] Implementar un sistema de memoria más robusto
- [ ] Añadir soporte para interrupciones de hardware
- [ ] Implementar un planificador (scheduler) básico

### Fase 3 (Futuro)
- [ ] Añadir soporte para drivers básicos
- [ ] Implementar un sistema de archivos simple
- [ ] Mejorar la comunicación entre procesos

### Fase 4 (Largo Plazo)
- [ ] Añadir soporte para múltiples arquitecturas
- [ ] Implementar capacidades de red básicas
- [ ] Crear una interfaz de usuario simple

## Contribuir

¡Las contribuciones son bienvenidas! Si estás interesado en contribuir a IberOS, por favor:

1. Revisa los issues abiertos o crea uno nuevo para discutir lo que te gustaría cambiar.
2. Haz un fork del repositorio.
3. Crea una rama para tu característica (`git checkout -b feature/amazing-feature`).
4. Haz tus cambios y commitea (`git commit -m 'Add some amazing feature'`).
5. Empuja a la rama (`git push origin feature/amazing-feature`).
6. Abre un Pull Request.

## Licencia

Este proyecto está licenciado bajo los términos de la licencia Apache 2.0.
Consulta el archivo [LICENSE](LICENSE) para más detalles.

## Contacto

Jose Ibero - [ibersoft96@gmail.com](mailto:ibersoft96@gmail.com)

Enlace del proyecto: [https://github.com/joseibero19/IberOS](https://github.com/joseibero19/IberOS)

## Agradecimientos

* [Philipp Oppermann](https://os.phil-opp.com/) por su excelente serie "Writing an OS in Rust"
* La comunidad de Rust por sus herramientas y bibliotecas
* Todos los colaboradores y personas que han mostrado interés en este proyecto
