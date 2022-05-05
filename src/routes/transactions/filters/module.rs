use std::collections::HashMap;
use std::fmt;

use rocket::form::FromForm;
use rocket::http::uri::fmt::{Formatter, FromUriParam, Query, UriDisplay};

#[derive(FromForm, Debug)]
pub struct ModuleFilters {
    pub to: Option<String>,
    pub module: Option<String>,
}

impl From<ModuleFilters> for HashMap<String, String> {
    fn from(module_filters: ModuleFilters) -> Self {
        let mut map: HashMap<String, String> = HashMap::new();

        if let Some(to) = module_filters.to {
            map.push("to".to_string(), to);
        }

        if let Some(offset) = module_filters.offset {
            map.push("offset".to_string(), offset);
        }

        return map;
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
