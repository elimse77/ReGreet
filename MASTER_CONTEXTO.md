# MASTER_CONTEXTO.md — Foto del contexto del proyecto

Estado a **2026-10-29**. Léelo después de `AGENTS.md` y `NOTAS.md`.
Si algo de aquí contradice al código o a `git log`, **mandan el código y `git log`**.
Este repositorio es público: no escribas aquí tokens, contraseñas ni datos personales.

## 1. Objetivos
1. Entender ReGreet (greeter de greetd en Rust + GTK4) y personalizarlo.
2. Rediseñar el login:
   - Al inicio solo la casilla de usuario y el botón Login.
   - La contraseña aparece debajo al pulsar Login o Enter.
   - Selector de sesión desplegable abajo a la derecha.
   - Reboot y Power Off como iconos abajo a la izquierda.
3. Teclado en pantalla integrado en Rust, sin dependencias externas, con un botón junto al
   selector de sesión.
4. Hacer lo mismo con VibePanel (fork propio).
5. Trabajar con varias IAs (Claude, Gemini y posiblemente Qwen) sobre los mismos proyectos, con
   todo documentado.

## 2. Roles
- **Daniel**: coordinador. Decide, revisa y aprueba. Es el único que hace `git push`.
- **Claude**: lee repos públicos y prepara parches. No tiene permiso de escritura en el fork.
- **Gemini**: se usa cuando se agota la ventana de uso de Claude. Ya ha hecho commits en `dev`.
- **Qwen**: posible tercera IA. Solo existe el puntero `QWEN.md`; no se ha verificado que lo lea.
- **Terceros**: Harish Rajagopal (autor de ReGreet) y `prankstr` (autor de VibePanel).

## 3. Enlaces
- Fork de ReGreet: https://github.com/elimse77/ReGreet (ramas `main` y `dev`)
- ReGreet original: https://github.com/rharish101/ReGreet
- VibePanel original: https://github.com/prankstr/vibepanel
- Fork de VibePanel: **aún no creado** (previsto: https://github.com/elimse77/vibepanel)
- Lectura directa de archivos: `https://raw.githubusercontent.com/elimse77/ReGreet/dev/<ruta>`

## 4. Entornos
- **PC personal** (CachyOS + niri): `~/proyectos/regreet`, rama `dev`. Compila y prueba con `--demo`.
- **VM** (greetd con ReGreet): solo recibe lo pulido. La configuración del sistema vive en
  `/etc/greetd/` (`regreet.css`, `regreet.toml`); hay respaldo en `config/`.
- **Celular** (Termux): git para tareas sencillas. No compila el proyecto.
- Entorno de Claude en la nube: GTK 4.14 (ReGreet pide 4.16 o superior), por lo que no puede
  compilar el proyecto completo.

## 5. ReGreet: arquitectura
- Versión 0.5.0, edición Rust 2024. GTK4 + Relm4 + Tokio + zbus + `greetd_ipc` + serde/toml.
- `src/main.rs`: argumentos (clap) y logging con rotación.
- `src/client.rs`: socket de greetd (`GREETD_SOCK`); modo `--demo`.
- `src/config.rs`, `src/tomlutils.rs`: configuración TOML; si falla, valores por defecto.
- `src/cache/`: último usuario y sesión (LRU propio).
- `src/sysutil.rs`: usuarios por D-Bus (AccountsService) y sesiones desde archivos `.desktop`.
- `src/constants.rs`: rutas fijadas en compilación.
- `src/gui/`: `model.rs` (lógica), `component.rs` (vista), `templates.rs` (layout),
  `messages.rs`, `widget/clock.rs` y `widget/keyboard.rs` (teclado).
- Flujo: Login → `create_session` → greetd pide secreto/visible/info/error → respuesta →
  `Success` → `start_session` → `exit(0)`.
- Modo demo (no toca el sistema): código `0248`, contraseña `pass`. Abre a pantalla completa.
  `XDG_CONFIG_HOME=/tmp cargo run -- --demo -l /tmp/regreet.log -c config/regreet.toml -s config/regreet.css`
- El estilo se cambia sin recompilar con `/etc/greetd/regreet.css`.

## 6. Historial de cambios
**`main`**
- `9cdf2a6`: nuevo layout (usuario + Login; contraseña debajo; sesión abajo a la derecha con
  botón de lápiz para modo manual; Reboot/Power Off como iconos con tooltip).

**`dev`**
- `27bcb79`: reglas iniciales `GEMINI.md` y `NOTAS.md`.
- `a278368` (Gemini): respaldo de la configuración de la VM en `config/`.
- `f282bf4` (Gemini): CSS con contenedores, botones y casillas transparentes.
- `8f1f2c4` (Claude): teclado en pantalla.
  - `src/gui/widget/keyboard.rs`; botón `kbd_toggle` (icono `input-keyboard-symbolic`) en
    `templates.rs`; conexión en `component.rs`.
  - Teclas: letras con ñ, Shift de una sola letra, capa `?123` con símbolos y acentos, borrar,
    espacio y Enter (equivale a Login). Las teclas no roban el foco.
  - Clases CSS: `.osk` y `.osk-key`.
  - Verificado: 15 pruebas de lógica en pantalla virtual; compila y se probó con `--demo`.
- `c35e13e`: `AGENTS.md` y punteros `CLAUDE.md`, `GEMINI.md`, `QWEN.md`.

## 7. Manual de trabajo (`AGENTS.md`)
- Una instrucción por mensaje; respuestas cortas; siempre en español.
- No inventar; toda prueba se reporta con comando y resultado, o "NO VERIFICADO".
- Nada destructivo sin permiso. Solo Daniel hace push.
- Commits con `Asistido-por: Claude|Gemini|Qwen`.
- `NOTAS.md`: estado, pendiente y bitácora; borrar de "Pendiente" lo ya hecho.
- Al empezar, la IA lee el repo por su cuenta: terminal → git; solo navegador → URLs de GitHub
  (rama `dev`); último recurso → pedir datos a Daniel.
- Daniel hace `git push` de `dev` al cerrar cada sesión para que las IAs vean lo último.

## 8. VibePanel (lo leído)
- Panel Wayland en Rust + GTK4: barra, notificaciones, ajustes rápidos y OSD (~114,000 líneas).
- Workspace: `crates/vibepanel-core` (config y temas) y `crates/vibepanel` (barra, widgets,
  servicios). Configuración en `config.toml` (se aplica al guardar). Soporta niri.
- Requiere GTK 4.22 o superior desde la versión 0.17.
- Mucho de lo visual se cambia solo con `config.toml`, sin recompilar.

## 9. Decisiones tomadas
- El fork `elimse77/...` es la versión oficial.
- `main` solo con lo probado; `dev` para trabajar; la VM recibe solo lo pulido.
- Teclado propio en Rust en vez de depender de `wvkbd`.
- Un solo manual (`AGENTS.md`) con punteros por IA.
- Todo cambio se documenta en commits y en `NOTAS.md`.
- Archivos de configuración se editan a mano con nano.

## 10. Pendientes
1. Subir el `AGENTS.md` actualizado (lectura del repo por GitHub) y este archivo.
2. Pegar el texto de arranque en las instrucciones de Gemini y probar que lea el repo; luego Qwen.
3. Mejorar el contraste de "Bienvenido" sobre el logo del fondo (sin decisión).
4. Pasar `dev` a `main` y llevarlo a la VM (método sin definir).
5. Fork y clon de VibePanel en `~/proyectos/vibepanel`, con los mismos archivos de manual.
6. Posible opción de configuración para activar o desactivar el botón del teclado.
7. Guardar de forma persistente la autenticación de GitHub en la PC (token nuevo).

## 11. No verificado
- Que Gemini y Qwen lean `GEMINI.md` y `QWEN.md` automáticamente.
- Que Gemini en chat web abra los enlaces de GitHub por sí solo.
- Aspecto real del teclado y funcionamiento con clic o pantalla táctil.
- La prueba con `XDG_CONFIG_HOME=/tmp` que Gemini afirmó haber hecho.

## 12. Puntos abiertos (Daniel debe responder)
- Qué equipo es cuál (PC y VM) y cómo se accede a la VM.
- Si la VM ya tiene el fork clonado.
- Qué ajustes quiere en el teclado; si la pantalla es táctil.
- Criterios para pasar `dev` a `main`.
- Si el objetivo es uso personal o publicar/compartir la versión propia.
- Qué quiere cambiar en VibePanel y qué versión de GTK tiene.
- Qué herramienta es Qwen y cómo usa Gemini (app web, celular o terminal).
