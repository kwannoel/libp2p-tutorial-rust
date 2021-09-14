use futures::executor::block_on;
use libp2p::{identity, PeerId};
use libp2p::ping::{Ping, PingConfig}
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // create network identity.
    // A Network identity is a product of public and private key.
    let local_key = identity::Keypair::generate_ed25519();

    // Derive a peer_id from the public key
    let local_peer_id = PeerId::from(local_key.public());

    println!("Local peer id: {:?}", local_peer_id);

    // How to send and receive bytes
    // Create a transport for this peer
    // Wait for this to exit and return a result.
    let transport = block_on(libp2p::development_transport(local_key))?;

    // What bytes to send?
    // Define the message
    // Also persist the connection so we can pingpong repeatedly
    let behaviour = Ping::new(PingConfig::new().with_keep_alive(true));

    // A transport needs to receive messages it should send
    // Behaviour is reactive to events
    // We need something to wire the two such that:
    // transport receives messages from behaviour to send
    // Behaviour receives incoming events from transport
    // For that we use a swarm:
    let mut swarm = Swarm::new(transport, behaviour, local_peer_id);

    // We can now listen for incoming connections.
    // This tell us to listen on all interfaces and a random OS-assigned port
    // NOTE: This is a multiaddr. See: https://docs.rs/libp2p/0.39.1/libp2p/struct.Multiaddr.html
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // Dial the peer identified by the multi-address
    // if it was supplied as a cli arg.
    if let Some(addr) = std::env::args().nth(1) {
        let remote = addr.parse()?;
        swarm.dial_addr(remote)?;
        println!("Dialed {}", addr)
    }

    // Event loop
    // 1. listen for incoming connection
    // 2. establish outgoing connection if address specified on cli
    block_on(future::poll_fn(move |cx| loop {
        match swarm.poll_next_unpin(cx) {
            Poll::Ready(Some(event)) => {
                if let SwarmEvent::NewListenAddr { address, .. } = event {
                    println!("Listening on {:?}", address);
                }
            }
            Poll::Ready(None) => return Poll::Ready(()),
            Poll::Pending => return Poll::Pending
        }
    }));

    Ok(())
}
