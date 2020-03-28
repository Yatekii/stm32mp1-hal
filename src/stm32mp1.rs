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
            let _ = $crate::stm32mp1::Interrupt::$Name;
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
            let _ = $crate::stm32mp1::Interrupt::$Name;
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
pub struct Stm32mp1;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum Interrupt {
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

// ****************************************************************************
//
// Public Data
//
// ****************************************************************************

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

impl Stm32mp1
{
    /// Returns an object the first time, None the second time.
    pub fn claim() -> Option<Stm32mp1> {
        static mut AVAILABLE: bool = true;
        unsafe {
            if AVAILABLE {
                let mut r = Stm32mp1 { };
                r.setup();
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

    }

//    /// Enable an interrupt on IPU1_C0
//    pub fn interrupt_enable(&mut self, interrupt: Interrupt) {
//        let mut peripherals = unsafe { cortex_m::Peripherals::steal() };
//        peripherals.NVIC.enable(interrupt);
//    }
//
//    /// Clear a pending interrupt on IPU1_C0
//    pub fn interrupt_clear(&mut self, interrupt: Interrupt) {
//        NVIC::unpend(interrupt);
//    }
//
//    /// Disable an interrupt on IPU1_C0
//    pub fn interrupt_disable(&mut self, interrupt: Interrupt) {
//        let mut peripherals = unsafe { cortex_m::Peripherals::steal() };
//        peripherals.NVIC.disable(interrupt);
//    }
//
//    /// Set interrupt priority on IPU1_C0
//    pub fn interrupt_priority_set(&mut self, interrupt: Interrupt, priority: InterruptPriority) {
//        let mut peripherals = unsafe { cortex_m::Peripherals::steal() };
//        unsafe { peripherals.NVIC.set_priority(interrupt, priority as u8) };
//    }
}

// ****************************************************************************
//
// Private Functions
//
// ****************************************************************************


// ****************************************************************************
//
// End Of File
//
// ****************************************************************************
