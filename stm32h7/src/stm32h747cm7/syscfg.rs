#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - peripheral mode configuration register"]
    pub pmcr: PMCR,
    #[doc = "0x08 - external interrupt configuration register 1"]
    pub exticr1: EXTICR1,
    #[doc = "0x0c - external interrupt configuration register 2"]
    pub exticr2: EXTICR2,
    #[doc = "0x10 - external interrupt configuration register 3"]
    pub exticr3: EXTICR3,
    #[doc = "0x14 - external interrupt configuration register 4"]
    pub exticr4: EXTICR4,
    _reserved5: [u8; 8usize],
    #[doc = "0x20 - compensation cell control/status register"]
    pub cccsr: CCCSR,
    #[doc = "0x24 - SYSCFG compensation cell value register"]
    pub ccvr: CCVR,
    #[doc = "0x28 - SYSCFG compensation cell code register"]
    pub cccr: CCCR,
    #[doc = "0x2c - SYSCFG Power Control Register"]
    pub pwrcr: PWRCR,
    _reserved9: [u8; 244usize],
    #[doc = "0x124 - SYSCFG package register"]
    pub pkgr: PKGR,
    _reserved10: [u8; 472usize],
    #[doc = "0x300 - SYSCFG user register 0"]
    pub ur0: UR0,
    _reserved11: [u8; 4usize],
    #[doc = "0x308 - SYSCFG user register 2"]
    pub ur2: UR2,
    #[doc = "0x30c - SYSCFG user register 3"]
    pub ur3: UR3,
    #[doc = "0x310 - SYSCFG user register 4"]
    pub ur4: UR4,
    #[doc = "0x314 - SYSCFG user register 5"]
    pub ur5: UR5,
    #[doc = "0x318 - SYSCFG user register 6"]
    pub ur6: UR6,
    #[doc = "0x31c - SYSCFG user register 7"]
    pub ur7: UR7,
    #[doc = "0x320 - SYSCFG user register 8"]
    pub ur8: UR8,
    #[doc = "0x324 - SYSCFG user register 9"]
    pub ur9: UR9,
    #[doc = "0x328 - SYSCFG user register 10"]
    pub ur10: UR10,
    #[doc = "0x32c - SYSCFG user register 11"]
    pub ur11: UR11,
    #[doc = "0x330 - SYSCFG user register 12"]
    pub ur12: UR12,
    #[doc = "0x334 - SYSCFG user register 13"]
    pub ur13: UR13,
    #[doc = "0x338 - SYSCFG user register 14"]
    pub ur14: UR14,
    #[doc = "0x33c - SYSCFG user register 15"]
    pub ur15: UR15,
    #[doc = "0x340 - SYSCFG user register 16"]
    pub ur16: UR16,
    #[doc = "0x344 - SYSCFG user register 17"]
    pub ur17: UR17,
}
#[doc = "peripheral mode configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmcr](pmcr) module"]
pub type PMCR = crate::Reg<u32, _PMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMCR;
#[doc = "`read()` method returns [pmcr::R](pmcr::R) reader structure"]
impl crate::Readable for PMCR {}
#[doc = "`write(|w| ..)` method takes [pmcr::W](pmcr::W) writer structure"]
impl crate::Writable for PMCR {}
#[doc = "peripheral mode configuration register"]
pub mod pmcr;
#[doc = "external interrupt configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr1](exticr1) module"]
pub type EXTICR1 = crate::Reg<u32, _EXTICR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTICR1;
#[doc = "`read()` method returns [exticr1::R](exticr1::R) reader structure"]
impl crate::Readable for EXTICR1 {}
#[doc = "`write(|w| ..)` method takes [exticr1::W](exticr1::W) writer structure"]
impl crate::Writable for EXTICR1 {}
#[doc = "external interrupt configuration register 1"]
pub mod exticr1;
#[doc = "external interrupt configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr2](exticr2) module"]
pub type EXTICR2 = crate::Reg<u32, _EXTICR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTICR2;
#[doc = "`read()` method returns [exticr2::R](exticr2::R) reader structure"]
impl crate::Readable for EXTICR2 {}
#[doc = "`write(|w| ..)` method takes [exticr2::W](exticr2::W) writer structure"]
impl crate::Writable for EXTICR2 {}
#[doc = "external interrupt configuration register 2"]
pub mod exticr2;
#[doc = "external interrupt configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr3](exticr3) module"]
pub type EXTICR3 = crate::Reg<u32, _EXTICR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTICR3;
#[doc = "`read()` method returns [exticr3::R](exticr3::R) reader structure"]
impl crate::Readable for EXTICR3 {}
#[doc = "`write(|w| ..)` method takes [exticr3::W](exticr3::W) writer structure"]
impl crate::Writable for EXTICR3 {}
#[doc = "external interrupt configuration register 3"]
pub mod exticr3;
#[doc = "external interrupt configuration register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr4](exticr4) module"]
pub type EXTICR4 = crate::Reg<u32, _EXTICR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTICR4;
#[doc = "`read()` method returns [exticr4::R](exticr4::R) reader structure"]
impl crate::Readable for EXTICR4 {}
#[doc = "`write(|w| ..)` method takes [exticr4::W](exticr4::W) writer structure"]
impl crate::Writable for EXTICR4 {}
#[doc = "external interrupt configuration register 4"]
pub mod exticr4;
#[doc = "compensation cell control/status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cccsr](cccsr) module"]
pub type CCCSR = crate::Reg<u32, _CCCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCCSR;
#[doc = "`read()` method returns [cccsr::R](cccsr::R) reader structure"]
impl crate::Readable for CCCSR {}
#[doc = "`write(|w| ..)` method takes [cccsr::W](cccsr::W) writer structure"]
impl crate::Writable for CCCSR {}
#[doc = "compensation cell control/status register"]
pub mod cccsr;
#[doc = "SYSCFG compensation cell value register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccvr](ccvr) module"]
pub type CCVR = crate::Reg<u32, _CCVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCVR;
#[doc = "`read()` method returns [ccvr::R](ccvr::R) reader structure"]
impl crate::Readable for CCVR {}
#[doc = "SYSCFG compensation cell value register"]
pub mod ccvr;
#[doc = "SYSCFG compensation cell code register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cccr](cccr) module"]
pub type CCCR = crate::Reg<u32, _CCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCCR;
#[doc = "`read()` method returns [cccr::R](cccr::R) reader structure"]
impl crate::Readable for CCCR {}
#[doc = "`write(|w| ..)` method takes [cccr::W](cccr::W) writer structure"]
impl crate::Writable for CCCR {}
#[doc = "SYSCFG compensation cell code register"]
pub mod cccr;
#[doc = "SYSCFG package register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkgr](pkgr) module"]
pub type PKGR = crate::Reg<u32, _PKGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PKGR;
#[doc = "`read()` method returns [pkgr::R](pkgr::R) reader structure"]
impl crate::Readable for PKGR {}
#[doc = "SYSCFG package register"]
pub mod pkgr;
#[doc = "SYSCFG user register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur0](ur0) module"]
pub type UR0 = crate::Reg<u32, _UR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UR0;
#[doc = "`read()` method returns [ur0::R](ur0::R) reader structure"]
impl crate::Readable for UR0 {}
#[doc = "SYSCFG user register 0"]
pub mod ur0;
#[doc = "SYSCFG user register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur2](ur2) module"]
pub type UR2 = crate::Reg<u32, _UR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UR2;
#[doc = "`read()` method returns [ur2::R](ur2::R) reader structure"]
impl crate::Readable for UR2 {}
#[doc = "`write(|w| ..)` method takes [ur2::W](ur2::W) writer structure"]
impl crate::Writable for UR2 {}
#[doc = "SYSCFG user register 2"]
pub mod ur2;
#[doc = "SYSCFG user register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur3](ur3) module"]
pub type UR3 = crate::Reg<u32, _UR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UR3;
#[doc = "`read()` method returns [ur3::R](ur3::R) reader structure"]
impl crate::Readable for UR3 {}
#[doc = "`write(|w| ..)` method takes [ur3::W](ur3::W) writer structure"]
impl crate::Writable for UR3 {}
#[doc = "SYSCFG user register 3"]
pub mod ur3;
#[doc = "SYSCFG user register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur4](ur4) module"]
pub type UR4 = crate::Reg<u32, _UR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UR4;
#[doc = "`read()` method returns [ur4::R](ur4::R) reader structure"]
impl crate::Readable for UR4 {}
#[doc = "SYSCFG user register 4"]
pub mod ur4;
#[doc = "SYSCFG user register 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur5](ur5) module"]
pub type UR5 = crate::Reg<u32, _UR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UR5;
#[doc = "`read()` method returns [ur5::R](ur5::R) reader structure"]
impl crate::Readable for UR5 {}
#[doc = "SYSCFG user register 5"]
pub mod ur5;
#[doc = "SYSCFG user register 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur6](ur6) module"]
pub type UR6 = crate::Reg<u32, _UR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UR6;
#[doc = "`read()` method returns [ur6::R](ur6::R) reader structure"]
impl crate::Readable for UR6 {}
#[doc = "SYSCFG user register 6"]
pub mod ur6;
#[doc = "SYSCFG user register 7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur7](ur7) module"]
pub type UR7 = crate::Reg<u32, _UR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UR7;
#[doc = "`read()` method returns [ur7::R](ur7::R) reader structure"]
impl crate::Readable for UR7 {}
#[doc = "SYSCFG user register 7"]
pub mod ur7;
#[doc = "SYSCFG user register 8\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur8](ur8) module"]
pub type UR8 = crate::Reg<u32, _UR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UR8;
#[doc = "`read()` method returns [ur8::R](ur8::R) reader structure"]
impl crate::Readable for UR8 {}
#[doc = "SYSCFG user register 8"]
pub mod ur8;
#[doc = "SYSCFG user register 9\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur9](ur9) module"]
pub type UR9 = crate::Reg<u32, _UR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UR9;
#[doc = "`read()` method returns [ur9::R](ur9::R) reader structure"]
impl crate::Readable for UR9 {}
#[doc = "SYSCFG user register 9"]
pub mod ur9;
#[doc = "SYSCFG user register 10\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur10](ur10) module"]
pub type UR10 = crate::Reg<u32, _UR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UR10;
#[doc = "`read()` method returns [ur10::R](ur10::R) reader structure"]
impl crate::Readable for UR10 {}
#[doc = "SYSCFG user register 10"]
pub mod ur10;
#[doc = "SYSCFG user register 11\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur11](ur11) module"]
pub type UR11 = crate::Reg<u32, _UR11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UR11;
#[doc = "`read()` method returns [ur11::R](ur11::R) reader structure"]
impl crate::Readable for UR11 {}
#[doc = "SYSCFG user register 11"]
pub mod ur11;
#[doc = "SYSCFG user register 12\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur12](ur12) module"]
pub type UR12 = crate::Reg<u32, _UR12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UR12;
#[doc = "`read()` method returns [ur12::R](ur12::R) reader structure"]
impl crate::Readable for UR12 {}
#[doc = "SYSCFG user register 12"]
pub mod ur12;
#[doc = "SYSCFG user register 13\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur13](ur13) module"]
pub type UR13 = crate::Reg<u32, _UR13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UR13;
#[doc = "`read()` method returns [ur13::R](ur13::R) reader structure"]
impl crate::Readable for UR13 {}
#[doc = "SYSCFG user register 13"]
pub mod ur13;
#[doc = "SYSCFG user register 14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur14](ur14) module"]
pub type UR14 = crate::Reg<u32, _UR14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UR14;
#[doc = "`read()` method returns [ur14::R](ur14::R) reader structure"]
impl crate::Readable for UR14 {}
#[doc = "`write(|w| ..)` method takes [ur14::W](ur14::W) writer structure"]
impl crate::Writable for UR14 {}
#[doc = "SYSCFG user register 14"]
pub mod ur14;
#[doc = "SYSCFG user register 15\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur15](ur15) module"]
pub type UR15 = crate::Reg<u32, _UR15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UR15;
#[doc = "`read()` method returns [ur15::R](ur15::R) reader structure"]
impl crate::Readable for UR15 {}
#[doc = "SYSCFG user register 15"]
pub mod ur15;
#[doc = "SYSCFG user register 16\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur16](ur16) module"]
pub type UR16 = crate::Reg<u32, _UR16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UR16;
#[doc = "`read()` method returns [ur16::R](ur16::R) reader structure"]
impl crate::Readable for UR16 {}
#[doc = "SYSCFG user register 16"]
pub mod ur16;
#[doc = "SYSCFG user register 17\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur17](ur17) module"]
pub type UR17 = crate::Reg<u32, _UR17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UR17;
#[doc = "`read()` method returns [ur17::R](ur17::R) reader structure"]
impl crate::Readable for UR17 {}
#[doc = "SYSCFG user register 17"]
pub mod ur17;
#[doc = "SYSCFG Power Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrcr](pwrcr) module"]
pub type PWRCR = crate::Reg<u32, _PWRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRCR;
#[doc = "`read()` method returns [pwrcr::R](pwrcr::R) reader structure"]
impl crate::Readable for PWRCR {}
#[doc = "`write(|w| ..)` method takes [pwrcr::W](pwrcr::W) writer structure"]
impl crate::Writable for PWRCR {}
#[doc = "SYSCFG Power Control Register"]
pub mod pwrcr;
