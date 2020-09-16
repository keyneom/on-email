use serde::{Deserialize, Serialize};

pub(crate) struct MailSearch {
    pub domain: String,
    pub email: String,
    pub password: String,
    pub mailbox: String,
}

pub(crate) struct MailQuery<'a> {
    pub subject: &'a Option<String>,
    pub to: &'a Option<String>,
    pub from: &'a Option<String>,
    pub body: &'a Option<String>,
    pub tag: &'a Option<String>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Mail {
    pub subject: String,
    pub to: String,
    pub from: String,
    pub time: String,
    pub body: String,
    pub tag: Option<String>,
}

impl MailSearch {
    pub fn search(&self, query: MailQuery) -> Result<Vec<Mail>, Error> {
        let tls = native_tls::TlsConnector::builder().build()?;

        let domain = &self.domain[..];
        // In the future refactor to store the session and the client for later use instead of re-connecting every time
        let client = imap::connect((domain, 993), domain, &tls)?;
        let mut imap_session = client.login(&self.email, &self.password).map_err(|e| e.0)?;

        imap_session.select(&self.mailbox)?;

        let mut search_string = "".to_string();
        search_string.push_str("UNSEEN");
        if let Some(subject) = query.subject {
            search_string.push_str(&format!(" SUBJECT \"{}\"", &subject[..]));
        }
        let sequence_set = imap_session.search(&search_string)?;
        let messages = if !sequence_set.is_empty() {
            Some(
                imap_session.fetch(
                    sequence_set
                        .into_iter()
                        .map(|seq_id| seq_id.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                    "ALL",
                )?,
            )
        } else {
            None
        };

        let mut matching_emails = vec![];
        if let Some(messages) = messages {
            matching_emails.reserve_exact(messages.len());
            for message in messages.iter() {
                // extract the message's body
                let envelope = message
                    .envelope()
                    .expect("message did not have an envelope!");
                let subject = std::str::from_utf8(envelope.subject.unwrap())
                    .expect("message was not valid utf-8")
                    .to_string();
                // let to = envelope.to.join(";");

                matching_emails.push(Mail {
                    subject: subject,
                    to: "".to_string(),
                    from: "".to_string(),
                    time: "".to_string(),
                    body: "".to_string(),
                    tag: query.tag.clone(),
                });
            }
        }

        search_string.clear();

        // be nice to the server and log out
        imap_session.logout()?;
        Ok(matching_emails)
    }
}

#[derive(Debug)]
pub(crate) enum Error {
    Tls(String),
    Imap(String),
}

impl From<native_tls::Error> for Error {
    fn from(error: native_tls::Error) -> Self {
        Error::Tls(error.to_string())
    }
}

impl From<imap::Error> for Error {
    fn from(error: imap::Error) -> Self {
        Error::Imap(error.to_string())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {
    #[must_use]
    fn description(&self) -> &str {
        match *self {
            Self::Tls(ref message) | Self::Imap(ref message) => message,
        }
    }

    #[must_use]
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match *self {
            Self::Tls(_) | Self::Imap(_) => None,
        }
    }
}
