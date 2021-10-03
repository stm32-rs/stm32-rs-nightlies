#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x PCLK1 + 6 x I2CCLK."]
    pub cr1: crate::Reg<cr1::CR1_SPEC>,
    #[doc = "0x04 - Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x PCLK1 + 6 x I2CCLK."]
    pub cr2: crate::Reg<cr2::CR2_SPEC>,
    #[doc = "0x08 - Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x PCLK1 + 6 x I2CCLK."]
    pub oar1: crate::Reg<oar1::OAR1_SPEC>,
    #[doc = "0x0c - Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x PCLK1 + 6 x I2CCLK."]
    pub oar2: crate::Reg<oar2::OAR2_SPEC>,
    #[doc = "0x10 - Access: No wait states"]
    pub timingr: crate::Reg<timingr::TIMINGR_SPEC>,
    #[doc = "0x14 - Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x PCLK1 + 6 x I2CCLK."]
    pub timeoutr: crate::Reg<timeoutr::TIMEOUTR_SPEC>,
    #[doc = "0x18 - Access: No wait states"]
    pub isr: crate::Reg<isr::ISR_SPEC>,
    #[doc = "0x1c - Access: No wait states"]
    pub icr: crate::Reg<icr::ICR_SPEC>,
    #[doc = "0x20 - Access: No wait states"]
    pub pecr: crate::Reg<pecr::PECR_SPEC>,
    #[doc = "0x24 - Access: No wait states"]
    pub rxdr: crate::Reg<rxdr::RXDR_SPEC>,
    #[doc = "0x28 - Access: No wait states"]
    pub txdr: crate::Reg<txdr::TXDR_SPEC>,
}
#[doc = "CR1 register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x PCLK1 + 6 x I2CCLK."]
pub mod cr1;
#[doc = "CR2 register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x PCLK1 + 6 x I2CCLK."]
pub mod cr2;
#[doc = "OAR1 register accessor: an alias for `Reg<OAR1_SPEC>`"]
pub type OAR1 = crate::Reg<oar1::OAR1_SPEC>;
#[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x PCLK1 + 6 x I2CCLK."]
pub mod oar1;
#[doc = "OAR2 register accessor: an alias for `Reg<OAR2_SPEC>`"]
pub type OAR2 = crate::Reg<oar2::OAR2_SPEC>;
#[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x PCLK1 + 6 x I2CCLK."]
pub mod oar2;
#[doc = "TIMINGR register accessor: an alias for `Reg<TIMINGR_SPEC>`"]
pub type TIMINGR = crate::Reg<timingr::TIMINGR_SPEC>;
#[doc = "Access: No wait states"]
pub mod timingr;
#[doc = "TIMEOUTR register accessor: an alias for `Reg<TIMEOUTR_SPEC>`"]
pub type TIMEOUTR = crate::Reg<timeoutr::TIMEOUTR_SPEC>;
#[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x PCLK1 + 6 x I2CCLK."]
pub mod timeoutr;
#[doc = "ISR register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Access: No wait states"]
pub mod isr;
#[doc = "ICR register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "Access: No wait states"]
pub mod icr;
#[doc = "PECR register accessor: an alias for `Reg<PECR_SPEC>`"]
pub type PECR = crate::Reg<pecr::PECR_SPEC>;
#[doc = "Access: No wait states"]
pub mod pecr;
#[doc = "RXDR register accessor: an alias for `Reg<RXDR_SPEC>`"]
pub type RXDR = crate::Reg<rxdr::RXDR_SPEC>;
#[doc = "Access: No wait states"]
pub mod rxdr;
#[doc = "TXDR register accessor: an alias for `Reg<TXDR_SPEC>`"]
pub type TXDR = crate::Reg<txdr::TXDR_SPEC>;
#[doc = "Access: No wait states"]
pub mod txdr;
