use project_root::get_project_root;
use utils::env::get_env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub wd_port: u16,
    pub port: u16,
    pub host: String,
    pub root_dir: String,
    pub anthropic_api_key: String,
}

impl Config {
    pub fn init() -> Self {
        let database_url = get_env("DATABASE_URL");
        let wd_port = get_env("CHROMEDRIVER_PORT");
        let port = get_env("BACKEND_PORT");
        let host = get_env("BACKEND_HOST");
        let anthropic_api_key = get_env("ANTHROPIC_API_KEY");
        let root_dir = get_root_dir();

        println!("configuration initialized: {host}:{port}, db: {database_url}, wd: {wd_port}, root: {root_dir}");

        Self {
            database_url,
            anthropic_api_key,
            port: port.parse().unwrap_or(3000),
            wd_port: wd_port.parse().unwrap_or(4000),
            host,
            root_dir,
        }
    }
}

fn get_root_dir() -> String {
    let path = get_project_root();
    if let Ok(mut path) = path {
        path.pop();
        path.to_str().unwrap().to_string()
    } else {
        get_env("APP_ROOT")
    }
}
