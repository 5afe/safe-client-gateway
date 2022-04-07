use super::QueryParam;
use rocket::form::FromForm;
use rocket::http::uri::fmt::{Formatter, FromUriParam, Query, UriDisplay};
use std::fmt;

#[derive(FromForm, Debug)]
pub struct MultisigFilters {
    pub date: Option<String>,
    pub to: Option<String>,
    pub value: Option<String>,
    pub nonce: Option<String>,
}

impl QueryParam for MultisigFilters {
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

        if let Some(nonce) = &self.nonce {
            query_params.push_str(&format!("nonce={}&", nonce))
        }

        return query_params;
    }
}

impl
    FromUriParam<
        Query,
        (
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
        ),
    > for MultisigFilters
{
    type Target = MultisigFilters;

    fn from_uri_param(
        (date, to, value, nonce): (
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
        ),
    ) -> Self::Target {
        MultisigFilters {
            date,
            to,
            value,
            nonce,
        }
    }
}

impl UriDisplay<Query> for MultisigFilters {
    fn fmt(&self, f: &mut Formatter<Query>) -> fmt::Result {
        f.write_named_value("to", &self.to)?;
        f.write_named_value("date", &self.date)?;
        f.write_named_value("value", &self.value)?;
        f.write_named_value("nonce", &self.nonce)
    }
}
