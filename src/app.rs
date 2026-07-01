#[derive(Debug, Clone)]
pub enum App {
    Default,
    FireFox,
}

impl From<App> for String {
    fn from(val: App) -> Self {
        match val {
            App::FireFox => "FireFox".to_string(),
            App::Default => "NiriIsLand".to_string(),
        }
    }
}
