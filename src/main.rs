use libp2p::{identity, PeerId};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("local key {:?}", local_key);
    println!("local peer id {:?}", local_peer_id);
    Ok(())
}
