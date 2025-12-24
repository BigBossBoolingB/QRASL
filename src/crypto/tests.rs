use crate::crypto::*;

#[test]
fn test_generate_keypair() {
    let (pk, sk) = generate_keypair();
    assert_eq!(pk.len(), PUBLIC_KEY_LENGTH);
    assert_eq!(sk.len(), SECRET_KEY_LENGTH);
}

#[test]
fn test_sign_and_verify() {
    let (pk, sk) = generate_keypair();
    let message = b"This is a test message.";
    let signature = sign(message, &sk);
    assert!(verify(&signature, message, &pk));
}

#[test]
fn test_invalid_signature() {
    let (pk, _) = generate_keypair();
    let message = b"This is a test message.";
    let (_, sk2) = generate_keypair();
    let signature = sign(message, &sk2);
    assert!(!verify(&signature, message, &pk));
}

#[test]
fn test_hash() {
    let data = b"This is some data to hash.";
    let hash = hash(data);
    assert_eq!(hash.len(), 32);
}
