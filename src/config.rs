#[derive(Debug)]
pub struct InertiaConfig {
    pub html_path: String,
    pub root_div_id: String,
}

impl InertiaConfig {
    pub fn new(html_path: String) -> Self {
        Self {
            html_path,
            root_div_id: "app".into(),
        }
    }

    pub fn with_root_div_id(mut self, root_div_id: String) -> Self {
        self.root_div_id = root_div_id;
        self
    }
}
