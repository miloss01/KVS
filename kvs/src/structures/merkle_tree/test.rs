use crate::{MerkleTree, PathItem};

#[test]
fn proba() {
    // let data: Vec<Vec<u8>> = vec![vec![1], vec![2], vec![3], vec![4]];
    let data: Vec<Vec<u8>> = (1..=9).map(|i| vec![i as u8]).collect();
    let merkle_tree: MerkleTree = MerkleTree::new(&data);

    for level in merkle_tree.nodes.iter().rev() {
        println!("{:?}", level);
    }
    // println!("{:?}", merkle_tree.nodes.len());
    // println!("{:?}", merkle_tree.leaves_count);
    // println!("{:?}", merkle_tree.root());

    let proof: Vec<PathItem> = merkle_tree.get_proof(vec![7]);
    println!("{:?}", proof);

    let proof_root: Vec<u8> = merkle_tree.get_root_from_proof(proof);
    println!("{:?}", proof_root);
    println!("{:?}", merkle_tree.root() == proof_root);
}

#[test]
fn test_write_and_load_from_file() {
    let data: Vec<Vec<u8>> = (1..=3).map(|i| vec![i as u8]).collect();
    let merkle_tree: MerkleTree = MerkleTree::new(&data);

    merkle_tree.to_file("test_data/merkle.txt");
    let loaded: MerkleTree = MerkleTree::from_file("test_data/merkle.txt");

    assert_eq!(merkle_tree.nodes, loaded.nodes);
    assert_eq!(merkle_tree.leaves_count, loaded.leaves_count);
}
