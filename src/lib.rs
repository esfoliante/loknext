#[macro_export]
macro_rules! kv_map {
	($($key: expr => $value: expr), *) => {{
		let mut map = HashMap::new();

		$(map.insert($key, $value); )*
		map
	}};
}
