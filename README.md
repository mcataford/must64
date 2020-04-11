# must64
A MIPS64 interpreter written in Rust

## Why?

Because MIPS is a hoot and so is Rust!

## Usage

The project does not yet have a distributed build, but you can make your own via `cargo build`.

Once that is done, you can simply run the built crate (or use `cargo run`) to execute it. The only currently valid usage pattern is `cargo run <MIPS file path>` (replace `cargo run` with the built executable if you choose that path).

The interpreter doesn't support heap allocation yet, and will print the state of its register bank after every instructions (all output is verbose).

## Instruction set support

### Core IS

| | | | |
|---|---|---|---|
|✔️ add|✔️ addi|✔️ sub|✔️ li|
|❌ addiu|❌ addu|❌ and|❌ andi|
|❌ beq|❌ bne|❌ j|❌ jal|
|❌ jr|❌ lbu|❌ lhu|❌ ll|
|❌ lui|❌ lw|❌ nor|❌ or|
|❌ ori|❌ slt|❌ sltu|❌ slti|
|❌ sltiu|❌ sll|❌ srl|❌ sb|
|❌ sc|❌ sh|❌ sw|❌ subu|

### Arithmetic Core IS

_None yet._

### Pseudoinstruction set

_None yet._
