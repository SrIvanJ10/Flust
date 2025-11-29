# 🎯 Flust - Flow-Based Visual Programming for Rust

**Flust** es un entorno de programación visual basado en flujos para Rust que **genera código fuente independiente y compilable**. Es una **herramienta de desarrollo** que ayuda a los programadores a organizar y visualizar su código Rust mediante un sistema de nodos y conexiones.

## 🔑 Conceptos Clave

### ¿Qué hace Flust?

Flust permite **diseñar programas Rust visualmente** mediante nodos conectados, y luego **genera código Rust estándar** que puedes compilar y ejecutar de forma completamente independiente.

### ¿Qué NO es Flust?

- ❌ **NO es un IDE no-code**: Flust está diseñado **para programadores**. Escribirás código Rust real en los nodos.
- ❌ **NO es un runtime o intérprete**: El código generado **no depende de Flust** para ejecutarse. Es Rust puro y estándar.
- ❌ **NO ejecuta tus flujos**: Flust solo genera código. Tú lo compilas con las herramientas estándar de Rust.

### Analogía

Piensa en Flust como un **IDE visual especializado** que:
- Te ayuda a **organizar** tu código en bloques reutilizables
- Te permite **visualizar** el flujo de ejecución
- **Genera** código Rust estándar que puedes compilar con `rustc` o `cargo`

Es como usar un diagrama de flujo, pero que **genera código real** en lugar de ser solo documentación.

## ✨ Estado del Proyecto

### Funcionalidades Implementadas

#### 🎨 Sistema de Plugins
- ✅ **Arquitectura modular basada en plugins JSON**
- ✅ Plugins cargados dinámicamente desde `frontend/public/plugins/`
- ✅ Fácil extensión sin modificar el core

#### 🧩 Bloques Disponibles

1. **Function Definition** (Definición de Funciones)
   - Contenedor para definir funciones Rust
   - Propiedades: nombre, argumentos, tipo de retorno
   - Soporta funciones async

2. **Start Node** (Nodo de Inicio)
   - Marca el punto de entrada de una función
   - Se coloca dentro de Function Definition

3. **Legacy Code** (Código Libre)
   - Escribe cualquier código Rust directamente
   - Ideal para lógica personalizada

4. **Call Function** (Llamada a Función)
   - Llama a funciones definidas
   - Control completo sobre variables:
     - Crear nueva variable o asignar a existente
     - Especificar mutabilidad (`mut`)
     - Tipo explícito o inferencia
   - Mapeo de argumentos mediante conexiones

5. **Debug** (Depuración)
   - Imprime variables con `println!`
   - Etiquetas opcionales

#### 🔗 Sistema de Conexiones
- ✅ Conexiones visuales entre nodos
- ✅ **Variable mapping** para llamadas a funciones
- ✅ Ordenamiento topológico automático
- ✅ Detección de ciclos

#### 🏗️ Generación de Código
- ✅ Genera código Rust válido y compilable
- ✅ Soporte para funciones async con Tokio
- ✅ Tipos de retorno configurables
- ✅ Variables mutables e inmutables
- ✅ Inferencia de tipos o anotaciones explícitas

#### 💾 Gestión de Flujos
- ✅ Guardar flujos como archivos `.flow.json`
- ✅ Cargar flujos guardados
- ✅ Descargar código Rust generado
- ✅ Compilar y ejecutar directamente desde la UI

#### 🎯 Interfaz de Usuario
- ✅ Editor visual drag-and-drop
- ✅ Panel de propiedades dinámico
- ✅ Nodos contenedores (funciones)
- ✅ Selección de edges (conexiones)
- ✅ Panel de logs y terminal
- ✅ Toolbar con controles

## 🚀 Inicio Rápido

### Requisitos

- **Rust** (1.70+)
- **Node.js** (18+)
- **npm** o **yarn**

### 1. Clonar el Repositorio

```bash
git clone <repository-url>
cd flust
```

### 2. Iniciar el Backend

```bash
cargo run --bin flust-server
```

El servidor estará disponible en `http://localhost:3000`

### 3. Iniciar el Frontend

```bash
cd frontend
npm install
npm run dev
```

La interfaz estará disponible en `http://localhost:5173`

### 4. Crear tu Primer Programa

1. Abre `http://localhost:5173` en tu navegador
2. Verás un flujo de ejemplo con una función `main`
3. Arrastra bloques desde el sidebar (izquierda)
4. Conecta bloques arrastrando desde los handles
5. Edita propiedades en el panel derecho
6. Haz clic en **▶️ Compilar y Ejecutar**
7. Ve el resultado en el panel de terminal

## 📖 Ejemplo: Función de Potencia

### Diseño Visual

```
┌─────────────────────────────────────┐
│ Pow Function (return_type: i32)     │
│  ┌─────────────────────────────┐    │
│  │ Start                       │    │
│  └──────────┬──────────────────┘    │
│             ↓                       │
│  ┌─────────────────────────────┐    │
│  │ Legacy Code:                │    │
│  │ let mut potencia: i32 = 1;  │    │
│  │ let mut i = 1;              │    │
|  |                             |    |
│  │ while i <= exp {            │    │
│  │     potencia *= num as i32; │    │
│  │     i += 1;                 │    │
│  │ }                           │    │
│  |                             |    |
│  │ return potencia;            │    │
│  └─────────────────────────────┘    │
└─────────────────────────────────────┘

┌─────────────────────────────────────┐
│ Main Function                       │
│  ┌─────────────────────────────┐    │
│  │ Start                       │    │
│  └──────────┬──────────────────┘    │
│             ↓                       │
│  ┌─────────────────────────────┐    │
│  │ Legacy Code:                │    │
│  │ let a: i8 = 2;              │    │
│  │ let b: i8 = 3;              │    │
│  └──────────┬──────────────────┘    │
│             ↓                       │
│  ┌─────────────────────────────┐    │
│  │ Call Function: pow          │    │
│  │ return_variable: potencia   │    │
│  │ return_type: i32            |    |
|  | Declare New Variable: yes   |    |
|  | Mutable: yes                │    │
│  │ mapping: {num: 2, exp: 3}   │    │
│  └──────────┬──────────────────┘    │
│             ↓                       │
│  ┌─────────────────────────────┐    │
│  │ Debug: potencia             │    │
│  └─────────────────────────────┘    │
└─────────────────────────────────────┘
```

### Código Generado

```rust
async fn pow(num: i8, exp: i8) -> i32 {
    let mut potencia: i32 = 1;
    let mut i = 1;
    while i <= exp {
        potencia *= num as i32;
        i += 1;
    }
    return potencia;

}

#[tokio::main]
async fn main() {
    let a: i8 = 2;
    let b: i8 = 3;
    let mut potencia: i32 = pow(a, b).await;
    println!("{:?}", potencia);
}
```

### Compilar y Ejecutar

El código generado es **Rust estándar**. Puedes compilarlo sin Flust:

```bash
# Opción 1: Con rustc (requiere tokio instalado)
rustc generated.rs && ./generated
```

**Salida:**
```
8
```

## 🏗️ Arquitectura

### Backend (Rust)

```
flust/
├── flust-core/             # Núcleo del sistema
│   ├── ir.rs               # Intermediate Representation (IR)
│   └── topological_sort.rs # Ordenamiento de grafos
├── flust-codegen/          # Generador de código
│   ├── generator.rs        # Lógica de generación
│   └── template_engine.rs  # Motor de plantillas Handlebars
└── flust-server/           # API REST
    └── main.rs             # Servidor Axum
```

### Frontend (React + TypeScript)

```
frontend/
├── src/
│   ├── components/            # Componentes React
│   │   ├── CustomNode.tsx       # Nodo visual
│   │   ├── PropertiesPanel.tsx
│   │   ├── Toolbar.tsx
│   │   └── LogsPanel.tsx
│   ├── hooks/                 # Hooks React
│   │   └── usePlugins.ts        # Carga de plugins
│   ├── types/                 # Tipos TypeScript
│   │   └── plugin.ts            # Tipos TypeScript
│   └── App.tsx                # Aplicación principal
└── public/plugins/            # Plugins (JSON + templates)
    ├── function-definition/
    ├── start-node/
    ├── legacy-code/
    ├── call-function/
    └── debug/
```

## 🔌 Sistema de Plugins

### Crear un Nuevo Plugin

En desarrollo...

## 🎯 Filosofía de Diseño

### Para Programadores, No Para Evitar Programar

Flust **no intenta reemplazar el código**. En su lugar:

- ✅ **Organiza** tu código en componentes visuales
- ✅ **Visualiza** el flujo de ejecución
- ✅ **Facilita** la reutilización de funciones
- ✅ **Genera** código Rust idiomático

- ✅ **Facilita** la Revisión y depuración del código

**Todavía escribes código Rust** en los nodos Legacy Code. Flust solo te ayuda a estructurarlo.

### Independencia del Código Generado

El código que genera Flust es **Rust estándar**:

- ✅ No tiene dependencias de Flust
- ✅ Se compila con `rustc` o `cargo`
- ✅ Se ejecuta sin ningún runtime especial
- ✅ Puedes editarlo manualmente después de generarlo

## 🧪 Testing

### Tests Unitarios

```bash
cargo test --lib
```

### Compilar un Flujo de Ejemplo

```bash
# Desde la UI: Descargar código generado
# O usar la API:
curl -X POST http://localhost:3000/api/compile \
  -H "Content-Type: application/json" \
  -d @my_flow.flow.json
```

## 🤝 Contribuir

Las contribuciones son bienvenidas. Por favor:

1. Fork el repositorio
2. Abre un Pull Request

## 📜 Licencia

GPLv3 - Ver [LICENSE](LICENSE)
