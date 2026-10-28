# NOTAS de trabajo - ReGreet

## Estado actual
- Fork: elimse77/ReGreet. Rama de trabajo: dev.
- Layout nuevo aplicado (Claude): selector de sesión abajo a la derecha,
  Reboot y Power Off como iconos abajo a la izquierda, contraseña debajo
  del usuario.
- Respaldados archivos de /etc/greetd/ en la carpeta config/.
- Corregida transparencia de contenedores (frame.background y frame > border) en config/regreet.css.
- Nota para pruebas locales: ejecutar demo con `XDG_CONFIG_HOME=/tmp` para evitar que el tema GTK4 del usuario sobreescriba `regreet.css`.

## Bitácora
- 2026-10-28 | Claude | Layout nuevo (commit 9cdf2a6 en main).
- 2026-10-28 | Gemini | Respaldo de configuración de la VM en config/ y prueba inicial con --demo (commit a278368).
- 2026-10-28 | Gemini | Ajuste en config/regreet.css para transparencia de frames y validación con XDG_CONFIG_HOME=/tmp.
