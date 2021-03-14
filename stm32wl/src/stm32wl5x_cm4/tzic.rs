#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TZIC interrupt enable register 1"]
    pub ier1: IER1,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - TZIC status register 1"]
    pub misr1: MISR1,
    _reserved2: [u8; 12usize],
    #[doc = "0x20 - TZIC interrupt status clear register 1"]
    pub icr1: ICR1,
}
#[doc = "TZIC interrupt enable register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier1](ier1) module"]
pub type IER1 = crate::Reg<u32, _IER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER1;
#[doc = "`read()` method returns [ier1::R](ier1::R) reader structure"]
impl crate::Readable for IER1 {}
#[doc = "`write(|w| ..)` method takes [ier1::W](ier1::W) writer structure"]
impl crate::Writable for IER1 {}
#[doc = "TZIC interrupt enable register 1"]
pub mod ier1;
#[doc = "TZIC status register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misr1](misr1) module"]
pub type MISR1 = crate::Reg<u32, _MISR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISR1;
#[doc = "`read()` method returns [misr1::R](misr1::R) reader structure"]
impl crate::Readable for MISR1 {}
#[doc = "TZIC status register 1"]
pub mod misr1;
#[doc = "TZIC interrupt status clear register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr1](icr1) module"]
pub type ICR1 = crate::Reg<u32, _ICR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR1;
#[doc = "`read()` method returns [icr1::R](icr1::R) reader structure"]
impl crate::Readable for ICR1 {}
#[doc = "`write(|w| ..)` method takes [icr1::W](icr1::W) writer structure"]
impl crate::Writable for ICR1 {}
#[doc = "TZIC interrupt status clear register 1"]
pub mod icr1;
