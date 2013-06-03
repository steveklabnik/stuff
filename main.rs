extern mod std;
use std::base64::ToBase64;

/// `hex_to_base64` converts a hex-encoded string to a base64 encoded one.
fn hex_to_base_64(hex_string: ~str) -> ~str{

  let bytes = str::to_bytes(hex_string);
  let mut nums: ~[u8] = ~[];

  for uint::range_step(0, bytes.len(), 2) |i| {
    let num = u8::parse_bytes(bytes.slice(i, i+2), 16);
    match num { Some(n) => nums.push(n), None => () }
  }

  nums.to_base64()
}

#[test]
fn convert_hex_to_base64() {
  let input = ~"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
  let expected = ~"SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
  
  assert_eq!(hex_to_base_64(input), expected);
}

