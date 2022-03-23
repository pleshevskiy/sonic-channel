use super::StreamCommand;
use crate::result::*;

#[derive(Debug, Default)]
pub struct PushCommand<'a> {
    pub collection: &'a str,
    pub bucket: &'a str,
    pub object: &'a str,
    pub text: &'a str,
    pub locale: Option<&'a str>,
}

impl StreamCommand for PushCommand<'_> {
    type Response = bool;

    fn message(&self) -> String {
        let mut message = format!(
            r#"PUSH {} {} {} "{}""#,
            self.collection,
            self.bucket,
            self.object,
            remove_multiline(self.text)
        );

        let locale = self.locale.or_else(|| {
            whatlang::detect(self.text).and_then(|info| {
                if info.confidence() == 1.0 {
                    Some(info.lang().code())
                } else {
                    None
                }
            })
        });

        if let Some(locale) = locale {
            message.push_str(&format!(" LANG({})", locale));
        }

        message.push_str("\r\n");
        message
    }

    fn receive(&self, message: String) -> Result<Self::Response> {
        if message == "OK\r\n" {
            Ok(true)
        } else {
            Err(Error::new(ErrorKind::WrongResponse))
        }
    }
}

fn remove_multiline(text: &str) -> String {
    text.lines()
        .enumerate()
        .fold(String::new(), |mut acc, (i, line)| {
            if i != 0 && !line.is_empty() && !acc.is_empty() && !acc.ends_with(' ') {
                acc.push(' ');
            }

            acc.push_str(line);
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::remove_multiline;

    #[test]
    fn should_make_single_line() {
        let text = "
Hello
World
";

        let expected_text = "Hello World";
        assert_eq!(remove_multiline(text), expected_text);
    }
}
