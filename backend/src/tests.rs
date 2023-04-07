#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use crate::config::Config;

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    /// Test to ensure creation of a config works
    #[tokio::test]
    async fn test_config_new() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config.json");

        let config = Config::load(config_path.to_str()).await;

        assert!(config.is_some());

        let config = config.unwrap();

        assert_eq!(config.port, 50051);
        assert_eq!(config.auto_lock, false);
        assert_eq!(config.regions.len(), 0);
        assert_eq!(config.loaded_path().to_str(), config_path.to_str());
    }

    /// Test to ensure json serializing a newly created Config works
    #[tokio::test]
    async fn test_config_new_serialize() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config.json");

        let config = Config::load(config_path.to_str()).await;
        let file_contents = tokio::fs::read_to_string(config_path).await;

        assert!(config.is_some());
        assert!(file_contents.is_ok());

        let config = config.unwrap();
        let file_contents = file_contents.unwrap();

        let serialized = serde_json::to_string_pretty(&config).unwrap();

        assert_eq!(serialized, file_contents);
    }

    /// Test to ensure saving a Config works
    #[tokio::test]
    async fn test_config_save() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config.json");

        let mut config = Config::load(config_path.to_str()).await.unwrap();

        config.port = 50052;
        config.auto_lock = true;

        config.save().await;

        let file_contents = tokio::fs::read_to_string(config_path).await;

        assert!(file_contents.is_ok());

        let file_contents = file_contents.unwrap();
        let serialized = serde_json::to_string_pretty(&config).unwrap();

        assert_eq!(serialized, file_contents);
    }
}
