use quickstock_rust::settings::Settings;

#[tokio::test]
async fn test_settings_save_and_load() {
    let settings = Settings {
        theme: "dark".to_string(),
        language: "en-US".to_string(),
    };

    let filename = "test_config.toml";
    settings.save(filename).await.unwrap();

    let loaded = Settings::load(filename).await.unwrap();
    assert_eq!(settings.theme, loaded.theme);
    assert_eq!(settings.language, loaded.language);

    // Cleanup test file
    tokio::fs::remove_file(filename).await.unwrap();
}
