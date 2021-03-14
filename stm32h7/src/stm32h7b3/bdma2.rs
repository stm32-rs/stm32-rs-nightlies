#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA interrupt status register"]
    pub isr: ISR,
    #[doc = "0x04 - DMA interrupt flag clear register"]
    pub ifcr: IFCR,
    #[doc = "0x08 - Channel cluster: CCR?, CNDTR?, CPAR?, CM0AR? and CM1AR registers?"]
    pub ch: [CH; 8],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - DMA channel x configuration register"]
    pub cr: self::ch::CR,
    #[doc = "0x04 - DMA channel x number of data register"]
    pub ndtr: self::ch::NDTR,
    #[doc = "0x08 - This register must not be written when the channel is enabled."]
    pub par: self::ch::PAR,
    #[doc = "0x0c - This register must not be written when the channel is enabled."]
    pub m0ar: self::ch::M0AR,
    #[doc = "0x10 - This register must not be written when the channel is enabled"]
    pub m1ar: self::ch::M1AR,
}
#[doc = r"Register block"]
#[doc = "Channel cluster: CCR?, CNDTR?, CPAR?, CM0AR? and CM1AR registers?"]
pub mod ch;
#[doc = "DMA interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "DMA interrupt status register"]
pub mod isr;
#[doc = "DMA interrupt flag clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifcr](ifcr) module"]
pub type IFCR = crate::Reg<u32, _IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFCR;
#[doc = "`write(|w| ..)` method takes [ifcr::W](ifcr::W) writer structure"]
impl crate::Writable for IFCR {}
#[doc = "DMA interrupt flag clear register"]
pub mod ifcr;
