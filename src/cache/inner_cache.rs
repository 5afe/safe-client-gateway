use crate::utils::errors::ApiError;

#[derive(Debug, PartialEq)]
pub(super) struct CachedWithCode {
    pub(super) code: u16,
    pub(super) data: String,
}

impl CachedWithCode {
    const SEPARATOR: &'static str = ";";

    pub(super) fn split(cached: &str) -> Self {
        let cached_with_code: Vec<&str> = cached.splitn(2, CachedWithCode::SEPARATOR).collect();
        CachedWithCode {
            code: cached_with_code
                .get(0)
                .expect("Must have a status code")
                .parse()
                .expect("Not a valid Http code"),
            data: cached_with_code.get(1).expect("Must have data").to_string(),
        }
    }

    pub(super) fn join(code: u16, data: &str) -> String {
        format!("{}{}{}", code, CachedWithCode::SEPARATOR, data)
    }

    pub(super) fn is_error(&self) -> bool {
        200 > self.code || self.code >= 400
    }

    pub(super) fn to_result(&self) -> Result<String, ApiError> {
        if self.is_error() {
            Err(ApiError::from_backend_error(self.code, &self.data))
        } else {
            Ok(String::from(&self.data))
        }
    }
}
