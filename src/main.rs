//! Test/Example binary for hbui.
//!
//! TODO: Move to examples directory.

#![allow(dead_code, unused_imports, unused_variables, unused_mut)]

#[macro_use] extern crate serde_derive;
extern crate rand;
extern crate hbui;

use std::collections::{BTreeSet, VecDeque};
use rand::Rng;
use hbui::hbbft::{
    broadcast::{Broadcast, BroadcastMessage},
    crypto::{
        SecretKeySet,
        poly::Poly,
    },
    messaging::{DistAlgorithm, NetworkInfo, SourcedMessage, Target, TargetedMessage},
    honey_badger::HoneyBadger,
    dynamic_honey_badger::{Error as HbError, DynamicHoneyBadger, Batch, Input, Message, Change},
    // queueing_honey_badger::{Error as HbError, QueueingHoneyBadger, Batch, Input},
};
use hbui::{Hbui};

const BATCH_SIZE: usize = 150;
const NODE_COUNT: usize = 20;
const TXN_COUNT: usize = 1000;
const TXN_BYTES: usize = 10;


/// A transaction.
#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Ord, PartialOrd, Debug, Clone)]
pub struct Transaction(pub Vec<u8>);

impl Transaction {
    fn random(len: usize) -> Transaction {
        Transaction(rand::thread_rng().gen_iter().take(len).collect())
    }
}


/// A Honeybadger test node.
struct TestNode {
    uid: usize,
    hb: DynamicHoneyBadger<Vec<Transaction>, usize>,
    peer_in_queue: VecDeque<(usize, TargetedMessage<Message<usize>, usize>)>,
    peer_out_queue: VecDeque<TargetedMessage<Message<usize>, usize>>,
    batch_out_queue: VecDeque<Batch<Transaction, usize>>,
}

impl TestNode {
    fn enqueue_outputs(&mut self) {
        for msg in self.hb.message_iter() {
            self.peer_out_queue.push_back(msg);
        }

        for txn in self.hb.output_iter() {
            self.batch_out_queue.push_back(txn);
        }
    }

    /// Pop the next message in the incoming (from peer) queue and submit it
    /// to honey badger.
    fn handle_next_message(&mut self) -> Result<(), ()> {
        match self.peer_in_queue.pop_front() {
            Some((src_node_idx, peer_msg)) => {
                self.hb.handle_message(&src_node_idx, peer_msg.message).unwrap();
                Ok(())
            },
            None => Err(()),
        }
    }
}


/// Given a `Vec` of `TestNodes`, iterate through the list, collect all
/// outgoing messages, then forward messages to the appropriate recipient(s).
fn forward_outgoing_messages(nodes: &mut Vec<TestNode>) -> Result<(), HbError> {
    // All peer-to-peer messages for this round:
    let mut peer_output_msgs: Vec<_> = nodes.iter_mut().flat_map(|node| {
            let node_uid = node.uid;
            node.peer_out_queue.drain(..).map(move |msg| (node_uid, msg))
        })
        .collect();

    // Exchange/forward messages:
    for (src_node_idx, peer_msg) in peer_output_msgs.drain(..) {
        match peer_msg.target {
            Target::Node(n_uid) => {
                nodes[n_uid].peer_in_queue.push_back((src_node_idx, peer_msg));
            },
            Target::All => {
                for n_uid in (0..NODE_COUNT).filter(|&id| id != src_node_idx) {
                    nodes[n_uid].peer_in_queue.push_back((src_node_idx, peer_msg.clone()));
                }
            },
        }
    }
    Ok(())
}

//
// current public key set, set of node ids
// buffer all messages
// epoch number from changed
// SecretKey::rand


fn main() -> Result<(), HbError> {
    let sk_set = SecretKeySet::random(0, &mut rand::thread_rng());
    let pk_set = sk_set.public_keys();

    let node_ids: BTreeSet<_> = (0..NODE_COUNT).collect();

    // Create HB Test nodes with user transactions input:
    let mut nodes = (0..NODE_COUNT).map(|id| {
            let netinfo = NetworkInfo::new(
                id,
                node_ids.clone(),
                sk_set.secret_key_share(id as u64),
                pk_set.clone(),
            );

            let hb = DynamicHoneyBadger::builder(netinfo).build();

            Ok(TestNode {
                uid: id,
                hb,
                peer_in_queue: VecDeque::new(),
                peer_out_queue: VecDeque::new(),
                batch_out_queue: VecDeque::new(),
            })
        })
        .collect::<Result<Vec<_>, HbError>>()?;

    let txns = (0..TXN_COUNT).map(|_| Transaction::random(TXN_BYTES)).collect::<Vec<_>>();

    // Stage messages and transactions for output and processing:
    for node in nodes.iter_mut() {
        for txn in txns.iter() {
            // node.hb.input(Input::User(txn.clone()))?;
        }
        node.hb.input(Input::User(txns.clone()))?;
        node.enqueue_outputs();
    }

    forward_outgoing_messages(&mut nodes)?;

    let epoch_ttl = TXN_COUNT / BATCH_SIZE;
    let mut epochs_done = 0;
    let mut batch_done = false;

    while epochs_done < epoch_ttl {
        while !batch_done {
            for node_idx in 0..NODE_COUNT {
                if let Err(_) = nodes[node_idx].handle_next_message() {
                    // Incoming message queue is empty.
                    // println!("Incoming message queue is empty for node: {}", node_idx);
                }

                nodes[node_idx].enqueue_outputs();
                forward_outgoing_messages(&mut nodes)?;

                if nodes[node_idx].batch_out_queue.len() > epochs_done {
                    // println!("First batch [{}]: {:?}", node_idx, nodes[node_idx].batch_out_queue);
                    batch_done = true;
                }
            }
        }
        epochs_done += 1;
        batch_done = false;
    }

    for node_idx in 1..NODE_COUNT {
        for batch_idx in 0..epoch_ttl {
            assert!(nodes[node_idx].batch_out_queue[batch_idx] ==
                nodes[0].batch_out_queue[batch_idx]);
        }
    }

    let mut peer_in_count = 0;
    let mut peer_out_count = 0;
    let mut batch_out_count = 0;

    for node in nodes.iter() {
        peer_in_count += node.peer_in_queue.len();
        peer_out_count += node.peer_out_queue.len();
        batch_out_count += node.batch_out_queue.len();
    }

    println!("Peer in count: {}", peer_in_count);
    println!("Peer out count: {}", peer_out_count);
    println!("Batch out count: {}", batch_out_count);

    Ok(())
}



