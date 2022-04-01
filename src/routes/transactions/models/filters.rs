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
