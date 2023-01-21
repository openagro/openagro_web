use openagro_extension::{ extension::ExtensionMetadata, assets::{ Assets, AssetType } };

pub fn metadata() -> ExtensionMetadata {
    let mut metadata = ExtensionMetadata {
        extension_id: "openagro.web.extension".to_string(),
        name: "Web".to_string(),
        description: None,
        category: Some("base".to_string()),
        author: "OpenAgro Developers".to_string(),
        version: "1.0.0".to_string(),
        data: vec![
            Assets{
                asset_type: AssetType::Css,
                assets: include_str!("../assets/css/main.css").to_string(),
            },
            Assets{
                asset_type: AssetType::Javascript,
                assets: include_str!("../assets/js/main.js").to_string(),
            },
            Assets{
                asset_type: AssetType::Data,
                assets: include_str!("../data/ir.company.csv").to_string(),
            }
        ],
        depends_on: vec![],
        application: false,
    };

    metadata
}