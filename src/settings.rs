// TODO: More settings:
// initial roid radius
// minimum roid size
// roid bumpiness
// initial roid speed
// initial number of roids
// bullet speed
// fire rate

pub struct Settings {
    pub screen_width: f32,
    pub screen_height: f32,
}

const DEFAULT_SCREEN_WIDTH: f32 = 800.0;
const DEFAULT_SCREEN_HEIGHT: f32 = 600.0;

impl Settings {
    pub fn load() -> Result<Settings, ()> {
        // Load the settings file
        let mut cfg = config::Config::default();

        // Add in `./Settings.toml`
        cfg
            // .merge(fig::File::with_name("Settings"))
            .merge(config::File::with_name("Settings").required(false))
            .unwrap();
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        cfg.merge(config::Environment::with_prefix("ROID_RAGE"))
            .unwrap();

        Ok(Settings {
            screen_width: cfg
                .get::<f32>("screen_width")
                .unwrap_or(DEFAULT_SCREEN_WIDTH),
            screen_height: cfg
                .get::<f32>("screen_height")
                .unwrap_or(DEFAULT_SCREEN_HEIGHT),
        })
    }
}
