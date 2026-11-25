// Generated from method-table.toml by tests/method_table: edit the table, then run
// `MOHO_ABI_REGENERATE=1 cargo test --test method_table`.

/// One call, ready for the trap: the system call number, the six words the crate root's table
/// places in registers, and how to read the six words the kernel answers.
///
/// This crate makes no call itself. A program puts [`number`](Self::number) and
/// [`words`](Self::words) in the registers, traps, and passes the six words it gets back to
/// [`answer`](Self::answer).
#[derive(Debug, Clone, Copy)]
pub struct Call<A> {
    /// The system call number, one of the five at the crate root.
    pub number: u64,
    /// Words 0 to 5, in the registers the crate root's table names.
    pub words: [u64; 6],
    answer: fn([u64; 6]) -> A,
}

impl<A> Call<A> {
    /// Read the six words the kernel answered this call with.
    pub fn answer(&self, words: [u64; 6]) -> A {
        (self.answer)(words)
    }
}

/// Words 0 to 5 of an answer when word 0 is not negative; otherwise word 0 as the negative
/// [`Error`](crate::Error) code it is, which `Error::try_from` decodes.
fn refusal(words: [u64; 6]) -> Result<[u64; 6], i64> {
    let first = words[0] as i64;
    if first < 0 { Err(first) } else { Ok(words) }
}

/// The call [`SYSTEM_CALL_EXIT`](crate::SYSTEM_CALL_EXIT) makes, and how its answer reads.
///
/// It never returns, so its answer has nothing to read.
pub fn exit(code: u64) -> Call<()> {
    Call {
        number: super::SYSTEM_CALL_EXIT,
        words: [code, 0, 0, 0, 0, 0],
        answer: |_| (),
    }
}

/// The call [`SYSTEM_CALL_YIELD`](crate::SYSTEM_CALL_YIELD) makes, and how its answer reads.
///
/// A refusal is the negative word 0, an [`Error`](crate::Error) code.
pub fn yield_now() -> Call<Result<(), i64>> {
    Call {
        number: super::SYSTEM_CALL_YIELD,
        words: [0, 0, 0, 0, 0, 0],
        answer: |words| refusal(words).map(|_| ()),
    }
}

/// The call [`SYSTEM_CALL_CAPABILITY_DELETE`](crate::SYSTEM_CALL_CAPABILITY_DELETE) makes, and how its answer reads.
///
/// A refusal is the negative word 0, an [`Error`](crate::Error) code.
pub fn capability_delete(slot: u64) -> Call<Result<(), i64>> {
    Call {
        number: super::SYSTEM_CALL_CAPABILITY_DELETE,
        words: [slot, 0, 0, 0, 0, 0],
        answer: |words| refusal(words).map(|_| ()),
    }
}

/// The call [`SYSTEM_CALL_EXIT_THREAD`](crate::SYSTEM_CALL_EXIT_THREAD) makes, and how its answer reads.
///
/// It never returns, so its answer has nothing to read.
pub fn exit_thread(word: u64) -> Call<()> {
    Call {
        number: super::SYSTEM_CALL_EXIT_THREAD,
        words: [word, 0, 0, 0, 0, 0],
        answer: |_| (),
    }
}

/// The typed calls and defined bits of the methods [`rendezvous`](crate::rendezvous) numbers.
pub mod rendezvous {
    /// The bits [`SEND`](crate::rendezvous::SEND) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const SEND_DEFINED_BITS: [u64; 4] = [u64::MAX, u64::MAX, u64::MAX, 0];

    /// The call [`SEND`](crate::rendezvous::SEND) makes, and how its answer reads.
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// The rights it reads: [`WRITE`](crate::rights::WRITE).
    ///
    /// Its own refusals: [`Gone`](crate::Error::Gone), [`NotPermitted`](crate::Error::NotPermitted).
    pub fn send(capability: u64, word_0: u64, word_1: u64, word_2: u64) -> super::Call<Result<(), i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::rendezvous::SEND, word_0, word_1, word_2, 0],
            answer: |words| super::refusal(words).map(|_| ()),
        }
    }

    /// The bits [`RECEIVE`](crate::rendezvous::RECEIVE) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const RECEIVE_DEFINED_BITS: [u64; 4] = [0, 0, 0, 0];

    /// The call [`RECEIVE`](crate::rendezvous::RECEIVE) makes, and how its answer reads.
    ///
    /// It answers `word_0` (word 0), `word_1` (word 1), `word_2` (word 2), `badge` (word 3), `tag` (word 4).
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// The rights it reads: [`READ`](crate::rights::READ).
    ///
    /// Its own refusals: [`Gone`](crate::Error::Gone).
    pub fn receive(capability: u64) -> super::Call<Result<[u64; 5], i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::rendezvous::RECEIVE, 0, 0, 0, 0],
            answer: |words| super::refusal(words).map(|words| [words[0], words[1], words[2], words[3], words[4]]),
        }
    }

    /// The bits [`SEND_CAPABILITY`](crate::rendezvous::SEND_CAPABILITY) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const SEND_CAPABILITY_DEFINED_BITS: [u64; 4] = [u64::MAX, super::super::rights::ALL, u64::MAX, 0];

    /// The call [`SEND_CAPABILITY`](crate::rendezvous::SEND_CAPABILITY) makes, and how its answer reads.
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// The rights it reads: [`WRITE`](crate::rights::WRITE), [`GRANT`](crate::rights::GRANT).
    ///
    /// Its own refusals: [`NoSuchSlot`](crate::Error::NoSuchSlot), [`NotPermitted`](crate::Error::NotPermitted), [`Gone`](crate::Error::Gone).
    pub fn send_capability(capability: u64, capability_slot: u64, rights: u64, word_0: u64) -> super::Call<Result<(), i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::rendezvous::SEND_CAPABILITY, capability_slot, rights, word_0, 0],
            answer: |words| super::refusal(words).map(|_| ()),
        }
    }

    /// The bits [`RECEIVE_CAPABILITY`](crate::rendezvous::RECEIVE_CAPABILITY) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const RECEIVE_CAPABILITY_DEFINED_BITS: [u64; 4] = [0, 0, 0, 0];

    /// The call [`RECEIVE_CAPABILITY`](crate::rendezvous::RECEIVE_CAPABILITY) makes, and how its answer reads.
    ///
    /// It answers `word_0` (word 0), `capability_slot` (word 1), `word_1` (word 2), `badge` (word 3), `tag` (word 4).
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// The rights it reads: [`READ`](crate::rights::READ).
    ///
    /// Its own refusals: [`Gone`](crate::Error::Gone).
    pub fn receive_capability(capability: u64) -> super::Call<Result<[u64; 5], i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::rendezvous::RECEIVE_CAPABILITY, 0, 0, 0, 0],
            answer: |words| super::refusal(words).map(|words| [words[0], words[1], words[2], words[3], words[4]]),
        }
    }

    /// The bits [`CALL`](crate::rendezvous::CALL) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const CALL_DEFINED_BITS: [u64; 4] = [u64::MAX, u64::MAX, 0, 0];

    /// The call [`CALL`](crate::rendezvous::CALL) makes, and how its answer reads.
    ///
    /// It answers `reply_0` (word 0), `reply_1` (word 1), `capability_slot` (word 2).
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// The rights it reads: [`WRITE`](crate::rights::WRITE).
    ///
    /// Its own refusals: [`Gone`](crate::Error::Gone), [`NotPermitted`](crate::Error::NotPermitted).
    pub fn call(capability: u64, word_0: u64, word_1: u64) -> super::Call<Result<[u64; 3], i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::rendezvous::CALL, word_0, word_1, 0, 0],
            answer: |words| super::refusal(words).map(|words| [words[0], words[1], words[2]]),
        }
    }

    /// The bits [`REAP`](crate::rendezvous::REAP) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const REAP_DEFINED_BITS: [u64; 4] = [u64::MAX, 0, 0, 0];

    /// The call [`REAP`](crate::rendezvous::REAP) makes, and how its answer reads.
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// The rights it reads: [`READ`](crate::rights::READ).
    ///
    /// Its own refusals: [`StillAlive`](crate::Error::StillAlive), [`NotSupervised`](crate::Error::NotSupervised), [`NotPermitted`](crate::Error::NotPermitted).
    pub fn reap(capability: u64, thread_id: u64) -> super::Call<Result<(), i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::rendezvous::REAP, thread_id, 0, 0, 0],
            answer: |words| super::refusal(words).map(|_| ()),
        }
    }

    /// The bits [`SURVEY`](crate::rendezvous::SURVEY) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const SURVEY_DEFINED_BITS: [u64; 4] = [u64::MAX, super::super::survey::record::STATE | super::super::survey::record::PLACEMENT | super::super::survey::record::CPU_TIME | super::super::survey::record::PROCESS, 0, 0];

    /// The call [`SURVEY`](crate::rendezvous::SURVEY) makes, and how its answer reads.
    ///
    /// It answers `next_cursor` (word 0), `thread_id` (word 1), `word` (word 2).
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// The rights it reads: [`ENUMERATE`](crate::rights::ENUMERATE).
    ///
    /// Its own refusals: [`BadMethod`](crate::Error::BadMethod).
    pub fn survey(capability: u64, cursor: u64, record: u64) -> super::Call<Result<[u64; 3], i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::rendezvous::SURVEY, cursor, record, 0, 0],
            answer: |words| super::refusal(words).map(|words| [words[0], words[1], words[2]]),
        }
    }

    /// The bits [`BADGE`](crate::rendezvous::BADGE) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const BADGE_DEFINED_BITS: [u64; 4] = [u64::MAX, 0, 0, 0];

    /// The call [`BADGE`](crate::rendezvous::BADGE) makes, and how its answer reads.
    ///
    /// It answers `slot` (word 0).
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// The rights it reads: [`GRANT`](crate::rights::GRANT).
    ///
    /// Its own refusals: [`NotPermitted`](crate::Error::NotPermitted), [`OutOfMemory`](crate::Error::OutOfMemory).
    pub fn badge(capability: u64, badge: u64) -> super::Call<Result<u64, i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::rendezvous::BADGE, badge, 0, 0, 0],
            answer: |words| super::refusal(words).map(|words| words[0]),
        }
    }
}

/// The typed calls and defined bits of the methods [`reply`](crate::reply) numbers.
pub mod reply {
    /// The bits [`REPLY`](crate::reply::REPLY) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const REPLY_DEFINED_BITS: [u64; 4] = [u64::MAX, u64::MAX, 0, 0];

    /// The call [`REPLY`](crate::reply::REPLY) makes, and how its answer reads.
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// The rights it reads: [`WRITE`](crate::rights::WRITE).
    pub fn reply(capability: u64, reply_0: u64, reply_1: u64) -> super::Call<Result<(), i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::reply::REPLY, reply_0, reply_1, 0, 0],
            answer: |words| super::refusal(words).map(|_| ()),
        }
    }

    /// The bits [`REPLY_CAPABILITY`](crate::reply::REPLY_CAPABILITY) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const REPLY_CAPABILITY_DEFINED_BITS: [u64; 4] = [u64::MAX, u64::MAX, u64::MAX, 0];

    /// The call [`REPLY_CAPABILITY`](crate::reply::REPLY_CAPABILITY) makes, and how its answer reads.
    ///
    /// It answers `copy_dropped` (word 0).
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// The rights it reads: [`WRITE`](crate::rights::WRITE), [`GRANT`](crate::rights::GRANT).
    ///
    /// Its own refusals: [`NoSuchSlot`](crate::Error::NoSuchSlot), [`NotPermitted`](crate::Error::NotPermitted).
    pub fn reply_capability(capability: u64, reply_0: u64, reply_1: u64, capability_slot: u64) -> super::Call<Result<u64, i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::reply::REPLY_CAPABILITY, reply_0, reply_1, capability_slot, 0],
            answer: |words| super::refusal(words).map(|words| words[0]),
        }
    }
}

/// The typed calls and defined bits of the methods [`memory_region`](crate::memory_region) numbers.
pub mod memory_region {
    /// The bits [`MAP`](crate::memory_region::MAP) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const MAP_DEFINED_BITS: [u64; 4] = [u64::MAX, 0, 0, 0];

    /// The call [`MAP`](crate::memory_region::MAP) makes, and how its answer reads.
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// Its own refusals: [`OutOfMemory`](crate::Error::OutOfMemory), [`NotPermitted`](crate::Error::NotPermitted), [`BadPointer`](crate::Error::BadPointer).
    pub fn map(capability: u64, virtual_address: u64) -> super::Call<Result<(), i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::memory_region::MAP, virtual_address, 0, 0, 0],
            answer: |words| super::refusal(words).map(|_| ()),
        }
    }

    /// The bits [`RETYPE`](crate::memory_region::RETYPE) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const RETYPE_DEFINED_BITS: [u64; 4] = [u64::MAX, 0, 0, 0];

    /// The call [`RETYPE`](crate::memory_region::RETYPE) makes, and how its answer reads.
    ///
    /// It answers `slot` (word 0).
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// Its own refusals: [`OutOfMemory`](crate::Error::OutOfMemory), [`NotPermitted`](crate::Error::NotPermitted).
    pub fn retype(capability: u64, pages: u64) -> super::Call<Result<u64, i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::memory_region::RETYPE, pages, 0, 0, 0],
            answer: |words| super::refusal(words).map(|words| words[0]),
        }
    }

    /// The bits [`RETYPE_OBJECT`](crate::memory_region::RETYPE_OBJECT) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const RETYPE_OBJECT_DEFINED_BITS: [u64; 4] = [super::super::object_type::RENDEZVOUS | super::super::object_type::ADDRESS_SPACE | super::super::object_type::THREAD_CONTROL_BLOCK | super::super::object_type::NOTIFICATION | super::super::object_type::TIMER | super::super::object_type::PROCESS, u64::MAX, u64::MAX, 0];

    /// The call [`RETYPE_OBJECT`](crate::memory_region::RETYPE_OBJECT) makes, and how its answer reads.
    ///
    /// It answers `slot` (word 0).
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// Its own refusals: [`BadMethod`](crate::Error::BadMethod), [`OutOfMemory`](crate::Error::OutOfMemory), [`NoSuchSlot`](crate::Error::NoSuchSlot), [`WrongObject`](crate::Error::WrongObject), [`NotPermitted`](crate::Error::NotPermitted).
    pub fn retype_object(capability: u64, object_type: u64, space: u64, floor: u64) -> super::Call<Result<u64, i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::memory_region::RETYPE_OBJECT, object_type, space, floor, 0],
            answer: |words| super::refusal(words).map(|words| words[0]),
        }
    }

    /// The bits [`SPLIT`](crate::memory_region::SPLIT) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const SPLIT_DEFINED_BITS: [u64; 4] = [u64::MAX, 0, 0, 0];

    /// The call [`SPLIT`](crate::memory_region::SPLIT) makes, and how its answer reads.
    ///
    /// It answers `slot` (word 0).
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// The rights it reads: [`WRITE`](crate::rights::WRITE).
    ///
    /// Its own refusals: [`OutOfMemory`](crate::Error::OutOfMemory).
    pub fn split(capability: u64, pages: u64) -> super::Call<Result<u64, i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::memory_region::SPLIT, pages, 0, 0, 0],
            answer: |words| super::refusal(words).map(|words| words[0]),
        }
    }

    /// The bits [`DESTROY`](crate::memory_region::DESTROY) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const DESTROY_DEFINED_BITS: [u64; 4] = [0, 0, 0, 0];

    /// The call [`DESTROY`](crate::memory_region::DESTROY) makes, and how its answer reads.
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// The rights it reads: [`WRITE`](crate::rights::WRITE).
    ///
    /// Its own refusals: [`NotPermitted`](crate::Error::NotPermitted).
    pub fn destroy(capability: u64) -> super::Call<Result<(), i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::memory_region::DESTROY, 0, 0, 0, 0],
            answer: |words| super::refusal(words).map(|_| ()),
        }
    }

    /// The bits [`USAGE`](crate::memory_region::USAGE) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const USAGE_DEFINED_BITS: [u64; 4] = [super::super::usage::SIZE_PAGES | super::super::usage::COMMITTED | super::super::usage::FRAMES | super::super::usage::RENDEZVOUS | super::super::usage::ADDRESS_SPACES | super::super::usage::THREADS | super::super::usage::CHILDREN, 0, 0, 0];

    /// The call [`USAGE`](crate::memory_region::USAGE) makes, and how its answer reads.
    ///
    /// It answers `pages` (word 0).
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// The rights it reads: [`ENUMERATE`](crate::rights::ENUMERATE).
    ///
    /// Its own refusals: [`BadMethod`](crate::Error::BadMethod), [`Gone`](crate::Error::Gone).
    pub fn usage(capability: u64, record: u64) -> super::Call<Result<u64, i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::memory_region::USAGE, record, 0, 0, 0],
            answer: |words| super::refusal(words).map(|words| words[0]),
        }
    }
}

/// The typed calls and defined bits of the methods [`page_frame`](crate::page_frame) numbers.
pub mod page_frame {
    /// The bits [`MAP`](crate::page_frame::MAP) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const MAP_DEFINED_BITS: [u64; 4] = [u64::MAX, 1, u64::MAX, 0];

    /// The call [`MAP`](crate::page_frame::MAP) makes, and how its answer reads.
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// The rights it reads: [`READ`](crate::rights::READ), [`WRITE`](crate::rights::WRITE).
    ///
    /// Its own refusals: [`BadPointer`](crate::Error::BadPointer), [`OutOfMemory`](crate::Error::OutOfMemory), [`NotPermitted`](crate::Error::NotPermitted), [`NoSuchSlot`](crate::Error::NoSuchSlot), [`WrongObject`](crate::Error::WrongObject).
    pub fn map(capability: u64, virtual_address: u64, writable: bool, memory_region_slot: u64) -> super::Call<Result<(), i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::page_frame::MAP, virtual_address, u64::from(writable), memory_region_slot, 0],
            answer: |words| super::refusal(words).map(|_| ()),
        }
    }

    /// The bits [`REVOKE`](crate::page_frame::REVOKE) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const REVOKE_DEFINED_BITS: [u64; 4] = [0, 0, 0, 0];

    /// The call [`REVOKE`](crate::page_frame::REVOKE) makes, and how its answer reads.
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// The rights it reads: [`GRANT`](crate::rights::GRANT).
    pub fn revoke(capability: u64) -> super::Call<Result<(), i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::page_frame::REVOKE, 0, 0, 0, 0],
            answer: |words| super::refusal(words).map(|_| ()),
        }
    }

    /// The bits [`SLICE`](crate::page_frame::SLICE) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const SLICE_DEFINED_BITS: [u64; 4] = [u64::MAX, u64::MAX, 0, 0];

    /// The call [`SLICE`](crate::page_frame::SLICE) makes, and how its answer reads.
    ///
    /// It answers `slot` (word 0).
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// The rights it reads: [`GRANT`](crate::rights::GRANT).
    ///
    /// Its own refusals: [`BadPointer`](crate::Error::BadPointer), [`OutOfMemory`](crate::Error::OutOfMemory).
    pub fn slice(capability: u64, first: u64, count: u64) -> super::Call<Result<u64, i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::page_frame::SLICE, first, count, 0, 0],
            answer: |words| super::refusal(words).map(|words| words[0]),
        }
    }
}

/// The typed calls and defined bits of the methods [`address_space`](crate::address_space) numbers.
pub mod address_space {
    /// The bits [`MAP_INTO`](crate::address_space::MAP_INTO) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const MAP_INTO_DEFINED_BITS: [u64; 4] = [u64::MAX, u64::MAX, super::super::address_space::MAP_READ_ONLY | super::super::address_space::MAP_READ_WRITE | super::super::address_space::MAP_CODE, 0];

    /// The call [`MAP_INTO`](crate::address_space::MAP_INTO) makes, and how its answer reads.
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// The rights it reads: [`WRITE`](crate::rights::WRITE).
    ///
    /// Its own refusals: [`OutOfMemory`](crate::Error::OutOfMemory), [`NotPermitted`](crate::Error::NotPermitted), [`NoSuchSlot`](crate::Error::NoSuchSlot), [`WrongObject`](crate::Error::WrongObject), [`BadPointer`](crate::Error::BadPointer).
    pub fn map_into(capability: u64, virtual_address: u64, frame_slot: u64, mode: u64) -> super::Call<Result<(), i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::address_space::MAP_INTO, virtual_address, frame_slot, mode, 0],
            answer: |words| super::refusal(words).map(|_| ()),
        }
    }

    /// The bits [`LIST`](crate::address_space::LIST) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const LIST_DEFINED_BITS: [u64; 4] = [u64::MAX, 0, 0, 0];

    /// The call [`LIST`](crate::address_space::LIST) makes, and how its answer reads.
    ///
    /// It answers `next_cursor` (word 0), `virtual_address` (word 1), `kind` (word 2).
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// The rights it reads: [`ENUMERATE`](crate::rights::ENUMERATE).
    ///
    /// Its own refusals: [`BadPointer`](crate::Error::BadPointer).
    pub fn list(capability: u64, cursor: u64) -> super::Call<Result<[u64; 3], i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::address_space::LIST, cursor, 0, 0, 0],
            answer: |words| super::refusal(words).map(|words| [words[0], words[1], words[2]]),
        }
    }

    /// The bits [`UNMAP`](crate::address_space::UNMAP) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const UNMAP_DEFINED_BITS: [u64; 4] = [u64::MAX, 0, 0, 0];

    /// The call [`UNMAP`](crate::address_space::UNMAP) makes, and how its answer reads.
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// The rights it reads: [`WRITE`](crate::rights::WRITE).
    ///
    /// Its own refusals: [`BadPointer`](crate::Error::BadPointer).
    pub fn unmap(capability: u64, virtual_address: u64) -> super::Call<Result<(), i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::address_space::UNMAP, virtual_address, 0, 0, 0],
            answer: |words| super::refusal(words).map(|_| ()),
        }
    }

    /// The bits [`WAIT`](crate::address_space::WAIT) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const WAIT_DEFINED_BITS: [u64; 4] = [u64::MAX, super::super::address_wait::PRIVATE | super::super::address_wait::WIDTH_MASK, 0xFFFF_FFFF, 0];

    /// The call [`WAIT`](crate::address_space::WAIT) makes, and how its answer reads.
    ///
    /// It answers `mismatched` (word 0).
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// The rights it reads: [`READ`](crate::rights::READ).
    ///
    /// Its own refusals: [`BadMethod`](crate::Error::BadMethod), [`WrongObject`](crate::Error::WrongObject), [`BadPointer`](crate::Error::BadPointer), [`Gone`](crate::Error::Gone).
    pub fn wait(capability: u64, virtual_address: u64, flags: u64, expected: u64) -> super::Call<Result<u64, i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::address_space::WAIT, virtual_address, flags, expected, 0],
            answer: |words| super::refusal(words).map(|words| words[0]),
        }
    }

    /// The bits [`WAKE`](crate::address_space::WAKE) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const WAKE_DEFINED_BITS: [u64; 4] = [u64::MAX, super::super::address_wait::PRIVATE | super::super::address_wait::WIDTH_MASK, u64::MAX, 0];

    /// The call [`WAKE`](crate::address_space::WAKE) makes, and how its answer reads.
    ///
    /// It answers `woken` (word 0).
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// The rights it reads: [`READ`](crate::rights::READ).
    ///
    /// Its own refusals: [`BadMethod`](crate::Error::BadMethod), [`WrongObject`](crate::Error::WrongObject), [`BadPointer`](crate::Error::BadPointer).
    pub fn wake(capability: u64, virtual_address: u64, flags: u64, count: u64) -> super::Call<Result<u64, i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::address_space::WAKE, virtual_address, flags, count, 0],
            answer: |words| super::refusal(words).map(|words| words[0]),
        }
    }
}

/// The typed calls and defined bits of the methods [`thread_control_block`](crate::thread_control_block) numbers.
pub mod thread_control_block {
    /// The bits [`CONFIGURE`](crate::thread_control_block::CONFIGURE) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const CONFIGURE_DEFINED_BITS: [u64; 4] = [u64::MAX, u64::MAX, u64::MAX, u64::MAX];

    /// The call [`CONFIGURE`](crate::thread_control_block::CONFIGURE) makes, and how its answer reads.
    ///
    /// It answers `current_cpu_page` (word 0).
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// The rights it reads: [`WRITE`](crate::rights::WRITE).
    ///
    /// Its own refusals: [`WrongObject`](crate::Error::WrongObject), [`OutOfMemory`](crate::Error::OutOfMemory), [`BadPointer`](crate::Error::BadPointer), [`NoSuchSlot`](crate::Error::NoSuchSlot), [`NotPermitted`](crate::Error::NotPermitted), [`Gone`](crate::Error::Gone).
    pub fn configure(capability: u64, entry: u64, user_stack_pointer: u64, address_space_slot: u64, thread_pointer: u64) -> super::Call<Result<u64, i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::thread_control_block::CONFIGURE, entry, user_stack_pointer, address_space_slot, thread_pointer],
            answer: |words| super::refusal(words).map(|words| words[0]),
        }
    }

    /// The bits [`CAPABILITY_INSERT`](crate::thread_control_block::CAPABILITY_INSERT) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const CAPABILITY_INSERT_DEFINED_BITS: [u64; 4] = [u64::MAX, super::super::rights::ALL, u64::MAX, 0];

    /// The call [`CAPABILITY_INSERT`](crate::thread_control_block::CAPABILITY_INSERT) makes, and how its answer reads.
    ///
    /// It answers `child_slot` (word 0).
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// The rights it reads: [`WRITE`](crate::rights::WRITE), [`GRANT`](crate::rights::GRANT).
    ///
    /// Its own refusals: [`OutOfMemory`](crate::Error::OutOfMemory), [`NoSuchSlot`](crate::Error::NoSuchSlot), [`WrongObject`](crate::Error::WrongObject), [`NotPermitted`](crate::Error::NotPermitted).
    pub fn capability_insert(capability: u64, capability_slot: u64, rights: u64, target: u64) -> super::Call<Result<u64, i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::thread_control_block::CAPABILITY_INSERT, capability_slot, rights, target, 0],
            answer: |words| super::refusal(words).map(|words| words[0]),
        }
    }

    /// The bits [`START`](crate::thread_control_block::START) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const START_DEFINED_BITS: [u64; 4] = [u64::MAX, u64::MAX, u64::MAX, 0];

    /// The call [`START`](crate::thread_control_block::START) makes, and how its answer reads.
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// The rights it reads: [`WRITE`](crate::rights::WRITE).
    ///
    /// Its own refusals: [`NoSuchSlot`](crate::Error::NoSuchSlot), [`WrongObject`](crate::Error::WrongObject), [`NotPermitted`](crate::Error::NotPermitted), [`OutOfMemory`](crate::Error::OutOfMemory), [`Gone`](crate::Error::Gone).
    pub fn start(capability: u64, argument_0: u64, argument_1: u64, argument_2: u64) -> super::Call<Result<(), i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::thread_control_block::START, argument_0, argument_1, argument_2, 0],
            answer: |words| super::refusal(words).map(|_| ()),
        }
    }

    /// The bits [`SET_THREAD_POINTER`](crate::thread_control_block::SET_THREAD_POINTER) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const SET_THREAD_POINTER_DEFINED_BITS: [u64; 4] = [u64::MAX, 0, 0, 0];

    /// The call [`SET_THREAD_POINTER`](crate::thread_control_block::SET_THREAD_POINTER) makes, and how its answer reads.
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// The rights it reads: [`WRITE`](crate::rights::WRITE).
    ///
    /// Its own refusals: [`WrongObject`](crate::Error::WrongObject), [`BadPointer`](crate::Error::BadPointer), [`NoSuchSlot`](crate::Error::NoSuchSlot).
    pub fn set_thread_pointer(capability: u64, value: u64) -> super::Call<Result<(), i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::thread_control_block::SET_THREAD_POINTER, value, 0, 0, 0],
            answer: |words| super::refusal(words).map(|_| ()),
        }
    }

    /// The bits [`SUSPEND`](crate::thread_control_block::SUSPEND) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const SUSPEND_DEFINED_BITS: [u64; 4] = [0, 0, 0, 0];

    /// The call [`SUSPEND`](crate::thread_control_block::SUSPEND) makes, and how its answer reads.
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// The rights it reads: [`WRITE`](crate::rights::WRITE).
    ///
    /// Its own refusals: [`WrongObject`](crate::Error::WrongObject), [`Gone`](crate::Error::Gone), [`NoSuchSlot`](crate::Error::NoSuchSlot).
    pub fn suspend(capability: u64) -> super::Call<Result<(), i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::thread_control_block::SUSPEND, 0, 0, 0, 0],
            answer: |words| super::refusal(words).map(|_| ()),
        }
    }

    /// The bits [`RESUME`](crate::thread_control_block::RESUME) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const RESUME_DEFINED_BITS: [u64; 4] = [0, 0, 0, 0];

    /// The call [`RESUME`](crate::thread_control_block::RESUME) makes, and how its answer reads.
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// The rights it reads: [`WRITE`](crate::rights::WRITE).
    ///
    /// Its own refusals: [`WrongObject`](crate::Error::WrongObject), [`Gone`](crate::Error::Gone), [`NoSuchSlot`](crate::Error::NoSuchSlot).
    pub fn resume(capability: u64) -> super::Call<Result<(), i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::thread_control_block::RESUME, 0, 0, 0, 0],
            answer: |words| super::refusal(words).map(|_| ()),
        }
    }
}

/// The typed calls and defined bits of the methods [`process`](crate::process) numbers.
pub mod process {
    /// The bits [`DESTROY`](crate::process::DESTROY) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const DESTROY_DEFINED_BITS: [u64; 4] = [0, 0, 0, 0];

    /// The call [`DESTROY`](crate::process::DESTROY) makes, and how its answer reads.
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// The rights it reads: [`WRITE`](crate::rights::WRITE).
    ///
    /// Its own refusals: [`Gone`](crate::Error::Gone).
    pub fn destroy(capability: u64) -> super::Call<Result<(), i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::process::DESTROY, 0, 0, 0, 0],
            answer: |words| super::refusal(words).map(|_| ()),
        }
    }
}

/// The typed calls and defined bits of the methods [`notification`](crate::notification) numbers.
pub mod notification {
    /// The bits [`SIGNAL`](crate::notification::SIGNAL) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const SIGNAL_DEFINED_BITS: [u64; 4] = [u64::MAX, 0, 0, 0];

    /// The call [`SIGNAL`](crate::notification::SIGNAL) makes, and how its answer reads.
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// The rights it reads: [`WRITE`](crate::rights::WRITE).
    ///
    /// Its own refusals: [`Gone`](crate::Error::Gone).
    pub fn signal(capability: u64, bits: u64) -> super::Call<Result<(), i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::notification::SIGNAL, bits, 0, 0, 0],
            answer: |words| super::refusal(words).map(|_| ()),
        }
    }

    /// The bits [`WAIT`](crate::notification::WAIT) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const WAIT_DEFINED_BITS: [u64; 4] = [0, 0, 0, 0];

    /// The call [`WAIT`](crate::notification::WAIT) makes, and how its answer reads.
    ///
    /// It answers `word` (word 0).
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// The rights it reads: [`READ`](crate::rights::READ).
    ///
    /// Its own refusals: [`Gone`](crate::Error::Gone).
    pub fn wait(capability: u64) -> super::Call<Result<u64, i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::notification::WAIT, 0, 0, 0, 0],
            answer: |words| super::refusal(words).map(|words| words[0]),
        }
    }

    /// The bits [`POLL`](crate::notification::POLL) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const POLL_DEFINED_BITS: [u64; 4] = [0, 0, 0, 0];

    /// The call [`POLL`](crate::notification::POLL) makes, and how its answer reads.
    ///
    /// It answers `word` (word 0).
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// The rights it reads: [`READ`](crate::rights::READ).
    ///
    /// Its own refusals: [`Gone`](crate::Error::Gone).
    pub fn poll(capability: u64) -> super::Call<Result<u64, i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::notification::POLL, 0, 0, 0, 0],
            answer: |words| super::refusal(words).map(|words| words[0]),
        }
    }

    /// The bits [`BIND`](crate::notification::BIND) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const BIND_DEFINED_BITS: [u64; 4] = [u64::MAX, 0, 0, 0];

    /// The call [`BIND`](crate::notification::BIND) makes, and how its answer reads.
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// The rights it reads: [`WRITE`](crate::rights::WRITE).
    ///
    /// Its own refusals: [`NotPermitted`](crate::Error::NotPermitted), [`WrongObject`](crate::Error::WrongObject), [`Gone`](crate::Error::Gone), [`NoSuchSlot`](crate::Error::NoSuchSlot).
    pub fn bind(capability: u64, thread_control_block_slot: u64) -> super::Call<Result<(), i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::notification::BIND, thread_control_block_slot, 0, 0, 0],
            answer: |words| super::refusal(words).map(|_| ()),
        }
    }
}

/// The typed calls and defined bits of the methods [`timer`](crate::timer) numbers.
pub mod timer {
    /// The bits [`ARM`](crate::timer::ARM) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const ARM_DEFINED_BITS: [u64; 4] = [u64::MAX, u64::MAX, u64::MAX, 0];

    /// The call [`ARM`](crate::timer::ARM) makes, and how its answer reads.
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// The rights it reads: [`WRITE`](crate::rights::WRITE).
    ///
    /// Its own refusals: [`WrongObject`](crate::Error::WrongObject), [`NotPermitted`](crate::Error::NotPermitted), [`NoSuchSlot`](crate::Error::NoSuchSlot), [`Gone`](crate::Error::Gone).
    pub fn arm(capability: u64, deadline: u64, notification_slot: u64, bits: u64) -> super::Call<Result<(), i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::timer::ARM, deadline, notification_slot, bits, 0],
            answer: |words| super::refusal(words).map(|_| ()),
        }
    }

    /// The bits [`CANCEL`](crate::timer::CANCEL) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const CANCEL_DEFINED_BITS: [u64; 4] = [0, 0, 0, 0];

    /// The call [`CANCEL`](crate::timer::CANCEL) makes, and how its answer reads.
    ///
    /// It answers `was_armed` (word 0).
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// The rights it reads: [`WRITE`](crate::rights::WRITE).
    ///
    /// Its own refusals: [`Gone`](crate::Error::Gone).
    pub fn cancel(capability: u64) -> super::Call<Result<u64, i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::timer::CANCEL, 0, 0, 0, 0],
            answer: |words| super::refusal(words).map(|words| words[0]),
        }
    }
}

/// The typed calls and defined bits of the methods [`interrupt`](crate::interrupt) numbers.
pub mod interrupt {
    /// The bits [`WAIT`](crate::interrupt::WAIT) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const WAIT_DEFINED_BITS: [u64; 4] = [0, 0, 0, 0];

    /// The call [`WAIT`](crate::interrupt::WAIT) makes, and how its answer reads.
    ///
    /// It answers `fired` (word 0).
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// Its own refusals: [`NotPermitted`](crate::Error::NotPermitted), [`WrongObject`](crate::Error::WrongObject).
    pub fn wait(capability: u64) -> super::Call<Result<u64, i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::interrupt::WAIT, 0, 0, 0, 0],
            answer: |words| super::refusal(words).map(|words| words[0]),
        }
    }

    /// The bits [`ACKNOWLEDGE`](crate::interrupt::ACKNOWLEDGE) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const ACKNOWLEDGE_DEFINED_BITS: [u64; 4] = [0, 0, 0, 0];

    /// The call [`ACKNOWLEDGE`](crate::interrupt::ACKNOWLEDGE) makes, and how its answer reads.
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// Its own refusals: [`NotPermitted`](crate::Error::NotPermitted).
    pub fn acknowledge(capability: u64) -> super::Call<Result<(), i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::interrupt::ACKNOWLEDGE, 0, 0, 0, 0],
            answer: |words| super::refusal(words).map(|_| ()),
        }
    }
}

/// The typed calls and defined bits of the methods [`virtual_io`](crate::virtual_io) numbers.
pub mod virtual_io {
    /// The bits [`READ_REGISTER`](crate::virtual_io::READ_REGISTER) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const READ_REGISTER_DEFINED_BITS: [u64; 4] = [u64::MAX, 0, 0, 0];

    /// The call [`READ_REGISTER`](crate::virtual_io::READ_REGISTER) makes, and how its answer reads.
    ///
    /// It answers `value` (word 0).
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// Its own refusals: [`WrongObject`](crate::Error::WrongObject), [`NotPermitted`](crate::Error::NotPermitted).
    pub fn read_register(capability: u64, offset: u64) -> super::Call<Result<u64, i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::virtual_io::READ_REGISTER, offset, 0, 0, 0],
            answer: |words| super::refusal(words).map(|words| words[0]),
        }
    }

    /// The bits [`WRITE_REGISTER`](crate::virtual_io::WRITE_REGISTER) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const WRITE_REGISTER_DEFINED_BITS: [u64; 4] = [u64::MAX, u64::MAX, 0, 0];

    /// The call [`WRITE_REGISTER`](crate::virtual_io::WRITE_REGISTER) makes, and how its answer reads.
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// Its own refusals: [`WrongObject`](crate::Error::WrongObject), [`NotPermitted`](crate::Error::NotPermitted).
    pub fn write_register(capability: u64, offset: u64, value: u64) -> super::Call<Result<(), i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::virtual_io::WRITE_REGISTER, offset, value, 0, 0],
            answer: |words| super::refusal(words).map(|_| ()),
        }
    }

    /// The bits [`SETUP_QUEUE`](crate::virtual_io::SETUP_QUEUE) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const SETUP_QUEUE_DEFINED_BITS: [u64; 4] = [u64::MAX, u64::MAX, 0, 0];

    /// The call [`SETUP_QUEUE`](crate::virtual_io::SETUP_QUEUE) makes, and how its answer reads.
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// Its own refusals: [`WrongObject`](crate::Error::WrongObject), [`NotPermitted`](crate::Error::NotPermitted).
    pub fn setup_queue(capability: u64, entries: u64, queue: u64) -> super::Call<Result<(), i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::virtual_io::SETUP_QUEUE, entries, queue, 0, 0],
            answer: |words| super::refusal(words).map(|_| ()),
        }
    }

    /// The bits [`NOTIFY`](crate::virtual_io::NOTIFY) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const NOTIFY_DEFINED_BITS: [u64; 4] = [u64::MAX, 0, 0, 0];

    /// The call [`NOTIFY`](crate::virtual_io::NOTIFY) makes, and how its answer reads.
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// Its own refusals: [`DeviceRefused`](crate::Error::DeviceRefused), [`WrongObject`](crate::Error::WrongObject), [`NotPermitted`](crate::Error::NotPermitted).
    pub fn notify(capability: u64, queue: u64) -> super::Call<Result<(), i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::virtual_io::NOTIFY, queue, 0, 0, 0],
            answer: |words| super::refusal(words).map(|_| ()),
        }
    }
}

/// The typed calls and defined bits of the methods [`port_range`](crate::port_range) numbers.
pub mod port_range {
    /// The bits [`REVOKE`](crate::port_range::REVOKE) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const REVOKE_DEFINED_BITS: [u64; 4] = [0, 0, 0, 0];

    /// The call [`REVOKE`](crate::port_range::REVOKE) makes, and how its answer reads.
    ///
    /// A refusal is the negative word 0, an [`Error`](crate::Error) code.
    ///
    /// The rights it reads: [`GRANT`](crate::rights::GRANT).
    pub fn revoke(capability: u64) -> super::Call<Result<(), i64>> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::port_range::REVOKE, 0, 0, 0, 0],
            answer: |words| super::refusal(words).map(|_| ()),
        }
    }
}

/// The typed calls and defined bits of the methods [`reboot`](crate::reboot) numbers.
pub mod reboot {
    /// The bits [`REBOOT`](crate::reboot::REBOOT) defines in argument words 2 to 5, word 2 first. A bit outside them has no
    /// meaning yet.
    pub const REBOOT_DEFINED_BITS: [u64; 4] = [0, 0, 0, 0];

    /// The call [`REBOOT`](crate::reboot::REBOOT) makes, and how its answer reads.
    ///
    /// It returns only when refused, and the answer is the refusal.
    ///
    /// Its own refusals: [`NoResetMechanism`](crate::Error::NoResetMechanism), [`ResetNotSupported`](crate::Error::ResetNotSupported), [`ResetDenied`](crate::Error::ResetDenied), [`ResetDidNotHappen`](crate::Error::ResetDidNotHappen).
    pub fn reboot(capability: u64) -> super::Call<i64> {
        super::Call {
            number: super::super::SYSTEM_CALL_INVOKE,
            words: [capability, super::super::reboot::REBOOT, 0, 0, 0, 0],
            answer: |words| words[0] as i64,
        }
    }
}
// End of the part generated from method-table.toml.
