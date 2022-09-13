use blockchainlib::*;
fn main () {
    let mut block = Block::new(0, 0, vec![0; 32], 0, "Genesis Block".to_owned());

    println!("{:?}", block);

    let h = block.hash();

    print!("{:?}", &h);

    block.hash = h;

    println!("{:?}", block);
}
