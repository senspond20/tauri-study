# Tauri Custom Windows

This template should help get you started developing with Tauri, React and Typescript in Vite.

+ https://v2.tauri.app/learn/window-customization/


| Permission                                      | Description                                                                 |
|------------------------------------------------|-----------------------------------------------------------------------------|
| core:window:default                            | Default permissions for the plugin. Except `window:allow-start-dragging`.  |
| core:window:allow-close                        | Enables the `close` command without any pre-configured scope.              |
| core:window:allow-minimize                     | Enables the `minimize` command without any pre-configured scope.           |
| core:window:allow-start-dragging              | Enables the `start_dragging` command without any pre-configured scope.     |
| core:window:allow-toggle-maximize              | Enables the `toggle_maximize` command without any pre-configured scope.    |
| core:window:allow-internal-toggle-maximize     | Enables the `internal_toggle_maximize` command without any pre-configured scope. |


+ tauri.config.json

```
    "windows": [
      {
        "title": "custom-windows",
        "width": 800,
        "height": 600,
        "decorations": false
      }
    ],
```

"decorations": false