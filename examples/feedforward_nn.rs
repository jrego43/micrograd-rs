extern crate micrograd_rs;
use micrograd_rs::optim::{Optimizer, SGDConfig};
use micrograd_rs::prelude::*;
use micrograd_rs::{Activation, Criterion, Linear, Module, Reduction, Sequential};

fn main() {
    let feedforward = sequential!(
        Ix1,
        [
            Linear::new(3, 4),
            Activation::Tanh,
            Linear::new(4, 4),
            Activation::Tanh,
            Linear::new(4, 1),
            Activation::Tanh
        ]
    );

    let xs = tensor![
        [2.0, 3.0, -1.0],
        [3.0, -1.0, 0.5],
        [0.5, 1.0, 1.0],
        [1.0, 1.0, -1.0]
    ];
    let ys = tensor!([[1.], [-1.], [-1.], [1.]], requires_grad = false);
    let mut ypred: Tensor<Ix2> = Tensor::zeros((4, 1));

    let criterion = Criterion::MSE;
    let mut optimizer = Optimizer::SGD(
        feedforward.parameters(),
        SGDConfig {
            lr: 0.1,
            momentum: 0.3,
            ..Default::default()
        },
    );

    for epoch in 0..20 {
        ypred = feedforward.forward_batch(xs.clone());
        let loss: Value = criterion.loss(Reduction::Sum, ypred.clone(), ys.clone());

        optimizer.zero_grad();
        loss.backward();
        optimizer.step();

        if epoch % 10 == 0 {
            println!("[EPOCH-{:?}] Loss: {:?}", epoch, loss.value());
        }
    }

    if !ypred.is_empty() {
        ypred.iter().for_each(|pred| println!("{:?}", pred.value()));
    }
}
