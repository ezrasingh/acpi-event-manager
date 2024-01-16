use std::num::ParseIntError;
use std::path::{Path, PathBuf};

use crate::acpi;
use crate::config::Config;

#[derive(Debug, Clone)]
pub struct BacklightConfig {
    pub brightness: i16,
    max: i16,
    min: i16,
}

impl BacklightConfig {
    // ? determine current backlight config from system files
    pub fn from_config(
        config: &Config,
        sys_class_path: Option<PathBuf>,
    ) -> Result<BacklightConfig, ParseIntError> {
        let base_dir = sys_class_path.unwrap_or("/sys/class/backlight".into());
        let device_config_dir = Path::new(&base_dir).join(&config.acpi_device);
        let max_brightness =
            acpi::read_acpi_config_value(device_config_dir.join("max_brightness"))?;
        let current_brightness =
            acpi::read_acpi_config_value(device_config_dir.join("brightness"))?;
        Ok(BacklightConfig {
            max: max_brightness,
            brightness: current_brightness,
            min: 1,
        })
    }

    // ? calulate current brightness as ratio
    pub fn percentage(&self) -> String {
        let ratio = Into::<f32>::into(self.brightness / self.max);
        format!("{:.3}", ratio)
    }

    // ? modify brigtness value
    pub fn change_brightness(&mut self, increment: i16) {
        let value = self.brightness + increment;
        if value > self.max {
            self.brightness = self.max
        } else if value <= 0 {
            self.brightness = self.min
        } else {
            self.brightness = value
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::acpi;
    use crate::backlight::BacklightConfig;
    use crate::config::Config;
    use std::path::PathBuf;

    struct Helpers {
        fixtures_path: PathBuf,
        acpi_device_path: PathBuf,
        config: Config,
    }

    // ? set intial contents of mock acpi config files
    fn prepare() -> Result<Helpers, std::io::Error> {
        let cwd = std::env::current_dir().unwrap();
        let fixtures_path = cwd.join("fixtures");
        let config = Config {
            acpi_device: "acpi_device".into(),
            xrandr_display: "xrandr_display".into(),
            brightness_increment: 11,
            acpi_events: acpi::AcpiEventsConfig {
                brightness_up: "brightness_up".into(),
                brightness_down: "brightness_down".into(),
            },
        };
        let helpers = Helpers {
            fixtures_path: fixtures_path.clone(),
            config,
            acpi_device_path: fixtures_path.clone().join("acpi_device"),
        };
        std::fs::write(helpers.acpi_device_path.join("brightness"), "27182")?;
        std::fs::write(helpers.acpi_device_path.join("max_brightness"), "31415")?;

        Ok(helpers)
    }

    // ? restores state of mock acpi config files
    fn cleanup() -> Result<(), std::io::Error> {
        let cwd = std::env::current_dir().unwrap();
        let fixtures_path = cwd.join("fixtures");
        let acpi_device_dir = fixtures_path.join("acpi_device");
        std::fs::write(acpi_device_dir.join("brightness"), "27182")?;
        std::fs::write(acpi_device_dir.join("max_brightness"), "31415")?;
        Ok(())
    }

    #[test]
    fn create_backlight_config() {
        let helpers = prepare().unwrap();
        let backlight =
            BacklightConfig::from_config(&helpers.config, Some(helpers.fixtures_path)).unwrap();

        assert_eq!(backlight.brightness, 27182);
        assert_eq!(backlight.max, 31415);

        cleanup().unwrap();
    }

    #[test]
    fn validate_percentage() {
        let helpers = prepare().unwrap();
        let mut backlight =
            BacklightConfig::from_config(&helpers.config, Some(helpers.fixtures_path)).unwrap();

        let expected = format!(
            "{:.3}",
            Into::<f32>::into(backlight.brightness / backlight.max)
        );
        assert_eq!(backlight.percentage(), expected);

        backlight.brightness = backlight.max;
        assert_eq!(backlight.percentage(), "1.000");

        cleanup().unwrap();
    }

    #[test]
    fn increase_brightness() {
        let helpers = prepare().unwrap();
        let mut backlight =
            BacklightConfig::from_config(&helpers.config, Some(helpers.fixtures_path)).unwrap();

        let expected = backlight.brightness + helpers.config.brightness_increment;
        backlight.change_brightness(helpers.config.brightness_increment);
        assert_eq!(backlight.brightness, expected);

        cleanup().unwrap();
    }

    #[test]
    fn overflow_brightness() {
        let helpers = prepare().unwrap();
        let mut backlight =
            BacklightConfig::from_config(&helpers.config, Some(helpers.fixtures_path)).unwrap();

        backlight.brightness = backlight.max;
        backlight.change_brightness(helpers.config.brightness_increment);
        assert_eq!(backlight.brightness, backlight.max);

        backlight.brightness = backlight.max + 123;
        backlight.change_brightness(helpers.config.brightness_increment);
        assert_eq!(backlight.brightness, backlight.max);

        cleanup().unwrap();
    }

    #[test]
    fn decrease_brightness() {
        let helpers = prepare().unwrap();
        let mut backlight =
            BacklightConfig::from_config(&helpers.config, Some(helpers.fixtures_path)).unwrap();

        let expected = backlight.brightness - helpers.config.brightness_increment;
        backlight.change_brightness(-helpers.config.brightness_increment);
        assert_eq!(backlight.brightness, expected);

        cleanup().unwrap();
    }

    #[test]
    fn underflow_brightness() {
        let helpers = prepare().unwrap();
        let mut backlight =
            BacklightConfig::from_config(&helpers.config, Some(helpers.fixtures_path)).unwrap();

        backlight.brightness = backlight.min;
        backlight.change_brightness(-helpers.config.brightness_increment);
        assert_eq!(backlight.brightness, backlight.min);

        backlight.brightness = 0;
        backlight.change_brightness(-helpers.config.brightness_increment);
        assert_eq!(backlight.brightness, backlight.min);

        backlight.brightness = -123;
        backlight.change_brightness(-helpers.config.brightness_increment);
        assert_eq!(backlight.brightness, backlight.min);

        cleanup().unwrap();
    }
}
