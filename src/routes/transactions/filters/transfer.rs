use super::QueryParam;
use rocket::form::FromForm;
use rocket::http::uri::fmt::{Formatter, FromUriParam, Query, UriDisplay};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use std::fmt;

#[derive(FromForm, Debug, JsonSchema)]
pub struct TransferFilters {
    #[field(name = "execution_date__gte")]
    pub execution_date_gte: Option<String>,
    #[field(name = "execution_date__lte")]
    pub execution_date_lte: Option<String>,
    pub to: Option<String>,
    pub value: Option<String>,
    pub token_address: Option<String>,
}

impl QueryParam for TransferFilters {
    fn as_query_param(&self) -> String {
        let mut query_params = String::new();

        if let Some(execution_date_gte) = &self.execution_date_gte {
            query_params.push_str(&format!("execution_date__gte={}&", execution_date_gte))
        }

        if let Some(execution_date_lte) = &self.execution_date_lte {
            query_params.push_str(&format!("execution_date__lte={}&", execution_date_lte))
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
            Option<String>,
        ),
    > for TransferFilters
{
    type Target = TransferFilters;

    fn from_uri_param(
        (execution_date_gte, execution_date_lte, to, value, token_address): (
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
        ),
    ) -> Self::Target {
        TransferFilters {
            execution_date_gte,
            execution_date_lte,
            to,
            value,
            token_address,
        }
    }
}

impl UriDisplay<Query> for TransferFilters {
    fn fmt(&self, f: &mut Formatter<Query>) -> fmt::Result {
        f.write_named_value("execution_date__gte", &self.execution_date_gte)?;
        f.write_named_value("execution_date__lte", &self.execution_date_lte)?;
        f.write_named_value("to", &self.to)?;
        f.write_named_value("token_address", &self.token_address)?;
        f.write_named_value("value", &self.value)
    }
}
