// rustfmt-edition: 2024
// rustfmt-style_edition: 2024
// rustfmt-max_width: 100
// rustfmt-chain_complexity_layout: true
// rustfmt-chain_head_width: 80

async fn example() {
    let err = login::<FakeHost, RecordingReporter>(&client(), opts(&registry, config_dir))
        .await
        .unwrap_err();
    let _ = packages.iter();
    let _ = importer.config_dependencies.insert("my-config".to_string(), Config { first, second });
    let _ = short.fields.await?.0.value;
    let _ = longer_name.field // preserve field comment
        .await?
        .value;
}

fn boundaries() {
    let _ = xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx.value;
    let _ = xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx.value;
    let _ = xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
        .value;
    let _ = xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx.0;
    let _ = xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx.0;
    let _ = xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
        .0;
    let _ = xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx.await?;
    let _ = xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx.await?;
    let _ = xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
        .await?;
    let _ = xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx.first
        .second
        .third;
    if ready {
        let longer_binding = settings.configuration.platform.current.target;
    }
    let _ = config.支持平台.操作系统.iter().count();
    let _ = config.éééééééééééééééééééééééééééééééééééééééééééééééééé.value;
}

fn nested_assignments() {
    let _ = xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
        .field;
    let _ = ééééééééééééééééééééééééééééééééééééééééééééééééééééééééééééééé
        .value;
    let _ = 中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中中x
        .value;
    let _ = xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
        .value;
    let _ = short.field;
    let _ = wrap(
        xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
            .field,
    );
    let _ = {
        let inner = xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
            .value;
        inner
    }
    .field;
}
