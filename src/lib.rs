use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use std::fs;
use std::process::{Child, Command};

#[derive(Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub scripts: serde_json::Map<String, serde_json::Value>,
    pub private: bool,
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

    pub fn run(&self, script_name: &str) -> Result<Child, Box<dyn Error>> {
        let program_name = match self.package_manager {
            PackageManager::Npm => "npm",
            PackageManager::Yarn => "yarn",
        };
        let chosen_script = if self.package.scripts.contains_key(script_name) {
            Ok(script_name)
        } else {
            Err("No script exists with the given name")
        };
        let child = Command::new(program_name)
            .arg("run")
            .arg(chosen_script.map_err(|err| err)?)
            .spawn()?;
        Ok(child)
    }
}

pub struct Config {
    pub contents: String,
    pub package_manager: PackageManager,
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
