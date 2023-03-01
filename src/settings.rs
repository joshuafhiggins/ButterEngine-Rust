use configurable::{Configurable, Config, Data, Error, LoadState};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    width: u32,
    height: u32,
    title: String,
    mode: Mode,
    swap_interval: u32
}

#[derive(Serialize, Deserialize)]
enum Mode {
    FullScreen,
    Windowed
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            width: 1280,
            height: 720,
            title: "".to_string(),
            mode: Mode::Windowed,
            swap_interval: 1
        }
    }
}

impl Config for Settings {}

impl Configurable for Settings {
    const ORGANIZATION: &'static str = "museun";
    const APPLICATION: &'static str = "foobar";
    const NAME: &'static str = "config.toml";

    fn ensure_dir() -> Result<std::path::PathBuf, Error> {
        <Self as Config>::ensure_dir()
    }
}