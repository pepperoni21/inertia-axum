use axum::{
    extract::Request,
    response::{Html, IntoResponse, Response},
};
use serde::Serialize;

use crate::{InertiaConfig, PageObject};

pub fn render_with_props(
    inertia_config: &InertiaConfig,
    request: &Request,
    component: &str,
    props: impl Serialize,
) -> Response {
    let headers = request.headers();

    let serialized_props: serde_json::Value = serde_json::to_value(props).unwrap();
    let page_object = PageObject {
        component: component.to_string(),
        props: serialized_props,
        url: request.uri().path().to_string(),
        version: "1".to_string(),
    };
    let serialized_page_object = serde_json::to_string(&page_object).unwrap();

    if headers.contains_key("X-Inertia") && headers.get("X-Inertia").unwrap() == "true" {
        return generate_json_page_object_response(&page_object);
    }

    let html_path = inertia_config.html_path.clone();
    let html = std::fs::read_to_string(html_path).unwrap();
    let root_div_id = inertia_config.root_div_id.clone();
    let root_div =
        format!(r#"<div id="{root_div_id}" data-page='{serialized_page_object}'></div>"#);
    let html = html.replace("@inertia", &root_div);
    Html(html).into_response()
}

pub fn render(inertia_config: &InertiaConfig, request: &Request, component: &str) -> Response {
    render_with_props(inertia_config, request, component, None::<()>)
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
