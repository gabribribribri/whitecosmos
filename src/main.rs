use std::io::BufReader;
use std::{fs::File, path::PathBuf};

use clap::{Parser, Subcommand, ValueEnum};
use whitecosmos::frontend::{classic_parser::ClassicParser, classic_parser::TokenValues};
use whitecosmos::backend::interpreter::Interpreter;
use whitecosmos::core::handler::Handler;
use whitecosmos::core::handler_errors::{EngineError, UsageError};

#[derive(Parser)]
#[command(name = "whitecosmos")]
#[command(about = "A whitespace interpreter written in Rust", long_about = None)]
#[command(args_conflicts_with_subcommands = true)]
struct Cli {
    #[arg(value_name = "FILE")]
    filename: Option<PathBuf>,

    #[arg(short, long, value_enum)]
    parser: Option<ParserType>,

    #[arg(short, long, value_enum)]
    runtime: Option<RuntimeType>,

    #[command(subcommand)]
    subcommand: Option<WhitecosmosSubcommand>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ParserType {
    WhiteSpace,
    FakeWhiteSpace,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum RuntimeType {
    Direct,
}

#[derive(Subcommand)]
enum WhitecosmosSubcommand {}

fn main() {
    // let path: String = std::env::args().nth(1).unwrap();

    // let file = File::open(path).unwrap();
    // let reader = Box::new(io::BufReader::new(file));
    // let tokens = classic_parser::TokenValues { lf: b'l', tab: b't', space: b's'};
    // let parser = classic_parser::WSParser::new(reader, tokens);

    // let runtime = direct_runtime::DirectRuntime::new(Box::new(std::io::stdout()));

    // let mut handler = handler::Handler::new(parser, runtime);
    // handler.run();

    let cli = Cli::parse();

    if let Some(_) = cli.filename {
        // SCENARIO 1 : No subcommand
        match execute_no_subcommand(cli) {
            Ok(()) => (),
            Err(e) => {
                println!("{e}");
                std::process::exit(1)
            }
        }
    } else if let Some(_) = cli.subcommand {
        // SCENARIO 2 : With subcommand
        match execute_with_subcommand(cli) {
            Ok(()) => (),
            Err(e) => {
                println!("{e}");
                std::process::exit(1)
            }
        }
    } else {
        // SCENARIO 3 : nothing, so error
        todo!() // figure out how to error properly
    }
}

fn execute_no_subcommand(cli: Cli) -> Result<(), EngineError> {
    // Figure out which Parser and Runtime to use
    let parser_type = find_parser_type(&cli)?;

    let runtime_type = cli.runtime.unwrap_or(RuntimeType::Direct);

    let path = cli.filename.unwrap();
    let file = File::open(path)?;
    let reader = Box::new(BufReader::new(file));

    let parser = match parser_type {
        ParserType::WhiteSpace => {
            let tokens = TokenValues {
                lf: b'\n',
                tab: b'\t',
                space: b' ',
            };
            Box::new(ClassicParser::new(reader, tokens))
        }
        ParserType::FakeWhiteSpace => {
            let tokens = TokenValues {
                lf: b'l',
                tab: b't',
                space: b's',
            };
            Box::new(ClassicParser::new(reader, tokens))
        }
    };

    let runtime = match runtime_type {
        RuntimeType::Direct => Box::new(Interpreter::new(Box::new(std::io::stdout()))),
    };

    let mut handler = Handler::new(parser, runtime);

    handler.run()
}

fn execute_with_subcommand(cli: Cli) -> Result<(), UsageError> {
    // this will be interesting
    todo!()
}

fn find_parser_type(cli: &Cli) -> Result<ParserType, UsageError> {
    if let Some(argument_provided_parser) = cli.parser {
        Ok(argument_provided_parser)
    } else if let Some(extension) = cli.filename.clone().unwrap().extension() {
        if extension == "ws" {
            Ok(ParserType::WhiteSpace)
        } else if extension == "fws" {
            Ok(ParserType::FakeWhiteSpace)
        } else {
            Err(UsageError::UnsupportedFileExtension)
        }
    } else {
        return Err(UsageError::UnspecifiedParserType);
    }
}
