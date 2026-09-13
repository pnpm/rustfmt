// rustfmt-edition: 2024
// rustfmt-style_edition: 2024
// rustfmt-max_width: 100
// rustfmt-chain_width: 40
// rustfmt-chain_head_width: 80
// rustfmt-chain_complexity_layout: true

fn examples() {
    let _ = x.iter().copied().collect::<Vec<_>>();
    let _ = x.map(|s| f(s)).count();
    let _ = descriptive_collection.map(|s| f(s)).collect::<Vec<_>>();
    let _ = descriptive_collection.contains(input.trim()).then_some(value);
    let _ = descriptive_collection
        .contains(input.trim().lowercase())
        .then_some(value);
    let _ = descriptive_collection
        .map(|s| f(g(s)))
        .collect::<Vec<_>>();
    let _ = descriptive_collection
        .map(|s| s.trim().lowercase())
        .collect::<Vec<_>>();
    let _ = descriptive_collection.map(|s| f(s)).collect::<Vec<_>>();
    let _ = descriptive_collection
        .iter()
        .copied()
        .collect::<Vec<_>>();
    let _ = x.map(|s| f(s)).count();
    let _ = x
        .iter() // preserve comment
        .copied()
        .count();
    let _ = xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
        .field
        .iter();
}

fn thresholds() {
    let _ = xxxx.iter().copied().collect::<Vec<_>>();
    let _ = xxxxx
        .iter()
        .copied()
        .collect::<Vec<_>>();
    let binding_binding_binding_binding_binding_binding_binding_binding_binding_ = short
        .field
        .iter();
    let _ = descriptive_collection
        .map(|s| {
            let result = f(s);
            result
        })
        .collect::<Vec<_>>();
    let _ = descriptive_collection
        .map(|s| {
            /* keep */
            f(s)
        })
        .collect::<Vec<_>>();
    let _ = descriptive_collection
        .map(|s| if ready { f(s) } else { g(s) })
        .collect::<Vec<_>>();
}
