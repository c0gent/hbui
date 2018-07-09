#![allow(dead_code, unused_imports)]

pub extern crate hbbft;
extern crate serde;
// #[macro_use]
// extern crate serde_derive;

use std::collections::VecDeque;
use serde::{Serialize, Deserialize};

use hbbft::{
    dynamic_honey_badger::{Error as DhbError, DynamicHoneyBadger, Input},
    traits::{Contribution, NodeUid},
    messaging::DistAlgorithm,
};




pub struct Hbui<C, N>
        where C: Contribution + Serialize + for<'r> Deserialize<'r>,
              N: NodeUid + Serialize + for<'r> Deserialize<'r> {
    pub dhb: DynamicHoneyBadger<C, N>,
    pub queue: VecDeque<C>,
}

impl<C, N> Hbui<C, N>
        where C: Contribution + Serialize + for<'r> Deserialize<'r>,
              N: NodeUid + Serialize + for<'r> Deserialize<'r>, {
    pub fn new(dhb: DynamicHoneyBadger<C, N>) -> Hbui<C, N> {
        Hbui {
            dhb,
            queue: VecDeque::new(),
        }
    }

    pub fn append_transaction(&mut self, txn: C) -> Result<(), DhbError> {
        self.dhb.input(Input::User(txn))
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



