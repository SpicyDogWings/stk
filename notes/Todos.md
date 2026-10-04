# Tareas pendientes v0.3.0-alpha

## - [ ] Usar ratatui

Investigar sobre esta implementación de ratatui para crear ui inline

``` rs
let backend = CrosstermBackend::new(std::io::stdout());
let mut terminal = Terminal::with_options(backend, TerminalOptions {
    viewport: Viewport::Inline(8),
})?;
```

## - [x] Parametro bold opcional

Convertir el bold en un parametro opcional

``` rs
let style = Style::new()
    .fg_color(Some(args.foreground.into()))
    .effects(Effects::BOLD); // opcional
```

## - [x] Agregar desc al comando log

La descrición del comando sale vacia
