use rocket::form::FromForm;

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
