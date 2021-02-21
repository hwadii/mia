use serde::{Deserialize, Serialize};
use serde_json;
use skim::prelude::*;
use std::error::Error;
use std::fs;
use std::process::{Child, Command};

type Scripts = serde_json::Map<String, serde_json::Value>;

#[derive(Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub scripts: Scripts,
    pub private: bool,
}

struct Script {
    name: String,
    description: String,
}

impl SkimItem for Script {
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.description)
    }

    fn output(&self) -> Cow<str> {
        Cow::Borrowed(&self.name)
    }
}

pub struct Instance {
    pub package: Package,
    pub package_manager: PackageManager,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PackageManager {
    Npm,
    Yarn,
}

impl Instance {
    pub fn new(config: Config) -> Result<Self, Box<dyn Error>> {
        let package: Package = serde_json::from_str(&config.contents)?;
        Ok(Self {
            package,
            package_manager: config.package_manager,
        })
    }

    pub fn run(&self) -> Result<Child, Box<dyn Error>> {
        let program_name = match self.package_manager {
            PackageManager::Npm => "npm",
            PackageManager::Yarn => "yarn",
        };
        let script_name = self.select_script();
        let child = Command::new(program_name)
            .arg("run")
            .arg(script_name)
            .spawn()?;
        Ok(child)
    }

    fn select_script(&self) -> String {
        let options = SkimOptionsBuilder::default()
            .height(Some("50%"))
            .build()
            .unwrap();

        let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();
        for (name, description) in &self.package.scripts {
            let _ = tx.send(Arc::new(Script {
                name: name.to_string(),
                description: format!("{} – {}", name, description),
            }));
        }
        drop(tx);

        let selected_items = Skim::run_with(&options, Some(rx))
            .map(|out| out.selected_items)
            .unwrap_or_else(|| Vec::new());

        selected_items[0].output().to_string()
    }
}

pub struct Config {
    contents: String,
    package_manager: PackageManager,
}

impl Config {
    pub fn new(filename: &str, package_manager: &str) -> Result<Self, Box<dyn Error>> {
        let contents = fs::read_to_string(filename)?;
        let package_manager: Result<PackageManager, &str> = match package_manager {
            "yarn" => Ok(PackageManager::Yarn),
            "npm" => Ok(PackageManager::Npm),
            _ => Err("Unknown package manager: expected \"yarn\" or \"npm\""),
        };
        Ok(Self {
            contents,
            package_manager: package_manager.map_err(|err| err)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
