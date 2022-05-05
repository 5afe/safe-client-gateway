use std::collections::HashMap;
use std::fmt;

use rocket::form::FromForm;
use rocket::http::uri::fmt::{Formatter, FromUriParam, Query, UriDisplay};

#[derive(FromForm, Debug)]
pub struct MultisigFilters {
    pub execution_date_gte: Option<String>,
    pub execution_date_lte: Option<String>,
    pub to: Option<String>,
    pub value: Option<String>,
    pub nonce: Option<String>,
}

impl From<MultisigFilters> for HashMap<String, String> {
    fn from(multisig_filters: MultisigFilters) -> Self {
        let mut map: HashMap<String, String> = HashMap::new();

        if let Some(execution_date_gte) = multisig_filters.execution_date_gte {
            map.push("execution_date__gte".to_string(), execution_date_gte);
        }

        if let Some(execution_date_lte) = multisig_filters.execution_date_lte {
            map.push("execution_date__lte".to_string(), execution_date_lte);
        }

        if let Some(to) = multisig_filters.to {
            map.push("to".to_string(), to);
        }

        if let Some(value) = multisig_filters.value {
            map.push("value".to_string(), value);
        }

        if let Some(nonce) = multisig_filters.nonce {
            map.push("nonce".to_string(), nonce);
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
    > for MultisigFilters
{
    type Target = MultisigFilters;

    fn from_uri_param(
        (execution_date_gte, execution_date_lte, to, value, nonce): (
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
        }
    }
}

impl UriDisplay<Query> for MultisigFilters {
    fn fmt(&self, f: &mut Formatter<Query>) -> fmt::Result {
        f.write_named_value("execution_date__gte", &self.execution_date_gte)?;
        f.write_named_value("execution_date__lte", &self.execution_date_lte)?;
        f.write_named_value("to", &self.to)?;
        f.write_named_value("value", &self.value)?;
        f.write_named_value("nonce", &self.nonce)
    }
}
