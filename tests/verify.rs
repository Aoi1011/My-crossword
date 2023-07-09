use lib::{verify, Message, PublicKey, Signature, ECMULT_CONTEXT, SecretKey};
use secp256k1_test::{rand::thread_rng, Message as SecpMessage, Secp256k1};

#[test]
fn test_verify() {
    let secp256k1 = Secp256k1::new();

    let message_arr = [5u8; 32];
    let (privkey, pubkey) = secp256k1.generate_keypair(&mut thread_rng());
    let message = SecpMessage::from_slice(&message_arr).unwrap();
    let signature = secp256k1.sign(&message, &privkey);

    let pubkey_a = pubkey.serialize_uncompressed();
    assert_eq!(pubkey_a.len(), 65);

    let ctx_pubkey = PublicKey::parse(&pubkey_a).unwrap();
    let ctx_message = Message::parse(&message_arr);
    let signature_a = signature.serialize_compact();
    assert_eq!(signature_a.len(), 64);
    let ctx_sig = Signature::parse_standard(&signature_a).expect("signature is valid");

    secp256k1.verify(&message, &signature, &pubkey).unwrap();
    assert!(verify(&ctx_message, &ctx_sig, &ctx_pubkey));
    let mut f_ctx_sig = ctx_sig;
    f_ctx_sig.r.set_int(0);
    if f_ctx_sig.r != ctx_sig.r {
        assert!(!ECMULT_CONTEXT.verify_raw(
            &f_ctx_sig.r,
            &ctx_sig.s,
            &ctx_pubkey.into(),
            &ctx_message.0
        ));
    }
    f_ctx_sig.r.set_int(1);
    if f_ctx_sig.r != ctx_sig.r {
        assert!(!ECMULT_CONTEXT.verify_raw(
            &f_ctx_sig.r,
            &ctx_sig.s,
            &ctx_pubkey.into(),
            &ctx_message.0
        ));
    }
}

#[test]
fn secret_clear_on_drop() {
    let secret: [u8; 32] = [1; 32];
    let mut seckey = SecretKey::parse(&secret).unwrap();

    clear_on_drop::clear::Clear::clear(&mut seckey);
    assert_eq!(seckey, SecretKey::default());
}
