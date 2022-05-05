use std::collections::HashMap;
use std::fmt;
use std::iter::Map;

use rocket::form::FromForm;
use rocket::http::uri::fmt::{Formatter, FromUriParam, Query, UriDisplay};

#[derive(FromForm, Debug)]
pub struct TransferFilters {
    pub execution_date_gte: Option<String>,
    pub execution_date_lte: Option<String>,
    pub to: Option<String>,
    pub value: Option<String>,
    pub token_address: Option<String>,
}

/// Maps [TransferFilters] into [HashMap]
/// This [HashMap] should be used to build the query parameters for the Safe Transaction Service
impl From<&TransferFilters> for HashMap<String, String> {
    fn from(transfer_filters: TransferFilters) -> Self {
        let mut map: HashMap<String, String> = HashMap::new();

        if let Some(execution_date_gte) = transfer_filters.execution_date_gte {
            map.push("execution_date__gte".to_string(), execution_date_gte)
        }

        if let Some(execution_date_lte) = transfer_filters.execution_date_lte {
            map.push("execution_date__lte".to_string(), execution_date_lte)
        }

        if let Some(to) = transfer_filters.to {
            map.push("to".to_string(), to);
        }

        if let Some(value) = transfer_filters.value {
            map.push("value".to_string(), value);
        }

        if let Some(token_address) = transfer_filters.token_address {
            map.push("token_address".to_string(), token_address);
        }

        return map;
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
