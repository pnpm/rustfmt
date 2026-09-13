// rustfmt-edition: 2024
// rustfmt-style_edition: 2024
// rustfmt-max_width: 60
// rustfmt-indent_style: Visual
// rustfmt-chain_complexity_layout: true

fn fields_and_methods() {
    let _ = parsed.dlx.os.is_empty();
    let _ = x.a.b.iter().count();
    let _ = parsed.dlx.os.iter().count();
    let _ = parsed.get().dlx.os.iter().count();
    let _ = parsed.0.1.iter().count();
}

async fn awaited_methods() {
    let _ = client.registry.fetch().await?;
    let _ = client.registry.fetch().await?.text().await?;
}

fn long_field_prefixes() {
    let _ = packages.configuration.platform.supported_operating_systems;
    let _ = packages.configuration.platform.supported_operating_systems.is_empty();
    let _ = packages.configuration.platform.supported_operating_systems.iter().count();
    let _ = packages.configuration.platform.supported_operating_systems.contains(current_platform);
}

fn unicode_field_prefixes() {
    let _ = packages.configuration.platform.支持平台.操作系统;
    let _ = packages.configuration.platform.支持平台.操作系统.is_empty();
    let _ = packages.configuration.platform.支持平台.操作系统.iter().count();
    let _ = packages.configuration.platform.éééééééééééééééééé;
    let _ = packages.configuration.platform.éééééééééééééééééé.iter().count();
}

fn trailing_fields() {
    let _ = metadata.get("package").expect("metadata").details.deprecated.as_deref();
    let _ = metadata.get("package").expect("metadata").0.1.as_deref();
    let _ = metadata.get("package").0.1;
}

fn single_complex_method_calls() {
    let _ = data.insert(
        "my-config".to_string(),
        Config { name: "workspace:*".to_string(), value: package },
    );
    let _ = values.map(|value| { value.normalize(); value.finish() });
    let _ = values.contains(input.trim().to_lowercase());
    let _ = values.map(|value| { value.normalize(); value.finish() }).metadata.field;
    let _ = values.map(|value| { value.normalize(); value.finish() }).0.1;
    let _ = values.find(nested_function(argument)).metadata;
}
