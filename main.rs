extern mod std;
use std::base64::ToBase64;

fn hex_to_base64(source: &str) -> ~str {
  let bytes = str::to_bytes(source);
  let mut nums: ~[u8] = ~[];

  for uint::range_step(0, bytes.len(), 2) |i| {
    let num = u8::parse_bytes(bytes.slice(i, i+2), 16);
    match num { Some(n) => nums.push(n), None => () }
  }

  nums.to_base64()
}

fn to_hex(bytes: ~[u8]) -> ~str {
  let hex: @mut ~str = @mut ~"";
  for bytes.each |byte| {
    hex.push_str(u8::to_str_radix(*byte, 16));
  };
  (*hex).clone()
}

fn xor_sum(one: &str, two: &str) -> ~str {
  if one.len() != two.len() {
    fail!(~"lol");
  }
  let bytes  = str::to_bytes(one);
  let bytes2 = str::to_bytes(two);
  let mut nums: ~[u8] = ~[];

  for uint::range_step(0, bytes.len(), 2) |i| {
    let num  = u8::parse_bytes( bytes.slice(i, i+2), 16);
    let num2 = u8::parse_bytes(bytes2.slice(i, i+2), 16);
    match (num, num2) {
      (Some(a), Some(b)) => {
        nums.push(a ^ b)
      }, 
      _ => ()
    }
  }

  to_hex(nums)
}

#[test]
fn test_xor_sum() {
  let input1 = ~"1c0111001f010100061a024b53535009181c";
  let input2 = ~"686974207468652062756c6c277320657965";

  let expected = ~"746865206b696420646f6e277420706c6179";

  assert_eq!(xor_sum(input1, input2), expected);
}

 

#[test]
fn test_hex_to_base64() {
  let input = ~"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
  let expected = ~"SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
  let outcome = hex_to_base64(input);

  assert_eq!(outcome, expected);
}

fn main() {
  let nums = [1,2,3,4,5,6,7];

  for uint::range_step(0, nums.len() - 1, 2) |i| {
    println(fmt!("%d & %d", nums[i], nums[i+1]));
  }
}
