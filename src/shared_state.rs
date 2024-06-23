use axum::extract::Request;
use serde::Serialize;
use serde_json::{Map, Value};

#[derive(Clone, Debug, Default)]
pub struct SharedState(pub Map<String, Value>);

pub fn add_shared_state(req: &mut Request, obj: impl Serialize) {
    let obj = serde_json::to_value(obj)
        .unwrap()
        .as_object()
        .cloned()
        .unwrap_or(Map::new())
        .clone();
    let mut shared_state = req
        .extensions()
        .get::<SharedState>()
        .cloned()
        .unwrap_or_default();
    shared_state.0.extend(obj);
    req.extensions_mut().insert(shared_state);
}
