// rustfmt-indent_style: Visual
// rustfmt-edition: 2024
// rustfmt-style_edition: 2024
// rustfmt-chain_complexity_layout: true
// rustfmt-chain_width: 40
// rustfmt-chain_head_width: 80
// rustfmt-max_width: 100

fn short_receivers() {
    let descriptive_binding = x
                               .iter()
                               .map(transform_element)
                               .collect::<Vec<_>>();
    let descriptive_binding = deps
                                  .iter()
                                  .map(transform_element)
                                  .collect::<Vec<_>>();
    let descriptive_binding = items
                                   .iter()
                                   .map(transform_element)
                                   .collect::<Vec<_>>();
    let descriptive_binding = values
                                    .iter()
                                    .map(transform_element)
                                    .collect::<Vec<_>>();
    let descriptive_binding = self
                                  .iter()
                                  .map(transform_element)
                                  .collect::<Vec<_>>();
    let descriptive_binding = x.a
                                 .iter()
                                 .map(transform_element)
                                 .collect::<Vec<_>>();
    let descriptive_binding = self.a
                                    .iter()
                                    .map(transform_element)
                                    .collect::<Vec<_>>();
    let descriptive_binding = get()
                                   .iter()
                                   .map(transform_element)
                                   .collect::<Vec<_>>();
    let descriptive_binding = ééééé
                                   .iter()
                                   .map(transform_element)
                                   .collect::<Vec<_>>();
    let descriptive_binding = 中中
                                  .iter()
                                  .map(transform_element)
                                  .collect::<Vec<_>>();
    let descriptive_binding = 中中中
                                    .iter()
                                    .map(transform_element)
                                    .collect::<Vec<_>>();
    let _ = x
             .a()
             .b()
             .some_much_longer_final_method_name();
    let _ = x
             .map(nested_call(value))
             .some_much_longer_final_method_name();
    let _ = x
             .map(|value| {
                 let result = process(value);
                 result
             })
             .collect::<Vec<_>>();
    let _ = x
             .iter()
             .field
             .await?
             .map(transform_element)
             .collect::<Vec<_>>();
    let _ = x
             .iter()
             .map(transform_element)
             .field
             .await?
             .collect::<Vec<_>>();
    let _ = items /* receiver comment */
                 .iter()
                 .map(transform_element)
                 .collect::<Vec<_>>();
    let _ = items
                 .iter() // method comment
                 .map(transform_element)
                 .collect::<Vec<_>>();
    let _ = items
                 .insert(first_long_argument_name,
                         second_long_argument_name,
                         third_long_argument_name)
                 .map(transform_element)
                 .collect::<Vec<_>>();
}

fn prefix_widths() {
    x.iter()
            .map(transform_element)
            .collect::<Vec<_>>();
    let value = x
                 .iter()
                 .map(transform_element)
                 .collect::<Vec<_>>();
    self.iter()
               .map(transform_element)
               .collect::<Vec<_>>();
    let value = self
                    .iter()
                    .map(transform_element)
                    .collect::<Vec<_>>();
    deps.iter()
               .map(transform_element)
               .collect::<Vec<_>>();
    let value = deps
                    .iter()
                    .map(transform_element)
                    .collect::<Vec<_>>();
    items
         .iter()
         .map(transform_element)
         .collect::<Vec<_>>();
    let value = items
                     .iter()
                     .map(transform_element)
                     .collect::<Vec<_>>();
    中中.iter()
               .map(transform_element)
               .collect::<Vec<_>>();
    let value = 中中
                    .iter()
                    .map(transform_element)
                    .collect::<Vec<_>>();
    let entries = text
                      .lines()
                      .filter_map(parse_entry)
                      .collect::<Vec<_>>();
    if x.iter()
               .map(transform_element)
               .all(predicate)
    {}
    if xs
         .iter()
         .map(transform_element)
         .all(predicate)
    {}
    while x
           .iter()
           .map(transform_element)
           .all(predicate)
    {}
    return x
            .iter()
            .map(transform_element)
            .collect::<Vec<_>>();
}

async fn assignment_fallback() {
    let url = args
                  .documentation_url(&config_for(&server.url()))
                  .await
                  .expect("docs URL must resolve");
    let _ = args
                .documentation_url(&config_for(&server.url()))
                .await?
                .field
                .expect("resolve");
    let _ = args
                .map(nested_call(value))
                .expect("some longer expectation text");
    let _ = short.field;
    args.documentation_url(&config_for(&server.url())).await
                                                            .expect("docs URL must resolve");
    let _ = outer
                 .map(|value| {
                     let inner = args
                                     .documentation_url(&config_for(&server.url()))
                                     .await
                                     .expect("docs URL must resolve");
                     inner
                 })
                 .expect("resolve");
}

fn long_assignment_prefix() {
    let long_binding_long_binding_long_binding_long_binding_long_binding_long_binding_ =
        args
            .documentation_url(&config_for(&server.url()))
            .expect("docs URL must resolve");
    args.documentation_url(&config_for(&server.url()))
                                                      .expect("docs URL must resolve");
}

async fn wrapped_assignment_values() {
    let url = args
                  .documentation_url(&config_for(&server.url()))
                  .await
                  .expect("docs URL must resolve")?;
    let url = (x
                .documentation_url(&config_for(&server.url()))
                .await
                .expect("docs URL must resolve"));
    let url = (x
                .documentation_url(&config_for(&server.url()))
                .await
                .expect("docs URL must resolve"))?;
    let url = (x
                .documentation_url(&config_for(&server.url()))
                .await
                .expect("docs URL must resolve"));
}
