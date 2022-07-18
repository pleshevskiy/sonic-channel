use super::{ChannelMode, SonicChannel, SonicStream};
use crate::commands::*;
use crate::result::Result;
use std::net::ToSocketAddrs;

/// The Sonic Channel Control mode is used for administration purposes.
/// Once in this mode, you cannot switch to other modes or gain access
/// to commands from other modes.
///
/// ### Available commands
///
/// In this mode you can use `consolidate`, `backup`, `restore`,
/// `ping` and `quit` commands.
///
/// **Note:** This mode requires enabling the `control` feature.
#[derive(Debug)]
pub struct ControlChannel(SonicStream);

impl SonicChannel for ControlChannel {
    type Channel = ControlChannel;

    fn stream(&self) -> &SonicStream {
        &self.0
    }

    fn start<A, S>(addr: A, password: S) -> Result<Self::Channel>
    where
        A: ToSocketAddrs,
        S: ToString,
    {
        SonicStream::connect_with_start(ChannelMode::Control, addr, password).map(Self)
    }
}

impl ControlChannel {
    init_command!(
        /// Stop connection.
        ///
        /// ```rust,no_run
        /// # use sonic_channel::*;
        /// # fn main() -> result::Result<()> {
        /// let channel = ControlChannel::start(
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
        /// let channel = ControlChannel::start(
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

impl ControlChannel {
    init_command!(
        /// Trigger control action.
        ///
        /// Note: This method requires enabling the `control` feature and start connection in
        /// Control mode
        ///
        /// ```rust,no_run
        /// # use sonic_channel::*;
        /// # fn main() -> result::Result<()> {
        /// let control_channel = ControlChannel::start(
        ///     "localhost:1491",
        ///     "SecretPassword",
        /// )?;
        ///
        /// control_channel.trigger(TriggerRequest::Consolidate)?;
        /// # Ok(())
        /// # }
        use TriggerCommand for fn trigger(
            req: TriggerRequest,
        )
    );

    /// Consolidate indexed search data instead of waiting for the next automated
    /// consolidation tick.
    ///
    /// Note: This method requires enabling the `control` feature and start
    /// connection in Control mode.
    ///
    /// ```rust,no_run
    /// # use sonic_channel::*;
    /// # fn main() -> result::Result<()> {
    /// let control_channel = ControlChannel::start(
    ///     "localhost:1491",
    ///     "SecretPassword",
    /// )?;
    ///
    /// control_channel.consolidate()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn consolidate(&self) -> Result<()> {
        self.trigger(TriggerRequest::Consolidate)
    }

    /// Backup KV + FST to <path>/<BACKUP_{KV/FST}_PATH>
    /// See [sonic backend source code](https://github.com/valeriansaliou/sonic/blob/master/src/channel/command.rs#L808)
    /// for more information.
    ///
    /// Note: This method requires enabling the `control` feature and start
    /// connection in Control mode.
    ///
    /// ```rust,no_run
    /// # use sonic_channel::*;
    /// # fn main() -> result::Result<()> {
    /// let control_channel = ControlChannel::start(
    ///     "localhost:1491",
    ///     "SecretPassword",
    /// )?;
    ///
    /// control_channel.backup("2020-08-07T23-48")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn backup(&self, path: &str) -> Result<()> {
        self.trigger(TriggerRequest::Backup(path))
    }

    /// Restore KV + FST from <path> if you already have backup with the same name.
    ///
    /// Note: This method requires enabling the `control` feature and start
    /// connection in Control mode.
    ///
    /// ```rust,no_run
    /// # use sonic_channel::*;
    /// # fn main() -> result::Result<()> {
    /// let control_channel = ControlChannel::start(
    ///     "localhost:1491",
    ///     "SecretPassword",
    /// )?;
    ///
    /// let result = control_channel.restore("2020-08-07T23-48")?;
    /// assert_eq!(result, ());
    /// # Ok(())
    /// # }
    /// ```
    pub fn restore(&self, path: &str) -> Result<()> {
        self.trigger(TriggerRequest::Restore(path))
    }
}
