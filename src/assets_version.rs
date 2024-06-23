use axum::extract::Request;

#[derive(Clone, Debug)]
pub enum AssetsVersion {
    String(String),
    Number(u64),
}

impl Default for AssetsVersion {
    fn default() -> Self {
        AssetsVersion::Number(1)
    }
}

impl ToString for AssetsVersion {
    fn to_string(&self) -> String {
        match self {
            AssetsVersion::String(s) => s.clone(),
            AssetsVersion::Number(n) => n.to_string(),
        }
    }
}

pub fn set_assets_version(req: &mut Request, version: AssetsVersion) {
    req.extensions_mut().insert(version);
}
