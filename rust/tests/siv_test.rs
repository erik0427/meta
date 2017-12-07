extern crate miscreant;

mod siv_vectors;

use miscreant::Buffer;
use miscreant::siv::{Aes128Siv, Aes256Siv, Aes128PmacSiv, Aes256PmacSiv};
use siv_vectors::{AesSivExample, AesPmacSivExample};

const IV_SIZE: usize = 16;

#[test]
fn aes_siv_examples_seal() {
    let examples = AesSivExample::load_all();

    for example in examples {
        let len = example.plaintext.len();
        let mut buffer = Buffer::from(vec![0; len + IV_SIZE]);
        buffer.mut_msg_slice().copy_from_slice(&example.plaintext);

        match example.key.len() {
            32 => {
                let mut siv = Aes128Siv::new(&example.key);
                siv.seal_in_place(&example.ad, &mut buffer);
            }
            64 => {
                let mut siv = Aes256Siv::new(&example.key);
                siv.seal_in_place(&example.ad, &mut buffer);
            }
            _ => panic!("unexpected key size: {}", example.key.len()),
        };

        assert_eq!(buffer.as_slice(), example.ciphertext.as_slice());
    }
}

#[test]
fn aes_siv_examples_open() {
    let examples = AesSivExample::load_all();

    for example in examples {
        let mut buffer = Buffer::from(example.ciphertext.clone());

        match example.key.len() {
            32 => {
                let mut siv = Aes128Siv::new(&example.key);
                siv.open_in_place(&example.ad, &mut buffer)
            }
            64 => {
                let mut siv = Aes256Siv::new(&example.key);
                siv.open_in_place(&example.ad, &mut buffer)
            }
            _ => panic!("unexpected key size: {}", example.key.len()),
        }.expect("successful decrypt");

        assert_eq!(buffer.msg_slice(), example.plaintext.as_slice());
    }
}

#[test]
fn aes_pmac_siv_examples_seal() {
    let examples = AesPmacSivExample::load_all();

    for example in examples {
        let len = example.plaintext.len();
        let mut buffer = Buffer::from(vec![0; len + IV_SIZE]);
        buffer.mut_msg_slice().copy_from_slice(&example.plaintext);

        match example.key.len() {
            32 => {
                let mut siv = Aes128PmacSiv::new(&example.key);
                siv.seal_in_place(&example.ad, &mut buffer);
            }
            64 => {
                let mut siv = Aes256PmacSiv::new(&example.key);
                siv.seal_in_place(&example.ad, &mut buffer);
            }
            _ => panic!("unexpected key size: {}", example.key.len()),
        };

        assert_eq!(buffer.as_slice(), example.ciphertext.as_slice());
    }
}

#[test]
fn aes_pmac_siv_examples_open() {
    let examples = AesPmacSivExample::load_all();

    for example in examples {
        let mut buffer = Buffer::from(example.ciphertext.clone());

        match example.key.len() {
            32 => {
                let mut siv = Aes128PmacSiv::new(&example.key);
                siv.open_in_place(&example.ad, &mut buffer)
            }
            64 => {
                let mut siv = Aes256PmacSiv::new(&example.key);
                siv.open_in_place(&example.ad, &mut buffer)
            }
            _ => panic!("unexpected key size: {}", example.key.len()),
        }.expect("successful decrypt");

        assert_eq!(buffer.msg_slice(), example.plaintext.as_slice());
    }
}
