use std::{
    str::FromStr,
    error::Error,
};

use anyhow::{
    Context,
    Result as AnyResult,
};

lazy_static::lazy_static!{
    pub static ref ENV: Variables = Variables::init().unwrap();
}


#[derive(Debug)]
pub struct Variables {
    pub port0: u16,
}

impl Variables {
    fn init() -> AnyResult<Self> {
        Ok(Variables {
            port0: get("PORT0")?
        })
    }
}

fn get<T, E>(name: &str) -> AnyResult<T>
    where
        T: FromStr<Err = E>,
        E: Send + Sync + Error + 'static,
    {
    std::env::var(name)
        .context(format!("Failed To Get Env Var {name}"))?
        .parse()
        .context(format!("Failed To Parse Env Var {name}"))
}