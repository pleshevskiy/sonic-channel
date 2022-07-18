use super::{ChannelMode, SonicChannel, SonicStream};
use crate::commands::*;
use crate::result::Result;
use std::net::ToSocketAddrs;

/// The Sonic Channel Ingest mode is used for altering the search index
/// (push, pop and flush). Once in this mode, you cannot switch to other
/// modes or gain access to commands from other modes.
///
/// ### Available commands
///
/// In this mode you can use `push`, `pop`, `flushc`, `flushb`, `flusho`,
/// `bucket_count`, `object_count`, `word_count`, `ping` and `quit` commands.
///
/// **Note:** This mode requires enabling the `ingest` feature.
#[derive(Debug)]
pub struct IngestChannel(SonicStream);

impl SonicChannel for IngestChannel {
    type Channel = IngestChannel;

    fn stream(&self) -> &SonicStream {
        &self.0
    }

    fn start<A, S>(addr: A, password: S) -> Result<Self::Channel>
    where
        A: ToSocketAddrs,
        S: ToString,
    {
        SonicStream::connect_with_start(ChannelMode::Ingest, addr, password).map(Self)
    }
}

impl IngestChannel {
    init_command!(
        /// Stop connection.
        ///
        /// ```rust,no_run
        /// # use sonic_channel::*;
        /// # fn main() -> result::Result<()> {
        /// let channel = IngestChannel::start(
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
        /// let channel = IngestChannel::start(
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

impl IngestChannel {
    init_command!(
        /// Push search data in the index.
        ///
        /// Note: This method requires enabling the `ingest` feature and start
        /// connection in Ingest mode.
        ///
        /// ```rust,no_run
        /// # use sonic_channel::*;
        /// # fn main() -> result::Result<()> {
        /// let ingest_channel = IngestChannel::start(
        ///     "localhost:1491",
        ///     "SecretPassword",
        /// )?;
        ///
        /// let result = ingest_channel.push(PushRequest::new(
        ///     Dest::col("search").obj("recipe:295"),
        ///     "Sweet Teriyaki Beef Skewers"
        /// ))?;
        /// assert_eq!(result, ());
        /// # Ok(())
        /// # }
        /// ```
        use PushCommand for fn push<'a>(
            req: PushRequest,
        );
    );

    init_command!(
        /// Pop search data from the index. Returns removed words count as usize type.
        ///
        /// Note: This method requires enabling the `ingest` feature and start
        /// connection in Ingest mode.
        ///
        /// ```rust,no_run
        /// # use sonic_channel::*;
        /// # fn main() -> result::Result<()> {
        /// let ingest_channel = IngestChannel::start(
        ///     "localhost:1491",
        ///     "SecretPassword",
        /// )?;
        ///
        /// let dest = Dest::col("search").obj("recipe:295");
        /// let result = ingest_channel.pop(PopRequest::new(dest, "beef"))?;
        /// assert_eq!(result, 1);
        /// # Ok(())
        /// # }
        /// ```
        use PopCommand for fn pop(
            req: PopRequest,
        );
    );

    init_command!(
        /// Flush all indexed data from collections.
        ///
        /// Note: This method requires enabling the `ingest` feature and start
        /// connection in Ingest mode.
        ///
        /// ```rust,no_run
        /// # use sonic_channel::*;
        /// # fn main() -> result::Result<()> {
        /// let ingest_channel = IngestChannel::start(
        ///     "localhost:1491",
        ///     "SecretPassword",
        /// )?;
        ///
        /// let flushc_count = ingest_channel.flush(FlushRequest::collection("search"))?;
        /// dbg!(flushc_count);
        /// let flushb_count = ingest_channel.flush(FlushRequest::bucket("search", "default"))?;
        /// dbg!(flushb_count);
        /// let flusho_count = ingest_channel.flush(
        ///     FlushRequest::object("search", "default", "recipe:295")
        /// )?;
        /// dbg!(flusho_count);
        /// # Ok(())
        /// # }
        /// ```
        use FlushCommand for fn flush(
            req: FlushRequest,
        );
    );

    init_command!(
        /// Count indexed search data of your collection.
        ///
        /// Note: This method requires enabling the `ingest` feature and start
        /// connection in Ingest mode.
        ///
        /// ```rust,no_run
        /// # use sonic_channel::*;
        /// # fn main() -> result::Result<()> {
        /// let ingest_channel = IngestChannel::start(
        ///     "localhost:1491",
        ///     "SecretPassword",
        /// )?;
        ///
        /// let bucket_count = ingest_channel.count(CountRequest::buckets("search"))?;
        /// dbg!(bucket_count);
        /// let object_count = ingest_channel.count(CountRequest::objects("search", "default"))?;
        /// dbg!(object_count);
        /// let word_count = ingest_channel.count(
        ///     CountRequest::words("search", "default", "recipe:256")
        /// )?;
        /// dbg!(object_count);
        /// # Ok(())
        /// # }
        /// ```
        use CountCommand for fn count(
            req: CountRequest,
        );
    );
}
