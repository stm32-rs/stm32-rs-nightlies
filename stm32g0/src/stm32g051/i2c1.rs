#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register 1"]
    pub i2c_cr1: crate::Reg<i2c_cr1::I2C_CR1_SPEC>,
    #[doc = "0x04 - Control register 2"]
    pub i2c_cr2: crate::Reg<i2c_cr2::I2C_CR2_SPEC>,
    #[doc = "0x08 - Own address register 1"]
    pub i2c_oar1: crate::Reg<i2c_oar1::I2C_OAR1_SPEC>,
    #[doc = "0x0c - Own address register 2"]
    pub i2c_oar2: crate::Reg<i2c_oar2::I2C_OAR2_SPEC>,
    #[doc = "0x10 - Timing register"]
    pub i2c_timingr: crate::Reg<i2c_timingr::I2C_TIMINGR_SPEC>,
    #[doc = "0x14 - Status register 1"]
    pub i2c_timeoutr: crate::Reg<i2c_timeoutr::I2C_TIMEOUTR_SPEC>,
    #[doc = "0x18 - Interrupt and Status register"]
    pub i2c_isr: crate::Reg<i2c_isr::I2C_ISR_SPEC>,
    #[doc = "0x1c - Interrupt clear register"]
    pub i2c_icr: crate::Reg<i2c_icr::I2C_ICR_SPEC>,
    #[doc = "0x20 - PEC register"]
    pub i2c_pecr: crate::Reg<i2c_pecr::I2C_PECR_SPEC>,
    #[doc = "0x24 - Receive data register"]
    pub i2c_rxdr: crate::Reg<i2c_rxdr::I2C_RXDR_SPEC>,
    #[doc = "0x28 - Transmit data register"]
    pub i2c_txdr: crate::Reg<i2c_txdr::I2C_TXDR_SPEC>,
}
#[doc = "I2C_CR1 register accessor: an alias for `Reg<I2C_CR1_SPEC>`"]
pub type I2C_CR1 = crate::Reg<i2c_cr1::I2C_CR1_SPEC>;
#[doc = "Control register 1"]
pub mod i2c_cr1;
#[doc = "I2C_CR2 register accessor: an alias for `Reg<I2C_CR2_SPEC>`"]
pub type I2C_CR2 = crate::Reg<i2c_cr2::I2C_CR2_SPEC>;
#[doc = "Control register 2"]
pub mod i2c_cr2;
#[doc = "I2C_OAR1 register accessor: an alias for `Reg<I2C_OAR1_SPEC>`"]
pub type I2C_OAR1 = crate::Reg<i2c_oar1::I2C_OAR1_SPEC>;
#[doc = "Own address register 1"]
pub mod i2c_oar1;
#[doc = "I2C_OAR2 register accessor: an alias for `Reg<I2C_OAR2_SPEC>`"]
pub type I2C_OAR2 = crate::Reg<i2c_oar2::I2C_OAR2_SPEC>;
#[doc = "Own address register 2"]
pub mod i2c_oar2;
#[doc = "I2C_TIMINGR register accessor: an alias for `Reg<I2C_TIMINGR_SPEC>`"]
pub type I2C_TIMINGR = crate::Reg<i2c_timingr::I2C_TIMINGR_SPEC>;
#[doc = "Timing register"]
pub mod i2c_timingr;
#[doc = "I2C_TIMEOUTR register accessor: an alias for `Reg<I2C_TIMEOUTR_SPEC>`"]
pub type I2C_TIMEOUTR = crate::Reg<i2c_timeoutr::I2C_TIMEOUTR_SPEC>;
#[doc = "Status register 1"]
pub mod i2c_timeoutr;
#[doc = "I2C_ISR register accessor: an alias for `Reg<I2C_ISR_SPEC>`"]
pub type I2C_ISR = crate::Reg<i2c_isr::I2C_ISR_SPEC>;
#[doc = "Interrupt and Status register"]
pub mod i2c_isr;
#[doc = "I2C_ICR register accessor: an alias for `Reg<I2C_ICR_SPEC>`"]
pub type I2C_ICR = crate::Reg<i2c_icr::I2C_ICR_SPEC>;
#[doc = "Interrupt clear register"]
pub mod i2c_icr;
#[doc = "I2C_PECR register accessor: an alias for `Reg<I2C_PECR_SPEC>`"]
pub type I2C_PECR = crate::Reg<i2c_pecr::I2C_PECR_SPEC>;
#[doc = "PEC register"]
pub mod i2c_pecr;
#[doc = "I2C_RXDR register accessor: an alias for `Reg<I2C_RXDR_SPEC>`"]
pub type I2C_RXDR = crate::Reg<i2c_rxdr::I2C_RXDR_SPEC>;
#[doc = "Receive data register"]
pub mod i2c_rxdr;
#[doc = "I2C_TXDR register accessor: an alias for `Reg<I2C_TXDR_SPEC>`"]
pub type I2C_TXDR = crate::Reg<i2c_txdr::I2C_TXDR_SPEC>;
#[doc = "Transmit data register"]
pub mod i2c_txdr;
