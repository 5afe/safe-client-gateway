use super::QueryParam;
use rocket::form::FromForm;
use rocket::http::uri::fmt::{Formatter, FromUriParam, Query, UriDisplay};
use std::fmt;

#[derive(FromForm, Debug)]
pub struct ModuleFilters {
    pub to: Option<String>,
    pub module: Option<String>,
}

impl QueryParam for ModuleFilters {
    fn as_query_param(&self) -> String {
        let mut query_params = String::new();

        if let Some(to) = &self.to {
            query_params.push_str(&format!("to={}&", to))
        }

        if let Some(module) = &self.module {
            query_params.push_str(&format!("module={}&", module))
        }

        return query_params;
    }
}

impl FromUriParam<Query, (Option<String>, Option<String>)> for ModuleFilters {
    type Target = ModuleFilters;

    fn from_uri_param((to, module): (Option<String>, Option<String>)) -> Self::Target {
        ModuleFilters { to, module }
    }
}

impl UriDisplay<Query> for ModuleFilters {
    fn fmt(&self, f: &mut Formatter<Query>) -> fmt::Result {
        f.write_named_value("to", &self.to)?;
        f.write_named_value("module", &self.module)
    }
}
