/* # Developer notes

- Symbols that start with a double underscore (__) are considered "private"

- Symbols that start with a single underscore (_) are considered "semi-public"; they can be
  overridden in a user linker script, but should not be referred from user code (e.g. `extern "C" {
  static mut __sbss }`).

- `EXTERN` forces the linker to keep a symbol in the final binary. We use this to make sure a
  symbol if not dropped if it appears in or near the front of the linker arguments and "it's not
  needed" by any of the preceding objects (linker arguments)

- `PROVIDE` is used to provide default values that can be overridden by a user linker script

- On alignment: it's important for correctness that the VMA boundaries of both .bss and .data *and*
  the LMA of .data are all 4-byte aligned. These alignments are assumed by the RAM initialization
  routine. There's also a second benefit: 4-byte aligned boundaries means that you won't see
  "Address (..) is out of bounds" in the disassembly produced by `objdump`.
*/

MEMORY
{
  /* NOTE K = KiBi = 1024 bytes */
  /* For the STM32MP1 IPU1 */
  /* We don't really have FLASH and RAM, just DDR
     but we keep the two segments to compatibility with cortex-m-rt */
  m_interrupts (RX)  : ORIGIN = 0x00000000, LENGTH = 0x00000298
  FLASH        (RWX) : ORIGIN = 0x10000000, LENGTH = 64K
  RAM          (RW)  : ORIGIN = 0x10020000, LENGTH = 64K
  IPC_DATA     (RW)  : ORIGIN = 0x10040000, LENGTH = 32K
}