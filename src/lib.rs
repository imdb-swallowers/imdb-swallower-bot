use html_escape::encode_text;
use imdb_swallower::prelude::results::by_title::TitleSearchItem;
use teloxide::types::ParseMode;

pub trait HtmlCreator {
    fn to_html_code(&self) -> String;

    fn to_html_bold(&self) -> String;

    fn to_html_italic(&self) -> String;

    fn to_html_hyperlink(&self, link: &str) -> String;
}

impl HtmlCreator for String {
    fn to_html_code(&self) -> String {
        format!("<code>{}</code>", encode_text(self))
    }

    fn to_html_bold(&self) -> String {
        format!("<b>{}</b>", encode_text(self))
    }

    fn to_html_hyperlink(&self, link: &str) -> String {
        format!("<a href=\"{}\">{}</a>", link, encode_text(self))
    }

    fn to_html_italic(&self) -> String {
        format!("<i>{}</i>", encode_text(self))
    }
}

impl HtmlCreator for str {
    fn to_html_code(&self) -> String {
        format!("<code>{}</code>", encode_text(self))
    }

    fn to_html_bold(&self) -> String {
        format!("<b>{}</b>", encode_text(self))
    }

    fn to_html_hyperlink(&self, link: &str) -> String {
        format!("<a href=\"{}\">{}</a>", link, encode_text(self))
    }

    fn to_html_italic(&self) -> String {
        format!("<i>{}</i>", encode_text(self))
    }
}

pub trait TelegramMessage {
    fn to_telegram_message(&self, parse_mode: ParseMode) -> String;
}

impl TelegramMessage for TitleSearchItem {
    fn to_telegram_message(&self, parse_mode: ParseMode) -> String {
        match parse_mode {
            ParseMode::MarkdownV2 => todo!(),
            ParseMode::Html => {
                let title = self.title();

                let directors_text = self
                    .directors()
                    .iter()
                    .map(|d| {
                        d.name()
                            .to_html_hyperlink(&format!("https://www.imdb.com{}", d.link()))
                    })
                    .collect::<Vec<String>>()
                    .join(", ");

                let stars_text = self
                    .stars()
                    .iter()
                    .map(|d| {
                        d.name()
                            .to_html_hyperlink(&format!("https://www.imdb.com{}", d.link()))
                    })
                    .collect::<Vec<String>>()
                    .join(", ");

                let people_text = self.join_peoples(
                    |g| format!("- {}: ", g),
                    "\n",
                    |p| {
                        p.name()
                            .to_html_hyperlink(&format!("https://www.imdb.com{}", p.link()))
                    },
                    ", ",
                );

                format!(
                    "🍿 {} {}\n[{}]\n⭐ Rating: {}\n\n- {}\n\n{}",
                    title.text().to_html_hyperlink(
                        format!("https://www.imdb.com{}", title.link()).as_ref()
                    ),
                    self.years().to_html_bold(),
                    self.info(),
                    self.rating().to_html_code(),
                    self.summery(),
                    people_text
                )
            }
            _ => todo!(),
        }
    }
}
