# pkg — Módulo WASM principal (academia_wasm)

Módulo Rust compilado a WebAssembly con **~40 funciones** para análisis de texto y NLP.

## 📦 Información

- **Crate**: `academia-wasm` v0.1.0
- **Lenguaje**: Rust (edición 2021)
- **Target**: WebAssembly (wasm32-unknown-unknown)
- **Binding**: `wasm-bindgen` 0.2.128
- **Tamaño compilado**: ~163 KB

## 🛠️ Funciones principales

- Análisis de texto: contar palabras, oraciones, caracteres
- Detección de idioma avanzada
- Detección de clickbait y discurso de odio
- Detección de PII (emails, teléfonos, URLs)
- Legibilidad: Flesch, Gunning-Fog, densidad léxica
- NLP: tópicos, entidades, n-gramas, TF-IDF
- Utilidades: SHA-256, sanitizar, normalizar

## 🚀 Compilar

```bash
cd pkg
wasm-pack build --target web --release --out-dir . --out-name academia_wasm
cd ~/academia-repo

# 1. Backup de pkg actual
mv pkg pkg_backup_$(date +%Y%m%d)

# 2. Copiar desde academia-wasm
mkdir -p pkg/src
cp ~/academia-wasm/Cargo.toml pkg/Cargo.toml
cp ~/academia-wasm/Cargo.lock pkg/Cargo.lock
cp ~/academia-wasm/src/lib.rs pkg/src/lib.rs
cp ~/academia-wasm/pkg/*.js pkg/ 2>/dev/null
cp ~/academia-wasm/pkg/*.wasm pkg/ 2>/dev/null
cp ~/academia-wasm/pkg/*.d.ts pkg/ 2>/dev/null
cp ~/academia-wasm/pkg/package.json pkg/ 2>/dev/null

# 3. .gitignore de pkg
cat > pkg/.gitignore << 'EOF'
*.wasm
*.js
*.d.ts
package.json
!Cargo.toml
!Cargo.lock
!src/
target/
node-test/
