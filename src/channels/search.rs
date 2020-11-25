use super::{ChannelMode, SonicChannel, SonicStream};
use crate::commands::*;
use crate::result::Result;
use std::net::ToSocketAddrs;

/// The Sonic Channel Search mode is used for querying the search index.
/// Once in this mode, you cannot switch to other modes or gain access
/// to commands from other modes.
///
/// ### Available commands
///
/// In this mode you can use `query`, `suggest`, `ping` and `quit` commands.
///
/// **Note:** This mode requires enabling the `search` feature.
#[derive(Debug)]
pub struct SearchChannel(SonicStream);

impl SonicChannel for SearchChannel {
    type Channel = SearchChannel;

    fn stream(&self) -> &SonicStream {
        &self.0
    }

    fn start<A, S>(addr: A, password: S) -> Result<Self::Channel>
    where
        A: ToSocketAddrs,
        S: ToString,
    {
        SonicStream::connect_with_start(ChannelMode::Search, addr, password).map(Self)
    }
}

impl SearchChannel {
    init_command!(
        /// Stop connection.
        ///
        /// ```rust,no_run
        /// # use sonic_channel::*;
        /// # fn main() -> result::Result<()> {
        /// let channel = SearchChannel::start(
        ///     "localhost:1491",
        ///     "SecretPassword",
        /// )?;
        ///
        /// channel.quit()?;
        /// # Ok(())
        /// # }
        use QuitCommand for fn quit();
    );

    init_command!(
        /// Ping server.
        ///
        /// ```rust,no_run
        /// # use sonic_channel::*;
        /// # fn main() -> result::Result<()> {
        /// let channel = SearchChannel::start(
        ///     "localhost:1491",
        ///     "SecretPassword",
        /// )?;
        ///
        /// channel.ping()?;
        /// # Ok(())
        /// # }
        use PingCommand for fn ping();
    );
}

impl SearchChannel {
    init_command!(
        /// Query objects in database.
        ///
        /// Note: This method requires enabling the `search` feature and start
        /// connection in Search mode.
        ///
        /// ```rust,no_run
        /// # use sonic_channel::*;
        /// # fn main() -> result::Result<()> {
        /// let search_channel = SearchChannel::start(
        ///     "localhost:1491",
        ///     "SecretPassword",
        /// )?;
        ///
        /// let result = search_channel.query("search", "default", "Beef")?;
        /// dbg!(result);
        /// # Ok(())
        /// # }
        /// ```
        use QueryCommand for fn query<'a>(
            collection: &'a str,
            bucket: &'a str,
            terms: &'a str,
        );
    );

    init_command!(
        /// Query limited objects in database. This method similar query but
        /// you can configure limit of result.
        ///
        /// Note: This method requires enabling the `search` feature and start
        /// connection in Search mode.
        ///
        /// ```rust,no_run
        /// # use sonic_channel::*;
        /// # fn main() -> result::Result<()> {
        /// let search_channel = SearchChannel::start(
        ///     "localhost:1491",
        ///     "SecretPassword",
        /// )?;
        ///
        /// let result = search_channel.query_with_limit(
        ///     "search",
        ///     "default",
        ///     "Beef",
        ///     10,
        /// )?;
        /// dbg!(result);
        /// # Ok(())
        /// # }
        /// ```
        use QueryCommand for fn query_with_limit<'a>(
            collection: &'a str,
            bucket: &'a str,
            terms: &'a str,
            limit: usize => Some(limit),
        );
    );

    init_command!(
        /// Query limited objects in database. This method similar
        /// query_with_limit but you can put offset in your query.
        ///
        /// Note: This method requires enabling the `search` feature and start
        /// connection in Search mode.
        ///
        /// ```rust,no_run
        /// # use sonic_channel::*;
        /// # fn main() -> result::Result<()> {
        /// let search_channel = SearchChannel::start(
        ///     "localhost:1491",
        ///     "SecretPassword",
        /// )?;
        ///
        /// let result = search_channel.query_with_limit_and_offset(
        ///     "search",
        ///     "default",
        ///     "Beef",
        ///     10,
        ///     10,
        /// )?;
        /// dbg!(result);
        /// # Ok(())
        /// # }
        /// ```
        use QueryCommand for fn query_with_limit_and_offset<'a>(
            collection: &'a str,
            bucket: &'a str,
            terms: &'a str,
            limit: usize => Some(limit),
            offset: usize => Some(offset),
        )
    );

    init_command!(
        /// Suggest auto-completes words.
        ///
        /// Note: This method requires enabling the `search` feature and start
        /// connection in Search mode.
        ///
        /// ```rust,no_run
        /// # use sonic_channel::*;
        /// # fn main() -> result::Result<()> {
        /// let search_channel = SearchChannel::start(
        ///     "localhost:1491",
        ///     "SecretPassword",
        /// )?;
        ///
        /// let result = search_channel.suggest("search", "default", "Beef")?;
        /// dbg!(result);
        /// # Ok(())
        /// # }
        /// ```
        use SuggestCommand for fn suggest<'a>(
            collection: &'a str,
            bucket: &'a str,
            word: &'a str,
        );
    );

    init_command!(
        /// Suggest auto-completes words with limit.
        ///
        /// Note: This method requires enabling the `search` feature and start
        /// connection in Search mode.
        ///
        /// ```rust,no_run
        /// # use sonic_channel::*;
        /// # fn main() -> result::Result<()> {
        /// let search_channel = SearchChannel::start(
        ///     "localhost:1491",
        ///     "SecretPassword",
        /// )?;
        ///
        /// let result = search_channel.suggest_with_limit("search", "default", "Beef", 5)?;
        /// dbg!(result);
        /// # Ok(())
        /// # }
        /// ```
        use SuggestCommand for fn suggest_with_limit<'a>(
            collection: &'a str,
            bucket: &'a str,
            word: &'a str,
            limit: usize => Some(limit),
        );
    );
}
