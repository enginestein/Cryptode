use num_bigint::{BigInt, BigUint};
use num_integer::Integer;
use num_traits::{One, Zero};
use rand::Rng;

pub struct DiffieHellman {
    p: BigUint, 
    g: BigUint, 
    private_key: BigUint,
    public_key: Option<BigUint>,
    shared_secret: Option<BigUint>,
}

impl DiffieHellman {
    pub fn new() -> DiffieHellman {
        let mut rng = rand::thread_rng();
        let p = generate_prime();
        let g = generate_generator(&p);
        let private_key = generate_private_key(&p, &mut rng);

        DiffieHellman {
            p,
            g,
            private_key,
            public_key: None,
            shared_secret: None,
        }
    }

    pub fn generate_public_key(&mut self) -> BigUint {
        let public_key = self.g.modpow(&self.private_key, &self.p);
        self.public_key = Some(public_key.clone());
        public_key
    }

    pub fn compute_shared_secret(&mut self, other_public_key: &BigUint) -> BigUint {
        let shared_secret = other_public_key.modpow(&self.private_key, &self.p);
        self.shared_secret = Some(shared_secret.clone());
        shared_secret
    }

    pub fn get_public_key(&self) -> Option<&BigUint> {
        self.public_key.as_ref()
    }

    pub fn get_shared_secret(&self) -> Option<&BigUint> {
        self.shared_secret.as_ref()
    }
}

fn generate_prime() -> BigUint {
    let mut rng = rand::thread_rng();
    let prime_size = 256; 

    let prime = loop {
        let candidate = BigUint::from(rng.gen_biguint(prime_size));
        if candidate.is_prime() {
            break candidate;
        }
    };

    prime
}

fn generate_generator(p: &BigUint) -> BigUint {
    let two = BigUint::from(2u32);
    let phi_p = p - BigUint::one();
    let generator = loop {
        let candidate = BigUint::from(rand::thread_rng().gen_range(2..=&phi_p));
        if candidate.modpow(&two, p) != BigUint::one()
            && candidate.modpow(&phi_p, p) != BigUint::one()
        {
            break candidate;
        }
    };

    generator
}

fn generate_private_key(p: &BigUint, rng: &mut rand::rngs::ThreadRng) -> BigUint {
    let private_key = BigUint::from(rng.gen_range(2..=p - BigUint::one()));
    private_key
}
