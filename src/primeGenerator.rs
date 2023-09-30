
struct SimpleRNG {
  state: u64,
}

impl SimpleRNG {
    fn new() -> Self {
      let seed = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
      SimpleRNG { state: seed }
    }

    fn new_with_seed(seed: u64) -> Self {
      SimpleRNG { state: seed }
    }

    fn next(&mut self) -> u64 {
      self.state = self.state.wrapping_mul(6364136223846793061).wrapping_add(1);
      self.state >> 16
    }

    fn next_range(&mut self, min: u64, max: u64) -> u64 {
      let scaled_range = max - min;
      let scaled_random = self.next() % scaled_range;
      min + scaled_random
    }
}

pub struct PrimeGenerator {
   known_primes: Vec<i64>
}

impl PrimeGenerator {

  pub fn new() -> Self {
    PrimeGenerator { known_primes: Self::eratosthenes_generator(1000) }
  }
  
  pub fn eratosthenes_generator(n: i64) -> Vec<i64> {
    let mut init_list: Vec<i64> = (2..n).collect();
    let mut index = 0;
    while  index < init_list.len() {
      let num = init_list[index];
      let mut num2_index = index + 1;
      while num2_index < init_list.len() {
          let num2 = init_list[num2_index];
          if num2 % num == 0 {
              init_list.remove(num2_index);
          }
          num2_index += 1;
      }
      index += 1;
  }
  init_list
  }

  pub fn is_prime(&self, n: i64) -> bool {
    if n <= 1 {
      return false;
    }
    let sqr = (n as f64).sqrt() as i64;
    for &prime in &self.known_primes {
      if prime > sqr {
          break;
      }
      if n % prime == 0 {
          return false;
      }
    }
    for i in (self.known_primes.last().unwrap() + 2..=sqr).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
  }

  pub fn get_big_prime(&self) -> i64 {
    let mut number = SimpleRNG::new().next() as i64;
    while !(&self.is_prime(number)) {
      number+=1;
    }
    return number;
  }
      
}
