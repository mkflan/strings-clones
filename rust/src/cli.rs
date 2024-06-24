use crate::config::{Config, ConfigBuilder};

/// Parse the CLI arguments to construct the desired configuration for this execution of the program.
pub fn parse_cli_args(args: Vec<String>) -> Config {
    let mut args = args.into_iter().peekable();
    let mut config_builder = ConfigBuilder::default();

    for arg in args {
        // This argument is a flag.
        if arg.starts_with('-') {
        } else {
        }
    }

    config_builder.build()
}
