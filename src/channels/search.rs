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
        /// let result = search_channel.query(QueryRequest::new(
        ///     Dest::col("search"),
        ///     "Beef",
        /// ))?;
        /// dbg!(result);
        ///
        /// let result = search_channel.query(
        ///     QueryRequest::new(Dest::col("search"), "Beef").limit(10)
        /// )?;
        /// dbg!(result);
        /// # Ok(())
        /// # }
        /// ```
        use QueryCommand for fn query(
            req: QueryRequest,
        );
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
        /// let result = search_channel.suggest(
        ///     SuggestRequest::new(Dest::col("search"), "Beef")
        /// )?;
        /// dbg!(result);
        ///
        /// let result = search_channel.suggest(
        ///     SuggestRequest::new(Dest::col("search"), "Beef").limit(2)
        /// )?;
        /// dbg!(result);
        /// # Ok(())
        /// # }
        /// ```
        use SuggestCommand for fn suggest(
            req: SuggestRequest,
        );
    );
}
