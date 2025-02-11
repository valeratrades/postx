use color_eyre::eyre::{Result, WrapErr as _};
use v_utils::{io::ExpandedPath, macros::MyConfigPrimitives};

#[derive(Clone, Debug, Default, MyConfigPrimitives)]
pub struct AppConfig {
	pub twitter: TwitterConf,
}

#[derive(Clone, Debug, Default, MyConfigPrimitives)]
pub struct TwitterConf {
	pub username: String,
	pub password: String,
}

impl AppConfig {
	pub fn read(path: Option<ExpandedPath>) -> Result<Self> {
		let app_name = env!("CARGO_PKG_NAME");
		let xdg_dirs = xdg::BaseDirectories::with_prefix(app_name).unwrap();
		let xdg_conf_dir = xdg_dirs.get_config_home().parent().unwrap().display().to_string();

		let locations = [
			format!("{xdg_conf_dir}/{app_name}"),
			format!("{xdg_conf_dir}/{app_name}/config"), //
		];

		let mut builder = config::Config::builder().add_source(config::Environment::default());

		match path {
			Some(path) => {
				let builder = builder.add_source(config::File::with_name(&path.to_string()).required(true));
				Ok(builder.build()?.try_deserialize()?)
			}
			None => {
				for location in locations.iter() {
					builder = builder.add_source(config::File::with_name(location).required(false));
				}
				let raw: config::Config = builder.build()?;

				raw.try_deserialize().wrap_err("Config file does not exist or is invalid")
			}
		}
	}
}
