use crate::primeGenerator::PrimeGenerator;



struct PrivateKey {
  P: i64,
  Q: i64,
  d: i64,
  e: i64
}

impl PrivateKey {
    pub fn new() -> Self {
      let p = PrimeGenerator::new().get_big_prime();
      let q = PrimeGenerator::new().get_big_prime();
      PrivateKey { P: p, Q: q,e : 65537, d: 0 }
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
  privateKey: Option<PrivateKey>

} 

impl RSAA {
  pub fn new() -> Self {
    RSAA { privateKey: None }
  }

  

}