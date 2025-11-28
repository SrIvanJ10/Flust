# 🎯 Flust - Visual Rust Programming

Sistema de programación visual para Rust que genera código compilable y ejecutable.

## ✨ Estado del Proyecto

**✅ COMPLETAMENTE FUNCIONAL**

- Sistema simplificado a 2 bloques: **Legacy Code** y **Debug**
- Arquitectura refactorizada siguiendo principios **SOLID**
- Generación de código basada en plantillas
- **Todas las pruebas pasando** (7/7 tests unitarios)
- **Código generado compila y ejecuta correctamente**

## 🚀 Inicio Rápido

### 1. Iniciar el Backend (Rust)

```bash
cd /home/ivan/Documentos/proyectos/flust
cargo run --bin flust-server
```

El servidor estará disponible en `http://localhost:3000`

### 2. Iniciar el Frontend (React)

```bash
cd /home/ivan/Documentos/proyectos/flust/frontend
npm run dev
```

La interfaz estará disponible en `http://localhost:5173`

### 3. Usar la Aplicación

1. Abre `http://localhost:5173` en tu navegador
2. Arrastra bloques desde el sidebar al canvas
3. Edita las propiedades de cada bloque
4. Conecta los bloques arrastrando entre ellos
5. Haz clic en **Play** (▶️) para compilar
6. Revisa el código generado en el panel de logs

## 📦 Bloques Disponibles

### Legacy Code (Código Libre)
- **Categoría:** Code Blocks
- **Uso:** Escribe cualquier código Rust
- **Propiedades:**
  - `code`: Código Rust personalizado

### Debug (Depuración)
- **Categoría:** Utilities  
- **Icono:** 🐛
- **Uso:** Imprime variables en consola
- **Propiedades:**
  - `variable`: Nombre de la variable a imprimir
  - `label`: Etiqueta opcional para el output

## 🧪 Ejemplo: Hello World

### Opción 1: Via API

```bash
curl -X POST http://localhost:3000/api/compile \
  -H "Content-Type: application/json" \
  -d '{
    "nodes": [{
      "id": "node1",
      "type": "legacy-code",
      "label": "Hello",
      "code": "println!(\"Hello, World!\");"
    }],
    "connections": []
  }'
```

### Opción 2: Via UI

1. Arrastra "Legacy Code" al canvas
2. Edita el código: `println!("Hello, World!");`
3. Haz clic en Play ▶️
4. Copia el código generado
5. Compila: `rustc generated.rs && ./generated`

## 🏗️ Arquitectura

```
flust/
├── flust-core/          # IR y utilidades core
│   ├── ir.rs           # Representación intermedia (plugin-based)
│   └── topological_sort.rs  # Ordenamiento de nodos
├── flust-codegen/      # Generación de código
│   ├── generator.rs    # Generador principal
│   └── template_engine.rs   # Motor de plantillas
├── flust-server/       # API REST (Axum)
│   └── main.rs         # Servidor HTTP
└── frontend/           # UI React
    ├── src/
    │   ├── components/ # Componentes UI (SOLID refactored)
    │   └── plugins/    # Sistema de plugins
    └── public/plugins/
        ├── legacy-code/    # Plugin de código libre
        └── debug/          # Plugin de debug
```

## 🎨 Principios SOLID Aplicados

### Single Responsibility Principle (SRP)
- `PropertyField`: Solo renderiza propiedades
- `TemplateEngine`: Solo procesa plantillas
- `TopologicalSort`: Solo ordena grafos

### Open/Closed Principle (OCP)
- Nuevos bloques via JSON, sin cambios de código
- Sistema de plantillas extensible

### Liskov Substitution Principle (LSP)
- Todos los plugins siguen la misma interfaz

### Interface Segregation Principle (ISP)
- Interfaces específicas: `PluginProperty`, `Plugin`, `FlowNode`

### Dependency Inversion Principle (DIP)
- Dependencias en abstracciones, no implementaciones

## 📊 Pruebas

### Ejecutar Tests Unitarios

```bash
cargo test --lib
```

**Resultado:** 7/7 tests pasando ✅

### Tests E2E

Ver `VERIFICATION_TESTS.md` para resultados completos de:
- ✅ Hello World simple
- ✅ Flujo completo con debug
- ✅ Verificación UI
- ✅ Compilación y ejecución

## 📝 Documentación Adicional

- [`walkthrough.md`](file:///home/ivan/.gemini/antigravity/brain/dfef5d0d-d17e-4e17-a35e-3be180123c55/walkthrough.md) - Guía completa de cambios
- [`VERIFICATION_TESTS.md`](file:///home/ivan/Documentos/proyectos/flust/VERIFICATION_TESTS.md) - Resultados de pruebas E2E
- [`implementation_plan.md`](file:///home/ivan/.gemini/antigravity/brain/dfef5d0d-d17e-4e17-a35e-3be180123c55/implementation_plan.md) - Plan técnico detallado

## 🔧 Desarrollo

### Añadir un Nuevo Bloque

1. Crear carpeta en `frontend/public/plugins/mi-bloque/`
2. Crear `plugin.json`:
```json
{
  "id": "mi-bloque",
  "name": "Mi Bloque",
  "category": "Utilities",
  "icon": "⚡",
  "description": "Descripción",
  "properties": [...]
}
```
3. Crear `template.rs`:
```rust
// Código con {{variables}}
```
4. Añadir `"mi-bloque"` a `PLUGIN_IDS` en `usePlugins.ts`
5. Añadir template al generador en `generator.rs`

¡Sin necesidad de modificar el core! 🎉

## 📌 Estado Actual

- [x] 2 bloques implementados
- [x] Generación de código funcional
- [x] UI completa y responsive
- [x] Topological sort implementado
- [x] SOLID principles aplicados
- [x] Tests pasando
- [x] Código generado compila
- [x] E2E verificado

## 🎯 Próximos Pasos (Opcional)

1. Cargar templates desde archivos
2. Más opciones de debug (pretty-print)
3. Tracking de variables entre bloques
4. Syntax highlighting en editor
5. Generación de `Cargo.toml`
6. Botón "Run" integrado

## 📜 Licencia

GPLv3 - Ver LICENSE

---

**Hecho con ❤️ siguiendo las mejores prácticas de ingeniería de software**
