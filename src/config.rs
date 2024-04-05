use clap::{App, Arg};
pub struct Config {
    pub tps: u8,
    pub url: String,
    pub duration: u32,
    pub nonce: u32,
    pub setup_required: bool,
}

pub fn get_config() -> Result<Config, Box<dyn std::error::Error>> {
    let matches = App::new("Load tester")
        .version("0.1.0")
        .author("Uday <uday@zkx.fi>")
        .about("Load testing tool for substrate")
        .arg(
            Arg::with_name("tps")
                .short("t")
                .long("tps")
                .help("Sets the number of transactions per second (u8) / default 10")
                .takes_value(true)
                .validator(|val| match val.parse::<u8>().is_ok() {
                    true => Ok(()),
                    _ => Err(String::from("unexpected value of tps")),
                })
                .default_value("10")
                .required(false)
                .conflicts_with("setup_not_required"),
        )
        .arg(
            Arg::with_name("url")
                .short("u")
                .long("url")
                .help("Sets the URL to connect to - default ws://127.0.0.1:9944")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("nonce")
                .short("n")
                .long("nonce")
                .help("Initial nonce for execute_trade caller accounts / default 0")
                .takes_value(true)
                .validator(|val| match val.parse::<u32>().is_ok() {
                    true => Ok(()),
                    _ => Err(String::from("unexpected value of duration")),
                })
                .default_value("0")
                .required(false),
        )
        .arg(
            Arg::with_name("duration")
                .short("d")
                .long("duration")
                .help("Sets the approx. duration for sending tps in seconds (u32) / default 20 s")
                .takes_value(true)
                .validator(|val| match val.parse::<u32>().is_ok() {
                    true => Ok(()),
                    _ => Err(String::from("unexpected value of duration")),
                })
                .default_value("20")
                .required(false),
        )
        .arg(
            Arg::with_name("setup_not_required")
                .short("s")
                .long("setup-not-required")
                .help("Indicates setup is not required - default setup required")
                .takes_value(false)
                .conflicts_with("tps"),
        )
        .get_matches();

    let tps = matches
        .value_of("tps")
        .map(|val| val.parse::<u8>().unwrap())
        .unwrap();
    let url = matches
        .value_of("url")
        .unwrap_or("ws://127.0.0.1:9944")
        .to_string();
    let duration = matches
        .value_of("duration")
        .map(|val| val.parse::<u32>().unwrap())
        .unwrap();
    let nonce = matches
        .value_of("nonce")
        .map(|val| val.parse::<u32>().unwrap())
        .unwrap();
    let setup_required = !matches.is_present("setup_not_required");

    // Use the parsed values here
    println!("TPS: {:?}", tps);
    println!("URL: {}", url.clone());
    println!("Duration: {:?}", duration);
    println!("Nonce: {:?}", nonce);
    println!("Setup required: {:?}", setup_required);

    Ok(Config {
        tps,
        url,
        duration,
        nonce,
        setup_required,
    })
}
