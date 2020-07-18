# Sonic Channel

Rust client for [sonic] search backend.

We recommend you start with the [documentation].


## Installation

Add `sonic-channel = { version = "0.1" }` as a dependency in `Cargo.toml`.

`Cargo.toml` example:

```toml
[package]
name = "my-crate"
version = "0.1.0"
authors = ["Me <user@rust-lang.org>"]

[dependencies]
sonic-channel = { version = "0.1" }
```


## Example usage

```rust
use std::itconfig;
use std::env;
//use dotenv::dotenv;

config! {
    DEBUG: bool => false,
    
    #[env_name = "APP_HOST"]
    HOST: String => "127.0.0.1",
    
    DATABASE_URL < (
        "postgres://",
        POSTGRES_USERNAME => "user",
        ":",
        POSTGRES_PASSWORD => "pass",
        "@",
        POSTGRES_HOST => "localhost:5432",
        "/",
        POSTGRES_DB => "test",
    ),
    
    APP {
        static BASE_URL => "/api", // &'static str by default
    
        ARTICLE {
            static PER_PAGE: u32 => 15,
        }
        
        #[cfg(feature = "companies")]
        COMPANY {
            #[env_name = "INSTITUTIONS_PER_PAGE"]
            static PER_PAGE: u32 => 15,
        }
    }
    
    FEATURE {
        NEW_MENU: bool => false,
    
        COMPANY {
            PROFILE: bool => false,
        }
    }
}

fn main () {
    // dotenv().expect("dotenv setup to be successful");
    // or
    env::set_var("FEATURE_NEW_MENU", "t");
    
    config::init();
    assert_eq!(config::HOST(), String::from("127.0.0.1"));
    assert_eq!(config::DATABASE_URL(), String::from("postgres://user:pass@localhost:5432/test"));
    assert_eq!(config::APP:ARTICLE:PER_PAGE(), 15);
    assert_eq!(config::FEATURE::NEW_MENU(), true);
}
```


Macro is an optional feature, disabled by default. You can use this library without macro

```rust
use itconfig::*;
use std::env;
// use dotenv::dotenv;

fn main() {
    // dotenv().expect("dotenv setup to be successful");
    // or
    env::set_var("DATABASE_URL", "postgres://127.0.0.1:5432/test");

    let database_url = get_env::<String>("DATABASE_URL").unwrap();
    let new_profile: bool = get_env_or_default("FEATURE_NEW_PROFILE", false);
    let articles_per_page: u32 = get_env_or_set_default("ARTICLES_PER_PAGE", 10);
}
```


## Available features

* **default** - ["search"]
* **search** - Add sonic search mode with methods
* **ignite** - Add sonic ignite mode with methods
* **control** - Add sonic control mode with methods


[sonic]: https://github.com/valeriansaliou/sonic
[documentation]: https://docs.rs/sonic-channel