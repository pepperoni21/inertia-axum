use axum::extract::Request;
use serde_json::{Map, Value};

#[derive(Clone, Debug, Default)]
pub struct SharedState(pub Map<String, Value>);

pub fn add_shared_state(req: &mut Request, obj: Map<String, Value>) {
    let mut shared_state = req
        .extensions()
        .get::<SharedState>()
        .cloned()
        .unwrap_or_default();
    shared_state.0.extend(obj);
    req.extensions_mut().insert(shared_state);
}
