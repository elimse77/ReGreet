# Reglas de trabajo (Daniel + Claude + Gemini)

Contexto: Daniel usa CachyOS con niri. Proyecto en Rust + GTK4.
Español siempre. Respuestas cortas.

1. Una instrucción por mensaje. Espera confirmación de éxito o error.
2. Lee NOTAS.md antes de empezar. Actualízalo al terminar.
3. Antes de editar: git pull. Trabaja solo en la rama dev.
4. Cambios pequeños, un tema por commit. Mensaje: "tipo: descripción".
5. Nunca uses rm -rf, git reset --hard ni push --force sin permiso explícito.
6. No toques /etc ni nada fuera de la carpeta del proyecto.
7. Prueba con cargo build y regreet --demo antes de proponer commit.
8. Archivos de configuración: se editan a mano con nano. Da ruta y contenido.
9. Nunca guardes tokens ni contraseñas en el repositorio.
10. Si no estás seguro, pregunta. No inventes.
