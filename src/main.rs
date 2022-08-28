use imdb_swallower::prelude::{by::ByTitle, ImdbSearchEngine};

use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use teloxide::utils::command::BotCommands;
use teloxide::RequestError;
use teloxide::{
    prelude::*,
    types::{
        InlineQueryResult, InlineQueryResultArticle, InputMessageContent, InputMessageContentText,
    },
    Bot,
};

use dotenv::dotenv;

#[derive(BotCommands, PartialEq, Debug, Clone)]
#[command(rename = "kebab-case")]
enum Commands {
    Start,
}

#[tokio::main]
async fn main() {
    dotenv().ok().unwrap();

    let imdb_engine = ImdbSearchEngine::new();
    let bot = Bot::from_env().auto_send();

    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .filter_command::<Commands>()
                .endpoint(on_commands),
        )
        .branch(Update::filter_inline_query().endpoint(on_inline_query));

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![imdb_engine])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn on_commands(
    bot: AutoSend<Bot>,
    message: Message,
    cmd: Commands,
    me: teloxide::types::Me,
) -> Result<(), RequestError> {
    match cmd {
        Commands::Start => {
            let markup = InlineKeyboardMarkup::default();
            let markup = markup.append_row(vec![
                InlineKeyboardButton::switch_inline_query_current_chat("Try It.", "Star Wars"),
            ]);

            bot.send_message(
                message.chat.id,
                format!(
                    "Hello Let's search for some movies.\n\nTry using me in inline mode:\n<code>@{} Star Wars</code>",
                    me.username()
                ),
            )
            .parse_mode(teloxide::types::ParseMode::Html)
            .reply_markup(markup)
            .await?
        }
    };

    Ok(())
}

async fn on_inline_query(
    bot: AutoSend<Bot>,
    inline: InlineQuery,
    engine: ImdbSearchEngine,
) -> Result<(), RequestError> {
    let search_result = engine
        .search_by(ByTitle::default(), &inline.query)
        .await
        .unwrap();

    let mut results = vec![];

    for (i, item) in search_result.items().iter().enumerate() {
        results.push(InlineQueryResult::Article(InlineQueryResultArticle::new(
            i.to_string(),
            item.title().text(),
            InputMessageContent::Text(InputMessageContentText::new(item.to_string())),
        )));
    }

    bot.answer_inline_query(inline.id.to_string(), results)
        .await?;

    Ok(())
}
