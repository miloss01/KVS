use crate::{MerkleTree, PathItem};
use sha2::{Digest, Sha256};

fn hash(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
    // data.to_vec()
}

#[test]
fn test_even_leaves_no_addings() {
    let data: Vec<Vec<u8>> = (1..=8).map(|i| i.to_string().into_bytes()).collect();
    let merkle_tree: MerkleTree = MerkleTree::new(&data);

    let data: Vec<Vec<u8>> = data.iter().map(|leaf| hash(&leaf)).collect();
    let n12: Vec<u8> = hash(&[&data[0][..], &data[1][..]].concat());
    let n34: Vec<u8> = hash(&[&data[2][..], &data[3][..]].concat());
    let n56: Vec<u8> = hash(&[&data[4][..], &data[5][..]].concat());
    let n78: Vec<u8> = hash(&[&data[6][..], &data[7][..]].concat());
    let n1234: Vec<u8> = hash(&[&n12[..], &n34[..]].concat());
    let n5678: Vec<u8> = hash(&[&n56[..], &n78[..]].concat());
    let root: Vec<u8> = hash(&[&n1234[..], &n5678[..]].concat());

    assert_eq!(merkle_tree.root(), root);
    assert_eq!(merkle_tree.leaves_count, 8);
}

#[test]
fn test_even_leaves_with_addings() {
    let data: Vec<Vec<u8>> = (1..=10).map(|i| i.to_string().into_bytes()).collect();
    let merkle_tree: MerkleTree = MerkleTree::new(&data);

    let data: Vec<Vec<u8>> = data.iter().map(|leaf| hash(&leaf)).collect();
    let n12: Vec<u8> = hash(&[&data[0][..], &data[1][..]].concat());
    let n34: Vec<u8> = hash(&[&data[2][..], &data[3][..]].concat());
    let n56: Vec<u8> = hash(&[&data[4][..], &data[5][..]].concat());
    let n78: Vec<u8> = hash(&[&data[6][..], &data[7][..]].concat());
    let n910: Vec<u8> = hash(&[&data[8][..], &data[9][..]].concat());
    let n1234: Vec<u8> = hash(&[&n12[..], &n34[..]].concat());
    let n5678: Vec<u8> = hash(&[&n56[..], &n78[..]].concat());
    let n910910: Vec<u8> = hash(&[&n910[..], &n910[..]].concat());
    let n12345678: Vec<u8> = hash(&[&n1234[..], &n5678[..]].concat());
    let n910910910910: Vec<u8> = hash(&[&n910910[..], &n910910[..]].concat());
    let root: Vec<u8> = hash(&[&n12345678[..], &n910910910910[..]].concat());

    assert_eq!(merkle_tree.root(), root);
    assert_eq!(merkle_tree.leaves_count, 10);
}

#[test]
fn test_odd_leaves() {
    let data: Vec<Vec<u8>> = (1..=9).map(|i| i.to_string().into_bytes()).collect();
    let merkle_tree: MerkleTree = MerkleTree::new(&data);

    let data: Vec<Vec<u8>> = data.iter().map(|leaf| hash(&leaf)).collect();
    let n12: Vec<u8> = hash(&[&data[0][..], &data[1][..]].concat());
    let n34: Vec<u8> = hash(&[&data[2][..], &data[3][..]].concat());
    let n56: Vec<u8> = hash(&[&data[4][..], &data[5][..]].concat());
    let n78: Vec<u8> = hash(&[&data[6][..], &data[7][..]].concat());
    let n99: Vec<u8> = hash(&[&data[8][..], &data[8][..]].concat());
    let n1234: Vec<u8> = hash(&[&n12[..], &n34[..]].concat());
    let n5678: Vec<u8> = hash(&[&n56[..], &n78[..]].concat());
    let n9999: Vec<u8> = hash(&[&n99[..], &n99[..]].concat());
    let n12345678: Vec<u8> = hash(&[&n1234[..], &n5678[..]].concat());
    let n99999999: Vec<u8> = hash(&[&n9999[..], &n9999[..]].concat());
    let root: Vec<u8> = hash(&[&n12345678[..], &n99999999[..]].concat());

    assert_eq!(merkle_tree.root(), root);
    assert_eq!(merkle_tree.leaves_count, 10);
}

#[test]
fn test_merkle_tree_proof() {
    for num_of_elements in 1..10 {
        let data: Vec<Vec<u8>> = (1..=num_of_elements)
            .map(|i| i.to_string().into_bytes())
            .collect();
        let merkle_tree: MerkleTree = MerkleTree::new(&data);

        for el in 1..num_of_elements {
            let proof: Vec<PathItem> = merkle_tree.get_proof(el.to_string().into_bytes());
            let proof_root: Vec<u8> = MerkleTree::get_root_from_proof(proof);
            assert_eq!(merkle_tree.root(), proof_root);
        }
    }
}

#[test]
fn test_write_and_load_from_file() {
    let data: Vec<Vec<u8>> = (1..=3).map(|i| vec![i as u8]).collect();
    let merkle_tree: MerkleTree = MerkleTree::new(&data);

    merkle_tree.to_file("test_data/merkle.txt");
    let loaded: MerkleTree = MerkleTree::from_file("test_data/merkle.txt");

    assert_eq!(merkle_tree, loaded);
}
