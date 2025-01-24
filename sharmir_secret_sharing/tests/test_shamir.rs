use ark_bn254::Fq;
use sharmir_secret_sharing::shamir_secret_sharing::{recover_secret, shares};
#[test]
fn test_recover_secret() {
    // 2x^3 + x^2 + 5
    // f(0) = 5
    let secret = Fq::from(5);
    let threshold = 4;
    let number_of_shares = 10;

    let shares = shares(secret, threshold, number_of_shares);

    let recovered_secret = recover_secret(shares);

    // let wrong_secret = Fq::from(2);
    assert_eq!(recovered_secret, secret);
}

#[test]
fn test_recover_wrong_secret_fails() {
    // 2x^3 + x^2 + 5
    // f(0) = 5
    let secret = Fq::from(5);
    let threshold = 3;
    let number_of_shares = 9;

    let shares = shares( secret, threshold, number_of_shares);

    let recovered_secret = recover_secret(shares);

    assert_ne!(recovered_secret, Fq::from(10));
}
