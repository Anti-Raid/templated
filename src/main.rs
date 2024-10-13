mod bundler;

use clap::Parser;
use std::io::Read;
use std::io::Write; // 0.8

/// Luau entrypoint
const ENTRYPOINT_LUA: &str = include_str!("entrypoint.luau");

/// Project name
const TEMPLATED_NAME: &str = env!("CARGO_PKG_NAME");

/// Version of templated
const TEMPLATED_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum TemplatedOptions {
    /// Input can be file or stdin, output can be file or stdout
    WrapFile,
    /// Bundle up a directory, input must be the path to a directory, output can be file or stdout
    BundleDir,
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
        TemplatedOptions::BundleDir => {
            println!("Reading directory {}", &args.input);
            let dir_files = read_input_directory(&args.input).expect("Failed to read input");

            let mut bundles = bundler::BundleList { files: Vec::new() };

            for (i, file) in dir_files.iter().enumerate() {
                println!("[{}/{}] {}", i + 1, dir_files.len(), file);

                let code = read_input(file).expect("Failed to read file");
                let ast = full_moon::parse_fallible(&code, full_moon::LuaVersion::luau());

                if !ast.errors().is_empty() {
                    let mut errors = Vec::new();

                    for error in ast.errors() {
                        errors.push(format!("{}", error));
                    }

                    panic!("Failed to parse file: {}", errors.join("\n"));
                }

                let ast = ast.into_ast();
                println!("AST-Str: {}", ast.to_string());

                let bundled_file = bundler::BundledFile {
                    file_path: file.to_string(),
                    ast,
                };

                bundles.files.push(bundled_file);
            }

            println!("Applying transformations");

            println!("SymbolMangle");
            bundles = bundler::apply_symbol_mangling(bundles);

            println!("ImportInline");

            bundles = bundler::apply_import_inling(bundles);

            for (i, file) in bundles.files.iter().enumerate() {
                println!("[{}/{}] {}", i + 1, bundles.files.len(), file.file_path);

                println!("AST-Str: {}", file.ast.to_string());
            }
        }
    }
}
