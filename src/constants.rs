//!

use crate::VERSION;
use sl_oblivious::label::Label;

/// LABEL for the keygen protocol
pub const DKG_LABEL: Label = Label::new(VERSION, 100);

/// LABEL for the commitment 1
pub const COMMITMENT_1_LABEL: Label = Label::new(VERSION, 101);

/// LABEL for the commitment 2
pub const COMMITMENT_2_LABEL: Label = Label::new(VERSION, 102);

/// LABEL for the DLOG proof 1
pub const DLOG_PROOF1_LABEL: Label = Label::new(VERSION, 103);

/// LABEL for the DLOG proof 2
pub const DLOG_PROOF2_LABEL: Label = Label::new(VERSION, 104);

/// LABEL for the signature protocol
pub const DSG_LABEL: Label = Label::new(VERSION, 200);

/// LABEL for the commitment
pub const COMMITMENT_LABEL: Label = Label::new(VERSION, 201);

/// LABEL for Pairwise MtA
pub const PAIRWISE_MTA_LABEL: Label = Label::new(VERSION, 203);
