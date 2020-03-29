// We leave in definitions of registers we're not using for completeness.
#![allow(dead_code)]

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

use super::resource_table as rt;
use cortex_m::peripheral::NVIC;
use cortex_m::{self, interrupt::Nr};
use stm32mp1_pac::Interrupt;
use volatile_register::{RO, RW};

extern "C" {
    fn WWDG1_IRQHandler();
    fn PVD_AVD_IRQHandler();
    fn TAMP_IRQHandler();
    fn RTC_WKUP_ALARM_IRQHandler();
    fn RESERVED4_IRQHandler();
    fn RCC_IRQHandler();
    fn EXTI0_IRQHandler();
    fn EXTI1_IRQHandler();
    fn EXTI2_IRQHandler();
    fn EXTI3_IRQHandler();
    fn EXTI4_IRQHandler();
    fn DMA1_Stream0_IRQHandler();
    fn DMA1_Stream1_IRQHandler();
    fn DMA1_Stream2_IRQHandler();
    fn DMA1_Stream3_IRQHandler();
    fn DMA1_Stream4_IRQHandler();
    fn DMA1_Stream5_IRQHandler();
    fn DMA1_Stream6_IRQHandler();
    fn ADC1_IRQHandler();
    fn FDCAN1_IT0_IRQHandler();
    fn FDCAN2_IT0_IRQHandler();
    fn FDCAN1_IT1_IRQHandler();
    fn FDCAN2_IT1_IRQHandler();
    fn EXTI5_IRQHandler();
    fn TIM1_BRK_IRQHandler();
    fn TIM1_UP_IRQHandler();
    fn TIM1_TRG_COM_IRQHandler();
    fn TIM1_CC_IRQHandler();
    fn TIM2_IRQHandler();
    fn TIM3_IRQHandler();
    fn TIM4_IRQHandler();
    fn I2C1_EV_IRQHandler();
    fn I2C1_ER_IRQHandler();
    fn I2C2_EV_IRQHandler();
    fn I2C2_ER_IRQHandler();
    fn SPI1_IRQHandler();
    fn SPI2_IRQHandler();
    fn USART1_IRQHandler();
    fn USART2_IRQHandler();
    fn USART3_IRQHandler();
    fn EXTI10_IRQHandler();
    fn RTC_TIMESTAMP_IRQHandler();
    fn EXTI11_IRQHandler();
    fn TIM8_BRK_IRQHandler();
    fn TIM8_UP_IRQHandler();
    fn TIM8_TRG_COM_IRQHandler();
    fn TIM8_CC_IRQHandler();
    fn DMA1_Stream7_IRQHandler();
    fn FMC_IRQHandler();
    fn SDMMC1_IRQHandler();
    fn TIM5_IRQHandler();
    fn SPI3_IRQHandler();
    fn UART4_IRQHandler();
    fn UART5_IRQHandler();
    fn TIM6_IRQHandler();
    fn TIM7_IRQHandler();
    fn DMA2_Stream0_IRQHandler();
    fn DMA2_Stream1_IRQHandler();
    fn DMA2_Stream2_IRQHandler();
    fn DMA2_Stream3_IRQHandler();
    fn DMA2_Stream4_IRQHandler();
    fn ETH1_IRQHandler();
    fn ETH1_WKUP_IRQHandler();
    fn FDCAN_CAL_IRQHandler();
    fn EXTI6_IRQHandler();
    fn EXTI7_IRQHandler();
    fn EXTI8_IRQHandler();
    fn EXTI9_IRQHandler();
    fn DMA2_Stream5_IRQHandler();
    fn DMA2_Stream6_IRQHandler();
    fn DMA2_Stream7_IRQHandler();
    fn USART6_IRQHandler();
    fn I2C3_EV_IRQHandler();
    fn I2C3_ER_IRQHandler();
    fn USBH_OHCI_IRQHandler();
    fn USBH_EHCI_IRQHandler();
    fn EXTI12_IRQHandler();
    fn EXTI13_IRQHandler();
    fn DCMI_IRQHandler();
    fn CRYP1_IRQHandler();
    fn HASH1_IRQHandler();
    fn FPU_IRQHandler();
    fn UART7_IRQHandler();
    fn UART8_IRQHandler();
    fn SPI4_IRQHandler();
    fn SPI5_IRQHandler();
    fn SPI6_IRQHandler();
    fn SAI1_IRQHandler();
    fn LTDC_IRQHandler();
    fn LTDC_ER_IRQHandler();
    fn ADC2_IRQHandler();
    fn SAI2_IRQHandler();
    fn QUADSPI_IRQHandler();
    fn LPTIM1_IRQHandler();
    fn CEC_IRQHandler();
    fn I2C4_EV_IRQHandler();
    fn I2C4_ER_IRQHandler();
    fn SPDIF_RX_IRQHandler();
    fn OTG_IRQHandler();
    fn RESERVED99_IRQHandler();
    fn IPCC_RX0_IRQHandler();
    fn IPCC_TX0_IRQHandler();
    fn DMAMUX1_OVR_IRQHandler();
    fn IPCC_RX1_IRQHandler();
    fn IPCC_TX1_IRQHandler();
    fn CRYP2_IRQHandler();
    fn HASH2_IRQHandler();
    fn I2C5_EV_IRQHandler();
    fn I2C5_ER_IRQHandler();
    fn GPU_IRQHandler();
    fn DFSDM1_FLT0_IRQHandler();
    fn DFSDM1_FLT1_IRQHandler();
    fn DFSDM1_FLT2_IRQHandler();
    fn DFSDM1_FLT3_IRQHandler();
    fn SAI3_IRQHandler();
    fn DFSDM1_FLT4_IRQHandler();
    fn TIM15_IRQHandler();
    fn TIM16_IRQHandler();
    fn TIM17_IRQHandler();
    fn TIM12_IRQHandler();
    fn MDIOS_IRQHandler();
    fn EXTI14_IRQHandler();
    fn MDMA_IRQHandler();
    fn DSI_IRQHandler();
    fn SDMMC2_IRQHandler();
    fn HSEM_IT2_IRQHandler();
    fn DFSDM1_FLT5_IRQHandler();
    fn EXTI15_IRQHandler();
    fn nCTIIRQ1_IRQHandler();
    fn nCTIIRQ2_IRQHandler();
    fn TIM13_IRQHandler();
    fn TIM14_IRQHandler();
    fn DAC_IRQHandler();
    fn RNG1_IRQHandler();
    fn RNG2_IRQHandler();
    fn I2C6_EV_IRQHandler();
    fn I2C6_ER_IRQHandler();
    fn SDMMC3_IRQHandler();
    fn LPTIM2_IRQHandler();
    fn LPTIM3_IRQHandler();
    fn LPTIM4_IRQHandler();
    fn LPTIM5_IRQHandler();
    fn ETH1_LPI_IRQHandler();
    fn RESERVED143_IRQHandler();
    fn MPU_SEV_IRQHandler();
    fn RCC_WAKEUP_IRQHandler();
    fn SAI4_IRQHandler();
    fn DTS_IRQHandler();
    fn RESERVED148_IRQHandler();
    fn WAKEUP_PIN_IRQHandler();
}

// ****************************************************************************
//
// Sub-modules
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Macros
//
// ****************************************************************************

///  Macro to override a device specific interrupt handler
///
///  # Syntax
///
///  ``` ignore
///  interrupt!(
///      // Name of the interrupt
///      $Name:ident,
///
///      // Path to the interrupt handler (a function)
///      $handler:path,
///
///      // Optional, state preserved across invocations of the handler
///      state: $State:ty = $initial_state:expr,
///  );
///  ```
///
///  Where `$Name` must match the name of one of the variants of the `Interrupt`
///  enum.
///
///  The handler must have signature `fn()` is no state was associated to it;
///  otherwise its signature must be `fn(&mut $State)`.
///
/// This implementation taken from tm4c123x v0.7
#[macro_export]
macro_rules! interrupt {
    ($Name:ident, $handler:path,state: $State:ty = $initial_state:expr) => {
        #[allow(unsafe_code)]
        #[deny(private_no_mangle_fns)]
        #[no_mangle]
        pub unsafe extern "C" fn $Name() {
            static mut STATE: $State = $initial_state;
            // Compile-time check this is a valid interrupt name
            let _ = stm32mp1_pac::Interrupt::$Name;
            let f: fn(&mut $State) = $handler;
            f(&mut STATE)
        }
    };
    ($Name:ident, $handler:path) => {
        #[allow(unsafe_code)]
        #[deny(private_no_mangle_fns)]
        #[no_mangle]
        pub unsafe extern "C" fn $Name() {
            // Compile-time check this is a valid interrupt name
            let _ = stm32mp1_pac::Interrupt::$Name;
            let f: fn() = $handler;
            f()
        }
    };
}

// ****************************************************************************
//
// Public Types / Traits
//
// ****************************************************************************

/// A singleton, representing our chip.
pub struct Stm32mp1<'a, T>
where
    T: rt::AddressMapper,
    T: 'a,
{
    mapper: &'a T,
}

#[derive(Debug, Copy, Clone)]
pub enum MailboxUser {
    User0,
    User1,
    User2,
    User3,
}

#[derive(Debug, Copy, Clone)]
pub enum MailboxSlot {
    Slot0,
    Slot1,
    Slot2,
    Slot3,
    Slot4,
    Slot5,
    Slot6,
    Slot7,
    Slot8,
    Slot9,
    Slot10,
    Slot11,
}

#[derive(Debug, Copy, Clone)]
pub enum MailboxId {
    Mailbox1,
    Mailbox2,
    Mailbox3,
    Mailbox4,
    Mailbox5,
    Mailbox6,
    Mailbox7,
    Mailbox8,
    Mailbox9,
    Mailbox10,
    Mailbox11,
    Mailbox12,
    Mailbox13,
}

#[derive(Debug, Copy, Clone)]
pub struct MailboxLocation {
    pub id: MailboxId,
    pub user: MailboxUser,
    pub slot: MailboxSlot,
}

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum CacheFlushMode {
    WriteBack = CFG_MAINT_CLEAN,
    Invalidate = CFG_MAINT_INVALIDATE,
    InvalidateWriteBack = CFG_MAINT_CLEAN | CFG_MAINT_INVALIDATE,
}

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum CacheFlushAllMode {
    Flush = MMU_MAINT_G_FLUSH,
    WriteBack = MMU_MAINT_CLEAN,
    Invalidate = MMU_MAINT_INVALIDATE,
    InvalidateWriteBack = MMU_MAINT_INVALIDATE | MMU_MAINT_CLEAN,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum _Interrupt {
    WWDG1,
    PVD_AVD,
    TAMP,
    RTC_WKUP_ALARM,
    RESERVED4,
    RCC,
    EXTI0,
    EXTI1,
    EXTI2,
    EXTI3,
    EXTI4,
    DMA1_Stream0,
    DMA1_Stream1,
    DMA1_Stream2,
    DMA1_Stream3,
    DMA1_Stream4,
    DMA1_Stream5,
    DMA1_Stream6,
    ADC1,
    FDCAN1_IT0,
    FDCAN2_IT0,
    FDCAN1_IT1,
    FDCAN2_IT1,
    EXTI5,
    TIM1_BRK,
    TIM1_UP,
    TIM1_TRG_COM,
    TIM1_CC,
    TIM2,
    TIM3,
    TIM4,
    I2C1_EV,
    I2C1_ER,
    I2C2_EV,
    I2C2_ER,
    SPI1,
    SPI2,
    USART1,
    USART2,
    USART3,
    EXTI10,
    RTC_TIMESTAMP,
    EXTI11,
    TIM8_BRK,
    TIM8_UP,
    TIM8_TRG_COM,
    TIM8_CC,
    DMA1_Stream7,
    FMC,
    SDMMC1,
    TIM5,
    SPI3,
    UART4,
    UART5,
    TIM6,
    TIM7,
    DMA2_Stream0,
    DMA2_Stream1,
    DMA2_Stream2,
    DMA2_Stream3,
    DMA2_Stream4,
    ETH1,
    ETH1_WKUP,
    FDCAN_CAL,
    EXTI6,
    EXTI7,
    EXTI8,
    EXTI9,
    DMA2_Stream5,
    DMA2_Stream6,
    DMA2_Stream7,
    USART6,
    I2C3_EV,
    I2C3_ER,
    USBH_OHCI,
    USBH_EHCI,
    EXTI12,
    EXTI13,
    DCMI,
    CRYP1,
    HASH1,
    FPU,
    UART7,
    UART8,
    SPI4,
    SPI5,
    SPI6,
    SAI1,
    LTDC,
    LTDC_ER,
    ADC2,
    SAI2,
    QUADSPI,
    LPTIM1,
    CEC,
    I2C4_EV,
    I2C4_ER,
    SPDIF_RX,
    OTG,
    RESERVED99,
    IPCC_RX0,
    IPCC_TX0,
    DMAMUX1_OVR,
    IPCC_RX1,
    IPCC_TX1,
    CRYP2,
    HASH2,
    I2C5_EV,
    I2C5_ER,
    GPU,
    DFSDM1_FLT0,
    DFSDM1_FLT1,
    DFSDM1_FLT2,
    DFSDM1_FLT3,
    SAI3,
    DFSDM1_FLT4,
    TIM15,
    TIM16,
    TIM17,
    TIM12,
    MDIOS,
    EXTI14,
    MDMA,
    DSI,
    SDMMC2,
    HSEM_IT2,
    DFSDM1_FLT5,
    EXTI15,
    nCTIIRQ1,
    nCTIIRQ2,
    TIM13,
    TIM14,
    DAC,
    RNG1,
    RNG2,
    I2C6_EV,
    I2C6_ER,
    SDMMC3,
    LPTIM2,
    LPTIM3,
    LPTIM4,
    LPTIM5,
    ETH1_LPI,
    RESERVED143,
    MPU_SEV,
    RCC_WAKEUP,
    SAI4,
    DTS,
    RESERVED148,
    WAKEUP_PIN,
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
/// Lower numeric values have higher 'urgency'; that is, they can pre-empty
/// interrupts with higher numeric values (i.e. lower urgency). The bottom 4
/// bits of the 8-bit priority registers are ignored on this chip, so 16 is
/// the next-highest priority after zero.
pub enum InterruptPriority {
    Prio00 = 0 << 4,
    Prio01 = 1 << 4,
    Prio02 = 2 << 4,
    Prio03 = 3 << 4,
    Prio04 = 4 << 4,
    Prio05 = 5 << 4,
    Prio06 = 6 << 4,
    Prio07 = 7 << 4,
    Prio08 = 8 << 4,
    Prio09 = 9 << 4,
    Prio10 = 10 << 4,
    Prio11 = 11 << 4,
    Prio12 = 12 << 4,
    Prio13 = 13 << 4,
    Prio14 = 14 << 4,
    Prio15 = 15 << 4,
}

/// This is the mapped address as the HW maps 0x4000_0000 to 0x5508_0000 by
/// default. See Table 7-64. This is a Cortex-M4 device address, so it doesn't
/// need mapping through the resource table.
const UNICACHE_CFG_ADDR: usize = 0x4000_0000;

/// This is the mapped address as the HW maps 0x4000_0000 to 0x5508_0000 by
/// default. See Table 7-64. This is a Cortex-M4 device address, so it doesn't
/// need mapping through the resource table.
const WUGEN_ADDR: usize = 0x4000_1000;

pub const L3_OCMC_RAM: usize = 0x4030_0000;
pub const L4_PERIPHERAL_L4PER1: usize = 0x4800_0000;
pub const L4_PERIPHERAL_L4PER2: usize = 0x4840_0000;
pub const L4_PERIPHERAL_L4PER3: usize = 0x4880_0000;
pub const L4_PERIPHERAL_L4CFG: usize = 0x4A00_0000;
pub const L3_PERIPHERAL_PRUSS: usize = 0x4B20_0000;
pub const L3_PERIPHERAL_DMM: usize = 0x4E00_0000;
pub const L4_PERIPHERAL_L4EMU: usize = 0x5400_0000;
pub const L3_IVAHD_CONFIG: usize = 0x5A00_0000;
pub const L3_IVAHD_SL2: usize = 0x5B00_0000;
pub const L3_TILER_MODE_0_1: usize = 0x6000_0000;
pub const L3_TILER_MODE_2: usize = 0x7000_0000;
pub const L3_TILER_MODE_3: usize = 0x7800_0000;
pub const L3_EMIF_SDRAM: usize = 0xA000_0000;

const CFG_CONFIG_UNLOCK_MAIN: u32 = 1 << 4;
const CFG_CONFIG_UNLOCK_PORT: u32 = 1 << 3;
const CFG_CONFIG_UNLOCK_INT: u32 = 1 << 2;
const CFG_CONFIG_DISABLE_BYPASS: u32 = 1 << 1;
const CFG_CONFIG_CACHE_LOCK: u32 = 1 << 0;

const CFG_OCP_CLEANBUF: u32 = 1 << 5;
const CFG_OCP_PREFETCH: u32 = 1 << 4;
const CFG_OCP_CACHED: u32 = 1 << 3;
const CFG_OCP_WRALLOCATE: u32 = 1 << 2;
const CFG_OCP_WRBUFFER: u32 = 1 << 1;
const CFG_OCP_WRAP: u32 = 1 << 0;

const CFG_MAINT_INTERRUPT: u32 = 1 << 5;
const CFG_MAINT_INVALIDATE: u32 = 1 << 4;
const CFG_MAINT_CLEAN: u32 = 1 << 3;
const CFG_MAINT_UNLOCK: u32 = 1 << 2;
const CFG_MAINT_LOCK: u32 = 1 << 1;
const CFG_MAINT_PRELOAD: u32 = 1 << 0;

const CFG_INT_READ: u32 = 1 << 4;
const CFG_INT_WRITE: u32 = 1 << 3;
const CFG_INT_MAINT: u32 = 1 << 2;
const CFG_INT_PAGEFAULT: u32 = 1 << 1;
const CFG_INT_CONFIG: u32 = 1 << 0;

/// This is the mapped address as the HW maps 0x4000_0000 to 0x5508_0000 by
/// default. See Table 7-64. This is a Cortex-M4 device address, so it doesn't
/// need mapping through the resource table.
const UNICACHE_MMU_ADDR: usize = 0x4000_0800;

const MMU_POLICY_ENABLED: u32 = (1 << 0);
const MMU_POLICY_LARGE: u32 = (1 << 1);
const MMU_POLICY_CACHEABLE: u32 = (1 << 16);
const MMU_POLICY_POSTED: u32 = (1 << 17);
const MMU_POLICY_ALLOCATE: u32 = (1 << 18);
const MMU_POLICY_WRITE_BACK: u32 = (1 << 19);

const MMU_MAINT_G_FLUSH: u32 = 1 << 10;
const MMU_MAINT_L1_CACHE1: u32 = 1 << 7;
const MMU_MAINT_CPU_INTERRUPT: u32 = 1 << 6;
const MMU_MAINT_HOST_INTERRUPT: u32 = 1 << 5;
const MMU_MAINT_INVALIDATE: u32 = 1 << 4;
const MMU_MAINT_CLEAN: u32 = 1 << 3;
const MMU_MAINT_UNLOCK: u32 = 1 << 2;
const MMU_MAINT_LOCK: u32 = 1 << 1;
const MMU_MAINT_PRELOAD: u32 = 1 << 0;

const MMU_CONFIG_PRIVILEGE: u32 = 1 << 1;
const MMU_CONFIG_MMU_LOCK: u32 = 1 << 0;

// Default IRQ mappings
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [unsafe extern "C" fn(); 150] = [
    WWDG1_IRQHandler,
    PVD_AVD_IRQHandler,
    TAMP_IRQHandler,
    RTC_WKUP_ALARM_IRQHandler,
    RESERVED4_IRQHandler,
    RCC_IRQHandler,
    EXTI0_IRQHandler,
    EXTI1_IRQHandler,
    EXTI2_IRQHandler,
    EXTI3_IRQHandler,
    EXTI4_IRQHandler,
    DMA1_Stream0_IRQHandler,
    DMA1_Stream1_IRQHandler,
    DMA1_Stream2_IRQHandler,
    DMA1_Stream3_IRQHandler,
    DMA1_Stream4_IRQHandler,
    DMA1_Stream5_IRQHandler,
    DMA1_Stream6_IRQHandler,
    ADC1_IRQHandler,
    FDCAN1_IT0_IRQHandler,
    FDCAN2_IT0_IRQHandler,
    FDCAN1_IT1_IRQHandler,
    FDCAN2_IT1_IRQHandler,
    EXTI5_IRQHandler,
    TIM1_BRK_IRQHandler,
    TIM1_UP_IRQHandler,
    TIM1_TRG_COM_IRQHandler,
    TIM1_CC_IRQHandler,
    TIM2_IRQHandler,
    TIM3_IRQHandler,
    TIM4_IRQHandler,
    I2C1_EV_IRQHandler,
    I2C1_ER_IRQHandler,
    I2C2_EV_IRQHandler,
    I2C2_ER_IRQHandler,
    SPI1_IRQHandler,
    SPI2_IRQHandler,
    USART1_IRQHandler,
    USART2_IRQHandler,
    USART3_IRQHandler,
    EXTI10_IRQHandler,
    RTC_TIMESTAMP_IRQHandler,
    EXTI11_IRQHandler,
    TIM8_BRK_IRQHandler,
    TIM8_UP_IRQHandler,
    TIM8_TRG_COM_IRQHandler,
    TIM8_CC_IRQHandler,
    DMA1_Stream7_IRQHandler,
    FMC_IRQHandler,
    SDMMC1_IRQHandler,
    TIM5_IRQHandler,
    SPI3_IRQHandler,
    UART4_IRQHandler,
    UART5_IRQHandler,
    TIM6_IRQHandler,
    TIM7_IRQHandler,
    DMA2_Stream0_IRQHandler,
    DMA2_Stream1_IRQHandler,
    DMA2_Stream2_IRQHandler,
    DMA2_Stream3_IRQHandler,
    DMA2_Stream4_IRQHandler,
    ETH1_IRQHandler,
    ETH1_WKUP_IRQHandler,
    FDCAN_CAL_IRQHandler,
    EXTI6_IRQHandler,
    EXTI7_IRQHandler,
    EXTI8_IRQHandler,
    EXTI9_IRQHandler,
    DMA2_Stream5_IRQHandler,
    DMA2_Stream6_IRQHandler,
    DMA2_Stream7_IRQHandler,
    USART6_IRQHandler,
    I2C3_EV_IRQHandler,
    I2C3_ER_IRQHandler,
    USBH_OHCI_IRQHandler,
    USBH_EHCI_IRQHandler,
    EXTI12_IRQHandler,
    EXTI13_IRQHandler,
    DCMI_IRQHandler,
    CRYP1_IRQHandler,
    HASH1_IRQHandler,
    FPU_IRQHandler,
    UART7_IRQHandler,
    UART8_IRQHandler,
    SPI4_IRQHandler,
    SPI5_IRQHandler,
    SPI6_IRQHandler,
    SAI1_IRQHandler,
    LTDC_IRQHandler,
    LTDC_ER_IRQHandler,
    ADC2_IRQHandler,
    SAI2_IRQHandler,
    QUADSPI_IRQHandler,
    LPTIM1_IRQHandler,
    CEC_IRQHandler,
    I2C4_EV_IRQHandler,
    I2C4_ER_IRQHandler,
    SPDIF_RX_IRQHandler,
    OTG_IRQHandler,
    RESERVED99_IRQHandler,
    IPCC_RX0_IRQHandler,
    IPCC_TX0_IRQHandler,
    DMAMUX1_OVR_IRQHandler,
    IPCC_RX1_IRQHandler,
    IPCC_TX1_IRQHandler,
    CRYP2_IRQHandler,
    HASH2_IRQHandler,
    I2C5_EV_IRQHandler,
    I2C5_ER_IRQHandler,
    GPU_IRQHandler,
    DFSDM1_FLT0_IRQHandler,
    DFSDM1_FLT1_IRQHandler,
    DFSDM1_FLT2_IRQHandler,
    DFSDM1_FLT3_IRQHandler,
    SAI3_IRQHandler,
    DFSDM1_FLT4_IRQHandler,
    TIM15_IRQHandler,
    TIM16_IRQHandler,
    TIM17_IRQHandler,
    TIM12_IRQHandler,
    MDIOS_IRQHandler,
    EXTI14_IRQHandler,
    MDMA_IRQHandler,
    DSI_IRQHandler,
    SDMMC2_IRQHandler,
    HSEM_IT2_IRQHandler,
    DFSDM1_FLT5_IRQHandler,
    EXTI15_IRQHandler,
    nCTIIRQ1_IRQHandler,
    nCTIIRQ2_IRQHandler,
    TIM13_IRQHandler,
    TIM14_IRQHandler,
    DAC_IRQHandler,
    RNG1_IRQHandler,
    RNG2_IRQHandler,
    I2C6_EV_IRQHandler,
    I2C6_ER_IRQHandler,
    SDMMC3_IRQHandler,
    LPTIM2_IRQHandler,
    LPTIM3_IRQHandler,
    LPTIM4_IRQHandler,
    LPTIM5_IRQHandler,
    ETH1_LPI_IRQHandler,
    RESERVED143_IRQHandler,
    MPU_SEV_IRQHandler,
    RCC_WAKEUP_IRQHandler,
    SAI4_IRQHandler,
    DTS_IRQHandler,
    RESERVED148_IRQHandler,
    WAKEUP_PIN_IRQHandler,
];

// ****************************************************************************
//
// Private Types / Traits
//
// ****************************************************************************

// ****************************************************************************
//
// Private Data
//
// ****************************************************************************

// ****************************************************************************
//
// Public Functions
//
// ****************************************************************************

impl<'a, T> Stm32mp1<'a, T>
where
    T: rt::AddressMapper,
{
    /// Returns an object the first time, None the second time.
    pub fn claim(mapper: &'a T) -> Option<Stm32mp1<'a, T>> {
        static mut AVAILABLE: bool = true;
        unsafe {
            if AVAILABLE {
                let mut r = Stm32mp1 { mapper };
                // r.setup();
                AVAILABLE = false;
                Some(r)
            } else {
                None
            }
        }
    }

    /// Enable interrupts. Unsafe as you must only call this function once.
    pub unsafe fn setup(&mut self) {
        cortex_m::interrupt::disable();
        unicache_mmu_setup();
        let unicache_cfg = get_unicache_config();
        unicache_cfg
            .config
            .write(CFG_CONFIG_UNLOCK_MAIN | CFG_CONFIG_UNLOCK_PORT | CFG_CONFIG_UNLOCK_INT);
        // OCP_CACHED is on by default. Turn it off to disable caching.
        unicache_cfg.ocp.write(0x0000_0000);

        // This is what the TI code does but it doesn't make any sense.
        while (unicache_cfg.maint.read() & 0x1F) != 0 {
            cortex_m::asm::nop();
        }
        self.cache_flush_all(CacheFlushAllMode::Flush);
        self.cache_enable();

        // Need to enable interrupt for non-empty and not for non-full!

        let core = CtrlCoreIpu1::get(self.mapper).expect("CtrlCoreIpu1 bad RT");
        // Crossbar 250 is Mailbox 5 User 1

        // Disable all the interrupts!
        core.irq_23_24.set_lower(0);
        core.irq_23_24.set_higher(0);
        core.irq_25_26.set_lower(0);
        core.irq_25_26.set_higher(0);
        core.irq_27_28.set_lower(0);
        core.irq_27_28.set_higher(0);
        core.irq_29_30.set_lower(0);
        core.irq_29_30.set_higher(0);
        core.irq_31_32.set_lower(0);
        core.irq_31_32.set_higher(0);
        core.irq_33_34.set_lower(0);
        core.irq_33_34.set_higher(0);
        core.irq_35_36.set_lower(0);
        core.irq_35_36.set_higher(0);
        core.irq_37_38.set_lower(0);
        core.irq_37_38.set_higher(0);
        core.irq_39_40.set_lower(0);
        core.irq_39_40.set_higher(0);
        core.irq_41_42.set_lower(0);
        core.irq_41_42.set_higher(0);
        core.irq_43_44.set_lower(0);
        core.irq_43_44.set_higher(250);
        core.irq_45_46.set_lower(0);
        core.irq_45_46.set_higher(0);
        core.irq_47_48.set_lower(0);
        core.irq_47_48.set_higher(0);
        core.irq_49_50.set_lower(0);
        core.irq_49_50.set_higher(0);
        core.irq_51_52.set_lower(0);
        core.irq_51_52.set_higher(0);
        core.irq_53_54.set_lower(0);
        core.irq_53_54.set_higher(0);
        core.irq_55_56.set_lower(0);
        core.irq_55_56.set_higher(0);
        core.irq_57_58.set_lower(0);
        core.irq_57_58.set_higher(0);
        core.irq_59_60.set_lower(0);
        core.irq_59_60.set_higher(0);
        core.irq_61_62.set_lower(0);
        core.irq_61_62.set_higher(0);
        core.irq_63_64.set_lower(0);
        core.irq_63_64.set_higher(0);
        core.irq_65_66.set_lower(0);
        core.irq_65_66.set_higher(0);
        core.irq_67_68.set_lower(0);
        core.irq_67_68.set_higher(0);
        core.irq_69_70.set_lower(0);
        core.irq_69_70.set_higher(0);
        core.irq_71_72.set_lower(0);
        core.irq_71_72.set_higher(0);
        core.irq_73_74.set_lower(0);
        core.irq_73_74.set_higher(0);
        core.irq_75_76.set_lower(0);
        core.irq_75_76.set_higher(0);
        core.irq_77_78.set_lower(0);
        core.irq_77_78.set_higher(0);
        core.irq_79_80.set_lower(0);
        core.irq_79_80.set_higher(0);

        // // Enable mailbox interrupts
        // let wugen = get_wugen();
        // wugen.wake_on_interrupt(Interrupt::Ipu1Irq44);

        // self.interrupt_disable(Interrupt::Ipu1Irq44);
    }

    /// Enable the L1 'Unicache'
    pub fn cache_enable(&mut self) {
        unsafe {
            let unicache_cfg = get_unicache_config();
            // Turn the cache on
            unicache_cfg
                .config
                .modify(|w| w | CFG_CONFIG_DISABLE_BYPASS);
            // Ensure write is complete
            let _ = unicache_cfg.config.read();
        }
    }

    /// Disable the L1 'Unicache'
    pub fn cache_disable(&mut self) {
        unsafe {
            let unicache_cfg = get_unicache_config();
            // Turn the cache on
            unicache_cfg
                .config
                .modify(|w| w & !CFG_CONFIG_DISABLE_BYPASS);
            // Ensure write is complete
            let _ = unicache_cfg.config.read();
        }
    }

    /// Cache flush everything using the Unicache AMMU.
    ///
    /// It's unclear what the difference is between flushing the L1 Cache
    /// (through UnicacheConfig) and flushing the AMMU (through UnicacheMmu).
    /// The TI code uses the former for small regions and the latter only with
    /// 0x0000_0000/0xFFFF_FFFF. Maybe it's a performance thing?
    pub fn cache_flush_all(&mut self, mode: CacheFlushAllMode) {
        let unicache_mmu = get_unicache_mmu();
        unsafe {
            unicache_mmu.mstart.write(0x00000000);
            unicache_mmu.mend.write(0xffffffff);
            unicache_mmu.maint.modify(|w| w | (mode as u32));
            while (unicache_mmu.maint.read() & (mode as u32)) != 0 {
                cortex_m::asm::nop();
            }
        }
    }

    /// Tell the unicache to invalidate/writeback a specific object from the
    /// L1 cache.
    pub fn cache_flush<M>(&mut self, obj: &M, len: usize, mode: CacheFlushMode) {
        unsafe {
            let address = obj as *const _ as usize;
            self.cache_flush_address(address, len, mode);
        }
    }

    /// Tell the unicache to invalidate/writeback a specific address range
    /// from the L1 cache.
    pub unsafe fn cache_flush_address(&mut self, address: usize, len: usize, mode: CacheFlushMode) {
        let unicache_cfg = get_unicache_config();
        unicache_cfg.mt_start.write(address);
        unicache_cfg.mt_end.write(address + len - 1);
        let mode: u32 = mode as u32;
        unicache_cfg.maint.modify(|v| v | mode);
        while (unicache_cfg.maint.read() & 0x1f) != 0 {
            cortex_m::asm::nop();
        }
    }

    /// Send a message to the host.
    ///
    /// This invoves processor 6 talking to processor 8.
    pub fn send_message(&mut self, id: u32, location: MailboxLocation) {
        let mailbox = get_mailbox(location.id, self.mapper).expect("Bad resource_table");
        while mailbox.msg_status[location.slot as usize].read() != 0 {
            // spin
        }
        unsafe {
            mailbox.message[location.slot as usize].write(id);
        }
    }

    /// Get any message the host may have for us.
    ///
    /// This invoves processor 8 talking to processor 6.
    pub fn get_message(&mut self, location: MailboxLocation) -> Option<u32>
    where
        T: rt::AddressMapper,
    {
        let mailbox = get_mailbox(location.id, self.mapper).expect("Bad resource_table");
        match mailbox.get_message(location.slot) {
            Some(m) => Some(m),
            None => {
                // mailbox.clear_data_interrupt(location.user, location.slot);
                None
            }
        }
    }

    /// Enable an interrupt on IPU1_C0
    pub fn interrupt_enable(&mut self, interrupt: Interrupt) {
        let mut peripherals = unsafe { cortex_m::Peripherals::steal() };
        peripherals.NVIC.enable(interrupt);
    }

    /// Clear a pending interrupt on IPU1_C0
    pub fn interrupt_clear(&mut self, interrupt: Interrupt) {
        let mut peripherals = unsafe { cortex_m::Peripherals::steal() };
        peripherals.NVIC.clear_pending(interrupt);
    }

    /// Disable an interrupt on IPU1_C0
    pub fn interrupt_disable(&mut self, interrupt: Interrupt) {
        let mut peripherals = unsafe { cortex_m::Peripherals::steal() };
        peripherals.NVIC.disable(interrupt);
    }

    /// Set interrupt priority on IPU1_C0
    pub fn interrupt_priority_set(&mut self, interrupt: Interrupt, priority: InterruptPriority) {
        let mut peripherals = unsafe { cortex_m::Peripherals::steal() };
        unsafe { peripherals.NVIC.set_priority(interrupt, priority as u8) };
    }

    pub fn enable_mailbox_data_interrupt(&mut self, location: MailboxLocation) {
        let mailbox = get_mailbox(location.id, self.mapper).expect("Bad resource_table");
        mailbox.enable_data_interrupt(location.user, location.slot);
    }

    pub fn disable_mailbox_interrupts(&mut self, id: MailboxId, user: MailboxUser) {
        let mailbox = get_mailbox(id, self.mapper).expect("Bad resource_table");
        mailbox.disable_interrupts(user);
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct MailboxIrq {
    status_raw: RW<u32>,
    status_clr: RW<u32>,
    enable_set: RW<u32>,
    enable_clr: RW<u32>,
}

#[repr(C)]
#[allow(dead_code)]
pub struct Mailbox {
    /// Address Offset 0x0000_0000
    revision: RO<u32>,
    _padding: [u32; 3],
    /// Address Offset 0x0000_0010
    sysconfig: RW<u32>,
    _padding2: [u32; 11],
    /// Address Offset 0x0000_0040
    message: [RW<u32>; 12],
    _padding3: [u32; 4],
    /// Address Offset 0x0000_0080
    fifo_status: [RW<u32>; 12],
    _padding4: [u32; 4],
    /// Address Offset 0x0000_00C0
    msg_status: [RW<u32>; 12],
    _padding5: [u32; 4],
    /// Address Offset 0x0000_0100
    irq: [MailboxIrq; 4],
    /// Address Offset 0x0000_0140
    irq_eoi: RW<u32>,
}

impl Mailbox {
    pub fn get_message(&mut self, slot: MailboxSlot) -> Option<u32> {
        // As per Section 19.4.1.3.2
        if self.msg_status[slot as usize].read() != 0 {
            let msg = self.message[slot as usize].read();
            Some(msg)
        } else {
            None
        }
    }

    /// Mark an interrupt as handled
    pub fn clear_interrupts(&mut self, user: MailboxUser) {
        unsafe {
            self.irq[user as usize].status_clr.write(0xFFFFFFFF);
        }
    }

    /// Enable interrupt from the Mailbox on data received
    pub fn enable_data_interrupt(&mut self, user: MailboxUser, slot: MailboxSlot) {
        unsafe {
            // Clear all interrupts
            self.irq[user as usize].status_clr.write(0xFFFFFFFF);
            self.irq[user as usize].enable_clr.write(0xFFFFFFFF);
            // We want to know when there's data on this specific slot
            self.irq[user as usize]
                .enable_set
                .write(slot.get_data_bit());
        }
    }

    /// Disable interrupts from the Mailbox
    pub fn disable_interrupts(&mut self, user: MailboxUser) {
        unsafe {
            self.irq[user as usize].status_clr.write(0xFFFFFFFF);
            self.irq[user as usize].enable_clr.write(0xFFFFFFFF);
        }
    }

    pub fn get_raw(&self, user: MailboxUser) -> u32 {
        self.irq[user as usize].status_raw.read()
    }

    pub fn get_masked(&self, user: MailboxUser) -> u32 {
        self.irq[user as usize].status_clr.read()
    }
}

impl MailboxSlot {
    pub fn get_data_bit(&self) -> u32 {
        1 << ((*self as u32) * 2)
    }

    pub fn get_space_bit(&self) -> u32 {
        1 << (((*self as u32) + 1) * 2)
    }
}

// ****************************************************************************
//
// Private Functions
//
// ****************************************************************************

/// Configure the MMU paging. We basically make it transparent.
fn unicache_mmu_setup() {
    // This maps 512 MiB from 0x0000_0000 with no translation.
    // This region contains machine-code and is cacheable.
    unicache_mmu_configure_large_page(
        0,
        0x00000000,
        0xFFFFFFFF,
        MMU_POLICY_POSTED | MMU_POLICY_CACHEABLE | MMU_POLICY_LARGE | MMU_POLICY_ENABLED,
    );
    // These next three map in 1.5 GiB from 0x6000_0000 to 0xC000_0000 with no translation.
    // This region contains peripherals. It is non-cacheable.
    unicache_mmu_configure_large_page(
        1,
        0x60000000,
        0xFFFFFFFF,
        MMU_POLICY_POSTED | MMU_POLICY_LARGE | MMU_POLICY_ENABLED,
    );
    // This region contains shared memory and IPC data. It is cacheable.
    unicache_mmu_configure_large_page(
        2,
        0x80000000,
        0xFFFFFFFF,
        MMU_POLICY_POSTED | MMU_POLICY_CACHEABLE | MMU_POLICY_LARGE | MMU_POLICY_ENABLED,
    );
    // This region is for DMM and TILER. It is cacheable.
    unicache_mmu_configure_large_page(
        3,
        0xa0000000,
        0xFFFFFFFF,
        MMU_POLICY_POSTED | MMU_POLICY_CACHEABLE | MMU_POLICY_LARGE | MMU_POLICY_ENABLED,
    );
    // There are two small pages mapped by the hardware. 0x????_???? to
    // 0x5502_0000 and 0x4000_0000 to 0x5508_0000. We need to make the latter
    // of these larger as it defaults to 4 KiB, which doesn't cover the inter-
    // core interrupt peripheral.
    unicache_mmu_configure_small_page(
        1,
        0x40000000,
        0x55080000,
        MMU_POLICY_LARGE | MMU_POLICY_ENABLED,
    );
    // Map 64 KiB of L2RAM to 0x2000_0000 using four 16 KiB pages.
    unicache_mmu_configure_small_page(
        2,
        0x20000000,
        0x55020000,
        MMU_POLICY_POSTED
            | MMU_POLICY_CACHEABLE
            | MMU_POLICY_LARGE
            | MMU_POLICY_ALLOCATE
            | MMU_POLICY_WRITE_BACK
            | MMU_POLICY_ENABLED,
    );
    unicache_mmu_configure_small_page(
        3,
        0x20004000,
        0x55024000,
        MMU_POLICY_POSTED
            | MMU_POLICY_CACHEABLE
            | MMU_POLICY_LARGE
            | MMU_POLICY_ALLOCATE
            | MMU_POLICY_WRITE_BACK
            | MMU_POLICY_ENABLED,
    );
    unicache_mmu_configure_small_page(
        4,
        0x20008000,
        0x55028000,
        MMU_POLICY_POSTED
            | MMU_POLICY_CACHEABLE
            | MMU_POLICY_LARGE
            | MMU_POLICY_ALLOCATE
            | MMU_POLICY_WRITE_BACK
            | MMU_POLICY_ENABLED,
    );
    unicache_mmu_configure_small_page(
        5,
        0x2000C000,
        0x5502C000,
        MMU_POLICY_POSTED
            | MMU_POLICY_CACHEABLE
            | MMU_POLICY_LARGE
            | MMU_POLICY_ALLOCATE
            | MMU_POLICY_WRITE_BACK
            | MMU_POLICY_ENABLED,
    );
}

/// Configure a large MMU page.
fn unicache_mmu_configure_large_page(idx: usize, addr: usize, xlte: usize, policy: u32) {
    unsafe {
        let unicache_mmu = get_unicache_mmu();
        unicache_mmu.large_addr[idx].write(addr);
        unicache_mmu.large_xlte[idx].write(xlte);
        unicache_mmu.large_policy[idx].write(policy);
    }
}

/// Configure a small MMU page.
fn unicache_mmu_configure_small_page(idx: usize, addr: usize, xlte: usize, policy: u32) {
    unsafe {
        let unicache_mmu = get_unicache_mmu();
        unicache_mmu.small_addr[idx].write(addr);
        unicache_mmu.small_xlte[idx].write(xlte);
        unicache_mmu.small_policy[idx].write(policy);
    }
}

/// Get a reference to the Unicache MMU peripheral. This is local to the M4 so
/// does not need mapping.
fn get_unicache_mmu() -> &'static mut UnicacheMmu {
    unsafe { &mut *(UNICACHE_MMU_ADDR as *mut UnicacheMmu) }
}

/// Get a reference to the Unicache config peripheral. This is local to the M4
/// so does not need mapping.
fn get_unicache_config() -> &'static mut UnicacheConfig {
    unsafe { &mut *(UNICACHE_CFG_ADDR as *mut UnicacheConfig) }
}

/// Get a reference to the WuGen peripheral. This is local to the M4
/// so does not need mapping.
fn get_wugen() -> &'static mut WuGen {
    unsafe { &mut *(WUGEN_ADDR as *mut WuGen) }
}

/// Get a reference to a specific mailbox instance. The mailboxes are remote
/// to the M4 so we need something that can map the fixed physical address of
/// the peripheral to the device address we need as mapped in the IOMMU.
pub fn get_mailbox<T>(mbox_type: MailboxId, mapper: &T) -> Option<&'static mut Mailbox>
where
    T: rt::AddressMapper,
{
    // See Table 19-24 in the TRM, Mailbox Instance Summary
    // These addresses are in L4_PERIPHERAL_L4PER3 or L4_PERIPHERAL_L4CFG.
    let pa = match mbox_type {
        MailboxId::Mailbox1 => 0x4A0F_4000,
        MailboxId::Mailbox2 => 0x4883_a000,
        MailboxId::Mailbox3 => 0x4883_c000,
        MailboxId::Mailbox4 => 0x4883_e000,
        MailboxId::Mailbox5 => 0x4884_0000,
        MailboxId::Mailbox6 => 0x4884_2000,
        MailboxId::Mailbox7 => 0x4884_4000,
        MailboxId::Mailbox8 => 0x4884_6000,
        MailboxId::Mailbox9 => 0x4885_e000,
        MailboxId::Mailbox10 => 0x4886_0000,
        MailboxId::Mailbox11 => 0x4886_2000,
        MailboxId::Mailbox12 => 0x4886_4000,
        MailboxId::Mailbox13 => 0x4880_2000,
    };
    match mapper.pa_to_da(pa) {
        Some(pa) => unsafe { Some(&mut *(pa as *mut Mailbox)) },
        None => None,
    }
}

/// The Wake-Up Generator. Allows our core to come out of idle.
/// Corresponds to IPUx_WUGEN in section 7.4.7
#[repr(C)]
struct WuGen {
    /// Used to interrup the other core
    cortexm4_ctrl: RW<u32>,
    /// Set the standby protocol
    standby_core_sysconfig: RW<u32>,
    /// Set the idle protocol
    idle_core_sysconfig: RW<u32>,
    /// Interrupt mask for interrupts 0-31
    wugen_evt0: RW<u32>,
    /// Interrupt mask for interrupts 32-63
    wugen_evt1: RW<u32>,
    _reserved: RW<u32>,
}

/// Corresponds to IPUx_UNICACHE_CFG in section 7.4.2
#[repr(C)]
struct UnicacheConfig {
    info: RW<u32>,
    config: RW<u32>,
    int: RW<u32>,
    ocp: RW<u32>,
    maint: RW<u32>,
    mt_start: RW<usize>,
    mt_end: RW<usize>,
    ct_addr: RW<usize>,
    ct_data: RW<u32>,
}

#[repr(C)]
struct UnicacheMmu {
    // large pages (4 out of 8)
    large_addr: [RW<usize>; 4],
    _padding: [usize; 4],

    large_xlte: [RW<usize>; 4],
    _padding2: [usize; 4],

    large_policy: [RW<u32>; 4],
    _padding3: [usize; 4],

    // medium pages (2 out of 16)
    medium_addr: [RW<usize>; 2],
    _padding4: [usize; 14],

    medium_xlte: [RW<usize>; 2],
    _padding5: [usize; 14],

    medium_policy: [RW<u32>; 2],
    _padding6: [usize; 14],

    // small pages (10 out of 32)
    small_addr: [RW<usize>; 10],
    _padding7: [usize; 22],

    small_xlte: [RW<usize>; 10],
    _padding8: [usize; 22],

    small_policy: [RW<u32>; 10],
    _padding9: [usize; 22],

    small_maint: [RW<u32>; 10],
    _padding10: [usize; 22],

    // lines
    line_addr: [RW<usize>; 32],
    line_xlte: [RW<usize>; 32],
    line_policy: [RW<u32>; 32],

    // debug
    debug_xlte: RW<usize>,
    debug_policy: RW<u32>,

    // maintenance
    maint: RW<u32>,
    mstart: RW<usize>,
    mend: RW<usize>,
    maint_status: RW<u32>,
    mmu_config: RW<u32>,
}

impl WuGen {
    /// Enable a wake-up trigger on IPU1 when this interrupt fires.
    pub fn wake_on_interrupt(&mut self, interrupt: Interrupt) {
        let num = interrupt.nr() as u32;
        if num >= 48 {
            // Interrupt bit is in second register of 32..63
            unsafe {
                self.wugen_evt1.modify(|m| m | num);
            }
        } else if num >= 16 {
            // Interrupt bit is in first register of 0..31
            unsafe {
                self.wugen_evt0.modify(|m| m | num);
            }
        } else {
            panic!("Can't enable WuGen interrupt when < 16")
        }
    }
}

/// Stores two 9-bit values with 16-bit alignment
#[repr(C)]
struct IrqRegister {
    field: RW<u32>,
}

impl IrqRegister {
    fn set_higher(&mut self, crossbar_irq: u16) {
        unsafe {
            self.field.modify(|mut w| {
                w &= 0x0000_FFFF;
                w |= (crossbar_irq as u32) << 16;
                w
            });
        }
    }

    fn set_lower(&mut self, crossbar_irq: u16) {
        unsafe {
            self.field.modify(|mut w| {
                w &= 0xFFFF_0000;
                w |= (crossbar_irq as u32) << 0;
                w
            });
        }
    }

    fn get_higher(&mut self) -> u16 {
        (self.field.read() >> 16) as u16
    }

    fn get_lower(&mut self) -> u16 {
        (self.field.read() >> 0) as u16
    }
}

/// Controls the Crossbar IRQ mapping for IPU1. Use these registers to send a
/// Crossbar IRQ into the IPU on the given interrupt line.
#[repr(C)]
struct CtrlCoreIpu1 {
    irq_23_24: IrqRegister,
    irq_25_26: IrqRegister,
    irq_27_28: IrqRegister,
    irq_29_30: IrqRegister,
    irq_31_32: IrqRegister,
    irq_33_34: IrqRegister,
    irq_35_36: IrqRegister,
    irq_37_38: IrqRegister,
    irq_39_40: IrqRegister,
    irq_41_42: IrqRegister,
    irq_43_44: IrqRegister,
    irq_45_46: IrqRegister,
    irq_47_48: IrqRegister,
    irq_49_50: IrqRegister,
    irq_51_52: IrqRegister,
    irq_53_54: IrqRegister,
    irq_55_56: IrqRegister,
    irq_57_58: IrqRegister,
    irq_59_60: IrqRegister,
    irq_61_62: IrqRegister,
    irq_63_64: IrqRegister,
    irq_65_66: IrqRegister,
    irq_67_68: IrqRegister,
    irq_69_70: IrqRegister,
    irq_71_72: IrqRegister,
    irq_73_74: IrqRegister,
    irq_75_76: IrqRegister,
    irq_77_78: IrqRegister,
    irq_79_80: IrqRegister,
}

impl CtrlCoreIpu1 {
    /// Get a reference to a specific IPU1 IRQ mapping instance. The peripheral is remote
    /// to the M4 so we need something that can map the fixed physical address of
    /// the peripheral to the device address we need as mapped in the IOMMU.
    fn get<T>(mapper: &T) -> Option<&'static mut CtrlCoreIpu1>
    where
        T: rt::AddressMapper,
    {
        // See Table 18-28. CTRL_MODULE_CORE Registers Mapping Summary
        match mapper.pa_to_da(0x4A00_27E0) {
            Some(pa) => unsafe { Some(&mut *(pa as *mut CtrlCoreIpu1)) },
            None => None,
        }
    }
}
