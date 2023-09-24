mod config;

pub use config::Config;

mod tests {
    use super::*;

    #[test]
    fn load_default() {
        let c = Config::load_params();

        match c {
            Ok(config) => assert_eq!(config.population_size, 100),
            Err(_msg) => (),
        }
    }
}
