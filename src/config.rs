use clap::Clap;

#[derive(Clap, Debug)]
#[clap(name = "on-email")]
struct OnEmailArgs {
    #[clap(short, long)]
    domain: String,
    #[clap(short, long)]
    email: String,
    #[clap(short, long)]
    password: String,
    #[clap(short, long, default_value = "INBOX")]
    mailbox: String,
    #[clap(flatten)]
    watch: OnEmailWatch,
    #[clap()]
    more_watches: Vec<String>,
}

#[derive(Clap, Debug)]
pub(crate) struct OnEmailWatch {
    #[clap(long)]
    pub tag: Option<String>,
    #[clap(short, long)]
    pub subject: Option<String>,
    #[clap(short, long)]
    pub to: Option<String>,
    #[clap(short, long)]
    pub from: Option<String>,
    #[clap(short, long)]
    pub body: Option<String>,
}

#[derive(Debug)]
pub(crate) struct Config {
    pub domain: String,
    pub email: String,
    pub password: String,
    pub mailbox: String,
    pub watches: Vec<OnEmailWatch>,
}

impl Config {
    pub fn parse(args: std::env::Args) -> Self {
        // Split command line arguments into groups separated by '--watch' delimiter
        let arguments: Vec<String> = args.collect();
        let mut watch_groups = arguments.split(|argument| argument == "watch");

        // First group of parameters is basic without any information for specific watch
        let on_email_args_parameters = watch_groups.next().unwrap();

        // Parse basic parameters
        let basic = OnEmailArgs::parse_from(on_email_args_parameters);

        // Create vector to be filled with all parsed watches
        let mut watches = Vec::new();

        // Add already parsed watch to list of all watches
        watches.push(basic.watch);

        // Iterate over rest of groups each containing complete sensor information
        for watch_parameters in watch_groups {
            // Add an additional value to the front to be thrown away since the first positional argument
            // is treated as if it were the executable
            let watch_parameters = [&["on-email".to_string()][..], watch_parameters].concat();
            // Parse each watch
            let watch = OnEmailWatch::parse_from(watch_parameters);

            // Fill sensors vector
            watches.push(watch);
        }

        // Return result
        Config {
            domain: basic.domain,
            email: basic.email,
            password: basic.password,
            mailbox: basic.mailbox,
            watches,
        }
    }
}
