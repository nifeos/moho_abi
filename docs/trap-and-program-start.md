# The trap

| | aarch64 | riscv64 | `x86_64` |
|---|---|---|---|
| instruction | `svc #0` | `ecall` | `syscall` |
| written by every call | `x0` to `x5` | `a0` to `a5` | `rdi`, `rsi`, `rdx`, `r10`, `r8`, `r9`, `rcx`, `r11` |

A program loads the number and the six words into the registers that the crate root's
table names and runs the instruction. Execution continues at the next instruction, with the answer in the same six
registers. Word 0 comes back where it went in, so on `x86_64` the result is in `rdi`, not in
`rax` as on Linux. [`system_call::Call`] holds one call's number and words, and each typed call
in [`system_call`] says which words its method reads and which it answers.

A caller treats all six word registers as written by every call, including the words a method
does not answer. On `x86_64` the `syscall` instruction itself also writes `rcx` (the return
address) and `r11` (the flags), so a caller treats those two as written too. Every other register comes
back as it was:

- the number register (`x8`, `a7`, `rax`) and the stack pointer;
- the thread pointer (`tpidr_el0`, `tp`, the `fs` base);
- every other general register;
- the floating-point and vector registers, which the kernel keeps for each thread: `q0` to `q31` with
  `fpcr` and `fpsr` on aarch64, `f0` to `f31` with `fcsr` on riscv64, and the `x87` registers,
  `mxcsr` and `xmm0` to `xmm15` on `x86_64`.

The condition flags are not promised. The wider vector units (the Scalable Vector Extension on
aarch64, the vector extension on riscv64, the Advanced Vector Extensions on `x86_64`) are off,
and an instruction that uses one faults.

# How a program starts

A thread starts at the entry and on the stack that
[`thread_control_block::CONFIGURE`] gave it. The stack pointer is that value exactly: nothing
aligns it or pushes onto it, so a starter passes a 16-byte-aligned address. On `x86_64` a C
function expects the stack pointer 8 bytes below such an address, as if a `call` had pushed a
return address, so an entry point written as one first runs a `call` of its own. The thread
pointer is the one `CONFIGURE` gave, zero if none.

The thread finds four start words in the registers that carry a C function's first four
arguments, so an entry point may be an `extern "C"` function of four `u64` parameters:

| start word | aarch64 | riscv64 | `x86_64` |
|---|---|---|---|
| 0 | `x0` | `a0` | `rdi` |
| 1 | `x1` | `a1` | `rsi` |
| 2 | `x2` | `a2` | `rdx` |
| 3 | `x3` | `a3` | `rcx` |

On `x86_64` start word 3 is in `rcx`, where a C function reads its fourth argument, and not in
`r10`, where the trap carries word 3. Every other general register is zero, and so are the
floating-point and vector registers.

A thread started with [`thread_control_block::START`] gets the words its starter passed. The
first program, the progenitor, gets its words from the kernel:

| start word | the progenitor receives |
|---|---|
| 0 | the console's register shape, read with [`uart::RegisterShape`] |
| 1 | the length in bytes of the initial ramdisk archive, mapped read-only before it starts |
| 2 | the rights on the file service endpoint, or 0 when the machine has no file service |
| 3 | the version pair of the kernel's interface |

The version pair is one `u32` in the low half of the word: the breaking part in bits 31 to 16
and the additions part in bits 15 to 0. A program runs on a kernel whose breaking part equals
the one the program declares and whose additions part is at least as large.

The fourth start word arrives with version 2.0 of this interface. A kernel of version 1 passes
three: `START` carries three words, and the register of start word 3 is zero, as every other
register is.

Names here are provisional.
