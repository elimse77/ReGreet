# NOTAS de trabajo - ReGreet

## Estado actual
- Fork: elimse77/ReGreet. Rama de trabajo: dev.
- Layout nuevo aplicado (Claude): selector de sesión abajo a la derecha,
  Reboot y Power Off como iconos abajo a la izquierda, contraseña debajo
  del usuario.
- Respaldados archivos de /etc/greetd/ en la carpeta config/.
- Corregida transparencia de contenedores (frame.background y frame > border) en config/regreet.css.
- Nota para pruebas locales: ejecutar demo con `XDG_CONFIG_HOME=/tmp` para evitar que el tema GTK4 del usuario sobreescriba `regreet.css`.
- Teclado en pantalla integrado (Claude): src/gui/widget/keyboard.rs. Botón junto al selector de
  sesión (icono input-keyboard-symbolic). Teclas: letras con ñ, Shift (una sola letra),
  capa ?123 con símbolos y acentos, borrar, espacio y Enter (= botón Login).
  Las teclas no roban el foco: escriben en la casilla activa (o en la primera visible).
  Clases CSS para estilizar: .osk (marco) y .osk-key (teclas).
- Teclado compilado y probado con --demo en la PC de Daniel: funciona.

## Bitácora
- 2026-10-28 | Claude | Layout nuevo (commit 9cdf2a6 en main).
- 2026-10-28 | Gemini | Respaldo de configuración de la VM en config/ y prueba inicial con --demo (commit a278368).
- 2026-10-28 | Gemini | Ajuste en config/regreet.css para transparencia de frames y validación con XDG_CONFIG_HOME=/tmp.
- 2026-10-28 | Claude | Teclado en pantalla (keyboard.rs + botón en templates.rs). Lógica probada con 15 pruebas en pantalla virtual; falta compilar y probar en la PC.
- 2026-10-28 | Claude | Manual AGENTS.md y punteros CLAUDE.md, GEMINI.md, QWEN.md. NO VERIFICADO que Gemini y Qwen los lean solos.
