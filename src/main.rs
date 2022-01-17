use ring::{
    rand,
    signature::{self, KeyPair},
};
use std::fs;
use hex::{encode};
use pkcs8::{  };
fn main() {
    let rng = rand::SystemRandom::new();
    let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
    let key_pair = signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).unwrap();
    
    /*const MESSAGE: &[u8] = b"hello, world";
    let sig = key_pair.sign(MESSAGE);
    
    let peer_public_key_bytes = key_pair.public_key().as_ref();
    let peer_public_key = signature::UnparsedPublicKey::new(&signature::ED25519, peer_public_key_bytes);
    peer_public_key.verify(MESSAGE, sig.as_ref());
    */
    fs::write("key_pair", encode(pkcs8_bytes)).unwrap();
    println!("La pair de clé à bien été crée");
}