mod p1_generic;
mod p2_1_trait;
mod p2_2_trait_bound;
mod p2_3_multiple_trait_bounds;
mod p2_3_where;
mod p2_4_return;

use p2_1_trait::{My1, MyTrait};

fn main() {
    let my1 = My1 { x: 10, y: 20 };
    my1.my_f();

    // println!("{}", my1.my_f());
}
