use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub fn first_letter_to_uppper_case (s1: &String) -> String {
    let mut c = s1.chars();
    match c.next() {
      None => String::new(),
      Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn random_key(len: usize) -> String {
  thread_rng()
      .sample_iter(&Alphanumeric)
      .take(len)
      .collect()
}