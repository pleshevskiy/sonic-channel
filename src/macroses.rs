macro_rules! init_command {
    (
        $(#[$outer:meta])*
        use $cmd_name:ident
        for fn $fn_name:ident $(<$($lt:lifetime)+>)? (
            $($arg_name:ident : $arg_type:ty $( => $arg_value:expr)?,)*
        )
        $(;)?
    ) => {
        $(#[$outer])*
        pub fn $fn_name $(<$($lt)+>)? (
            &self,
            $($arg_name: $arg_type),*
        ) -> $crate::result::Result<
            <$cmd_name as $crate::commands::StreamCommand>::Response,
        > {
            #[allow(clippy::needless_update)]
            let command = $cmd_name { $($arg_name $(: $arg_value)?,)* ..Default::default() };
            self.stream().run_command(command)
        }
    };
}
