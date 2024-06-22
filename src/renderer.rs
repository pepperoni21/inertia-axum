use axum::{
    extract::Request,
    response::{Html, IntoResponse, Response},
};
use serde::Serialize;
use serde_json::{Map, Value};

use crate::{shared_state::SharedState, InertiaConfig, PageObject};

pub struct InertiaRenderer<'a> {
    request: &'a Request,
    config: &'a InertiaConfig,
    component: String,
    props: Option<Map<String, Value>>,
}

impl<'a> InertiaRenderer<'a> {
    pub fn render(
        component: &str,
        request: &'a Request,
        config: &'a InertiaConfig,
    ) -> InertiaRenderer<'a> {
        InertiaRenderer {
            request,
            component: component.to_string(),
            props: None,
            config,
        }
    }

    pub fn with_props(mut self, props: impl Serialize) -> Self {
        let serialized_props: Map<String, Value> = serde_json::to_value(props)
            .unwrap()
            .as_object()
            .unwrap_or(&Map::new())
            .clone();
        self.props = Some(serialized_props);
        self
    }
}

impl<'a> IntoResponse for InertiaRenderer<'a> {
    fn into_response(self) -> Response {
        let headers = self.request.headers();

        let shared_state = self
            .request
            .extensions()
            .get::<SharedState>()
            .map(|s| s.clone())
            .unwrap_or_default();

        let props = self.props.unwrap_or_default();
        let combined_props = combine_shared_state_with_props(&shared_state, props);

        let page_object = PageObject {
            component: self.component,
            props: combined_props,
            url: self.request.uri().path().to_string(),
            version: "1".to_string(),
        };
        let serialized_page_object = serde_json::to_string(&page_object).unwrap();

        if headers.contains_key("X-Inertia") && headers.get("X-Inertia").unwrap() == "true" {
            return generate_json_page_object_response(&page_object);
        }

        let html_path = self.config.html_path.clone();
        let html = std::fs::read_to_string(html_path).unwrap();
        let root_div_id = self.config.root_div_id.clone();
        let root_div =
            format!(r#"<div id="{root_div_id}" data-page='{serialized_page_object}'></div>"#);
        let html = html.replace("@inertia", &root_div);
        Html(html).into_response()
    }
}

fn generate_json_page_object_response(page_object: &PageObject) -> Response {
    let serialized_page_object = serde_json::to_string(page_object).unwrap();
    let mut response = serialized_page_object.into_response();
    let headers = response.headers_mut();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("X-Inertia", "Accept".parse().unwrap());
    headers.insert("X-Inertia", "true".parse().unwrap());
    response
}

fn combine_shared_state_with_props(
    shared_state: &SharedState,
    props: Map<String, Value>,
) -> serde_json::Value {
    let mut shared_state = shared_state.0.clone();
    shared_state.extend(props);
    serde_json::Value::Object(shared_state)
}
