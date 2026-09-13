// rustfmt-edition: 2024
// rustfmt-style_edition: 2024
// rustfmt-max_width: 100
// rustfmt-struct_lit_width: 0
// rustfmt-struct_pattern_width: 0

fn patterns(command: Command, Pair { first, second }: Pair) {
    match command {
        EnvSubcommand::Use { package_name } => run(package_name),
        Command::Pair { first, second } => pair(first, second),
        Command::Rest { value, .. } => run(value),
        Command::Nested { inner: Inner { value, other }, .. } => pair(value, other),
        Command::Long { first_descriptive_field_name, second_descriptive_field_name } => run(first_descriptive_field_name),
        Command::Comment { first, // preserve field comment
            second } => pair(first, second),
        Command::Comment { /* keep */ first, second } => pair(first, second),
    }
    let Pair { first, second } = pair;
    if let EnvSubcommand::Use { package_name } = command { run(package_name); }
    while let Pair { first, second } = next() { pair(first, second); }
    let _ = |Pair { first, second }: Pair| pair(first, second);
    let Pair { inner: Inner { ref value, .. }, .. } = nested;
    let literal = Pair { first, second };
}

fn width_limit() {
    match command {
        ExtremelyDescriptiveAndLongCommandNameThatConsumesMostOfTheAvailableLineWidth::Use { package_name } => run(package_name),
    }
}
