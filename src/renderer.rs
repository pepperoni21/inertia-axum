use axum::{
    extract::Request,
    response::{Html, IntoResponse, Response},
};
use serde::Serialize;

use crate::{InertiaConfig, PageObject};

pub fn render_with_props(
    inertia_config: &InertiaConfig,
    request: &Request,
    component: String,
    props: impl Serialize,
) -> Response {
    let serialized_props: serde_json::Value = serde_json::to_value(props).unwrap();
    let html_path = inertia_config.html_path.clone();
    let html = std::fs::read_to_string(html_path).unwrap();
    let page_object = PageObject {
        component,
        props: serialized_props,
        url: request.uri().path().to_string(),
        version: "1".to_string(),
    };
    let serialized_page_object = serde_json::to_string(&page_object).unwrap();
    let root_div_id = inertia_config.root_div_id.clone();
    let root_div =
        format!(r#"<div id="{root_div_id}" data-page='{serialized_page_object}'></div>"#);
    let html = html.replace("@inertia", &root_div);
    Html(html).into_response()
}

pub fn render(inertia_config: &InertiaConfig, request: &Request, component: String) -> Response {
    render_with_props(inertia_config, request, component, None::<()>)
}
