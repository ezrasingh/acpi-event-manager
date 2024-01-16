use clap::Parser;
use std::path::Path;

pub mod acpi;
pub mod backlight;
pub mod config;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Installs acpi events and reloads acpi daemon
    #[clap(long)]
    configure: bool,

    /// Action from acpi event
    #[clap(short, long)]
    action: Option<acpi::AcpiEventAction>,

    /// Absolute path to config file
    #[arg(short, long, default_value = "config.toml")]
    config_file: String,

    /// Log config details
    #[clap(short, long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();
    let config = config::Config::new(&args.config_file);
    let mut backlight = backlight::BacklightConfig::from_config(&config, None).unwrap();
    if args.debug {
        println!("{:?}", &args);
        println!("{:?}", &config);
        println!("{:?}", &backlight);
        println!("{}%", &backlight.percentage());
    }
    if args.action.is_some() {
        config::sudo_check();
        match args.action {
            Some(acpi::AcpiEventAction::BrightnessUp) => {
                backlight.change_brightness(config.brightness_increment);
                acpi::save(config, backlight);
            }
            Some(acpi::AcpiEventAction::BrightnessDown) => {
                backlight.change_brightness(-config.brightness_increment);
                acpi::save(config, backlight);
            }
            None => {}
        }
    }
    if args.configure {
        let config_path = Path::new(&args.config_file);
        let config = config::Config::new(&args.config_file);
        config
            .apply_config(config_path)
            .expect("Could not apply ACPI config");
    }
}
