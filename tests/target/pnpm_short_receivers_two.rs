// rustfmt-edition: 2024
// rustfmt-style_edition: 2024
// rustfmt-chain_complexity_layout: true
// rustfmt-chain_width: 0
// rustfmt-chain_head_width: 80
// rustfmt-max_width: 100
// rustfmt-tab_spaces: 2

fn indentation_boundary() {
  let _ = x.iter()
    .copied()
    .collect::<Vec<_>>();
  let _ = ab.iter()
    .copied()
    .collect::<Vec<_>>();
  let _ = abc
    .iter()
    .copied()
    .collect::<Vec<_>>();
  let _ = self
    .iter()
    .copied()
    .collect::<Vec<_>>();
  let _ = items
    .iter()
    .copied()
    .collect::<Vec<_>>();
  let _ = abcdefgh
    .iter()
    .copied()
    .collect::<Vec<_>>();
  let _ = abcdefghi
    .iter()
    .copied()
    .collect::<Vec<_>>();
  let _ = 中.iter()
    .copied()
    .collect::<Vec<_>>();
  let _ = 中中
    .iter()
    .copied()
    .collect::<Vec<_>>();
}
