//! A user interface shim for HoneyBadgerBuilder.


#![allow(dead_code, unused_imports)]

pub extern crate hbbft;
extern crate serde;
// #[macro_use]
// extern crate serde_derive;


/// Common traits.
pub mod traits {
    use std::fmt::Debug;
    use std::hash::Hash;

    /// A transaction, user message, etc.
    pub trait Contribution: Clone + Eq + Debug + Hash {}
    impl<C> Contribution for C where C: Clone + Eq + Debug + Hash {}

    /// A peer node's unique identifier.
    pub trait NodeUid: Clone + Eq + Ord + Debug + Hash {}
    impl<N> NodeUid for N where N: Clone + Eq + Ord + Debug + Hash  {}
}

use std::collections::VecDeque;
use serde::{Serialize, Deserialize};
use hbbft::{
    dynamic_honey_badger::{Error as DhbError, DynamicHoneyBadger, Message},
    queueing_honey_badger::{Error as QhbError, QueueingHoneyBadger, Batch, Input},
    messaging::{DistAlgorithm, TargetedMessage},
    fault_log::FaultLog,
};
use traits::{Contribution, NodeUid};


/// An easy-to-use API for the honey badger consensus algorithm.
pub struct Hbui<C, N>
        where C: Contribution + Serialize + for<'r> Deserialize<'r>,
              N: NodeUid + Serialize + for<'r> Deserialize<'r> {
    pub qhb: QueueingHoneyBadger<C, N>,
    pub peer_out_queue: VecDeque<TargetedMessage<Message<N>, N>>,
    pub batch_out_queue: VecDeque<Batch<C, N>>,
}

impl<C, N> Hbui<C, N>
        where C: Contribution + Serialize + for<'r> Deserialize<'r>,
              N: NodeUid + Serialize + for<'r> Deserialize<'r>, {
    pub fn new(qhb: QueueingHoneyBadger<C, N>) -> Hbui<C, N> {
        Hbui {
            qhb,
            peer_out_queue: VecDeque::new(),
            batch_out_queue: VecDeque::new(),
        }
    }

    /// Adds a transaction/contribution to the queue.
    pub fn append_transaction(&mut self, txn: C) -> Result<FaultLog<N>, QhbError> {
        self.qhb.input(Input::User(txn))
    }

    /// Prepares peer messages and batch transactions for transmission and processing.
    pub fn enqueue_outputs(&mut self) {
        for msg in self.qhb.message_iter() {
            self.peer_out_queue.push_back(msg);
        }

        for txn in self.qhb.output_iter() {
            self.batch_out_queue.push_back(txn);
        }
    }

    /// Returns a shared reference to the internal honey badger instance.
    pub fn hb(&self) -> &QueueingHoneyBadger<C, N> {
        &self.qhb
    }

    /// Returns a unique reference to the internal honey badger instance.
    pub fn hb_mut(&mut self) -> &mut QueueingHoneyBadger<C, N> {
        &mut self.qhb
    }

    /// Returns a reference to the peer node network output queue.
    pub fn peer_out_queue(&self) -> &VecDeque<TargetedMessage<Message<N>, N>> {
        &self.peer_out_queue
    }

    /// Returns a reference to the peer node network output queue.
    pub fn peer_out_queue_mut(&mut self) -> &mut VecDeque<TargetedMessage<Message<N>, N>> {
        &mut self.peer_out_queue
    }

    /// Returns a reference to the batch (post-consensus) output queue.
    pub fn batch_out_queue(&self) -> &VecDeque<Batch<C, N>> {
        &self.batch_out_queue
    }

    /// Returns a reference to the batch (post-consensus) output queue.
    pub fn batch_out_queue_mut(&mut self) -> &mut VecDeque<Batch<C, N>> {
        &mut self.batch_out_queue
    }
}







// /// Types that provide a queue of transactions, etc.
// pub trait ContribQueue {
//     type Contribution;

// }

// pub struct Hbui<C, N, Q>
//         where C: Contribution + Serialize + for<'r> Deserialize<'r>,
//               N: NodeUid + Serialize + for<'r> Deserialize<'r>,
//               Q: ContribQueue<Contribution = C>, {
//     dhb: DynamicHoneyBadger<C, N>,
//     queue: Q,
// }

// impl<C, N, Q> Hbui<C, N, Q>
//         where C: Contribution + Serialize + for<'r> Deserialize<'r>,
//               N: NodeUid + Serialize + for<'r> Deserialize<'r>,
//               Q: ContribQueue<Contribution = C>, {
//     pub fn new(dhb: DynamicHoneyBadger<C, N>, queue: Q) -> Hbui<C, N, Q> {
//         Hbui {
//             dhb,
//             queue,
//         }
//     }

//     pub fn intake(&self) -> Result<(), ()> {
//         Ok(())
//     }
// }



