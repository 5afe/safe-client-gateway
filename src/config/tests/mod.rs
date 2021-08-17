#[test]
fn test_env() {
    for test_case in build_usize_test_cases() {
        test_case.assert_default();
        test_case.assert_env_key();
    }

    for test_case in build_u64_test_cases() {
        test_case.assert_default();
        test_case.assert_env_key();
    }
}

fn build_usize_test_cases() -> Vec<USizeEnvValue> {
    vec![
        USizeEnvValue {
            expected_default: 60 * 60,
            env_key: String::from("SAFE_INFO_CACHE_DURATION"),
            generator: Box::new(super::safe_info_cache_duration),
        },
        USizeEnvValue {
            expected_default: 60 * 60,
            env_key: String::from("ADDRESS_INFO_CACHE_DURATION"),
            generator: Box::new(super::address_info_cache_duration),
        },
        USizeEnvValue {
            expected_default: 60 * 60 * 24,
            env_key: String::from("TOKEN_INFO_CACHE_DURATION"),
            generator: Box::new(super::token_info_cache_duration),
        },
        USizeEnvValue {
            expected_default: 60 * 60,
            env_key: String::from("CHAIN_INFO_CACHE_DURATION"),
            generator: Box::new(super::chain_info_cache_duration),
        },
        USizeEnvValue {
            expected_default: 60 * 60 * 12,
            env_key: String::from("EXCHANGE_API_CACHE_DURATION"),
            generator: Box::new(super::exchange_api_cache_duration),
        },
        USizeEnvValue {
            expected_default: 60 * 60,
            env_key: String::from("REQUEST_CACHE_DURATION"),
            generator: Box::new(super::request_cache_duration),
        },
        USizeEnvValue {
            expected_default: 60 * 15,
            env_key: String::from("ABOUT_CACHE_DURATION"),
            generator: Box::new(super::about_cache_duration),
        },
        USizeEnvValue {
            expected_default: 60,
            env_key: String::from("BALANCES_REQUEST_CACHE_DURATION"),
            generator: Box::new(super::balances_cache_duration),
        },
        USizeEnvValue {
            expected_default: 60 * 60,
            env_key: String::from("SAFE_APP_MANIFEST_CACHE_DURATION"),
            generator: Box::new(super::safe_app_manifest_cache_duration),
        },
    ]
}

fn build_u64_test_cases() -> Vec<U64EnvValue> {
    vec![
        U64EnvValue {
            expected_default: 1000,
            env_key: String::from("INTERNAL_CLIENT_CONNECT_TIMEOUT"),
            generator: Box::new(super::internal_client_connect_timeout),
        },
        U64EnvValue {
            expected_default: 3000,
            env_key: String::from("SAFE_APP_INFO_REQUEST_TIMEOUT"),
            generator: Box::new(super::safe_app_info_request_timeout),
        },
    ]
}

trait TestCase {
    fn assert_default(&self);
    fn assert_env_key(&self);
}

struct USizeEnvValue {
    expected_default: usize,
    env_key: String,
    generator: Box<dyn Fn() -> usize>,
}

impl TestCase for USizeEnvValue {
    fn assert_default(&self) {
        std::env::remove_var(&self.env_key);
        let actual_default = (&self.generator)();
        assert_eq!(
            self.expected_default, actual_default,
            "Test default value for env key: {}",
            &self.env_key
        );
    }

    fn assert_env_key(&self) {
        let mock_env_var_value = 1;
        std::env::set_var(&self.env_key, &mock_env_var_value.to_string());
        let actual_env = (&self.generator)();
        assert_eq!(
            mock_env_var_value, actual_env,
            "Test env var for env key: {}",
            &self.env_key
        );
    }
}

struct U64EnvValue {
    expected_default: u64,
    env_key: String,
    generator: Box<dyn Fn() -> u64>,
}

impl TestCase for U64EnvValue {
    fn assert_default(&self) {
        std::env::remove_var(&self.env_key);
        let actual_default = (&self.generator)();
        assert_eq!(
            self.expected_default, actual_default,
            "Test default value for env key: {}",
            &self.env_key
        );
    }

    fn assert_env_key(&self) {
        let mock_env_var_value = 1;
        std::env::set_var(&self.env_key, &mock_env_var_value.to_string());
        let actual_env = (&self.generator)();
        assert_eq!(
            mock_env_var_value, actual_env,
            "Test env var for env key: {}",
            &self.env_key
        );
    }
}
