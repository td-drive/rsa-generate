extern crate rsa;
extern crate rand;
extern crate base64;

use std::fs::File;
use std::io::{self, Read, Write, BufReader};
use rsa::{PublicKey, RsaPrivateKey, RsaPublicKey, PaddingScheme};
use rand::rngs::OsRng;
use base64::{encode, decode};

//鍵のデータ
struct KeyData {
    privateKey: RsaPrivateKey,
    publicKey: RsaPublicKey
}

fn main() {
    //鍵長
    let bits: usize = 2048;

    //GenerateKeyを呼び出してRSA鍵を生成する
    let key: KeyData = GenerateKey(bits);

    println!("private key: {:?}", key.privateKey);
    println!("public key: {:?}", key.publicKey);
}

fn GenerateKey(bits: usize) -> KeyData {
    //乱数
    let mut rng = OsRng;

    //秘密鍵を生成する
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate a key");
    //公開鍵を生成する
    let public_key = RsaPublicKey::from(&private_key);

    return KeyData{privateKey: private_key, publicKey: public_key};
}