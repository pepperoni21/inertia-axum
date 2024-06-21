# Inertia Axum (WIP)
## A Rust library for interacting with the [Inertia.js](https://inertiajs.com/) protocol in Axum web applications.

### Usage
Simply create an `InertiaConfig` instance with the path of the root template (along with other optional fields) that you can add to your application's state.

```rust
struct AppState {
    ...
    inertia_config: InertiaConfig,
}
let inertia_config = InertiaConfig::new("index.html".into());
```

You can then use the render function to render a component:
```rust
#[derive(Serialize)]
struct RootData {
    user: String,
}


async fn root(State(app_state): State<Arc<AppState>>, request: Request) -> Response {
    render_with_props(
        &app_state.inertia_config,
        &request,
        "index".into(),
        RootData {
            user: "some-user".into(),
        },
    )
}
```
The props will then be available in the component, here's an example with a Svelte component:
```js
<script>
    export let user
</script>

Hey {user}, how are you?
```

By the way you can check the [Svelte example](/examples/svelte/), examples for React and Vue are coming soon.

### TODO
- [X] Initial page rendering
- [X] Partial reloads
- [ ] Assets versioning
- [ ] React and Vue examples
- [ ] Assets versioning
- [ ] Shared data