// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

pub enum StorageMutation {
    SetTerm(u64),
    SetVotedFor(Option<u64>),
    AppendEntries(Vec<u64>),
    TruncateFrom(u64),
    SaveSnapshot(Snapshot),
}

pub struct Snapshot {
    pub index: u64,
    pub term: u64,
    pub data: Vec<u8>,
}
