use std::io::BufReader;
use std::{fs::File, path::PathBuf};

use clap::{Parser, Subcommand, ValueEnum};
use whitecosmos::backend::interpreter::Interpreter;
use whitecosmos::backend::ir_producer::IrProducer;
use whitecosmos::backend::runtime::Runtime;
use whitecosmos::core::handler::Handler;
use whitecosmos::core::handler_errors::{EngineError, UsageError};
use whitecosmos::frontend::classic_parser::{FAKE_WS_TOKENS, WS_TOKENS};
use whitecosmos::frontend::ir_parser::IrParser;
use whitecosmos::frontend::parser;
use whitecosmos::frontend::{classic_parser::ClassicParser, classic_parser::ParsedLanguage};

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
    #[value(alias = "ir")]
    Ir
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
    let parser_type = find_parser_type(&cli)?;

    let runtime_type = cli.backend.unwrap_or(BackendType::Direct);

    let path = cli.filename.unwrap();
    let file = File::open(path)?;
    let reader = Box::new(BufReader::new(file));

    let parser: Box<dyn parser::Parser> = match parser_type {
        FrontendType::WhiteSpace => Box::new(ClassicParser::new(reader, WS_TOKENS)),
        FrontendType::FakeWhiteSpace => Box::new(ClassicParser::new(reader, FAKE_WS_TOKENS)),
        FrontendType::BracketWhiteSpace => Box::new(ClassicParser::new(
            reader,
            ParsedLanguage::WrittenWhitespace,
        )),
        FrontendType::IrWhiteSpace => Box::new(IrParser::new(reader)),
    };

    let runtime: Box<dyn Runtime> = match runtime_type {
        BackendType::Direct => Box::new(Interpreter::new(
            Box::new(std::io::stdin()),
            Box::new(std::io::stdout()),
        )),
        // TODO write in something else that stdout
        BackendType::Ir => Box::new(IrProducer::new(Box::new(std::io::stdout())))
    };

    let mut handler = Handler::new(parser, runtime);

    handler.run()
}

fn execute_with_subcommand(cli: Cli) -> Result<(), UsageError> {
    // this will be interesting
    todo!()
}

fn find_parser_type(cli: &Cli) -> Result<FrontendType, UsageError> {
    if let Some(argument_provided_parser) = cli.frontend {
        Ok(argument_provided_parser)
    } else if let Some(extension) = cli.filename.clone().unwrap().extension() {
        match extension.to_str() {
            Some("ws") => Ok(FrontendType::WhiteSpace),
            Some("fws") => Ok(FrontendType::FakeWhiteSpace),
            Some("bws") => Ok(FrontendType::BracketWhiteSpace),
            Some("iws") => Ok(FrontendType::IrWhiteSpace),
            _ => Err(UsageError::UnsupportedFileExtension),
        }
    } else {
        return Err(UsageError::UnspecifiedParserType);
    }
}
