use super::QueryParam;
use rocket::form::FromForm;
use rocket::http::uri::fmt::{Formatter, FromUriParam, Query, UriDisplay};
use std::fmt;

#[derive(FromForm, Debug)]
pub struct TransferFilters {
    pub date: Option<String>,
    pub to: Option<String>,
    pub value: Option<String>,
    pub token_address: Option<String>,
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

impl
    FromUriParam<
        Query,
        (
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
        ),
    > for TransferFilters
{
    type Target = TransferFilters;

    fn from_uri_param(
        (date, to, value, token_address): (
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
        ),
    ) -> Self::Target {
        TransferFilters {
            date,
            to,
            value,
            token_address,
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
