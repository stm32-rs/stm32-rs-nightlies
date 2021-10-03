#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA interrupt status register"]
    pub isr: crate::Reg<isr::ISR_SPEC>,
    #[doc = "0x04 - DMA interrupt flag clear register"]
    pub ifcr: crate::Reg<ifcr::IFCR_SPEC>,
    #[doc = "0x08..0xa8 - Channel cluster: CCR?, CNDTR?, CPAR?, CM0AR? and CM1AR registers?"]
    pub ch: [CH; 8],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - DMA channel x configuration register"]
    pub cr: crate::Reg<self::ch::cr::CR_SPEC>,
    #[doc = "0x04 - DMA channel x number of data register"]
    pub ndtr: crate::Reg<self::ch::ndtr::NDTR_SPEC>,
    #[doc = "0x08 - This register must not be written when the channel is enabled."]
    pub par: crate::Reg<self::ch::par::PAR_SPEC>,
    #[doc = "0x0c - This register must not be written when the channel is enabled."]
    pub m0ar: crate::Reg<self::ch::m0ar::M0AR_SPEC>,
    #[doc = "0x10 - This register must not be written when the channel is enabled"]
    pub m1ar: crate::Reg<self::ch::m1ar::M1AR_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Channel cluster: CCR?, CNDTR?, CPAR?, CM0AR? and CM1AR registers?"]
pub mod ch;
#[doc = "ISR register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "DMA interrupt status register"]
pub mod isr;
#[doc = "IFCR register accessor: an alias for `Reg<IFCR_SPEC>`"]
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
#[doc = "DMA interrupt flag clear register"]
pub mod ifcr;
