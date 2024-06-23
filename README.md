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

### Assets versioning
This library will use the `AssetVersion` extension to add a version to the assets.
You can then control the version of the assets from your handler or middleware:
```rust
async fn assets_versioning_middleware(mut req: Request, next: Next) -> Response {
    match req.uri().path() {
        "/counter" => set_assets_version(&mut req, AssetsVersion::String("counter".into())),
        "/" => set_assets_version(&mut req, AssetsVersion::String("root".into())),
        _ => {}
    };
    next.run(req).await
}
```
If no version is specified, the default version will be `AssetsVersion::Number(1)`

### Shared data
Shared data works similarly to the assets versioning, you can set shared data from your handler or middleware:
```rust
async fn shared_state_middleware(mut req: Request, next: Next) -> Response {
    match req.uri().path() {
        "/" | "counter" => add_shared_state(
            &mut req,
            serde_json::json!({
                "user": "John Doe"
            }),
        ),
        _ => {}
    };
    next.run(req).await
}
```
The above code will make the `user` property available in the index and counter components.

### TODO
- [X] Initial page rendering
- [X] Partial reloads
- [X] Assets versioning
- [ ] React and Vue examples
- [X] Shared data
- [ ] Support for Vite development server