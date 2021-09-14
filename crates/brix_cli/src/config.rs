// Copyright (c) 2021 Ethan Lerner, Caleb Cushing, and the Brix contributors
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use clap::ArgMatches;
use std::borrow::Cow;
use std::fmt::{self, Display, Formatter};

use crate::app;

pub struct Config {
    pub language: String,
    pub config_name: String,
    pub project: String,
    pub module: String,

    pub config_dir: String,
    // TODO: Add flags
    pub raw_matches: ArgMatches<'static>,
}

impl Config {
    /// Parses matches and sets into config
    #[rustfmt::skip]
    pub fn new(matches: ArgMatches<'static>) -> Self {
        let language = matches.value_of_lossy(app::LANGUAGE).unwrap().to_string();
        let config_name = matches.value_of_lossy(app::CONFIG_NAME).unwrap().to_string();
        let project = matches.value_of_lossy(app::PROJECT).unwrap().to_string();
        let module = matches.value_of_lossy(app::MODULE).unwrap().to_string();

        let config_dir = matches.value_of_lossy(app::CONFIG_DIR).unwrap_or(Cow::from(".config/brix")).to_string();

        Self {
            raw_matches: matches,
            language,
            config_name,
            project,
            config_dir,
            module,
        }
    }
}

impl Display for Config {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(
            formatter,
            "[LANGUAGE: {}, CONFIG_NAME: {}, PROJECT: {}, MODULE: {}, CONFIG_DIR: {}]",
            self.language, self.config_name, self.project, self.module, self.config_dir
        )
    }
}
