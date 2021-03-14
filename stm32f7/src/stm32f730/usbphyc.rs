#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USBPHYC PLL1 control register"]
    pub pll1: PLL1,
    _reserved1: [u8; 8usize],
    #[doc = "0x0c - USBPHYC tuning control register"]
    pub tune: TUNE,
    _reserved2: [u8; 8usize],
    #[doc = "0x18 - USBPHYC LDO control and status register"]
    pub ldo: LDO,
}
#[doc = "USBPHYC PLL1 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll1](pll1) module"]
pub type PLL1 = crate::Reg<u32, _PLL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL1;
#[doc = "`read()` method returns [pll1::R](pll1::R) reader structure"]
impl crate::Readable for PLL1 {}
#[doc = "`write(|w| ..)` method takes [pll1::W](pll1::W) writer structure"]
impl crate::Writable for PLL1 {}
#[doc = "USBPHYC PLL1 control register"]
pub mod pll1;
#[doc = "USBPHYC tuning control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tune](tune) module"]
pub type TUNE = crate::Reg<u32, _TUNE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TUNE;
#[doc = "`read()` method returns [tune::R](tune::R) reader structure"]
impl crate::Readable for TUNE {}
#[doc = "`write(|w| ..)` method takes [tune::W](tune::W) writer structure"]
impl crate::Writable for TUNE {}
#[doc = "USBPHYC tuning control register"]
pub mod tune;
#[doc = "USBPHYC LDO control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldo](ldo) module"]
pub type LDO = crate::Reg<u32, _LDO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LDO;
#[doc = "`read()` method returns [ldo::R](ldo::R) reader structure"]
impl crate::Readable for LDO {}
#[doc = "`write(|w| ..)` method takes [ldo::W](ldo::W) writer structure"]
impl crate::Writable for LDO {}
#[doc = "USBPHYC LDO control and status register"]
pub mod ldo;
