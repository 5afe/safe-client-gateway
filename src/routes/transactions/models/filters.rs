use std::fmt;

use rocket::form::FromForm;
use rocket::http::uri::fmt::{Formatter, FromUriParam, Query, UriDisplay};

#[derive(FromForm, Debug)]
pub struct TransferFilters {
    pub date: Option<String>,
    pub to: Option<String>,
    pub value: Option<String>,
    pub token_address: Option<String>,
}

#[derive(FromForm, Debug)]
pub struct ModuleFilters {
    pub date: Option<String>,
    pub to: Option<String>,
    pub module: Option<String>,
}

#[derive(FromForm, Debug)]
pub struct MultisigFilters {
    pub date: Option<String>,
    pub to: Option<String>,
    pub value: Option<String>,
    pub nonce: Option<String>,
}

pub trait QueryParam {
    fn as_query_param(&self) -> String;
}

impl QueryParam for TransferFilters {
    fn as_query_param(&self) -> String {
        let mut query_params = String::new();

        if let Some(date) = &self.date {
            query_params.push_str(&format!("date={}&", date))
        }

        if let Some(to) = &self.to {
            query_params.push_str(&format!("to={}&", to))
        }

        if let Some(value) = &self.value {
            query_params.push_str(&format!("value={}&", value))
        }

        if let Some(token_address) = &self.token_address {
            query_params.push_str(&format!("token_address={}&", token_address))
        }

        return query_params;
    }
}

impl FromUriParam<Query, (&str, &str, &str, &str)> for TransferFilters {
    type Target = TransferFilters;

    fn from_uri_param((date, to, value, token_address): (&str, &str, &str, &str)) -> Self::Target {
        TransferFilters {
            date: Some(date.to_string()),
            to: Some(to.to_string()),
            value: Some(value.to_string()),
            token_address: Some(token_address.to_string()),
        }
    }
}

impl UriDisplay<Query> for TransferFilters {
    fn fmt(&self, f: &mut Formatter<Query>) -> fmt::Result {
        f.write_named_value("to", &self.to)?;
        f.write_named_value("date", &self.date)?;
        f.write_named_value("token_address", &self.token_address)?;
        f.write_named_value("value", &self.value)
    }
}
