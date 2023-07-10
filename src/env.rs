use std::{
    error::Error,
    str::FromStr,
};

use crate::error::{
    Context,
    MyResult,
};

lazy_static::lazy_static! {
    pub static ref ENV: Variables = Variables::init().unwrap();
}

#[derive(Debug)]
pub struct Variables {
    pub port0: u16,
    pub gpt_api_key: String,
}

impl Variables {
    fn init() -> MyResult<Self> {
        Ok(Variables {
            port0: get("PORT0")?,
            gpt_api_key: get("GPT_API_KEY")?,
        })
    }
}

fn get<T, E>(name: &str) -> MyResult<T>
where
    T: FromStr<Err = E>,
    E: Send + Sync + Error + 'static,
{
    std::env::var(name)
        .context(format!("Failed To Get Env Var {name}"))?
        .parse()
        .context(format!("Failed To Parse Env Var {name}"))
}
