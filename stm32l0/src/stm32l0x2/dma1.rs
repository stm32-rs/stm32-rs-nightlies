#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - interrupt status register"]
    pub isr: ISR,
    #[doc = "0x04 - interrupt flag clear register"]
    pub ifcr: IFCR,
    #[doc = "0x08 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    pub ch1: CH,
    _reserved3: [u8; 4usize],
    #[doc = "0x1c - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    pub ch2: CH,
    _reserved4: [u8; 4usize],
    #[doc = "0x30 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    pub ch3: CH,
    _reserved5: [u8; 4usize],
    #[doc = "0x44 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    pub ch4: CH,
    _reserved6: [u8; 4usize],
    #[doc = "0x58 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    pub ch5: CH,
    _reserved7: [u8; 4usize],
    #[doc = "0x6c - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    pub ch6: CH,
    _reserved8: [u8; 4usize],
    #[doc = "0x80 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    pub ch7: CH,
    _reserved9: [u8; 24usize],
    #[doc = "0xa8 - channel selection register"]
    pub cselr: CSELR,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - channel x configuration register"]
    pub cr: self::ch::CR,
    #[doc = "0x04 - channel x number of data register"]
    pub ndtr: self::ch::NDTR,
    #[doc = "0x08 - channel x peripheral address register"]
    pub par: self::ch::PAR,
    #[doc = "0x0c - channel x memory address register"]
    pub mar: self::ch::MAR,
}
#[doc = r"Register block"]
#[doc = "Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
pub mod ch;
#[doc = "interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "interrupt status register"]
pub mod isr;
#[doc = "interrupt flag clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifcr](ifcr) module"]
pub type IFCR = crate::Reg<u32, _IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFCR;
#[doc = "`write(|w| ..)` method takes [ifcr::W](ifcr::W) writer structure"]
impl crate::Writable for IFCR {}
#[doc = "interrupt flag clear register"]
pub mod ifcr;
#[doc = "channel selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cselr](cselr) module"]
pub type CSELR = crate::Reg<u32, _CSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSELR;
#[doc = "`read()` method returns [cselr::R](cselr::R) reader structure"]
impl crate::Readable for CSELR {}
#[doc = "`write(|w| ..)` method takes [cselr::W](cselr::W) writer structure"]
impl crate::Writable for CSELR {}
#[doc = "channel selection register"]
pub mod cselr;
