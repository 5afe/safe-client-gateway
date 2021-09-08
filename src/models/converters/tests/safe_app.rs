use crate::models::backend::safe_app::{
    SafeApp as BackendSafeApp, SafeAppProvider as BackendSafeAppProvider,
};
use crate::models::service::safe_app::{SafeApp, SafeAppProvider};

#[test]
fn safe_apps_empty() {
    let backend_json = "[]";
    let expected: Vec<SafeApp> = vec![];
    let response = serde_json::from_str::<Vec<BackendSafeApp>>(backend_json)
        .expect("SafeApp deserialization failed");

    let actual: Vec<SafeApp> = response
        .into_iter()
        .map(|safe_app| safe_app.into())
        .collect();

    assert_eq!(expected, actual);
}

#[test]
fn safe_apps_several_apps() {
    let response = serde_json::from_str::<Vec<BackendSafeApp>>(crate::json::POLYGON_SAFE_APPS)
        .expect("SafeApps deserialization failure");
}
