use clap::{Arg, ArgAction, ArgMatches, Command, value_parser};

#[derive(Debug)]
pub struct ScraperConfig {
    pub keyword: Vec<String>,
    pub output_file: bool,
    pub verbose: bool,
    pub status_code: Option<Vec<u16>>,
    pub file_format: Option<Vec<String>>
}

pub fn user_arg() -> ArgMatches {
    let matches = Command::new("Tor Site Scraper")
    .version(env!("CARGO_PKG_VERSION"))
    .about("Tor Site Scraper ")


    .arg(
            Arg::new("keywords")
                .short('S')
                .long("key")
                .value_delimiter(',')
                .required(true)
                .help("Comma-separated keywords to search for (e.g., -S forum,leaks)"),
        )


    .arg(
            Arg::new("output-file")
                .short('o')
                .long("out")
                .action(ArgAction::SetTrue)
                .help("Output file path to save results"),
        )


        
    .arg(
        Arg::new("verbose")
            .short('v')
            .long("verbose")
            .action(ArgAction::SetTrue) 
            .help("Enable verbose mode"),
        )
    
    
    .arg(
        Arg::new("status-code")
            .short('s')
            .long("status")
            .value_delimiter(',')
            .value_parser(value_parser!(u16)) 
            .help("Filter by HTTP status codes (e.g., -s 200,403,500)"),
        )
    
    
    .arg(
        Arg::new("file-format")
            .short('f')
            .long("format")
            .value_delimiter(',')
            .value_parser(["txt", "json", "csv"]) 
            .help("Output file format(s)"),
        ).get_matches();

    matches

}

pub fn arg_config() -> ScraperConfig {
    let mut ret = ScraperConfig { keyword: Vec::new(), output_file: false, verbose: false, status_code: None, file_format: None };
    let arg = user_arg();

    if let Some(key) = arg.get_many::<String>("keywords") {
        ret.keyword = key.cloned().collect();
    }

    if arg.get_flag("output-file") {
        ret.output_file = true
    }

    if arg.get_flag("verbose") {
        ret.verbose = true
    }

    if let Some(status) = arg.get_many::<u16>("status-code") {

        ret.status_code = Some(status.copied().collect())
    }

    if let Some(format) = arg.get_many::<String>("file-format") {
        ret.file_format = Some(format.cloned().collect())
    }

    ret

}

