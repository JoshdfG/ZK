use ark_bn254::Fq;
use gkr::circuits::circuit::Circuit;
use gkr::circuits::gate::{Gate, Operation};
use gkr::circuits::layer::Layer;
use gkr::gkr_p::{prove, verify};
#[test]
pub fn test_gkr_protocol1() {
    let gate1 = Gate::new(0, 1, 0, Operation::MUL);
    let gate2 = Gate::new(0, 1, 0, Operation::ADD);
    let gate3 = Gate::new(2, 3, 1, Operation::MUL);

    let layer0 = Layer::new(vec![gate1]);
    let layer1 = Layer::new(vec![gate2, gate3]);

    let mut circuit = Circuit::<Fq>::new(vec![layer0, layer1]);
    let inputs = vec![Fq::from(2), Fq::from(3), Fq::from(4), Fq::from(5)];

    let proof = prove(&mut circuit, &inputs);

    assert!(verify(&mut circuit, proof, &inputs));
}

#[test]
pub fn test_gkr_protocol2() {
    // Layer 0
    let gate1 = Gate::new(0, 1, 0, Operation::ADD);
    let layer0 = Layer::new(vec![gate1]);

    // Layer 1
    let gate2 = Gate::new(0, 1, 0, Operation::MUL);
    let gate3 = Gate::new(2, 3, 1, Operation::ADD);
    let layer1 = Layer::new(vec![gate2, gate3]);

    let gate4 = Gate::new(0, 1, 0, Operation::ADD);
    let gate5 = Gate::new(2, 3, 1, Operation::ADD);
    let gate6 = Gate::new(4, 5, 2, Operation::ADD);
    let gate7 = Gate::new(6, 7, 3, Operation::ADD);
    let layer2 = Layer::new(vec![gate4, gate5, gate6, gate7]);

    let mut circuit = Circuit::<Fq>::new(vec![layer0, layer1, layer2]);
    let inputs = vec![
        Fq::from(1),
        Fq::from(2),
        Fq::from(3),
        Fq::from(4),
        Fq::from(5),
        Fq::from(6),
        Fq::from(7),
        Fq::from(8),
    ];

    let proof = prove(&mut circuit, &inputs);

    assert!(verify(&mut circuit, proof, &inputs));
}
