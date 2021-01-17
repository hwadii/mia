use std::process::{Command, Child, ExitStatus};

const PACKAGE_JSON: &str = r#"
        {
          "name": "async-pipe-angular",
          "version": "0.0.0",
          "scripts": {
            "ng": "ng",
            "start": "ng serve",
            "build": "ng build",
            "test": "ng test",
            "lint": "ng lint"
          },
          "private": true,
          "dependencies": {
            "@angular/animations": "~10.1.4",
            "@angular/common": "~10.1.4",
            "@angular/compiler": "~10.1.4",
            "@angular/core": "~10.1.4",
            "@angular/forms": "~10.1.4",
            "@angular/platform-browser": "~10.1.4",
            "@angular/platform-browser-dynamic": "~10.1.4",
            "@angular/router": "~10.1.4",
            "rxjs": "~6.6.0",
            "tslib": "^2.0.0",
            "zone.js": "~0.10.2"
          },
          "devDependencies": {
            "@angular-devkit/build-angular": "~0.1001.4",
            "@angular/cli": "~10.1.4",
            "@angular/compiler-cli": "~10.1.4",
            "@types/node": "^12.11.1",
            "ts-node": "~8.3.0",
            "tslint": "~6.1.0",
            "typescript": "~4.0.2"
          }
        }
    "#;

fn get_package_json() {
    // get package.json from root of project
}

fn get_scripts() -> json::JsonValue {
    let pkg = parse_pkg(PACKAGE_JSON).unwrap();
    let scripts = &pkg["scripts"];
    json::from(scripts.dump())
}

fn parse_pkg(pkg: &str) -> Option<json::JsonValue> {
    match json::parse(pkg) {
        Ok(p) => Some(p),
        Err(_) => {
            println!("Could not parse `package.json`");
            None
        },
    }
}

fn run_script(selected_script: &str) -> ExitStatus {
    let mut child = Command::new("npm")
        .arg("run")
        .arg(selected_script)
        .spawn()
        .expect("failed to spawn child");
    child.wait().expect("failed to wait on child")
}

fn main() {
    let scripts = get_scripts();
    let selected_script = "sdkaj";
    let out = run_script(selected_script);
}
