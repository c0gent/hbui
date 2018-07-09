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
    messaging::{DistAlgorithm, NetworkInfo, SourcedMessage, Target},
    honey_badger::HoneyBadger,
    dynamic_honey_badger::{Error as DhbError, DynamicHoneyBadger, Input, Batch, Message, Change},
};
use hbui::{Hbui, /*ContribQueue*/};

const BATCH_SIZE: usize = 50;
const NODE_COUNT: usize = 20;
const TXN_START_COUNT: usize = 100;
const TXN_BYTES: usize = 15;


/// A transaction.
#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Ord, PartialOrd, Debug, Clone)]
pub struct Transaction(pub Vec<u8>);

impl Transaction {
    fn new(len: usize) -> Transaction {
        Transaction(rand::thread_rng().gen_iter().take(len).collect())
    }
}

struct TestNode {
    hb: Hbui<Transaction, usize>,
    // txns: Vec<Transaction>,
}



fn main() -> Result<(), DhbError> {
    let sk_set = SecretKeySet::random(0, &mut rand::thread_rng());
    let pk_set = sk_set.public_keys();

    let node_ids: BTreeSet<_> = (0..NODE_COUNT).collect();

    let mut nodes = (0..NODE_COUNT).map(|id| {
            let netinfo = NetworkInfo::new(
                id,
                node_ids.clone(),
                sk_set.secret_key_share(id as u64),
                pk_set.clone(),
            );

            let dhb = DynamicHoneyBadger::builder(netinfo)
                .max_future_epochs(0)
                .build()?;

            let mut hb = Hbui::new(dhb);

            for _ in 0..TXN_START_COUNT {
                hb.append_transaction(Transaction::new(TXN_BYTES))?;
            }

            Ok(TestNode {
                hb,
            })
        })
        .collect::<Result<Vec<_>, DhbError>>()?;

    Ok(())
}








// struct Queue {
//     queue: VecDeque<String>,
// }

// impl Queue {
//     pub fn new() -> Queue {
//         Queue { queue: VecDeque::new() }
//     }
// }

// impl ContribQueue for Queue {
//     type Contribution = String;

// }