pub(crate) mod tests {
    use ark_bn254::Fr;
    use gkr::gate::{Gate,Operation};
    use gkr::layer::{Layer};


    #[test]
    fn test_new() {
        let layer = vec![
            Gate::new(0, 1, 1, Operation::ADD),
            Gate::new(1, 2, 3, Operation::MUL),
            Gate::new(2, 4, 5, Operation::ADD)
        ];
        let res = Layer::new(layer);
        assert_eq!(res.layers[0][2].left, 2);
    }
    #[test]
    fn test_new_right() {
        let layer = vec![
            Gate::new(0, 1, 1, Operation::ADD),
            Gate::new(1, 2, 3, Operation::MUL),
            Gate::new(2, 4, 5, Operation::ADD)
        ];
        let res = Layer::new(layer);
        assert_eq!(res.layers[0][2].right, 4);
    }
    #[test]
    fn test_update_layer() {
        let layer0 = vec![
            Gate::new(0, 1, 1, Operation::ADD),
            Gate::new(1, 2, 3, Operation::MUL),
            Gate::new(2, 4, 5, Operation::ADD)
        ];
        let mut res = Layer::new(layer0);
        let layer1 = vec![
            Gate::new(0, 1, 1, Operation::ADD),
            Gate::new(1, 2, 3, Operation::MUL),
            Gate::new(2, 4, 5, Operation::ADD)
        ];
        res.update_layer(layer1);
        assert_eq!(res.layers.len(), 2);
        let layer2 = vec![
            Gate::new(3, 5, 15, Operation::MUL),
            Gate::new(3, 2, 6, Operation::MUL),
            Gate::new(4, 4, 5, Operation::ADD)
        ];
        res.update_layer(layer2);
        assert_eq!(res.layers.len(), 3);

    }
}