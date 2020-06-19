use crate::parser;

fn test_should_parse(should_parse: bool, input: &str) {
	let res = parser::parse(input);
	if should_parse {
		res.unwrap();
	} else {
		assert!(res.is_err());
	}
}

#[test]
fn should_parse() {
	test_should_parse(true,  "select foo");
	test_should_parse(false, "select foo;");
	test_should_parse(true,  "select foo from bar");
	test_should_parse(true,  "select foo from bar where true");
	test_should_parse(true,  "select foo from bar join baz");
	test_should_parse(true,  "select foo from bar join baz on true");
	test_should_parse(true,  "select foo from bar join baz join barbaz");
	test_should_parse(true,  "select foo from bar join baz on true join barbaz on false");
}
