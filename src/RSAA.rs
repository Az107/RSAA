use std::result;

use crate::primeGenerator::PrimeGenerator;



struct PrivateKey {
  pub P: i64,
  pub Q: i64,
  pub d: i64,
  pub e: i64
}

impl PrivateKey {
    pub fn new() -> Self {
      let p = PrimeGenerator::new().get_big_prime();
      let q = PrimeGenerator::new().get_big_prime();
      PrivateKey { P: p, Q: q,e : 65537, d: 0 }
    }

    fn get_N(&self) -> i64 {
      self.P * self.Q
    }

    fn euclid(a: i64,b: i64) -> i64 {
      let r = a%b;
      if r == 0 {return b}
      return Self::euclid(b,r);
    }

}

struct PublicKey {
  N: i64,
  e: i64
}

impl PublicKey {
  pub fn new(pk: PrivateKey) -> Self {
    let n = pk.P * pk.Q ;
    let totient = (pk.P - 1) * (pk.Q - 1); 
    PublicKey { N: n, e: 65537 }
  }
}

struct RSAA {
  privateKey: Option<PrivateKey>,
  publicKey: Option<PublicKey>

} 

impl RSAA {
  pub fn new() -> Self {
    RSAA { privateKey: None, publicKey: None }
  }

  fn encrypt(&self, message: String) -> Result<Vec<i64>, &str> {
    if self.publicKey.is_none() { return Err("no public key")}
    let mut result = Vec::new();


    return Ok(result);
  }

  fn decrypt(&self, encrypted: Vec<i64>) -> Result<String, &str> {
    let private_key = match &self.privateKey {
      Some(key) => key,
      None => return Err("No hay clave privada"),
  };

    let mut result = "".to_string();
    for i in encrypted.iter() {
      let decrypted = i.pow(private_key.d.try_into().unwrap()) % private_key.get_N();
      result.push_str(&decrypted.to_string());
    }
    return Ok(result);
  }
  

}