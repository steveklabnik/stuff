extern mod std;
use std::base64::ToBase64;

#[test]
fn convert_hex_to_base64() {
  let input = ~"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
  let expected = ~"SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
  
  assert_eq!(hex_to_base_64(input), expected);
}

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
fn test_xor_sum() {
  let plaintext = ~"1c0111001f010100061a024b53535009181c";
  let cypher = ~"686974207468652062756c6c277320657965";
  let cyphertext = ~"746865206b696420646f6e277420706c6179";
 
  assert_eq!(xor_sum(plaintext, cypher), cyphertext)
}

fn xor_sum(plaintext: &str, cypher: &str) -> ~str {
  let plaintext_bytes = str::to_bytes(plaintext);
  let cypher_bytes    = str::to_bytes(cypher);
  let sum: @mut ~str = @mut ~"";

  for uint::range_step(0, plaintext_bytes.len(), 2) |i| {
    let plaintext_num = u8::parse_bytes(plaintext_bytes.slice(i, i+2), 16);
    let cypher_num    = u8::parse_bytes(   cypher_bytes.slice(i, i+2), 16);
    match (plaintext_num, cypher_num) {
      (Some(n), Some(m)) => sum.push_str(u8::to_str_radix((n^m), 16)),
      _ => ()
    }
  }

  (*sum).clone()
}
