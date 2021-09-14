use libp2p::{identity, PeerId};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // create network identity.
    // A Network identity is a product of public and private key.
    let local_key = identity::Keypair::generate_ed25519();

    // Derive a peer_id from the public key
    let local_peer_id = PeerId::from(local_key.public());

    println!("Local peer id: {:?}", local_peer_id);

    Ok(())
}
