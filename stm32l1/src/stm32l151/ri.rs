#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - RI input capture register"]
    pub icr: ICR,
    #[doc = "0x08 - RI analog switches control register 1"]
    pub ascr1: ASCR1,
    #[doc = "0x0c - RI analog switches control register 2"]
    pub ascr2: ASCR2,
    #[doc = "0x10 - RI hysteresis control register 1"]
    pub hyscr1: HYSCR1,
    #[doc = "0x14 - RI hysteresis control register 2"]
    pub hyscr2: HYSCR2,
    #[doc = "0x18 - RI hysteresis control register 3"]
    pub hyscr3: HYSCR3,
    #[doc = "0x1c - Hysteresis control register"]
    pub hyscr4: HYSCR4,
}
#[doc = "RI input capture register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](icr) module"]
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
#[doc = "`read()` method returns [icr::R](icr::R) reader structure"]
impl crate::Readable for ICR {}
#[doc = "`write(|w| ..)` method takes [icr::W](icr::W) writer structure"]
impl crate::Writable for ICR {}
#[doc = "RI input capture register"]
pub mod icr;
#[doc = "RI analog switches control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ascr1](ascr1) module"]
pub type ASCR1 = crate::Reg<u32, _ASCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASCR1;
#[doc = "`read()` method returns [ascr1::R](ascr1::R) reader structure"]
impl crate::Readable for ASCR1 {}
#[doc = "`write(|w| ..)` method takes [ascr1::W](ascr1::W) writer structure"]
impl crate::Writable for ASCR1 {}
#[doc = "RI analog switches control register 1"]
pub mod ascr1;
#[doc = "RI analog switches control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ascr2](ascr2) module"]
pub type ASCR2 = crate::Reg<u32, _ASCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASCR2;
#[doc = "`read()` method returns [ascr2::R](ascr2::R) reader structure"]
impl crate::Readable for ASCR2 {}
#[doc = "`write(|w| ..)` method takes [ascr2::W](ascr2::W) writer structure"]
impl crate::Writable for ASCR2 {}
#[doc = "RI analog switches control register 2"]
pub mod ascr2;
#[doc = "RI hysteresis control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hyscr1](hyscr1) module"]
pub type HYSCR1 = crate::Reg<u32, _HYSCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HYSCR1;
#[doc = "`read()` method returns [hyscr1::R](hyscr1::R) reader structure"]
impl crate::Readable for HYSCR1 {}
#[doc = "`write(|w| ..)` method takes [hyscr1::W](hyscr1::W) writer structure"]
impl crate::Writable for HYSCR1 {}
#[doc = "RI hysteresis control register 1"]
pub mod hyscr1;
#[doc = "RI hysteresis control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hyscr2](hyscr2) module"]
pub type HYSCR2 = crate::Reg<u32, _HYSCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HYSCR2;
#[doc = "`read()` method returns [hyscr2::R](hyscr2::R) reader structure"]
impl crate::Readable for HYSCR2 {}
#[doc = "`write(|w| ..)` method takes [hyscr2::W](hyscr2::W) writer structure"]
impl crate::Writable for HYSCR2 {}
#[doc = "RI hysteresis control register 2"]
pub mod hyscr2;
#[doc = "RI hysteresis control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hyscr3](hyscr3) module"]
pub type HYSCR3 = crate::Reg<u32, _HYSCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HYSCR3;
#[doc = "`read()` method returns [hyscr3::R](hyscr3::R) reader structure"]
impl crate::Readable for HYSCR3 {}
#[doc = "`write(|w| ..)` method takes [hyscr3::W](hyscr3::W) writer structure"]
impl crate::Writable for HYSCR3 {}
#[doc = "RI hysteresis control register 3"]
pub mod hyscr3;
#[doc = "Hysteresis control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hyscr4](hyscr4) module"]
pub type HYSCR4 = crate::Reg<u32, _HYSCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HYSCR4;
#[doc = "`read()` method returns [hyscr4::R](hyscr4::R) reader structure"]
impl crate::Readable for HYSCR4 {}
#[doc = "`write(|w| ..)` method takes [hyscr4::W](hyscr4::W) writer structure"]
impl crate::Writable for HYSCR4 {}
#[doc = "Hysteresis control register"]
pub mod hyscr4;
