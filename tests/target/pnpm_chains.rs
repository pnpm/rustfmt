// rustfmt-edition: 2024
// rustfmt-style_edition: 2024
// rustfmt-max_width: 100
// rustfmt-chain_method_calls_one_per_line: true

fn field_receivers() {
    let _ = parsed.dlx.os;
    let _ = parsed.dlx.os.is_empty();
    let _ = packages.iter();
    let _ = x
        .iter()
        .count();
    let _ = x.a.b
        .iter()
        .count();
    let _ = parsed.dlx.os
        .iter()
        .count();
    let _ = packages
        .iter()
        .map(|package| package.name())
        .collect::<Vec<_>>();
}

fn tuple_receivers() {
    let _ = parsed.0.1;
    let _ = parsed.0.1.is_empty();
    let _ = parsed.0.1
        .iter()
        .count();
    let _ = parsed
        .first().0.1
        .iter()
        .count();
}

fn fields_after_methods() {
    let _ = parsed.get().dlx.os;
    let _ = parsed
        .get().dlx.os
        .is_empty();
    let _ = parsed
        .get().dlx.os
        .iter()
        .count();
    let _ = parsed
        .get().dlx.os
        .clone().length;
    let _ = parsed
        .get().dlx.os?
        .iter()
        .count();
}

fn nested_arguments() {
    let _ = output.contains(input.trim());
    let _ = output.contains(
        input
            .trim()
            .to_lowercase(),
    );
    let _ = output.values
        .map(|value| value.name.trim())
        .collect::<Vec<_>>();
    let _ = build(
        input
            .trim()
            .to_lowercase(),
    ).value
    .is_empty();
    let _ = Vec::new()
        .iter()
        .count();
}

async fn postfix_operators() {
    let _ = future.await?;
    let _ = client.registry.fetch().await?;
    let _ = client.registry
        .fetch().await?
        .text().await?;
    let _ = client.registry.fetch()?.value.name;
    let _ = client.registry
        .fetch()?.value.name
        .trim();
    let _ = future.await?.value.name.is_empty();
    let _ = future.await?.value.name
        .trim()
        .is_empty();
}

fn comments() {
    let _ = parsed.dlx // platform selection
        .os
        .is_empty();
    let _ = parsed // package metadata
        .dlx.os
        .iter()
        .count();
    let _ = parsed
        .get() // resolved package
        .dlx.os
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
