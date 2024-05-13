use std::fs;
use std::path::Path;
use structopt::StructOpt;
use handlebars::Handlebars;

#[derive(StructOpt, Debug)]
#[structopt(name = "vara")]
pub struct Cli {
    // define the cli options
    #[structopt(short = "n", long = "name")]
    pub name: String,

    #[structopt(short = "d", long = "description", default_value = "a new vara project")]
    pub description: String,
}

impl Cli {
    pub fn read_args() {
        let args = Cli::from_args();
        Self::create_structure(args);
    }

    pub fn create_structure(args: Cli) {
        let name = args.name;
        let description = args.description;

        println!("Creating project: {}", name);
        println!("Description: {}", description);

        // create the directories
        let base_path = Path::new(&name);
        fs::create_dir_all(base_path.join("src")).expect("Failed to create src directory");
        // fs::create_dir_all(base_path.join("io")).expect("Failed to create io directory");
        fs::create_dir_all(base_path.join("io/src")).expect("Failed to create io/src directory");
        // fs::create_dir_all(base_path.join("state")).expect("Failed to create state directory");
        fs::create_dir_all(base_path.join("state/src")).expect("Failed to create state/src directory");

        // Create a basic README
        fs::write(
            base_path.join("README.md"),
            format!("# {}\n\n{}", name, description)
        ).expect("Failed to create README.md");

        // create the necessary project files
        Self::create_toml_file(&name);
        Self::create_lib_rs_file(&name);
        Self::create_build_rs_file(&name);
    }

    fn create_lib_rs_file(name: &str) {
        // create main lib rs file
        let template_path = Path::new("src/templates/lib.rs.hbs");
        let output_path = Path::new(&name).join("src/lib.rs");
        let data = serde_json::json!({
            "project_name": &name
        });
        let _ = Self::apply_template(&template_path, &output_path, &data);

        // create state lib rs file
        let template_path = Path::new("src/templates/state/lib.rs.hbs");
        let output_path = Path::new(&name).join("state/src/lib.rs");
        let data = serde_json::json!({});
        let _ = Self::apply_template(&template_path, &output_path, &data);

        // create io lib rs file
        let template_path = Path::new("src/templates/io/lib.rs.hbs");
        let output_path = Path::new(&name).join("io/src/lib.rs");
        let data = serde_json::json!({});
        let _ = Self::apply_template(&template_path, &output_path, &data);
    }

    fn create_build_rs_file(name: &str) {
        // create main build rs file
        let template_path = Path::new("src/templates/build.rs.hbs");
        let output_path = Path::new(&name).join("build.rs");
        let data = serde_json::json!({});
        let _ = Self::apply_template(&template_path, &output_path, &data);

        // create state build rs file
        let template_path = Path::new("src/templates/state/build.rs.hbs");
        let output_path = Path::new(&name).join("state/build.rs");
        let data = serde_json::json!({});
        let _ = Self::apply_template(&template_path, &output_path, &data);
    }

    fn create_toml_file(name: &str) {
        // create cargo main toml file
        let template_path = Path::new("src/templates/Cargo.toml.hbs");
        let output_path = Path::new(&name).join("Cargo.toml");
        let data = serde_json::json!({
            "project_name": &name
        });
        let _ = Self::apply_template(&template_path, &output_path, &data);

        // create state cargo main toml file
        let template_path = Path::new("src/templates/state/Cargo.toml.hbs");
        let output_path = Path::new(&name).join("state/Cargo.toml");
        let data = serde_json::json!({});
        let _ = Self::apply_template(&template_path, &output_path, &data);

        // create cargo main toml file
        let template_path = Path::new("src/templates/io/Cargo.toml.hbs");
        let output_path = Path::new(&name).join("io/Cargo.toml");
        let data = serde_json::json!({});
        let _ = Self::apply_template(&template_path, &output_path, &data);
    }

    fn apply_template(
        template_path: &Path,
        output_path: &Path,
        data: &serde_json::Value,
    ) -> std::io::Result<()> {
        let template_name = "template";

        // render the template
        let mut handlebars = Handlebars::new();
        handlebars.register_template_file(template_name, template_path).unwrap();
    
        let output = handlebars.render(template_name, &data).unwrap();
    
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(output_path, output)?;
        Ok(())
    }
}