use clap::Parser;
use std::io::Read;
use std::io::Write; // 0.8

/// Luau entrypoint
const ENTRYPOINT_LUA: &str = include_str!("entrypoint.luau");

// Darklua configuration
const DARKLUA_CONFIG: &str = include_str!("darklua-config.json");

/// Project name
const TEMPLATED_NAME: &str = env!("CARGO_PKG_NAME");

/// Version of templated
const TEMPLATED_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum TemplatedOptions {
    /// Input can be file or stdin, output can be file or stdout
    WrapFile,
    /// Bundle up a single file using Anti-Raid compatible settings, input can be file or stdin, output can be file ONLY
    BundleFile,
}

/// Set of utilities to handling AntiRaid Luau templating
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Operation to perform
    #[arg(value_enum)]
    operation: TemplatedOptions,

    /// Input, valid parameter depends on operation
    #[arg(short, long)]
    input: String,

    /// Output, valid parameter depends on operation
    #[arg(short, long)]
    output: String,
}

/// Read input loc as a file or stdin, returning the contents
fn read_input(input_loc: &str) -> Result<String, std::io::Error> {
    if input_loc == "-" || input_loc == "stdin" {
        // Read from stdin until EOF
        let mut buffer = String::new();

        std::io::stdin().read_to_string(&mut buffer)?;

        let buffer = buffer.trim().to_string();

        Ok(buffer)
    } else {
        // Read from file
        let mut file = std::fs::File::open(input_loc)?;

        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        Ok(buffer)
    }
}

#[allow(dead_code)]
/// Read input loc as a directory, returning a set of paths
fn read_input_directory(input_loc: &str) -> Result<Vec<String>, std::io::Error> {
    let mut files = Vec::new();

    for entry in std::fs::read_dir(input_loc)? {
        let entry = entry?;

        let path = entry.path();

        if path.is_file() {
            files.push(path.to_str().unwrap().to_string());
        } else if path.is_dir() {
            let mut dir_files = read_input_directory(path.to_str().unwrap())?;

            files.append(&mut dir_files);
        }
    }

    Ok(files)
}

fn write_output(output_loc: &str, output: &str) -> Result<(), std::io::Error> {
    if output_loc == "-" || output_loc == "stdout" {
        // Write to stdout
        println!("{}", output);
    } else {
        // Write to file
        let mut file = std::fs::File::create(output_loc)?;

        file.write_all(output.as_bytes())?;
    }

    Ok(())
}

fn main() {
    let args = Args::parse();

    match args.operation {
        TemplatedOptions::WrapFile => {
            let input = read_input(&args.input).expect("Failed to read input");

            let mut tera = tera::Tera::default();
            let mut context = tera::Context::new();

            // Insert fields
            context
                .insert("body", &input)
                .expect("Failed to insert input");

            context
                .insert("proj_name", TEMPLATED_NAME)
                .expect("Failed to insert name");

            context
                .insert("proj_version", TEMPLATED_VERSION)
                .expect("Failed to insert version");

            let output = tera
                .render_str(ENTRYPOINT_LUA, &context)
                .expect("Failed to render template");

            write_output(&args.output, &output).expect("Failed to write output");
        }
        TemplatedOptions::BundleFile => {
            let input = read_input(&args.input).expect("Failed to read input");

            // Write input to temp file for darklua
            let (in_temp_file, out_path) = {
                if input == "stdin" {
                    let in_file_name = format!("templated-{}.in.luau", rand::random::<u64>());
                    let in_temp_file = std::env::temp_dir().join(in_file_name.clone());

                    let mut file =
                        std::fs::File::create(&in_temp_file).expect("Failed to create temp file");

                    file.write_all(input.as_bytes())
                        .expect("Failed to write to temp file");

                    // Create output temp file
                    let out_file_name = in_file_name.replace(".in.", ".out.");
                    let out_temp_file = std::env::temp_dir().join(out_file_name);

                    // Create file with empty contents
                    let mut file = std::fs::File::create(&out_temp_file)
                        .expect("Failed to create output temp file");

                    file.write_all(b"")
                        .expect("Failed to write to output temp file");

                    (in_temp_file, out_temp_file)
                } else {
                    let in_path = std::path::PathBuf::from(args.input.clone());
                    let out_path = std::path::PathBuf::from(args.output.clone());
                    (in_path.clone(), out_path)
                }
            };

            // Write darklua config to temp file
            let darklua_config_file = std::env::temp_dir().join("darklua-config.json");

            let mut file = std::fs::File::create(&darklua_config_file)
                .expect("Failed to create darklua config file");

            file.write_all(DARKLUA_CONFIG.as_bytes())
                .expect("Failed to write darklua config file");

            // Run darklua
            let resources = darklua_core::Resources::from_file_system();

            let process_options = darklua_core::Options::new(&in_temp_file)
                .with_output(&out_path)
                .with_generator_override(darklua_core::GeneratorParameters::Readable {
                    column_span: 4,
                })
                .fail_fast()
                .with_configuration_at(&darklua_config_file);

            darklua_core::process(&resources, process_options)
                .result()
                .expect("Failed to run darklua");
        }
    }
}
