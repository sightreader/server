use std::path::Path;
use std::env;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;

const APP_SETTINGS_FILENAME: &str = "app.config";
const APP_SETTINGS_DEFAULT_CONTENT: &str = "{}";

pub fn load_settings() {
    let mut program_dir = env::current_exe().unwrap();
    program_dir.pop();

    let app_settings_filepath = Path::new(&program_dir).join(APP_SETTINGS_FILENAME);
    let app_settings_filepath_str = app_settings_filepath.to_str().unwrap();

    if !(Path::new(app_settings_filepath_str).exists()) {

        let file = OpenOptions::new().write(true)
                             .create_new(true)
                             .open(app_settings_filepath_str);
        match file {
          Err(error) => {
            error!("Unable to create empty app settings file {app_settings_filepath_str}: {error}", app_settings_filepath_str=app_settings_filepath_str, error=error);
            panic!();
          },
          Ok(mut file) => {
            debug!("Empty app settings file {app_settings_filepath_str} was created.", app_settings_filepath_str=app_settings_filepath_str);

            match file.write_all(APP_SETTINGS_DEFAULT_CONTENT.as_bytes()) {
              Err(error) => {
                error!("Unable to write default content to app settings file {app_settings_filepath_str}: {error}", app_settings_filepath_str=app_settings_filepath_str, error=error);
                panic!();
              },
              Ok(_) => {
                debug!("Wrote default content {default_content} to app settings file {app_settings_filepath_str}.", app_settings_filepath_str=app_settings_filepath_str, default_content=APP_SETTINGS_DEFAULT_CONTENT);
              }
            }
          }
        }
    }

    let mut settings = config::Config::default();
    settings
        // Add in `./Settings.toml`
        .merge(config::File::new(app_settings_filepath_str, config::FileFormat::Hjson))
        .unwrap()
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .merge(config::Environment::with_prefix("APP"))
        .unwrap();

    // Print out our settings (as a HashMap)
    println!(
        "{:?}",
        settings.try_into::<HashMap<String, String>>().unwrap()
    );
}
