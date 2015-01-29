extern crate md5;

use md5::Md5;

#[test]
fn test_ussage() {
	let mut hasher = Md5::new();
	hasher.input_str("The quick brown fox jumps over the lazy dog");
	let output = &hasher.result_str()[];
	assert_eq!(output, "9e107d9d372bb6826bd81d3542a419d6");
}
