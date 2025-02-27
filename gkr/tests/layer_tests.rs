pub(crate) mod tests {
    use ark_bn254::Fr;
    use gkr::gate::{Gate,Operation};
    use gkr::layer::{Layer};


    #[test]
    fn test_new() {
        let layer = vec![
            Gate::new(0, 0, 1, Operation::ADD),
            Gate::new(1, 2, 3, Operation::MUL),
            Gate::new(2, 4, 5, Operation::ADD)
        ];
        let res = Layer::new(layer);
        assert_eq!(res.layers[0][0].left, 0);
    }
    #[test]
    fn test_update_layer() {
        let layer0 = vec![
            Gate::new(0, 0, 1, Operation::ADD),
            Gate::new(1, 2, 3, Operation::MUL),
            Gate::new(2, 4, 5, Operation::ADD)
        ];
        let mut res = Layer::new(layer0);
        let layer1 = vec![
            Gate::new(0, 0, 1, Operation::ADD),
            Gate::new(1, 2, 3, Operation::MUL),
            Gate::new(2, 4, 5, Operation::ADD)
        ];
        res.update_layer(layer1);
        assert_eq!(res.layers.len(), 2);
    }
}