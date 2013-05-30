extern mod std;
use std::base64::ToBase64;

fn hex_to_base64(source: &str) -> ~str {
  let bytes = str::to_bytes(source);
  let mut nums: ~[u8] = ~[];
  let mut i = 0;

  while i < bytes.len() {
    let num = u8::parse_bytes(bytes.slice(i, i+2), 16);
    match num { Some(n) => nums.push(n), None => () }
    i += 2;
  };
  nums.to_base64()
}

#[cfg(test)]
fn test_hex_to_base64() {
  let expected = ~"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
  let outcome = hex_to_base64(expected);

  assert_eq!(expected, outcome);
}
