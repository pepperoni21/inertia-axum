# Inertia Axum (WIP)
## A Rust library for interacting with the [Inertia.js](https://inertiajs.com/) protocol in Axum web applications.

### Usage
Simply create an `InertiaConfig` instance with the path of the root template (along with other optional fields) that you can add as an extension.

Here's how the root template could look like:
```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Inertia Axum Svelte</title>
    <script src="public/index.js" defer></script>
    <link rel="stylesheet" href="public/index.css">
</head>
<body>
    @inertia
</body>
</html>
```

You can then use the render function to render a component:
```rust
#[derive(Serialize)]
struct RootData {
    message: String,
}


async fn root(Extension(app_state): Extension<Arc<AppState>>, request: Request) -> Response {
    InertiaRenderer::render("index", &request, &inertia_config)
        .with_props(RootData {
            message: "Hey".into(),
        })
        .into_response()
}
```
The props will then be available in the component, here's an example with a Svelte component:
```js
<script>
    export let message
</script>

{message}
```

By the way you can check the [Svelte example](/examples/svelte/), examples for React and Vue are coming soon.

### TODO
- [X] Initial page rendering
- [X] Partial reloads
- [ ] Assets versioning
- [ ] React and Vue examples
- [X] Shared data
- [ ] Support for Vite development server