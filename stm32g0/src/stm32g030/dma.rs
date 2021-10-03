#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - low interrupt status register"]
    pub isr: crate::Reg<isr::ISR_SPEC>,
    #[doc = "0x04 - DMA interrupt flag clear register"]
    pub ifcr: crate::Reg<ifcr::IFCR_SPEC>,
    #[doc = "0x08..0x18 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    pub ch1: CH,
    _reserved3: [u8; 0x04],
    #[doc = "0x1c..0x2c - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    pub ch2: CH,
    _reserved4: [u8; 0x04],
    #[doc = "0x30..0x40 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    pub ch3: CH,
    _reserved5: [u8; 0x04],
    #[doc = "0x44..0x54 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    pub ch4: CH,
    _reserved6: [u8; 0x04],
    #[doc = "0x58..0x68 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    pub ch5: CH,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - DMA channel x configuration register"]
    pub cr: crate::Reg<self::ch::cr::CR_SPEC>,
    #[doc = "0x04 - DMA channel x number of data register"]
    pub ndtr: crate::Reg<self::ch::ndtr::NDTR_SPEC>,
    #[doc = "0x08 - DMA channel x peripheral address register"]
    pub par: crate::Reg<self::ch::par::PAR_SPEC>,
    #[doc = "0x0c - DMA channel x memory address register"]
    pub mar: crate::Reg<self::ch::mar::MAR_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
pub mod ch;
#[doc = "ISR register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "low interrupt status register"]
pub mod isr;
#[doc = "IFCR register accessor: an alias for `Reg<IFCR_SPEC>`"]
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
#[doc = "DMA interrupt flag clear register"]
pub mod ifcr;
