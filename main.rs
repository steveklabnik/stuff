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

#[cfg(test)]
fn test_hex_to_base64() {
  let expected = ~"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
  let outcome = hex_to_base64(expected);

  assert_eq!(expected, outcome);
}

fn main() {
  let nums = [1,2,3,4,5,6,7];

  for uint::range_step(0, nums.len() - 1, 2) |i| {
    println(fmt!("%d & %d", nums[i], nums[i+1]));
  }
}
