mod config;
mod mailsearch;
use config::Config;
use mailsearch::{MailQuery, MailSearch};
use std::env;

fn main() -> Result<(), mailsearch::Error> {
    let config = Config::parse(env::args());
    let mailsearch = MailSearch {
        domain: config.domain,
        mailbox: config.mailbox,
        email: config.email,
        password: config.password,
    };

    for ref command in config.watches {
        let matching_emails = mailsearch.search(command.into())?;
        for mail in matching_emails {
            println!("{}", serde_json::to_string(&mail).unwrap());
        }
    }

    Ok(())
}

impl config::OnEmailWatch {
    fn into(&self) -> MailQuery {
        MailQuery {
            subject: &self.subject,
            to: &self.to,
            from: &self.from,
            body: &self.body,
            tag: &self.tag,
        }
    }
}
