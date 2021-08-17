#[test]
fn test_env() {
    for test_case in build_test_cases() {
        test_case.assert_default();
        test_case.assert_env_var();
    }
}

fn build_test_cases() -> Vec<impl TestCase> {
    vec![USizeEnvValue {
        expected_default: 60 * 15,
        expected_env: 1,
        env_key: String::from("ABOUT_CACHE_DURATION"),
        generator: Box::new(super::about_cache_duration),
    }]
}

trait TestCase {
    fn assert_default(&self);
    fn assert_env_var(&self);
}

struct USizeEnvValue {
    expected_default: usize,
    expected_env: usize,
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

    fn assert_env_var(&self) {
        std::env::set_var(&self.env_key, &self.expected_env.to_string());
        let actual_env = (&self.generator)();
        assert_eq!(
            self.expected_env, actual_env,
            "Test env var for env key: {}",
            &self.env_key
        );
    }
}
