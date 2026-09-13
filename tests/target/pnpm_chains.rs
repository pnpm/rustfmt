// rustfmt-edition: 2024
// rustfmt-style_edition: 2024
// rustfmt-max_width: 100
// rustfmt-chain_complexity_layout: true

fn field_receivers() {
    let _ = parsed.dlx.os;
    let _ = parsed.dlx.os.is_empty();
    let _ = packages.iter();
    let _ = x.iter().count();
    let _ = x.a.b.iter().count();
    let _ = parsed.dlx.os.iter().count();
    let _ = packages
        .iter()
        .map(|package| package.name())
        .collect::<Vec<_>>();
}

fn tuple_receivers() {
    let _ = parsed.0.1;
    let _ = parsed.0.1.is_empty();
    let _ = parsed.0.1.iter().count();
    let _ = parsed
        .first()
        .0
        .1
        .iter()
        .count();
}

fn fields_after_methods() {
    let _ = parsed.get().dlx.os;
    let _ = parsed.get().dlx.os.is_empty();
    let _ = parsed
        .get()
        .dlx
        .os
        .iter()
        .count();
    let _ = parsed.get().dlx.os.clone().length;
    let _ = parsed
        .get()
        .dlx
        .os?
        .iter()
        .count();
}

fn nested_arguments() {
    let _ = output.contains(input.trim());
    let _ = output.contains(input.trim().to_lowercase());
    let _ = output.values
        .map(|value| value.name.trim())
        .collect::<Vec<_>>();
    let _ = build(input.trim().to_lowercase()).value.is_empty();
    let _ = Vec::new().iter().count();
}

async fn postfix_operators() {
    let _ = future.await?;
    let _ = client.registry.fetch().await?;
    let _ = client.registry.fetch().await?.text().await?;
    let _ = client.registry.fetch()?.value.name;
    let _ = client.registry.fetch()?.value.name.trim();
    let _ = future.await?.value.name.is_empty();
    let _ = future.await?.value.name.trim().is_empty();
}

fn comments() {
    let _ = parsed.dlx // platform selection
        .os
        .is_empty();
    let _ = parsed // package metadata
        .dlx
        .os
        .iter()
        .count();
    let _ = parsed
        .get() // resolved package
        .dlx
        .os
        .iter()
        .count();
    let _ = parsed.dlx.os
        // filter by platform
        .iter()
        .count();
    let _ = parsed.dlx /* selected platform */
        .os
        .is_empty();
}

fn long_single_calls() {
    let _ = parsed.configuration.platform.operating_system
        .matches_any_of_the_supported_operating_systems(operating_system);
    let _ = parsed.configuration.platform.operating_system
        .matches_any_of_the_supported_operating_systems(first, second, third, fourth);
    let _ = parsed.configuration.platform.operating_system.with_platform_configuration(
        |configuration| {
            let supported = configuration.supported_platforms();
            supported.contains(current_platform)
        },
    );
}

fn long_field_prefixes() {
    let _ = packages.configuration.platform.operating_system.supported_platforms
        .supported_operating_systems;
    let _ = packages.configuration.platform.operating_system.supported_platforms
        .supported_operating_systems
        .iter()
        .count();
    let _ = packages.configuration.platform.operating_system.supported_platforms.支持平台.操作系统
        .iter()
        .count();
    let _ = packages.configuration.platform.operating_system.supported_platforms.éééééééééééééééééé
        .iter()
        .count();
}

fn simple_arguments() {
    let _ = map.get(&key).is_some();
    let _ = map.get(&settings.key).is_some();
    let _ = map.get(&mut key).is_some();
    let _ = map.get("key").is_some();
    let _ = map.get(keys::DEFAULT).is_some();
    let _ = map.get((key)).is_some();
    let _ = map.get(-1).is_some();
    let _ = map.get(&*key).is_some();
    let _ = map
        .get(key)
        .unwrap()
        .is_empty();
}

fn complex_arguments() {
    let _ = map
        .get(make_key())
        .is_some();
    let _ = map
        .get(&make_key())
        .is_some();
    let _ = map
        .get(keys[index])
        .is_some();
    let _ = map
        .get(key + suffix)
        .is_some();
    let _ = map
        .get(Key { value: key })
        .is_some();
    let _ = map
        .get({
            let selected = key;
            selected
        })
        .is_some();
    let _ = map
        .get(if condition { first } else { second })
        .is_some();
    let _ = packages
        .iter()
        .any(|package| package.enabled);
    let _ = packages.any(|package| package.enabled);
}

fn multiline_arguments() {
    let _ = map.get(&key).is_some();
    let _ = map.get("first\nsecond").is_some();
    let _ = map
        .get(
            "first
second",
        )
        .is_some();
    let _ = map
        .get(
            r#"first
second"#,
        )
        .is_some();
    let _ = receiver
        .set(
            first_argument_with_long_name,
            second_argument_with_long_name,
            third_argument_with_long_name,
        )
        .finish();
    let _ = receiver
        .get()
        .set(
            first_argument_with_long_name,
            second_argument_with_long_name,
            third_argument_with_long_name,
        );
    let _ = receiver.set(
        first_argument_with_long_name,
        second_argument_with_long_name,
        third_argument_with_long_name,
    );
}

fn conditions() {
    if map.get(&key).is_some() {}
    if map
        .get(&key)
        .unwrap()
        .is_empty()
    {}
    if packages
        .iter()
        .any(|package| package.enabled)
    {}
    while map.get(&key).is_some() {}
    while map
        .get(make_key())
        .is_some()
    {}
    match value {
        Some(value) if value.get(&key).is_some() => (),
        _ => (),
    }
}

fn multiline_receivers() {
    let _ = build_with_callback(|value| {
        let selected = value.enabled;
        selected
    })
    .is_empty();
    let _ = {
        let value = get_value();
        value
    }
    .is_empty();
}

fn trailing_fields_in_vertical_chains() {
    let _ = metadata.get("package").expect("package metadata").deprecated;
    let _ = metadata
        .get("package")
        .expect("package metadata")
        .deprecated
        .as_deref();
    let _ = metadata
        .get("package")
        .expect("package metadata")
        .details
        .deprecated
        .as_deref();
    let _ = metadata
        .get("package")
        .expect("package metadata")
        .0
        .1
        .as_deref();
    let _ = metadata.get(package_name()).details.deprecated;
    let _ = metadata.registry.packages
        .get("package")
        .expect("package metadata")
        .deprecated
        .as_deref();
    let _ = metadata
        .get("package")
        .expect("package metadata")
        .details // keep details
        .deprecated
        .as_deref();
}

async fn fields_after_await_in_vertical_chains() {
    let _ = client.registry
        .fetch().await?
        .value
        .name
        .trim()
        .is_empty();
    let _ = client.registry.fetch(request()).await?.value.name;
}

fn single_complex_method_calls() {
    let _ = importer.config_dependencies.insert(
        "my-config".to_string(),
        SpecifierAndResolution {
            specifier: "workspace:*".to_string(),
            resolution: resolved_package,
        },
    );
    let _ = values.map(|value| {
        value.normalize();
        value.finish()
    });
    let _ = values.contains(input.trim().to_lowercase());
    let _ = values.map(|value| {
        value.normalize();
        value.finish()
    })
    .metadata
    .field;
    let _ = values.map(|value| {
        value.normalize();
        value.finish()
    })
    .0
    .1;
    let _ =
        values.registry.packages.find(very_long_nested_function_name(long_argument_name)).metadata;
}

fn function_call_receivers() {
    let excludes = minimum_release_age_excludes(&advisories, &HashMap::new(), age_cutoff())
        .expect("compute excludes");
    let _ = find_package(name).expect("package exists");
    let _ = find_package(name)
        .map(|value| {
            value.normalize();
            value.finish()
        });
    let _ = find_package(name).metadata
        .insert(
            "my-config".to_string(),
            SpecifierAndResolution {
                specifier: "workspace:*".to_string(),
                resolution: resolved_package,
            },
        );
}
