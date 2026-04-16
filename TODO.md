# TODO

## feat

### core

- [ ] Save settings to `XDG_CONFIG_HOME/kairo/kairo.toml`
- [ ] Allow configuring fixed browsers for specific URLs using regex
- [ ] Allow filtering apps from the handlers list
- [ ] Linux: support for executing `Actions` in `desktop` files
- [ ] macOS: support to search available browsers and open URLs

### desktop

- [ ] Allow opening the app without a URL argument and show all browsers
- [ ] Ask user if they want to register Kairo as the default browser
- [ ] Keyboard navigation with arrows and `ESC` or `q` to exit
- [ ] Pre-select last browser chosen in previous session
- [ ] Save window size and position
- [ ] Create an app settings screen
- [ ] Send system notification in case of error opening a URL (wrong format, no handler, etc)
- [ ] Publish on Flatpack

### cli

- [ ] Pre-select last browser chosen in previous session

## fix

- [ ] Memory allocation error (#13)
- [ ] Zombie application processes when rendering the `iced` UI:
    ```
    Mär 07 12:49:19.920158 rigel slack[1212915]:  INFO  iced_wgpu::window::compositor       > Available alpha modes: [
    Mär 07 12:49:19.920158 rigel slack[1212915]:     Opaque,
    Mär 07 12:49:19.920158 rigel slack[1212915]:     PreMultiplied,
    Mär 07 12:49:19.920158 rigel slack[1212915]: ]
    Mär 07 12:49:19.920158 rigel slack[1212915]:  INFO  iced_wgpu::window::compositor       > Selected format: Rgba8Unorm with alpha mode: PreMultiplied
    Mär 07 12:49:19.977603 rigel slack[1212915]:  INFO  kairo_desktop::app                  > Rendering URL handler selection UI
    ```
- [ ] Kairo stays open after browser is selected
    - Happens only when it's the first window opened by the browser
    - Problematic code:
        ```rust
        let mut program = Command::new(cmd).args(args).spawn()?;
        let status = program.wait()?;
        ```
- [ ] Allow specifying file system file paths in arguments: replace them with `file://`
    - Fixes `invalid value 'example.html' for 'URL': relative URL without a base`
- [ ] Startup error loop:
    ```
    WARN  iced_futures::subscription::tracker > Error sending event to subscription: TrySendError { kind: Full }
    ERROR iced_winit                          > Error Other when presenting surface.
    ERROR iced_winit                          > Error Other when presenting surface.
    ERROR iced_winit                          > Error Other when presenting surface.
    ERROR iced_winit                          > Error Other when presenting surface.
    ...
    ```

## perf

### desktop

- [ ] Window opening speed
- [ ] Reduce app memory usage (currently ~100Mb)
