// Generated from method-table.toml by tests/method_table: edit the table, then run
// `MOHO_ABI_REGENERATE=1 cargo test --test method_table`.
#![cfg_attr(rustfmt, rustfmt::skip)]

//! Each typed call places its arguments in the words the table gives them and reads its
//! answer back.

use moho_abi::system_call;

/// An answer whose words are told apart by value.
const ANSWER: [u64; 6] = [201, 202, 203, 204, 205, 206];
/// `BadPointer`, as the kernel leaves it in a register.
const REFUSED: u64 = -4_i64 as u64;

#[test]
fn a_word_0_of_zero_is_an_answer() {
    assert_eq!(system_call::yield_now().answer([0; 6]), Ok(()));
}

#[test]
fn exit() {
    let call = system_call::exit(100);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_EXIT);
    assert_eq!(call.words, [100, 0, 0, 0, 0, 0]);
    assert_eq!(call.answer(ANSWER), ());
    assert_eq!(call.answer([REFUSED; 6]), ());
}

#[test]
fn yield_now() {
    let call = system_call::yield_now();
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_YIELD);
    assert_eq!(call.words, [0, 0, 0, 0, 0, 0]);
    assert_eq!(call.answer(ANSWER), Ok(()));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn capability_delete() {
    let call = system_call::capability_delete(100);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_CAPABILITY_DELETE);
    assert_eq!(call.words, [100, 0, 0, 0, 0, 0]);
    assert_eq!(call.answer(ANSWER), Ok(()));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn exit_thread() {
    let call = system_call::exit_thread(100);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_EXIT_THREAD);
    assert_eq!(call.words, [100, 0, 0, 0, 0, 0]);
    assert_eq!(call.answer(ANSWER), ());
    assert_eq!(call.answer([REFUSED; 6]), ());
}

#[test]
fn rendezvous_send() {
    let call = system_call::rendezvous::send(100, 101, 102, 103);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::rendezvous::SEND, 101, 102, 103, 0]);
    assert_eq!(call.answer(ANSWER), Ok(()));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn rendezvous_receive() {
    let call = system_call::rendezvous::receive(100);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::rendezvous::RECEIVE, 0, 0, 0, 0]);
    assert_eq!(call.answer(ANSWER), Ok([201, 202, 203, 204, 205]));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn rendezvous_send_capability() {
    let call = system_call::rendezvous::send_capability(100, 101, 102, 103);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::rendezvous::SEND_CAPABILITY, 101, 102, 103, 0]);
    assert_eq!(call.answer(ANSWER), Ok(()));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn rendezvous_receive_capability() {
    let call = system_call::rendezvous::receive_capability(100);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::rendezvous::RECEIVE_CAPABILITY, 0, 0, 0, 0]);
    assert_eq!(call.answer(ANSWER), Ok([201, 202, 203, 204, 205]));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn rendezvous_call() {
    let call = system_call::rendezvous::call(100, 101, 102);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::rendezvous::CALL, 101, 102, 0, 0]);
    assert_eq!(call.answer(ANSWER), Ok([201, 202, 203]));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn rendezvous_reap() {
    let call = system_call::rendezvous::reap(100, 101);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::rendezvous::REAP, 101, 0, 0, 0]);
    assert_eq!(call.answer(ANSWER), Ok(()));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn rendezvous_survey() {
    let call = system_call::rendezvous::survey(100, 101, 102);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::rendezvous::SURVEY, 101, 102, 0, 0]);
    assert_eq!(call.answer(ANSWER), Ok([201, 202, 203]));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn rendezvous_badge() {
    let call = system_call::rendezvous::badge(100, 101);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::rendezvous::BADGE, 101, 0, 0, 0]);
    assert_eq!(call.answer(ANSWER), Ok(201));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn reply_reply() {
    let call = system_call::reply::reply(100, 101, 102);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::reply::REPLY, 101, 102, 0, 0]);
    assert_eq!(call.answer(ANSWER), Ok(()));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn reply_reply_capability() {
    let call = system_call::reply::reply_capability(100, 101, 102, 103);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::reply::REPLY_CAPABILITY, 101, 102, 103, 0]);
    assert_eq!(call.answer(ANSWER), Ok(201));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn memory_region_map() {
    let call = system_call::memory_region::map(100, 101);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::memory_region::MAP, 101, 0, 0, 0]);
    assert_eq!(call.answer(ANSWER), Ok(()));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn memory_region_retype() {
    let call = system_call::memory_region::retype(100, 101);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::memory_region::RETYPE, 101, 0, 0, 0]);
    assert_eq!(call.answer(ANSWER), Ok(201));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn memory_region_retype_object() {
    let call = system_call::memory_region::retype_object(100, 101, 102, 103);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::memory_region::RETYPE_OBJECT, 101, 102, 103, 0]);
    assert_eq!(call.answer(ANSWER), Ok(201));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn memory_region_split() {
    let call = system_call::memory_region::split(100, 101);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::memory_region::SPLIT, 101, 0, 0, 0]);
    assert_eq!(call.answer(ANSWER), Ok(201));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn memory_region_destroy() {
    let call = system_call::memory_region::destroy(100);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::memory_region::DESTROY, 0, 0, 0, 0]);
    assert_eq!(call.answer(ANSWER), Ok(()));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn memory_region_usage() {
    let call = system_call::memory_region::usage(100, 101);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::memory_region::USAGE, 101, 0, 0, 0]);
    assert_eq!(call.answer(ANSWER), Ok(201));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn page_frame_map() {
    let call = system_call::page_frame::map(100, 101, true, 103);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::page_frame::MAP, 101, 1, 103, 0]);
    assert_eq!(call.answer(ANSWER), Ok(()));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn page_frame_revoke() {
    let call = system_call::page_frame::revoke(100);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::page_frame::REVOKE, 0, 0, 0, 0]);
    assert_eq!(call.answer(ANSWER), Ok(()));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn page_frame_slice() {
    let call = system_call::page_frame::slice(100, 101, 102);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::page_frame::SLICE, 101, 102, 0, 0]);
    assert_eq!(call.answer(ANSWER), Ok(201));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn address_space_map_into() {
    let call = system_call::address_space::map_into(100, 101, 102, 103);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::address_space::MAP_INTO, 101, 102, 103, 0]);
    assert_eq!(call.answer(ANSWER), Ok(()));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn address_space_list() {
    let call = system_call::address_space::list(100, 101);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::address_space::LIST, 101, 0, 0, 0]);
    assert_eq!(call.answer(ANSWER), Ok([201, 202, 203]));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn address_space_unmap() {
    let call = system_call::address_space::unmap(100, 101);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::address_space::UNMAP, 101, 0, 0, 0]);
    assert_eq!(call.answer(ANSWER), Ok(()));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn address_space_wait() {
    let call = system_call::address_space::wait(100, 101, 102, 103);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::address_space::WAIT, 101, 102, 103, 0]);
    assert_eq!(call.answer(ANSWER), Ok(201));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn address_space_wake() {
    let call = system_call::address_space::wake(100, 101, 102, 103);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::address_space::WAKE, 101, 102, 103, 0]);
    assert_eq!(call.answer(ANSWER), Ok(201));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn thread_control_block_configure() {
    let call = system_call::thread_control_block::configure(100, 101, 102, 103, 104);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::thread_control_block::CONFIGURE, 101, 102, 103, 104]);
    assert_eq!(call.answer(ANSWER), Ok(201));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn thread_control_block_capability_insert() {
    let call = system_call::thread_control_block::capability_insert(100, 101, 102, 103);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::thread_control_block::CAPABILITY_INSERT, 101, 102, 103, 0]);
    assert_eq!(call.answer(ANSWER), Ok(201));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn thread_control_block_start() {
    let call = system_call::thread_control_block::start(100, 101, 102, 103);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::thread_control_block::START, 101, 102, 103, 0]);
    assert_eq!(call.answer(ANSWER), Ok(()));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn thread_control_block_set_thread_pointer() {
    let call = system_call::thread_control_block::set_thread_pointer(100, 101);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::thread_control_block::SET_THREAD_POINTER, 101, 0, 0, 0]);
    assert_eq!(call.answer(ANSWER), Ok(()));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn thread_control_block_suspend() {
    let call = system_call::thread_control_block::suspend(100);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::thread_control_block::SUSPEND, 0, 0, 0, 0]);
    assert_eq!(call.answer(ANSWER), Ok(()));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn thread_control_block_resume() {
    let call = system_call::thread_control_block::resume(100);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::thread_control_block::RESUME, 0, 0, 0, 0]);
    assert_eq!(call.answer(ANSWER), Ok(()));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn process_destroy() {
    let call = system_call::process::destroy(100);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::process::DESTROY, 0, 0, 0, 0]);
    assert_eq!(call.answer(ANSWER), Ok(()));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn notification_signal() {
    let call = system_call::notification::signal(100, 101);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::notification::SIGNAL, 101, 0, 0, 0]);
    assert_eq!(call.answer(ANSWER), Ok(()));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn notification_wait() {
    let call = system_call::notification::wait(100);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::notification::WAIT, 0, 0, 0, 0]);
    assert_eq!(call.answer(ANSWER), Ok(201));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn notification_poll() {
    let call = system_call::notification::poll(100);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::notification::POLL, 0, 0, 0, 0]);
    assert_eq!(call.answer(ANSWER), Ok(201));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn notification_bind() {
    let call = system_call::notification::bind(100, 101);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::notification::BIND, 101, 0, 0, 0]);
    assert_eq!(call.answer(ANSWER), Ok(()));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn timer_arm() {
    let call = system_call::timer::arm(100, 101, 102, 103);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::timer::ARM, 101, 102, 103, 0]);
    assert_eq!(call.answer(ANSWER), Ok(()));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn timer_cancel() {
    let call = system_call::timer::cancel(100);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::timer::CANCEL, 0, 0, 0, 0]);
    assert_eq!(call.answer(ANSWER), Ok(201));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn interrupt_wait() {
    let call = system_call::interrupt::wait(100);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::interrupt::WAIT, 0, 0, 0, 0]);
    assert_eq!(call.answer(ANSWER), Ok(201));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn interrupt_acknowledge() {
    let call = system_call::interrupt::acknowledge(100);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::interrupt::ACKNOWLEDGE, 0, 0, 0, 0]);
    assert_eq!(call.answer(ANSWER), Ok(()));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn virtual_io_read_register() {
    let call = system_call::virtual_io::read_register(100, 101);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::virtual_io::READ_REGISTER, 101, 0, 0, 0]);
    assert_eq!(call.answer(ANSWER), Ok(201));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn virtual_io_write_register() {
    let call = system_call::virtual_io::write_register(100, 101, 102);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::virtual_io::WRITE_REGISTER, 101, 102, 0, 0]);
    assert_eq!(call.answer(ANSWER), Ok(()));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn virtual_io_setup_queue() {
    let call = system_call::virtual_io::setup_queue(100, 101, 102);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::virtual_io::SETUP_QUEUE, 101, 102, 0, 0]);
    assert_eq!(call.answer(ANSWER), Ok(()));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn virtual_io_notify() {
    let call = system_call::virtual_io::notify(100, 101);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::virtual_io::NOTIFY, 101, 0, 0, 0]);
    assert_eq!(call.answer(ANSWER), Ok(()));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn port_range_revoke() {
    let call = system_call::port_range::revoke(100);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::port_range::REVOKE, 0, 0, 0, 0]);
    assert_eq!(call.answer(ANSWER), Ok(()));
    assert_eq!(call.answer([REFUSED; 6]), Err(-4));
}

#[test]
fn reboot_reboot() {
    let call = system_call::reboot::reboot(100);
    assert_eq!(call.number, moho_abi::SYSTEM_CALL_INVOKE);
    assert_eq!(call.words, [100, moho_abi::reboot::REBOOT, 0, 0, 0, 0]);
    assert_eq!(call.answer(ANSWER), 201);
    assert_eq!(call.answer([REFUSED; 6]), -4);
}
// End of the part generated from method-table.toml.
