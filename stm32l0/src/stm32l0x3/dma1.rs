#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - interrupt status register"]
    pub isr: crate::Reg<isr::ISR_SPEC>,
    #[doc = "0x04 - interrupt flag clear register"]
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
    _reserved7: [u8; 0x04],
    #[doc = "0x6c..0x7c - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    pub ch6: CH,
    _reserved8: [u8; 0x04],
    #[doc = "0x80..0x90 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    pub ch7: CH,
    _reserved9: [u8; 0x18],
    #[doc = "0xa8 - channel selection register"]
    pub cselr: crate::Reg<cselr::CSELR_SPEC>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - channel x configuration register"]
    pub cr: crate::Reg<self::ch::cr::CR_SPEC>,
    #[doc = "0x04 - channel x number of data register"]
    pub ndtr: crate::Reg<self::ch::ndtr::NDTR_SPEC>,
    #[doc = "0x08 - channel x peripheral address register"]
    pub par: crate::Reg<self::ch::par::PAR_SPEC>,
    #[doc = "0x0c - channel x memory address register"]
    pub mar: crate::Reg<self::ch::mar::MAR_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
pub mod ch;
#[doc = "ISR register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "interrupt status register"]
pub mod isr;
#[doc = "IFCR register accessor: an alias for `Reg<IFCR_SPEC>`"]
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
#[doc = "interrupt flag clear register"]
pub mod ifcr;
#[doc = "CSELR register accessor: an alias for `Reg<CSELR_SPEC>`"]
pub type CSELR = crate::Reg<cselr::CSELR_SPEC>;
#[doc = "channel selection register"]
pub mod cselr;
