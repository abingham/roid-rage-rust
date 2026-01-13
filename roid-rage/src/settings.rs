/// Create a Settings struct that holds all of our application settings.
///
/// The struct has static `load()` method which returns a fully configured Settings object.
use std::net::{IpAddr, Ipv6Addr, SocketAddr};

macro_rules! initialize_settings {
    ( $( ($setting:ident, $type:ident, $default_value:expr) ),* ) => {
            #[derive(Clone)]
            pub struct Settings {
                $(
                    pub $setting: $type,
                )*
            }

            impl Settings {
                fn default() -> Settings {
                    Settings {
                        $(
                            $setting: $default_value,
                        )*
                   }
                }

                pub fn load() -> Result<Settings, ()> {
                    // Load the settings file
                    let cfg = config::Config::builder()
                        // Add in `./Settings.toml`
                        // .merge(fig::File::with_name("Settings"))
                        .add_source(config::File::with_name("Settings").required(false))
                        // Add in settings from the environment (with a prefix of APP)
                        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
                        .add_source(config::Environment::with_prefix("ROID_RAGE"))
                        .build()
                        .map_err(|_| ())?;

                    let mut default = Settings::default();
                    $(
                        default.$setting = cfg.get::<$type>(stringify!($setting)).unwrap_or(default.$setting);
                    )*

                    Ok(default)
                }
            }
    };
}

initialize_settings!(
    // (setting-name, setting-type, default-value)
    (screen_width, f32, 800.0),
    (screen_height, f32, 600.0),
    (minimum_roid_radius, f32, 15.0),
    (maximum_roid_radius, f32, 42.5),
    (roid_bumpiness, f32, 0.1),
    (rate_of_fire, f32, 0.5),
    (bullet_speed, f32, 1000.0),
    (min_initial_roid_speed, f32, 50.0),
    (max_initial_roid_speed, f32, 100.0),
    (initial_roid_count, u32, 10),
    (ship_length, f32, 10.0),
    (ship_width, f32, 5.0),
    (ship_mass, f32, 1.0),
    (ship_thrust, f32, 300.0),
    (ship_rotational_speed, f32, 6.0),
    (
        pilot_registration_url,
        SocketAddr,
        SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 50051)
    )
);
