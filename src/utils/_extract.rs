use std::path::{PathBuf};

use anyhow::anyhow;
use anyhow::Result as AnyResult;
use subprocess::{Popen, PopenConfig};
use thiserror::Error;

pub const __7Z_PATH: &str = "./lib7z.dll";

#[derive(Debug, Error)]
pub enum SevenZipError {
    #[error("Error from stderr")]
    FromStdErr(Popen),
}

#[derive(Debug, Clone)]
pub struct SevenZip {
    options: SevenZipOptions,
}

impl SevenZip {
    pub fn new(options: SevenZipOptions) -> Self {
        Self { options }
    }
}

impl SevenZip {
    pub async fn unzip(
        &self,
        path_to_archive: &PathBuf,
        target: &PathBuf,
        overwrite: bool,
        options: SevenZipAddOptions,
    ) -> AnyResult<Popen> {
        let mut args = vec![
            path_to_archive
                .to_str()
                .ok_or(anyhow!("Can't to str"))?
                .to_string(),
            format!("-o{}", target.to_str().ok_or(anyhow!("Can't to str"))?),
            "-r".to_string(),
        ];
        if overwrite {
            args.push("-aoa".to_string());
        } else {
            args.push("-aos".to_string());
        }

        self.call(
            "x",
            SevenZipAddOptions {
                switches: args,
                popen: options.popen,
            },
        )
            .await
    }

    pub async fn call(&self, subcommand: &str, options: SevenZipAddOptions) -> AnyResult<Popen> {
        let args = vec![subcommand.to_owned()];
        let mut args = [
            args,
            options.switches.to_owned(),
            self.options.switches.clone(),
        ]
            .concat();
        args.insert(0, self.options.path.clone());
        let prss = Popen::create(&args, options.popen)?;

        Ok(prss)
    }
}

#[derive(Debug, Clone)]
pub struct SevenZipOptions {
    pub path: String,
    pub switches: Vec<String>,
}

impl Default for SevenZipOptions {
    fn default() -> Self {
        Self {
            path: __7Z_PATH.to_string(),
            switches: vec![],
        }
    }
}

pub struct SevenZipAddOptions {
    pub switches: Vec<String>,
    pub popen: PopenConfig,
}

impl Default for SevenZipAddOptions {
    fn default() -> Self {
        Self {
            switches: vec![],
            popen: PopenConfig::default(),
        }
    }
}
