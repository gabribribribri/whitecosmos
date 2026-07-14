use std::io::{BufReader, Read, Write};
use std::{fs::File, path::PathBuf};

use clap::{Parser, Subcommand, ValueEnum};
use whitecosmos::{
    backend::{interpreter::Interpreter, ir_producer::IrProducer, runtime::Runtime},
    core::{
        handler::Handler,
        handler_errors::{EngineError, UsageError},
    },
    frontend::{
        classic_parser::{ClassicParser, FAKE_WS_TOKENS, ParsedLanguage, WS_TOKENS},
        ir_parser::IrParser,
        parser,
    },
};

#[derive(Parser)]
#[command(name = "whitecosmos")]
#[command(about = "A whitespace interpreter written in Rust", long_about = None)]
#[command(args_conflicts_with_subcommands = true)]
struct Cli {
    #[arg(value_name = "FILE")]
    filename: Option<PathBuf>,

    #[arg(short, long, value_enum)]
    frontend: Option<FrontendType>,

    #[arg(short, long, value_enum)]
    backend: Option<BackendType>,

    #[command(subcommand)]
    subcommand: Option<WhitecosmosSubcommand>,

    #[arg(short, long, value_enum)]
    input: Option<PathBuf>,

    #[arg(short, long, value_enum)]
    output: Option<PathBuf>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum FrontendType {
    #[value(alias = "ws")]
    WhiteSpace,
    #[value(alias = "fws")]
    FakeWhiteSpace,
    #[value(alias = "bws")]
    BracketWhiteSpace,
    #[value(alias = "iws")]
    IrWhiteSpace,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum BackendType {
    Direct,
    // #[value(alias = "ir")]
    Ir,
}

#[derive(Subcommand)]
enum WhitecosmosSubcommand {}

fn main() {
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
        println!("{}", EngineError::usage(UsageError::MissingFilename));
    }
}

fn execute_no_subcommand(cli: Cli) -> Result<(), EngineError> {
    // Figure out which Parser and Runtime to use
    let frontend_type = find_parser_type(&cli)?;

    let backend_type = cli.backend.unwrap_or(BackendType::Direct);

    let path = cli.filename.unwrap();
    let file = File::open(path)?;
    let reader = Box::new(BufReader::new(file));

    let input: Box<dyn Read> = match cli.input {
        Some(path) => Box::new(File::open(path)?),
        None => Box::new(std::io::stdin()),
    };

    let output: Box<dyn Write> = match cli.output {
        Some(path) => Box::new(File::create(path)?),
        None => Box::new(std::io::stdout()),
    };

    let parser: Box<dyn parser::Parser> = match frontend_type {
        FrontendType::WhiteSpace => Box::new(ClassicParser::new(reader, WS_TOKENS)),
        FrontendType::FakeWhiteSpace => Box::new(ClassicParser::new(reader, FAKE_WS_TOKENS)),
        FrontendType::BracketWhiteSpace => Box::new(ClassicParser::new(
            reader,
            ParsedLanguage::BracketWhitespace,
        )),
        FrontendType::IrWhiteSpace => Box::new(IrParser::new(reader)),
    };

    let runtime: Box<dyn Runtime> = match backend_type {
        BackendType::Direct => Box::new(Interpreter::new(input, output)),
        BackendType::Ir => Box::new(IrProducer::new(output)),
    };

    let mut handler = Handler::new(parser, runtime);
    handler.run()
}

#[allow(unused)] // TODO remove
fn execute_with_subcommand(cli: Cli) -> Result<(), UsageError> {
    // this will be interesting
    todo!()
}

fn find_parser_type(cli: &Cli) -> Result<FrontendType, UsageError> {
    if let Some(argument_provided_parser) = cli.frontend {
        Ok(argument_provided_parser)
    } else if let Some(extension) = cli.filename.clone().unwrap().extension() {
        // maybe unwrap here is not a good idea ?
        match extension.to_str().unwrap() {
            "ws" => Ok(FrontendType::WhiteSpace),
            "fws" => Ok(FrontendType::FakeWhiteSpace),
            "bws" => Ok(FrontendType::BracketWhiteSpace),
            "iws" => Ok(FrontendType::IrWhiteSpace),
            _ => Err(UsageError::UnsupportedFileExtension),
        }
    } else {
        return Err(UsageError::UnspecifiedParserType);
    }
}
