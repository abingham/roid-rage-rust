// TODO: More settings:
// initial roid radius
// roid bumpiness
// initial roid speed
// initial number of roids
// bullet speed
// fire rate

pub struct Settings {
    pub screen_width: f32,
    pub screen_height: f32,
    pub minimum_roid_radius: f32,
}

impl Settings {
    fn default() -> Settings {
        Settings {
            screen_width: 800.0,
            screen_height: 600.0,
            minimum_roid_radius: 15.0,
        }
    }
}

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

        let default = Settings::default();

        Ok(Settings {
            // TODO: Macro for doing this get/unwrap_or dance
            screen_width: cfg
                .get::<f32>("screen_width")
                .unwrap_or(default.screen_width),
            screen_height: cfg
                .get::<f32>("screen_height")
                .unwrap_or(default.screen_height),
            minimum_roid_radius: cfg
                .get::<f32>("minimum_roid_radius")
                .unwrap_or(default.minimum_roid_radius),
        })
    }
}
