# AGENTS.md — Manual de trabajo para IAs

Este archivo es la fuente única de reglas para toda IA que trabaje en este repositorio
(Claude, Gemini, Qwen u otra). Léelo completo antes de hacer cualquier cosa.

## 1. Roles

- **Daniel** es el coordinador. Decide, revisa y aprueba. Es el único que ejecuta `git push`.
- **Las IAs** proponen y escriben cambios. Cada una es reemplazable: ninguna recuerda lo que
  hizo otra. **La memoria compartida es el repositorio**, en especial `NOTAS.md` y `git log`.

## 2. Contexto fijo

- Daniel usa CachyOS (Arch) con niri. Idioma de trabajo: **español**.
- Lenguaje de los proyectos: Rust + GTK4.
- Cada proyecto vive en `~/proyectos/<nombre>` y su fork en GitHub es `elimse77/<nombre>`.
- El repositorio original (upstream) es de otra persona: nunca se le envía nada.

## 3. Reglas de oro (no negociables)

1. **Una instrucción por mensaje.** Espera a que Daniel confirme éxito o error antes de dar la
   siguiente. Nada de listas de pasos.
2. **Respuestas cortas.** Sin relleno ni repeticiones.
3. **No inventes.** Si no estás seguro, dilo y pregunta.
4. **No afirmes pruebas que no hiciste.** Si dices que algo funciona, indica el comando exacto y
   su resultado. Si no se probó, escribe "NO VERIFICADO".
5. **Nada destructivo sin permiso explícito:** `rm -rf`, `git reset --hard`, `git push --force`,
   borrar ramas, sobrescribir archivos.
6. **No toques nada fuera de la carpeta del proyecto** (por ejemplo `/etc`) sin pedirlo antes.
7. **Nunca guardes secretos** (tokens, contraseñas, claves) en el repositorio ni en el chat.
8. **Archivos de configuración:** se editan a mano con `nano`. Da la ruta y el contenido a
   escribir. No uses `tee` ni heredocs para escribirlos.
9. **Mantén el alcance.** Haz solo lo que Daniel pidió. Si ves otra mejora, menciónala en una
   línea, no la hagas.

## 4. Flujo de una sesión

**Al empezar (obligatorio, antes de responder cualquier petición)**

Si tienes acceso a archivos o terminal, haz esto **tú mismo**, sin pedírselo a Daniel:
1. Lee `AGENTS.md` y `NOTAS.md`.
2. Ejecuta `git branch --show-current`, `git status -sb` y `git --no-pager log --oneline -10`.
3. Ejecuta `git ls-files` para conocer la estructura. Si hay cambios sin commit, revisa
   `git --no-pager diff`.
4. Contrasta `NOTAS.md` con `git log`: si las notas afirman algo que el historial no respalda,
   avísalo.
5. Resume en máximo 3 líneas el estado actual y pregunta qué toca hacer. Espera la respuesta.

Si **no** tienes terminal pero **sí puedes abrir páginas web** (chat web), lee tú mismo el
repositorio desde GitHub, sin pedírselo a Daniel. Abre estas direcciones (rama `dev`):
- `https://raw.githubusercontent.com/elimse77/ReGreet/dev/AGENTS.md`
- `https://raw.githubusercontent.com/elimse77/ReGreet/dev/NOTAS.md`
- `https://github.com/elimse77/ReGreet/commits/dev` (historial de commits)
- `https://github.com/elimse77/ReGreet/tree/dev` (estructura de archivos)
- Para leer cualquier archivo: `https://raw.githubusercontent.com/elimse77/ReGreet/dev/<ruta>`

Limitación: GitHub solo muestra lo que Daniel ya subió con `git push`. No ve los cambios locales
sin subir. Si sospechas que hay trabajo sin subir, pregúntaselo en una línea.

Solo si **no puedes** ni ejecutar comandos ni abrir páginas web, pide a Daniel que pegue:
`NOTAS.md`, `git status -sb` y `git --no-pager log --oneline -10`.

Después:
- Trabaja solo en la rama `dev`. Si Daniel está en otra, avísale.
- Lee solo los archivos que necesites para la tarea. No cargues todo el repositorio.

**Durante el trabajo**
- Cambios pequeños: un tema por commit.
- Compila con `cargo build` y prueba con `--demo` antes de proponer el commit.
- Si hay un error de compilación, pide que te lo pegue completo. No adivines.

**Al terminar (o si se acaba tu ventana de uso)**
1. Actualiza `NOTAS.md` (ver sección 6).
2. Propón el commit y deja a Daniel ejecutarlo y hacer el push.

## 5. Git

- `main`: solo lo probado. Es lo que baja a la VM.
- `dev`: rama de trabajo. Se pasa a `main` solo cuando Daniel lo aprueba tras probar.
- Mensaje de commit: `tipo: descripción corta en español`.
  Tipos: `feat`, `fix`, `style`, `docs`, `chore`, `refactor`.
- Añade al final del mensaje la IA que ayudó, para poder rastrearlo con `git log`:
  `Asistido-por: Claude` (o `Gemini`, o `Qwen`).
- Antes de empezar: `git pull`. Si hay conflictos, detente y avisa.
- Los cambios entre IAs viajan **por el repositorio**, no por conversaciones pegadas.

## 6. NOTAS.md: el relevo entre IAs

Estructura obligatoria:

```
## Estado actual
- Lo que ya funciona (una línea por punto).

## Pendiente
- Lo que falta, con su prioridad.

## Bitácora
- AAAA-MM-DD | IA | Qué se hizo (commit abc1234). Cómo se verificó.
```

Reglas:
- **Cuando termines algo, bórralo de "Pendiente"** o márcalo hecho. Una lista desactualizada
  engaña a la siguiente IA.
- Cada entrada de bitácora indica la IA y cómo se verificó (comando y resultado).
- Se escribe para que otra IA, sin contexto, pueda continuar.

## 7. Verificación y honestidad

- Diferencia siempre **hecho**, **probado** y **no verificado**.
- Si una IA anterior afirmó algo en `NOTAS.md`, trátalo como no confirmado hasta comprobarlo.
- Si algo no compila o no se puede probar en tu entorno, dilo antes de entregar.

## 8. Estilo de código

- Sigue el estilo del archivo que editas. No reformatees archivos enteros.
- Comentarios en inglés dentro del código (el proyecto original lo está); la comunicación con
  Daniel y `NOTAS.md` va en español.
- No añadas dependencias nuevas sin permiso. Prefiere no depender de programas externos.

## 9. Comandos útiles

```
cargo build
XDG_CONFIG_HOME=/tmp cargo run -- --demo -l /tmp/regreet.log -c config/regreet.toml -s config/regreet.css
git status -sb
git --no-pager log --oneline -5
```

En modo demo: código de un solo uso `0248`, contraseña `pass`. Es seguro: no abre sesiones reales.

## 10. Notas por IA

| IA | Archivo que lee la herramienta | Nota |
|---|---|---|
| Claude | `CLAUDE.md` | Puede leer repos públicos y editar, pero no hacer push al fork de Daniel. |
| Gemini | `GEMINI.md` | Verifica sus afirmaciones de pruebas; pide el comando y el resultado. |
| Qwen | `QWEN.md` | Igual que Gemini: pide comando y resultado de cada prueba. |

Si usas el chat web de una IA (sin acceso al repositorio), Daniel debe pegar este archivo al
inicio y el contenido de `NOTAS.md`.

Los archivos `CLAUDE.md`, `GEMINI.md` y `QWEN.md` solo contienen una línea que apunta aquí:

```
Lee y sigue AGENTS.md en la raíz de este repositorio antes de hacer cualquier cosa.
```

## 11. Instrucción de arranque (para pegar en las instrucciones de cada IA)

Texto corto para las "instrucciones personalizadas" o el prompt de sistema de cada IA. Así,
aunque no cargue el repositorio, sabe qué hacer al empezar:

```
Trabajas con Daniel en proyectos de Rust + GTK4 junto a otras IAs (Claude, Gemini, Qwen).
Responde siempre en español, corto, con UNA instrucción por mensaje, y espera mi confirmación.
Al empezar cada sesión, antes de cualquier cosa, lee el repositorio TÚ MISMO, sin pedirme que
pegue nada: si tienes terminal, usa git; si solo tienes navegador, abre en GitHub (rama dev)
AGENTS.md, NOTAS.md y el historial de commits en github.com/elimse77/<proyecto>
(archivos en raw.githubusercontent.com/elimse77/<proyecto>/dev/<ruta>). Resume el estado en
3 líneas y pregunta qué toca. Solo si no puedes ni ejecutar comandos ni abrir páginas, pídeme
los datos. Nunca inventes ni afirmes pruebas que no hiciste. Nada destructivo sin mi permiso.
Solo yo hago git push. Antes de terminar, actualiza NOTAS.md.
```

## 12. Proyectos

- **ReGreet** (`~/proyectos/regreet`): greeter de greetd. Código en `src/gui/`. Respaldos de la
  configuración del sistema en `config/`. Teclado en pantalla en `src/gui/widget/keyboard.rs`.
  GitHub: `https://github.com/elimse77/ReGreet` (rama `dev`).
- **VibePanel** (`~/proyectos/vibepanel`): panel de Wayland. Mucho de lo visual se cambia solo
  con `config.toml`, sin recompilar. Pendiente de hacer el fork; cuando exista, su GitHub será
  `https://github.com/elimse77/vibepanel`.

Regla para Daniel: haz `git push` de `dev` al cerrar cada sesión de trabajo. Si no, las IAs que
leen por GitHub no verán tus últimos cambios.
