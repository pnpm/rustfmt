// rustfmt-edition: 2024
// rustfmt-style_edition: 2024
// rustfmt-chain_complexity_layout: true
// rustfmt-chain_width: 0
// rustfmt-chain_head_width: 80
// rustfmt-max_width: 100
// rustfmt-tab_spaces: 8
// rustfmt-hard_tabs: true

fn indentation_boundary() {
	let _ = x
		.iter()
		.copied()
		.collect::<Vec<_>>();
	let _ = ab
		.iter()
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
	let _ = 中
		.iter()
		.copied()
		.collect::<Vec<_>>();
	let _ = 中中
		.iter()
		.copied()
		.collect::<Vec<_>>();
	let _ = x
		.a()
		.b()
		.some_much_longer_final_method_name();
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
	items.iter()
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
	if xs.iter()
		.map(transform_element)
		.all(predicate)
	{}
	while x.iter()
		.map(transform_element)
		.all(predicate)
	{}
	return x.iter()
		.map(transform_element)
		.collect::<Vec<_>>();
}
