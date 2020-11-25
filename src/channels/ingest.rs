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
        /// let result = ingest_channel.push(
        ///     "search",
        ///     "default",
        ///     "recipe:295",
        ///     "Sweet Teriyaki Beef Skewers",
        /// )?;
        /// assert_eq!(result, true);
        /// # Ok(())
        /// # }
        /// ```
        use PushCommand for fn push<'a>(
            collection: &'a str,
            bucket: &'a str,
            object: &'a str,
            text: &'a str,
        );
    );

    init_command!(
        /// Push search data in the index with locale parameter in ISO 639-3 code.
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
        /// let result = ingest_channel.push_with_locale(
        ///     "search",
        ///     "default",
        ///     "recipe:296",
        ///     "Гренки с жареным картофелем и сыром",
        ///     "rus",
        /// )?;
        /// assert_eq!(result, true);
        /// # Ok(())
        /// # }
        /// ```
        use PushCommand for fn push_with_locale<'a>(
            collection: &'a str,
            bucket: &'a str,
            object: &'a str,
            text: &'a str,
            locale: &'a str => Some(locale),
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
        /// let result = ingest_channel.pop("search", "default", "recipe:295", "beef")?;
        /// assert_eq!(result, 1);
        /// # Ok(())
        /// # }
        /// ```
        use PopCommand for fn pop<'a>(
            collection: &'a str,
            bucket: &'a str,
            object: &'a str,
            text: &'a str,
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
        /// let flushc_count = ingest_channel.flushc("search")?;
        /// dbg!(flushc_count);
        /// # Ok(())
        /// # }
        /// ```
        use FlushCommand for fn flushc<'a>(
            collection: &'a str,
        );
    );

    init_command!(
        /// Flush all indexed data from bucket in a collection.
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
        /// let flushb_count = ingest_channel.flushb("search", "default")?;
        /// dbg!(flushb_count);
        /// # Ok(())
        /// # }
        /// ```
        use FlushCommand for fn flushb<'a>(
            collection: &'a str,
            bucket: &'a str => Some(bucket),
        );
    );

    init_command!(
        /// Flush all indexed data from an object in a bucket in collection.
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
        /// let flusho_count = ingest_channel.flusho("search", "default", "recipe:296")?;
        /// dbg!(flusho_count);
        /// # Ok(())
        /// # }
        /// ```
        use FlushCommand for fn flusho<'a>(
            collection: &'a str,
            bucket: &'a str => Some(bucket),
            object: &'a str => Some(object),
        );
    );

    init_command!(
        /// Bucket count in indexed search data of your collection.
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
        /// let bucket_count = ingest_channel.bucket_count("search")?;
        /// dbg!(bucket_count);
        /// # Ok(())
        /// # }
        /// ```
        use CountCommand for fn bucket_count<'a>(
            collection: &'a str,
        );
    );

    init_command!(
        /// Object count of bucket in indexed search data.
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
        /// let object_count = ingest_channel.object_count("search", "default")?;
        /// dbg!(object_count);
        /// # Ok(())
        /// # }
        /// ```
        use CountCommand for fn object_count<'a>(
            collection: &'a str,
            bucket: &'a str => Some(bucket),
        );
    );

    init_command!(
        /// Object word count in indexed bucket search data.
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
        /// let word_count = ingest_channel.word_count("search", "default", "recipe:296")?;
        /// dbg!(word_count);
        /// # Ok(())
        /// # }
        /// ```
        use CountCommand for fn word_count<'a>(
            collection: &'a str,
            bucket: &'a str => Some(bucket),
            object: &'a str => Some(object),
        );
    );
}
