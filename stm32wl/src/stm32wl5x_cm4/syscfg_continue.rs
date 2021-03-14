#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IMR1"]
    pub imr1: IMR1,
    #[doc = "0x04 - IMR2"]
    pub imr2: IMR2,
    #[doc = "0x08 - C2IMR1"]
    pub c2imr1: C2IMR1,
    #[doc = "0x0c - C2IMR2"]
    pub c2imr2: C2IMR2,
}
#[doc = "IMR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr1](imr1) module"]
pub type IMR1 = crate::Reg<u32, _IMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR1;
#[doc = "`read()` method returns [imr1::R](imr1::R) reader structure"]
impl crate::Readable for IMR1 {}
#[doc = "`write(|w| ..)` method takes [imr1::W](imr1::W) writer structure"]
impl crate::Writable for IMR1 {}
#[doc = "IMR1"]
pub mod imr1;
#[doc = "IMR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr2](imr2) module"]
pub type IMR2 = crate::Reg<u32, _IMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR2;
#[doc = "`read()` method returns [imr2::R](imr2::R) reader structure"]
impl crate::Readable for IMR2 {}
#[doc = "`write(|w| ..)` method takes [imr2::W](imr2::W) writer structure"]
impl crate::Writable for IMR2 {}
#[doc = "IMR2"]
pub mod imr2;
#[doc = "C2IMR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2imr1](c2imr1) module"]
pub type C2IMR1 = crate::Reg<u32, _C2IMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2IMR1;
#[doc = "`read()` method returns [c2imr1::R](c2imr1::R) reader structure"]
impl crate::Readable for C2IMR1 {}
#[doc = "`write(|w| ..)` method takes [c2imr1::W](c2imr1::W) writer structure"]
impl crate::Writable for C2IMR1 {}
#[doc = "C2IMR1"]
pub mod c2imr1;
#[doc = "C2IMR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2imr2](c2imr2) module"]
pub type C2IMR2 = crate::Reg<u32, _C2IMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2IMR2;
#[doc = "`read()` method returns [c2imr2::R](c2imr2::R) reader structure"]
impl crate::Readable for C2IMR2 {}
#[doc = "`write(|w| ..)` method takes [c2imr2::W](c2imr2::W) writer structure"]
impl crate::Writable for C2IMR2 {}
#[doc = "C2IMR2"]
pub mod c2imr2;
