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
  ISR          (RX)  : ORIGIN = 0x00000000, LENGTH = 0x00000298
  FLASH        (RWX) : ORIGIN = 0x10000000, LENGTH = 64K
  RAM          (RW)  : ORIGIN = 0x10020000, LENGTH = 64K - 16K
  TRACE_DATA   (RW)  : ORIGIN = 0x1002C000, LENGTH = 16K
  IPC_DATA     (RW)  : ORIGIN = 0x10040000, LENGTH = 4K + 4K + 16K
}

/* # Entry point = reset vector */
ENTRY(Reset);
EXTERN(__RESET_VECTOR); /* depends on the `Reset` symbol */

/* # Exception vectors */
/* This is effectively weak aliasing at the linker level */
/* The user can override any of these aliases by defining the corresponding symbol themselves (cf.
   the `exception!` macro) */
EXTERN(__EXCEPTIONS); /* depends on all the these PROVIDED symbols */

EXTERN(DefaultHandler);

PROVIDE(NonMaskableInt = DefaultHandler);
EXTERN(HardFaultTrampoline);
PROVIDE(MemoryManagement = DefaultHandler);
PROVIDE(BusFault = DefaultHandler);
PROVIDE(UsageFault = DefaultHandler);
PROVIDE(SecureFault = DefaultHandler);
PROVIDE(SVCall = DefaultHandler);
PROVIDE(DebugMonitor = DefaultHandler);
PROVIDE(PendSV = DefaultHandler);
PROVIDE(SysTick = DefaultHandler);

PROVIDE(DefaultHandler = DefaultHandler_);
PROVIDE(HardFault = HardFault_);

/* # Interrupt vectors */
EXTERN(__INTERRUPTS); /* `static` variable similar to `__EXCEPTIONS` */

/* # Pre-initialization function */
/* If the user overrides this using the `pre_init!` macro or by creating a `__pre_init` function,
   then the function this points to will be called before the RAM is initialized. */
PROVIDE(__pre_init = DefaultPreInit);

/* # Sections */
SECTIONS
{
  PROVIDE(_stack_start = ORIGIN(RAM) + LENGTH(RAM));

  /* ## Sections in FLASH */
  /* ### Vector table */
  .vector_table ORIGIN(ISR) :
  {
    /* Initial Stack Pointer (SP) value */
    LONG(_stack_start);

    /* Reset vector */
    KEEP(*(.vector_table.reset_vector)); /* this is the `__RESET_VECTOR` symbol */
    __reset_vector = .;

    /* Exceptions */
    KEEP(*(.vector_table.exceptions)); /* this is the `__EXCEPTIONS` symbol */
    __eexceptions = .;

    /* Device specific interrupts */
    KEEP(*(.vector_table.interrupts)); /* this is the `__INTERRUPTS` symbol */
  } > ISR

  /* ### .text */
  .text ORIGIN(FLASH) :
  {
    *(.text .text.*);
    *(.HardFaultTrampoline);
    *(.HardFault.*);
    . = ALIGN(4);
    __etext = .;
  } > FLASH

  /* ### .rodata */
  .rodata __etext : ALIGN(4)
  {
    *(.rodata .rodata.*);
  } > FLASH

  /* ## Sections in RAM */
  /* ### .data */
  .data : ALIGN(4)
  {
    . = ALIGN(4);
    __sdata = .;
    *(.data .data.*);
    . = ALIGN(4); /* 4-byte align the end (VMA) of this section */
    __edata = .;
  } > RAM

  /* LMA of .data */
  __sidata = LOADADDR(.data);

  /* ### .bss */
  .bss : ALIGN(4)
  {
    . = ALIGN(4);
    __sbss = .;
    *(.bss .bss.*);
    . = ALIGN(4); /* 4-byte align the end (VMA) of this section */
    __ebss = .;
  } > RAM

  /* This is how we communicate with the kernel */
  .tracebuffer ORIGIN(TRACE_DATA) : {
      KEEP(*(.tracebuffer .tracebuffer.*))
  } > TRACE_DATA

  /* The kernel looks for a section with this name */
  .resource_table : {
      KEEP(*(.resource_table))
  } > FLASH

  /* The kernel looks for a section with this name */
  .version : {
      KEEP(*(.version))
  } > FLASH


  /* ### .uninit */
  .uninit (NOLOAD) : ALIGN(4)
  {
    . = ALIGN(4);
    *(.uninit .uninit.*);
    . = ALIGN(4);
  } > RAM

  /* Place the heap right after `.uninit` */
  . = ALIGN(4);
  __sheap = .;

  /* ## .got */
  /* Dynamic relocations are unsupported. This section is only used to detect relocatable code in
     the input files and raise an error if relocatable code is found */
  .got (NOLOAD) :
  {
    KEEP(*(.got .got.*));
  }

  /* ## Discarded sections */
  /DISCARD/ :
  {
    /* Unused exception related info that only wastes space */
    *(.ARM.exidx);
    *(.ARM.exidx.*);
    *(.ARM.extab.*);
  }
}

/* Default IRQ handlers as weak symbols */
PROVIDE(WWDG1 = DefaultHandler);
PROVIDE(PVD_AVD = DefaultHandler);
PROVIDE(TAMP = DefaultHandler);
PROVIDE(RTC_WKUP_ALARM = DefaultHandler);
PROVIDE(RESERVED4 = DefaultHandler);
PROVIDE(RCC = DefaultHandler);
PROVIDE(EXTI0 = DefaultHandler);
PROVIDE(EXTI1 = DefaultHandler);
PROVIDE(EXTI2 = DefaultHandler);
PROVIDE(EXTI3 = DefaultHandler);
PROVIDE(EXTI4 = DefaultHandler);
PROVIDE(DMA1_Stream0 = DefaultHandler);
PROVIDE(DMA1_Stream1 = DefaultHandler);
PROVIDE(DMA1_Stream2 = DefaultHandler);
PROVIDE(DMA1_Stream3 = DefaultHandler);
PROVIDE(DMA1_Stream4 = DefaultHandler);
PROVIDE(DMA1_Stream5 = DefaultHandler);
PROVIDE(DMA1_Stream6 = DefaultHandler);
PROVIDE(ADC1 = DefaultHandler);
PROVIDE(FDCAN1_IT0 = DefaultHandler);
PROVIDE(FDCAN2_IT0 = DefaultHandler);
PROVIDE(FDCAN1_IT1 = DefaultHandler);
PROVIDE(FDCAN2_IT1 = DefaultHandler);
PROVIDE(EXTI5 = DefaultHandler);
PROVIDE(TIM1_BRK = DefaultHandler);
PROVIDE(TIM1_UP = DefaultHandler);
PROVIDE(TIM1_TRG_COM = DefaultHandler);
PROVIDE(TIM1_CC = DefaultHandler);
PROVIDE(TIM2 = DefaultHandler);
PROVIDE(TIM3 = DefaultHandler);
PROVIDE(TIM4 = DefaultHandler);
PROVIDE(I2C1_EV = DefaultHandler);
PROVIDE(I2C1_ER = DefaultHandler);
PROVIDE(I2C2_EV = DefaultHandler);
PROVIDE(I2C2_ER = DefaultHandler);
PROVIDE(SPI1 = DefaultHandler);
PROVIDE(SPI2 = DefaultHandler);
PROVIDE(USART1 = DefaultHandler);
PROVIDE(USART2 = DefaultHandler);
PROVIDE(USART3 = DefaultHandler);
PROVIDE(EXTI10 = DefaultHandler);
PROVIDE(RTC_TIMESTAMP = DefaultHandler);
PROVIDE(EXTI11 = DefaultHandler);
PROVIDE(TIM8_BRK = DefaultHandler);
PROVIDE(TIM8_UP = DefaultHandler);
PROVIDE(TIM8_TRG_COM = DefaultHandler);
PROVIDE(TIM8_CC = DefaultHandler);
PROVIDE(DMA1_Stream7 = DefaultHandler);
PROVIDE(FMC = DefaultHandler);
PROVIDE(SDMMC1 = DefaultHandler);
PROVIDE(TIM5 = DefaultHandler);
PROVIDE(SPI3 = DefaultHandler);
PROVIDE(UART4 = DefaultHandler);
PROVIDE(UART5 = DefaultHandler);
PROVIDE(TIM6 = DefaultHandler);
PROVIDE(TIM7 = DefaultHandler);
PROVIDE(DMA2_Stream0 = DefaultHandler);
PROVIDE(DMA2_Stream1 = DefaultHandler);
PROVIDE(DMA2_Stream2 = DefaultHandler);
PROVIDE(DMA2_Stream3 = DefaultHandler);
PROVIDE(DMA2_Stream4 = DefaultHandler);
PROVIDE(ETH1 = DefaultHandler);
PROVIDE(ETH1_WKUP = DefaultHandler);
PROVIDE(FDCAN_CAL = DefaultHandler);
PROVIDE(EXTI6 = DefaultHandler);
PROVIDE(EXTI7 = DefaultHandler);
PROVIDE(EXTI8 = DefaultHandler);
PROVIDE(EXTI9 = DefaultHandler);
PROVIDE(DMA2_Stream5 = DefaultHandler);
PROVIDE(DMA2_Stream6 = DefaultHandler);
PROVIDE(DMA2_Stream7 = DefaultHandler);
PROVIDE(USART6 = DefaultHandler);
PROVIDE(I2C3_EV = DefaultHandler);
PROVIDE(I2C3_ER = DefaultHandler);
PROVIDE(USBH_OHCI = DefaultHandler);
PROVIDE(USBH_EHCI = DefaultHandler);
PROVIDE(EXTI12 = DefaultHandler);
PROVIDE(EXTI13 = DefaultHandler);
PROVIDE(DCMI = DefaultHandler);
PROVIDE(CRYP1 = DefaultHandler);
PROVIDE(HASH1 = DefaultHandler);
PROVIDE(FPU = DefaultHandler);
PROVIDE(UART7 = DefaultHandler);
PROVIDE(UART8 = DefaultHandler);
PROVIDE(SPI4 = DefaultHandler);
PROVIDE(SPI5 = DefaultHandler);
PROVIDE(SPI6 = DefaultHandler);
PROVIDE(SAI1 = DefaultHandler);
PROVIDE(LTDC = DefaultHandler);
PROVIDE(LTDC_ER = DefaultHandler);
PROVIDE(ADC2 = DefaultHandler);
PROVIDE(SAI2 = DefaultHandler);
PROVIDE(QUADSPI = DefaultHandler);
PROVIDE(LPTIM1 = DefaultHandler);
PROVIDE(CEC = DefaultHandler);
PROVIDE(I2C4_EV = DefaultHandler);
PROVIDE(I2C4_ER = DefaultHandler);
PROVIDE(SPDIF_RX = DefaultHandler);
PROVIDE(OTG = DefaultHandler);
PROVIDE(RESERVED99 = DefaultHandler);
PROVIDE(IPCC_RX0 = DefaultHandler);
PROVIDE(IPCC_TX0 = DefaultHandler);
PROVIDE(DMAMUX1_OVR = DefaultHandler);
PROVIDE(IPCC_RX1 = DefaultHandler);
PROVIDE(IPCC_TX1 = DefaultHandler);
PROVIDE(CRYP2 = DefaultHandler);
PROVIDE(HASH2 = DefaultHandler);
PROVIDE(I2C5_EV = DefaultHandler);
PROVIDE(I2C5_ER = DefaultHandler);
PROVIDE(GPU = DefaultHandler);
PROVIDE(DFSDM1_FLT0 = DefaultHandler);
PROVIDE(DFSDM1_FLT1 = DefaultHandler);
PROVIDE(DFSDM1_FLT2 = DefaultHandler);
PROVIDE(DFSDM1_FLT3 = DefaultHandler);
PROVIDE(SAI3 = DefaultHandler);
PROVIDE(DFSDM1_FLT4 = DefaultHandler);
PROVIDE(TIM15 = DefaultHandler);
PROVIDE(TIM16 = DefaultHandler);
PROVIDE(TIM17 = DefaultHandler);
PROVIDE(TIM12 = DefaultHandler);
PROVIDE(MDIOS = DefaultHandler);
PROVIDE(EXTI14 = DefaultHandler);
PROVIDE(MDMA = DefaultHandler);
PROVIDE(DSI = DefaultHandler);
PROVIDE(SDMMC2 = DefaultHandler);
PROVIDE(HSEM_IT2 = DefaultHandler);
PROVIDE(DFSDM1_FLT5 = DefaultHandler);
PROVIDE(EXTI15 = DefaultHandler);
PROVIDE(nCTIIRQ1 = DefaultHandler);
PROVIDE(nCTIIRQ2 = DefaultHandler);
PROVIDE(TIM13 = DefaultHandler);
PROVIDE(TIM14 = DefaultHandler);
PROVIDE(DAC = DefaultHandler);
PROVIDE(RNG1 = DefaultHandler);
PROVIDE(RNG2 = DefaultHandler);
PROVIDE(I2C6_EV = DefaultHandler);
PROVIDE(I2C6_ER = DefaultHandler);
PROVIDE(SDMMC3 = DefaultHandler);
PROVIDE(LPTIM2 = DefaultHandler);
PROVIDE(LPTIM3 = DefaultHandler);
PROVIDE(LPTIM4 = DefaultHandler);
PROVIDE(LPTIM5 = DefaultHandler);
PROVIDE(ETH1_LPI = DefaultHandler);
PROVIDE(RESERVED143 = DefaultHandler);
PROVIDE(MPU_SEV = DefaultHandler);
PROVIDE(RCC_WAKEUP = DefaultHandler);
PROVIDE(SAI4 = DefaultHandler);
PROVIDE(DTS = DefaultHandler);
PROVIDE(RESERVED148 = DefaultHandler);
PROVIDE(WAKEUP_PIN = DefaultHandler);

/* Fix the IRQ because the SVD is shit */
PROVIDE(CRYP = DefaultHandler);
PROVIDE(HASH_RNG = DefaultHandler);
PROVIDE(HSEM0 = DefaultHandler);