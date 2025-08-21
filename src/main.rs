use apollo_compiler::executable::ExecutableDocument;
use apollo_compiler::schema::Schema;
use apollo_compiler::validation::Valid;
use apollo_federation::schema::ValidFederationSchema;
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};

use apollo_federation::correctness::{CorrectnessError, compare_operations};

#[derive(Parser)]
struct Args {
    #[clap(long, short, default_value = "false")]
    verbose: bool,
    schema: PathBuf,
    first: PathBuf,
    second: PathBuf,
}

fn main() {
    let args = Args::parse();

    let schema = load_schema(&args.schema);
    let first = load_operation(&args.first, &schema);
    let second = load_operation(&args.second, &schema);

    // Check if `first` is a subset of `second`
    let le = compare_operations(&schema, &first, &second);

    // Check if `second` is a subset of `first`
    let ge = compare_operations(&schema, &second, &first);

    match (le, ge) {
        (Err(CorrectnessError::FederationError(e)), _) => {
            eprintln!("Error checking first ⊂ second: {e}");
        }
        (_, Err(CorrectnessError::FederationError(e))) => {
            eprintln!("Error checking second ⊂ first: {e}");
        }
        (Ok(()), Ok(())) => println!("≣ (Operations are equivalent)"),
        (Ok(()), Err(CorrectnessError::ComparisonError(e))) => {
            println!("⊂ (First operation is a subset of second)");
            if args.verbose {
                print_reason("⊅", &e.to_string());
            }
        }
        (Err(CorrectnessError::ComparisonError(e)), Ok(())) => {
            println!("⊃ (Second operation is a subset of first)");
            if args.verbose {
                print_reason("⊄", &e.to_string());
            }
        }
        (Err(CorrectnessError::ComparisonError(_)), Err(CorrectnessError::ComparisonError(_))) => {
            println!("⊄ / ⊅ (Operations are not comparable)");
        }
    }
}

fn load_schema(path: &Path) -> ValidFederationSchema {
    let doc = fs::read_to_string(path).expect("schema file accessible");
    let schema = Schema::parse_and_validate(&doc, path).expect("valid schema expected");
    ValidFederationSchema::new(schema).expect("valid federation schema expected")
}

fn load_operation(path: &Path, schema: &ValidFederationSchema) -> Valid<ExecutableDocument> {
    let doc = fs::read_to_string(path).expect("operation file accessible");
    ExecutableDocument::parse_and_validate(schema.schema(), &doc, path)
        .expect("valid operation expected")
}

fn print_reason(symbol: &str, reason: &str) {
    println!();
    println!("Reason for {symbol}:\n{reason}");
}
