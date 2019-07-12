#![feature(bind_by_move_pattern_guards)]
use clap::{App, Arg};
use log::{info, trace, LevelFilter};

mod logging;
mod runtime;
mod scanner;
mod tokenize;
mod utils;

static mut LOGGER: logging::SimpleLogger = logging::SimpleLogger {
    level: log::Level::Info,
};

fn main() {
    unsafe {
        log::set_logger(&LOGGER)
            .map(|()| log::set_max_level(LevelFilter::Trace))
            .expect("Could not initialise logging");
    }

    let matches = App::new("Just - JavaScript in Rust")
        .version("1.0")
        .author("Joseph B. <joseph@josephbanks.me>")
        .about("Executes JavaScript with the power of Rust")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .get_matches();

    let level = match matches.occurrences_of("verbosity") {
        0 => log::Level::Error,
        1 => log::Level::Warn,
        2 => log::Level::Info,
        3 => log::Level::Trace,
        _ => log::Level::Error,
    };

    unsafe {
        LOGGER.set_level(level);
    }

    info!("Opening {}", matches.value_of("INPUT").unwrap());

    let content = utils::read_file(matches.value_of("INPUT").unwrap());

    info!("Read file");
    trace!("{}", content);

    let scanner = scanner::Scanner::new(content);

    info!("Loaded code into scanner");

    let mut tokenizer = tokenize::Tokenizer::new();

    info!("Adding tokens into tokenizer");

    for text in scanner.text {
        tokenizer.add_token(text);
    }

    for token in &tokenizer.tokens {
        trace!("{:?}", token);
    }

    let mut executor = runtime::Executor::new(tokenizer.tokens.clone());

    executor.execute();
}
