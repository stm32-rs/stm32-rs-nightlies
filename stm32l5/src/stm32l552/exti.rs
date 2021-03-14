#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EXTI rising trigger selection register"]
    pub rtsr1: RTSR1,
    #[doc = "0x04 - EXTI falling trigger selection register"]
    pub ftsr1: FTSR1,
    #[doc = "0x08 - EXTI software interrupt event register"]
    pub swier1: SWIER1,
    #[doc = "0x0c - EXTI rising edge pending register"]
    pub rpr1: RPR1,
    #[doc = "0x10 - EXTI falling edge pending register"]
    pub fpr1: FPR1,
    #[doc = "0x14 - EXTI security configuration register"]
    pub seccfgr1: SECCFGR1,
    #[doc = "0x18 - EXTI privilege configuration register"]
    pub privcfgr1: PRIVCFGR1,
    _reserved7: [u8; 4usize],
    #[doc = "0x20 - EXTI rising trigger selection register"]
    pub rtsr2: RTSR2,
    #[doc = "0x24 - EXTI falling trigger selection register"]
    pub ftsr2: FTSR2,
    #[doc = "0x28 - EXTI software interrupt event register"]
    pub swier2: SWIER2,
    #[doc = "0x2c - EXTI rising edge pending register"]
    pub rpr2: RPR2,
    #[doc = "0x30 - EXTI falling edge pending register"]
    pub fpr2: FPR2,
    #[doc = "0x34 - EXTI security enable register"]
    pub privcfgr2: PRIVCFGR2,
    #[doc = "0x38 - EXTI security enable register"]
    pub seccfgr2: SECCFGR2,
    _reserved14: [u8; 36usize],
    #[doc = "0x60 - EXTI external interrupt selection register"]
    pub exticr1: EXTICR1,
    #[doc = "0x64 - EXTI external interrupt selection register"]
    pub exticr2: EXTICR2,
    #[doc = "0x68 - EXTI external interrupt selection register"]
    pub exticr3: EXTICR3,
    #[doc = "0x6c - EXTI external interrupt selection register"]
    pub exticr4: EXTICR4,
    #[doc = "0x70 - EXTI lock register"]
    pub lockrg: LOCKRG,
    _reserved19: [u8; 12usize],
    #[doc = "0x80 - EXTI CPU wakeup with interrupt mask register"]
    pub imr1: IMR1,
    #[doc = "0x84 - EXTI CPU wakeup with event mask register"]
    pub emr1: EMR1,
    _reserved21: [u8; 8usize],
    #[doc = "0x90 - EXTI CPUm wakeup with interrupt mask register"]
    pub imr2: IMR2,
    #[doc = "0x94 - EXTI CPU wakeup with event mask register"]
    pub emr2: EMR2,
}
#[doc = "EXTI rising trigger selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtsr1](rtsr1) module"]
pub type RTSR1 = crate::Reg<u32, _RTSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTSR1;
#[doc = "`read()` method returns [rtsr1::R](rtsr1::R) reader structure"]
impl crate::Readable for RTSR1 {}
#[doc = "`write(|w| ..)` method takes [rtsr1::W](rtsr1::W) writer structure"]
impl crate::Writable for RTSR1 {}
#[doc = "EXTI rising trigger selection register"]
pub mod rtsr1;
#[doc = "EXTI falling trigger selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftsr1](ftsr1) module"]
pub type FTSR1 = crate::Reg<u32, _FTSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FTSR1;
#[doc = "`read()` method returns [ftsr1::R](ftsr1::R) reader structure"]
impl crate::Readable for FTSR1 {}
#[doc = "`write(|w| ..)` method takes [ftsr1::W](ftsr1::W) writer structure"]
impl crate::Writable for FTSR1 {}
#[doc = "EXTI falling trigger selection register"]
pub mod ftsr1;
#[doc = "EXTI software interrupt event register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swier1](swier1) module"]
pub type SWIER1 = crate::Reg<u32, _SWIER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWIER1;
#[doc = "`read()` method returns [swier1::R](swier1::R) reader structure"]
impl crate::Readable for SWIER1 {}
#[doc = "`write(|w| ..)` method takes [swier1::W](swier1::W) writer structure"]
impl crate::Writable for SWIER1 {}
#[doc = "EXTI software interrupt event register"]
pub mod swier1;
#[doc = "EXTI rising edge pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpr1](rpr1) module"]
pub type RPR1 = crate::Reg<u32, _RPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPR1;
#[doc = "`read()` method returns [rpr1::R](rpr1::R) reader structure"]
impl crate::Readable for RPR1 {}
#[doc = "`write(|w| ..)` method takes [rpr1::W](rpr1::W) writer structure"]
impl crate::Writable for RPR1 {}
#[doc = "EXTI rising edge pending register"]
pub mod rpr1;
#[doc = "EXTI falling edge pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpr1](fpr1) module"]
pub type FPR1 = crate::Reg<u32, _FPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPR1;
#[doc = "`read()` method returns [fpr1::R](fpr1::R) reader structure"]
impl crate::Readable for FPR1 {}
#[doc = "`write(|w| ..)` method takes [fpr1::W](fpr1::W) writer structure"]
impl crate::Writable for FPR1 {}
#[doc = "EXTI falling edge pending register"]
pub mod fpr1;
#[doc = "EXTI security configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seccfgr1](seccfgr1) module"]
pub type SECCFGR1 = crate::Reg<u32, _SECCFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECCFGR1;
#[doc = "`read()` method returns [seccfgr1::R](seccfgr1::R) reader structure"]
impl crate::Readable for SECCFGR1 {}
#[doc = "`write(|w| ..)` method takes [seccfgr1::W](seccfgr1::W) writer structure"]
impl crate::Writable for SECCFGR1 {}
#[doc = "EXTI security configuration register"]
pub mod seccfgr1;
#[doc = "EXTI privilege configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [privcfgr1](privcfgr1) module"]
pub type PRIVCFGR1 = crate::Reg<u32, _PRIVCFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIVCFGR1;
#[doc = "`read()` method returns [privcfgr1::R](privcfgr1::R) reader structure"]
impl crate::Readable for PRIVCFGR1 {}
#[doc = "`write(|w| ..)` method takes [privcfgr1::W](privcfgr1::W) writer structure"]
impl crate::Writable for PRIVCFGR1 {}
#[doc = "EXTI privilege configuration register"]
pub mod privcfgr1;
#[doc = "EXTI rising trigger selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtsr2](rtsr2) module"]
pub type RTSR2 = crate::Reg<u32, _RTSR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTSR2;
#[doc = "`read()` method returns [rtsr2::R](rtsr2::R) reader structure"]
impl crate::Readable for RTSR2 {}
#[doc = "`write(|w| ..)` method takes [rtsr2::W](rtsr2::W) writer structure"]
impl crate::Writable for RTSR2 {}
#[doc = "EXTI rising trigger selection register"]
pub mod rtsr2;
#[doc = "EXTI falling trigger selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftsr2](ftsr2) module"]
pub type FTSR2 = crate::Reg<u32, _FTSR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FTSR2;
#[doc = "`read()` method returns [ftsr2::R](ftsr2::R) reader structure"]
impl crate::Readable for FTSR2 {}
#[doc = "`write(|w| ..)` method takes [ftsr2::W](ftsr2::W) writer structure"]
impl crate::Writable for FTSR2 {}
#[doc = "EXTI falling trigger selection register"]
pub mod ftsr2;
#[doc = "EXTI software interrupt event register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swier2](swier2) module"]
pub type SWIER2 = crate::Reg<u32, _SWIER2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWIER2;
#[doc = "`read()` method returns [swier2::R](swier2::R) reader structure"]
impl crate::Readable for SWIER2 {}
#[doc = "`write(|w| ..)` method takes [swier2::W](swier2::W) writer structure"]
impl crate::Writable for SWIER2 {}
#[doc = "EXTI software interrupt event register"]
pub mod swier2;
#[doc = "EXTI rising edge pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpr2](rpr2) module"]
pub type RPR2 = crate::Reg<u32, _RPR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPR2;
#[doc = "`read()` method returns [rpr2::R](rpr2::R) reader structure"]
impl crate::Readable for RPR2 {}
#[doc = "`write(|w| ..)` method takes [rpr2::W](rpr2::W) writer structure"]
impl crate::Writable for RPR2 {}
#[doc = "EXTI rising edge pending register"]
pub mod rpr2;
#[doc = "EXTI falling edge pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpr2](fpr2) module"]
pub type FPR2 = crate::Reg<u32, _FPR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPR2;
#[doc = "`read()` method returns [fpr2::R](fpr2::R) reader structure"]
impl crate::Readable for FPR2 {}
#[doc = "`write(|w| ..)` method takes [fpr2::W](fpr2::W) writer structure"]
impl crate::Writable for FPR2 {}
#[doc = "EXTI falling edge pending register"]
pub mod fpr2;
#[doc = "EXTI security enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seccfgr2](seccfgr2) module"]
pub type SECCFGR2 = crate::Reg<u32, _SECCFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECCFGR2;
#[doc = "`read()` method returns [seccfgr2::R](seccfgr2::R) reader structure"]
impl crate::Readable for SECCFGR2 {}
#[doc = "`write(|w| ..)` method takes [seccfgr2::W](seccfgr2::W) writer structure"]
impl crate::Writable for SECCFGR2 {}
#[doc = "EXTI security enable register"]
pub mod seccfgr2;
#[doc = "EXTI security enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [privcfgr2](privcfgr2) module"]
pub type PRIVCFGR2 = crate::Reg<u32, _PRIVCFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIVCFGR2;
#[doc = "`read()` method returns [privcfgr2::R](privcfgr2::R) reader structure"]
impl crate::Readable for PRIVCFGR2 {}
#[doc = "`write(|w| ..)` method takes [privcfgr2::W](privcfgr2::W) writer structure"]
impl crate::Writable for PRIVCFGR2 {}
#[doc = "EXTI security enable register"]
pub mod privcfgr2;
#[doc = "EXTI external interrupt selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr1](exticr1) module"]
pub type EXTICR1 = crate::Reg<u32, _EXTICR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTICR1;
#[doc = "`read()` method returns [exticr1::R](exticr1::R) reader structure"]
impl crate::Readable for EXTICR1 {}
#[doc = "`write(|w| ..)` method takes [exticr1::W](exticr1::W) writer structure"]
impl crate::Writable for EXTICR1 {}
#[doc = "EXTI external interrupt selection register"]
pub mod exticr1;
#[doc = "EXTI external interrupt selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr2](exticr2) module"]
pub type EXTICR2 = crate::Reg<u32, _EXTICR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTICR2;
#[doc = "`read()` method returns [exticr2::R](exticr2::R) reader structure"]
impl crate::Readable for EXTICR2 {}
#[doc = "`write(|w| ..)` method takes [exticr2::W](exticr2::W) writer structure"]
impl crate::Writable for EXTICR2 {}
#[doc = "EXTI external interrupt selection register"]
pub mod exticr2;
#[doc = "EXTI external interrupt selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr3](exticr3) module"]
pub type EXTICR3 = crate::Reg<u32, _EXTICR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTICR3;
#[doc = "`read()` method returns [exticr3::R](exticr3::R) reader structure"]
impl crate::Readable for EXTICR3 {}
#[doc = "`write(|w| ..)` method takes [exticr3::W](exticr3::W) writer structure"]
impl crate::Writable for EXTICR3 {}
#[doc = "EXTI external interrupt selection register"]
pub mod exticr3;
#[doc = "EXTI external interrupt selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr4](exticr4) module"]
pub type EXTICR4 = crate::Reg<u32, _EXTICR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTICR4;
#[doc = "`read()` method returns [exticr4::R](exticr4::R) reader structure"]
impl crate::Readable for EXTICR4 {}
#[doc = "`write(|w| ..)` method takes [exticr4::W](exticr4::W) writer structure"]
impl crate::Writable for EXTICR4 {}
#[doc = "EXTI external interrupt selection register"]
pub mod exticr4;
#[doc = "EXTI lock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lockrg](lockrg) module"]
pub type LOCKRG = crate::Reg<u32, _LOCKRG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOCKRG;
#[doc = "`read()` method returns [lockrg::R](lockrg::R) reader structure"]
impl crate::Readable for LOCKRG {}
#[doc = "`write(|w| ..)` method takes [lockrg::W](lockrg::W) writer structure"]
impl crate::Writable for LOCKRG {}
#[doc = "EXTI lock register"]
pub mod lockrg;
#[doc = "EXTI CPU wakeup with interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr1](imr1) module"]
pub type IMR1 = crate::Reg<u32, _IMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR1;
#[doc = "`read()` method returns [imr1::R](imr1::R) reader structure"]
impl crate::Readable for IMR1 {}
#[doc = "`write(|w| ..)` method takes [imr1::W](imr1::W) writer structure"]
impl crate::Writable for IMR1 {}
#[doc = "EXTI CPU wakeup with interrupt mask register"]
pub mod imr1;
#[doc = "EXTI CPU wakeup with event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emr1](emr1) module"]
pub type EMR1 = crate::Reg<u32, _EMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EMR1;
#[doc = "`read()` method returns [emr1::R](emr1::R) reader structure"]
impl crate::Readable for EMR1 {}
#[doc = "`write(|w| ..)` method takes [emr1::W](emr1::W) writer structure"]
impl crate::Writable for EMR1 {}
#[doc = "EXTI CPU wakeup with event mask register"]
pub mod emr1;
#[doc = "EXTI CPUm wakeup with interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr2](imr2) module"]
pub type IMR2 = crate::Reg<u32, _IMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR2;
#[doc = "`read()` method returns [imr2::R](imr2::R) reader structure"]
impl crate::Readable for IMR2 {}
#[doc = "`write(|w| ..)` method takes [imr2::W](imr2::W) writer structure"]
impl crate::Writable for IMR2 {}
#[doc = "EXTI CPUm wakeup with interrupt mask register"]
pub mod imr2;
#[doc = "EXTI CPU wakeup with event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emr2](emr2) module"]
pub type EMR2 = crate::Reg<u32, _EMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EMR2;
#[doc = "`read()` method returns [emr2::R](emr2::R) reader structure"]
impl crate::Readable for EMR2 {}
#[doc = "`write(|w| ..)` method takes [emr2::W](emr2::W) writer structure"]
impl crate::Writable for EMR2 {}
#[doc = "EXTI CPU wakeup with event mask register"]
pub mod emr2;
