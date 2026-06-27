🎮 Gráficos e Inputs: pixels + winit

- **winit:** Es la librería estándar en Rust para crear ventanas y manejar el bucle de eventos del sistema operativo. Te resolverá de forma nativa la lectura del teclado.

- **pixels:** Es una joya para emuladores. Te expone un buffer de píxeles plano (un simple array [u8] con formato RGBA) donde puedes dibujar directamente la pantalla de la GameBoy (160x144) o de la GBA (240x160). Por detrás, usa wgpu para renderizar y escalar la imagen con aceleración por hardware de forma perfecta.

🔊 Sonido: cpal o rodio

- **cpal:** Es la librería de audio de más bajo nivel en Rust. En un emulador donde generas muestras de audio en crudo ciclo por ciclo (desde tu componente APU), cpal es ideal porque te da control absoluto sobre el stream de audio, minimizando la latencia.

- **rodio:** Está construida sobre cpal. Es un poco más amigable, aunque para la emulación cruda de ondas sonoras a veces se prefiere interactuar directo con cpal.

🕹️ Inputs de Mando (Extra): 

- **gilrs**: Si quieres que tu emulador soporte mandos de Xbox, PlayStation o genéricos de verdad, winit no te bastará. gilrs (Game Input Library for Rust) es la opción por defecto para mapear gamepads de manera supersencilla.