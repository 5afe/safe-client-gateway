use super::QueryParam;
use rocket::form::FromForm;
use rocket::http::uri::fmt::{Formatter, FromUriParam, Query, UriDisplay};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use std::fmt;

#[derive(FromForm, Debug, JsonSchema)]
pub struct MultisigFilters {
    #[field(name = "execution_date__gte")]
    pub execution_date_gte: Option<String>,
    #[field(name = "execution_date__lte")]
    pub execution_date_lte: Option<String>,
    pub to: Option<String>,
    pub value: Option<String>,
    pub nonce: Option<String>,
    pub executed: Option<String>,
}

impl QueryParam for MultisigFilters {
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

        if let Some(nonce) = &self.nonce {
            query_params.push_str(&format!("nonce={}&", nonce))
        }

        if let Some(executed) = &self.executed {
            query_params.push_str(&format!("executed={}&", executed))
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
            Option<String>,
        ),
    > for MultisigFilters
{
    type Target = MultisigFilters;

    fn from_uri_param(
        (execution_date_gte, execution_date_lte, to, value, nonce, executed): (
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
        ),
    ) -> Self::Target {
        MultisigFilters {
            execution_date_gte,
            execution_date_lte,
            to,
            value,
            nonce,
            executed,
        }
    }
}

impl UriDisplay<Query> for MultisigFilters {
    fn fmt(&self, f: &mut Formatter<Query>) -> fmt::Result {
        f.write_named_value("execution_date__gte", &self.execution_date_gte)?;
        f.write_named_value("execution_date__lte", &self.execution_date_lte)?;
        f.write_named_value("to", &self.to)?;
        f.write_named_value("value", &self.value)?;
        f.write_named_value("nonce", &self.nonce)?;
        f.write_named_value("executed", &self.executed)
    }
}
