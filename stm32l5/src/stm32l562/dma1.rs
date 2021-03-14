#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - interrupt status register"]
    pub isr: ISR,
    #[doc = "0x04 - interrupt flag clear register"]
    pub ifcr: IFCR,
    #[doc = "0x08 - channel x configuration register"]
    pub ccr1: CCR1,
    #[doc = "0x0c - channel x number of data register"]
    pub cndtr1: CNDTR1,
    #[doc = "0x10 - channel x peripheral address register"]
    pub cpar1: CPAR1,
    #[doc = "0x14 - channel x memory address register"]
    pub cm0ar1: CM0AR1,
    #[doc = "0x18 - channel x memory address register"]
    pub cm1ar1: CM1AR1,
    #[doc = "0x1c - channel x configuration register"]
    pub ccr2: CCR2,
    #[doc = "0x20 - channel x number of data register"]
    pub cndtr2: CNDTR2,
    #[doc = "0x24 - channel x peripheral address register"]
    pub cpar2: CPAR2,
    #[doc = "0x28 - channel x memory address register"]
    pub cm0ar2: CM0AR2,
    #[doc = "0x2c - channel x memory address register"]
    pub cm1ar2: CM1AR2,
    #[doc = "0x30 - channel x configuration register"]
    pub ccr3: CCR3,
    #[doc = "0x34 - channel x number of data register"]
    pub cndtr3: CNDTR3,
    #[doc = "0x38 - channel x peripheral address register"]
    pub cpar3: CPAR3,
    #[doc = "0x3c - channel x memory address register"]
    pub cm0ar3: CM0AR3,
    #[doc = "0x40 - channel x memory address register"]
    pub cm1ar3: CM1AR3,
    #[doc = "0x44 - channel x configuration register"]
    pub ccr4: CCR4,
    #[doc = "0x48 - channel x number of data register"]
    pub cndtr4: CNDTR4,
    #[doc = "0x4c - channel x peripheral address register"]
    pub cpar4: CPAR4,
    #[doc = "0x50 - channel x memory address register"]
    pub cm0ar4: CM0AR4,
    #[doc = "0x54 - channel x memory address register"]
    pub cm1ar4: CM1AR4,
    #[doc = "0x58 - channel x configuration register"]
    pub ccr5: CCR5,
    #[doc = "0x5c - channel x number of data register"]
    pub cndtr5: CNDTR5,
    #[doc = "0x60 - channel x peripheral address register"]
    pub cpar5: CPAR5,
    #[doc = "0x64 - channel x memory address register"]
    pub cm0ar5: CM0AR5,
    #[doc = "0x68 - channel x memory address register"]
    pub cm1ar5: CM1AR5,
    #[doc = "0x6c - channel x configuration register"]
    pub ccr6: CCR6,
    #[doc = "0x70 - channel x number of data register"]
    pub cndtr6: CNDTR6,
    #[doc = "0x74 - channel x peripheral address register"]
    pub cpar6: CPAR6,
    #[doc = "0x78 - channel x memory address register"]
    pub cm0ar6: CM0AR6,
    #[doc = "0x7c - channel x memory address register"]
    pub cm1ar6: CM1AR6,
    #[doc = "0x80 - channel x configuration register"]
    pub ccr7: CCR7,
    #[doc = "0x84 - channel x number of data register"]
    pub cndtr7: CNDTR7,
    #[doc = "0x88 - channel x peripheral address register"]
    pub cpar7: CPAR7,
    #[doc = "0x8c - channel x memory address register"]
    pub cm0ar7: CM0AR7,
    #[doc = "0x90 - channel x memory address register"]
    pub cm1ar7: CM1AR7,
    #[doc = "0x94 - channel x configuration register"]
    pub ccr8: CCR8,
    #[doc = "0x98 - channel x number of data register"]
    pub cndtr8: CNDTR8,
    #[doc = "0x9c - channel x peripheral address register"]
    pub cpar8: CPAR8,
    #[doc = "0xa0 - channel x peripheral address register"]
    pub cm0ar8: CM0AR8,
    #[doc = "0xa4 - channel x peripheral address register"]
    pub cm1ar8: CM1AR8,
    #[doc = "0xa8 - channel selection register"]
    pub cselr: CSELR,
}
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
#[doc = "channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr1](ccr1) module"]
pub type CCR1 = crate::Reg<u32, _CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR1;
#[doc = "`read()` method returns [ccr1::R](ccr1::R) reader structure"]
impl crate::Readable for CCR1 {}
#[doc = "`write(|w| ..)` method takes [ccr1::W](ccr1::W) writer structure"]
impl crate::Writable for CCR1 {}
#[doc = "channel x configuration register"]
pub mod ccr1;
#[doc = "channel x number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndtr1](cndtr1) module"]
pub type CNDTR1 = crate::Reg<u32, _CNDTR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDTR1;
#[doc = "`read()` method returns [cndtr1::R](cndtr1::R) reader structure"]
impl crate::Readable for CNDTR1 {}
#[doc = "`write(|w| ..)` method takes [cndtr1::W](cndtr1::W) writer structure"]
impl crate::Writable for CNDTR1 {}
#[doc = "channel x number of data register"]
pub mod cndtr1;
#[doc = "channel x peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpar1](cpar1) module"]
pub type CPAR1 = crate::Reg<u32, _CPAR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPAR1;
#[doc = "`read()` method returns [cpar1::R](cpar1::R) reader structure"]
impl crate::Readable for CPAR1 {}
#[doc = "`write(|w| ..)` method takes [cpar1::W](cpar1::W) writer structure"]
impl crate::Writable for CPAR1 {}
#[doc = "channel x peripheral address register"]
pub mod cpar1;
#[doc = "channel x memory address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0ar1](cm0ar1) module"]
pub type CM0AR1 = crate::Reg<u32, _CM0AR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0AR1;
#[doc = "`read()` method returns [cm0ar1::R](cm0ar1::R) reader structure"]
impl crate::Readable for CM0AR1 {}
#[doc = "`write(|w| ..)` method takes [cm0ar1::W](cm0ar1::W) writer structure"]
impl crate::Writable for CM0AR1 {}
#[doc = "channel x memory address register"]
pub mod cm0ar1;
#[doc = "channel x memory address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm1ar1](cm1ar1) module"]
pub type CM1AR1 = crate::Reg<u32, _CM1AR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM1AR1;
#[doc = "`read()` method returns [cm1ar1::R](cm1ar1::R) reader structure"]
impl crate::Readable for CM1AR1 {}
#[doc = "`write(|w| ..)` method takes [cm1ar1::W](cm1ar1::W) writer structure"]
impl crate::Writable for CM1AR1 {}
#[doc = "channel x memory address register"]
pub mod cm1ar1;
#[doc = "channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr2](ccr2) module"]
pub type CCR2 = crate::Reg<u32, _CCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR2;
#[doc = "`read()` method returns [ccr2::R](ccr2::R) reader structure"]
impl crate::Readable for CCR2 {}
#[doc = "`write(|w| ..)` method takes [ccr2::W](ccr2::W) writer structure"]
impl crate::Writable for CCR2 {}
#[doc = "channel x configuration register"]
pub mod ccr2;
#[doc = "channel x number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndtr2](cndtr2) module"]
pub type CNDTR2 = crate::Reg<u32, _CNDTR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDTR2;
#[doc = "`read()` method returns [cndtr2::R](cndtr2::R) reader structure"]
impl crate::Readable for CNDTR2 {}
#[doc = "`write(|w| ..)` method takes [cndtr2::W](cndtr2::W) writer structure"]
impl crate::Writable for CNDTR2 {}
#[doc = "channel x number of data register"]
pub mod cndtr2;
#[doc = "channel x peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpar2](cpar2) module"]
pub type CPAR2 = crate::Reg<u32, _CPAR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPAR2;
#[doc = "`read()` method returns [cpar2::R](cpar2::R) reader structure"]
impl crate::Readable for CPAR2 {}
#[doc = "`write(|w| ..)` method takes [cpar2::W](cpar2::W) writer structure"]
impl crate::Writable for CPAR2 {}
#[doc = "channel x peripheral address register"]
pub mod cpar2;
#[doc = "channel x memory address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0ar2](cm0ar2) module"]
pub type CM0AR2 = crate::Reg<u32, _CM0AR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0AR2;
#[doc = "`read()` method returns [cm0ar2::R](cm0ar2::R) reader structure"]
impl crate::Readable for CM0AR2 {}
#[doc = "`write(|w| ..)` method takes [cm0ar2::W](cm0ar2::W) writer structure"]
impl crate::Writable for CM0AR2 {}
#[doc = "channel x memory address register"]
pub mod cm0ar2;
#[doc = "channel x memory address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm1ar2](cm1ar2) module"]
pub type CM1AR2 = crate::Reg<u32, _CM1AR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM1AR2;
#[doc = "`read()` method returns [cm1ar2::R](cm1ar2::R) reader structure"]
impl crate::Readable for CM1AR2 {}
#[doc = "`write(|w| ..)` method takes [cm1ar2::W](cm1ar2::W) writer structure"]
impl crate::Writable for CM1AR2 {}
#[doc = "channel x memory address register"]
pub mod cm1ar2;
#[doc = "channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr3](ccr3) module"]
pub type CCR3 = crate::Reg<u32, _CCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR3;
#[doc = "`read()` method returns [ccr3::R](ccr3::R) reader structure"]
impl crate::Readable for CCR3 {}
#[doc = "`write(|w| ..)` method takes [ccr3::W](ccr3::W) writer structure"]
impl crate::Writable for CCR3 {}
#[doc = "channel x configuration register"]
pub mod ccr3;
#[doc = "channel x number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndtr3](cndtr3) module"]
pub type CNDTR3 = crate::Reg<u32, _CNDTR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDTR3;
#[doc = "`read()` method returns [cndtr3::R](cndtr3::R) reader structure"]
impl crate::Readable for CNDTR3 {}
#[doc = "`write(|w| ..)` method takes [cndtr3::W](cndtr3::W) writer structure"]
impl crate::Writable for CNDTR3 {}
#[doc = "channel x number of data register"]
pub mod cndtr3;
#[doc = "channel x peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpar3](cpar3) module"]
pub type CPAR3 = crate::Reg<u32, _CPAR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPAR3;
#[doc = "`read()` method returns [cpar3::R](cpar3::R) reader structure"]
impl crate::Readable for CPAR3 {}
#[doc = "`write(|w| ..)` method takes [cpar3::W](cpar3::W) writer structure"]
impl crate::Writable for CPAR3 {}
#[doc = "channel x peripheral address register"]
pub mod cpar3;
#[doc = "channel x memory address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0ar3](cm0ar3) module"]
pub type CM0AR3 = crate::Reg<u32, _CM0AR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0AR3;
#[doc = "`read()` method returns [cm0ar3::R](cm0ar3::R) reader structure"]
impl crate::Readable for CM0AR3 {}
#[doc = "`write(|w| ..)` method takes [cm0ar3::W](cm0ar3::W) writer structure"]
impl crate::Writable for CM0AR3 {}
#[doc = "channel x memory address register"]
pub mod cm0ar3;
#[doc = "channel x memory address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm1ar3](cm1ar3) module"]
pub type CM1AR3 = crate::Reg<u32, _CM1AR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM1AR3;
#[doc = "`read()` method returns [cm1ar3::R](cm1ar3::R) reader structure"]
impl crate::Readable for CM1AR3 {}
#[doc = "`write(|w| ..)` method takes [cm1ar3::W](cm1ar3::W) writer structure"]
impl crate::Writable for CM1AR3 {}
#[doc = "channel x memory address register"]
pub mod cm1ar3;
#[doc = "channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr4](ccr4) module"]
pub type CCR4 = crate::Reg<u32, _CCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR4;
#[doc = "`read()` method returns [ccr4::R](ccr4::R) reader structure"]
impl crate::Readable for CCR4 {}
#[doc = "`write(|w| ..)` method takes [ccr4::W](ccr4::W) writer structure"]
impl crate::Writable for CCR4 {}
#[doc = "channel x configuration register"]
pub mod ccr4;
#[doc = "channel x number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndtr4](cndtr4) module"]
pub type CNDTR4 = crate::Reg<u32, _CNDTR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDTR4;
#[doc = "`read()` method returns [cndtr4::R](cndtr4::R) reader structure"]
impl crate::Readable for CNDTR4 {}
#[doc = "`write(|w| ..)` method takes [cndtr4::W](cndtr4::W) writer structure"]
impl crate::Writable for CNDTR4 {}
#[doc = "channel x number of data register"]
pub mod cndtr4;
#[doc = "channel x peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpar4](cpar4) module"]
pub type CPAR4 = crate::Reg<u32, _CPAR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPAR4;
#[doc = "`read()` method returns [cpar4::R](cpar4::R) reader structure"]
impl crate::Readable for CPAR4 {}
#[doc = "`write(|w| ..)` method takes [cpar4::W](cpar4::W) writer structure"]
impl crate::Writable for CPAR4 {}
#[doc = "channel x peripheral address register"]
pub mod cpar4;
#[doc = "channel x memory address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0ar4](cm0ar4) module"]
pub type CM0AR4 = crate::Reg<u32, _CM0AR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0AR4;
#[doc = "`read()` method returns [cm0ar4::R](cm0ar4::R) reader structure"]
impl crate::Readable for CM0AR4 {}
#[doc = "`write(|w| ..)` method takes [cm0ar4::W](cm0ar4::W) writer structure"]
impl crate::Writable for CM0AR4 {}
#[doc = "channel x memory address register"]
pub mod cm0ar4;
#[doc = "channel x memory address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm1ar4](cm1ar4) module"]
pub type CM1AR4 = crate::Reg<u32, _CM1AR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM1AR4;
#[doc = "`read()` method returns [cm1ar4::R](cm1ar4::R) reader structure"]
impl crate::Readable for CM1AR4 {}
#[doc = "`write(|w| ..)` method takes [cm1ar4::W](cm1ar4::W) writer structure"]
impl crate::Writable for CM1AR4 {}
#[doc = "channel x memory address register"]
pub mod cm1ar4;
#[doc = "channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr5](ccr5) module"]
pub type CCR5 = crate::Reg<u32, _CCR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR5;
#[doc = "`read()` method returns [ccr5::R](ccr5::R) reader structure"]
impl crate::Readable for CCR5 {}
#[doc = "`write(|w| ..)` method takes [ccr5::W](ccr5::W) writer structure"]
impl crate::Writable for CCR5 {}
#[doc = "channel x configuration register"]
pub mod ccr5;
#[doc = "channel x number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndtr5](cndtr5) module"]
pub type CNDTR5 = crate::Reg<u32, _CNDTR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDTR5;
#[doc = "`read()` method returns [cndtr5::R](cndtr5::R) reader structure"]
impl crate::Readable for CNDTR5 {}
#[doc = "`write(|w| ..)` method takes [cndtr5::W](cndtr5::W) writer structure"]
impl crate::Writable for CNDTR5 {}
#[doc = "channel x number of data register"]
pub mod cndtr5;
#[doc = "channel x peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpar5](cpar5) module"]
pub type CPAR5 = crate::Reg<u32, _CPAR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPAR5;
#[doc = "`read()` method returns [cpar5::R](cpar5::R) reader structure"]
impl crate::Readable for CPAR5 {}
#[doc = "`write(|w| ..)` method takes [cpar5::W](cpar5::W) writer structure"]
impl crate::Writable for CPAR5 {}
#[doc = "channel x peripheral address register"]
pub mod cpar5;
#[doc = "channel x memory address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0ar5](cm0ar5) module"]
pub type CM0AR5 = crate::Reg<u32, _CM0AR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0AR5;
#[doc = "`read()` method returns [cm0ar5::R](cm0ar5::R) reader structure"]
impl crate::Readable for CM0AR5 {}
#[doc = "`write(|w| ..)` method takes [cm0ar5::W](cm0ar5::W) writer structure"]
impl crate::Writable for CM0AR5 {}
#[doc = "channel x memory address register"]
pub mod cm0ar5;
#[doc = "channel x memory address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm1ar5](cm1ar5) module"]
pub type CM1AR5 = crate::Reg<u32, _CM1AR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM1AR5;
#[doc = "`read()` method returns [cm1ar5::R](cm1ar5::R) reader structure"]
impl crate::Readable for CM1AR5 {}
#[doc = "`write(|w| ..)` method takes [cm1ar5::W](cm1ar5::W) writer structure"]
impl crate::Writable for CM1AR5 {}
#[doc = "channel x memory address register"]
pub mod cm1ar5;
#[doc = "channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr6](ccr6) module"]
pub type CCR6 = crate::Reg<u32, _CCR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR6;
#[doc = "`read()` method returns [ccr6::R](ccr6::R) reader structure"]
impl crate::Readable for CCR6 {}
#[doc = "`write(|w| ..)` method takes [ccr6::W](ccr6::W) writer structure"]
impl crate::Writable for CCR6 {}
#[doc = "channel x configuration register"]
pub mod ccr6;
#[doc = "channel x number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndtr6](cndtr6) module"]
pub type CNDTR6 = crate::Reg<u32, _CNDTR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDTR6;
#[doc = "`read()` method returns [cndtr6::R](cndtr6::R) reader structure"]
impl crate::Readable for CNDTR6 {}
#[doc = "`write(|w| ..)` method takes [cndtr6::W](cndtr6::W) writer structure"]
impl crate::Writable for CNDTR6 {}
#[doc = "channel x number of data register"]
pub mod cndtr6;
#[doc = "channel x peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpar6](cpar6) module"]
pub type CPAR6 = crate::Reg<u32, _CPAR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPAR6;
#[doc = "`read()` method returns [cpar6::R](cpar6::R) reader structure"]
impl crate::Readable for CPAR6 {}
#[doc = "`write(|w| ..)` method takes [cpar6::W](cpar6::W) writer structure"]
impl crate::Writable for CPAR6 {}
#[doc = "channel x peripheral address register"]
pub mod cpar6;
#[doc = "channel x memory address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0ar6](cm0ar6) module"]
pub type CM0AR6 = crate::Reg<u32, _CM0AR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0AR6;
#[doc = "`read()` method returns [cm0ar6::R](cm0ar6::R) reader structure"]
impl crate::Readable for CM0AR6 {}
#[doc = "`write(|w| ..)` method takes [cm0ar6::W](cm0ar6::W) writer structure"]
impl crate::Writable for CM0AR6 {}
#[doc = "channel x memory address register"]
pub mod cm0ar6;
#[doc = "channel x memory address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm1ar6](cm1ar6) module"]
pub type CM1AR6 = crate::Reg<u32, _CM1AR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM1AR6;
#[doc = "`read()` method returns [cm1ar6::R](cm1ar6::R) reader structure"]
impl crate::Readable for CM1AR6 {}
#[doc = "`write(|w| ..)` method takes [cm1ar6::W](cm1ar6::W) writer structure"]
impl crate::Writable for CM1AR6 {}
#[doc = "channel x memory address register"]
pub mod cm1ar6;
#[doc = "channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr7](ccr7) module"]
pub type CCR7 = crate::Reg<u32, _CCR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR7;
#[doc = "`read()` method returns [ccr7::R](ccr7::R) reader structure"]
impl crate::Readable for CCR7 {}
#[doc = "`write(|w| ..)` method takes [ccr7::W](ccr7::W) writer structure"]
impl crate::Writable for CCR7 {}
#[doc = "channel x configuration register"]
pub mod ccr7;
#[doc = "channel x number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndtr7](cndtr7) module"]
pub type CNDTR7 = crate::Reg<u32, _CNDTR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDTR7;
#[doc = "`read()` method returns [cndtr7::R](cndtr7::R) reader structure"]
impl crate::Readable for CNDTR7 {}
#[doc = "`write(|w| ..)` method takes [cndtr7::W](cndtr7::W) writer structure"]
impl crate::Writable for CNDTR7 {}
#[doc = "channel x number of data register"]
pub mod cndtr7;
#[doc = "channel x peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpar7](cpar7) module"]
pub type CPAR7 = crate::Reg<u32, _CPAR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPAR7;
#[doc = "`read()` method returns [cpar7::R](cpar7::R) reader structure"]
impl crate::Readable for CPAR7 {}
#[doc = "`write(|w| ..)` method takes [cpar7::W](cpar7::W) writer structure"]
impl crate::Writable for CPAR7 {}
#[doc = "channel x peripheral address register"]
pub mod cpar7;
#[doc = "channel x memory address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0ar7](cm0ar7) module"]
pub type CM0AR7 = crate::Reg<u32, _CM0AR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0AR7;
#[doc = "`read()` method returns [cm0ar7::R](cm0ar7::R) reader structure"]
impl crate::Readable for CM0AR7 {}
#[doc = "`write(|w| ..)` method takes [cm0ar7::W](cm0ar7::W) writer structure"]
impl crate::Writable for CM0AR7 {}
#[doc = "channel x memory address register"]
pub mod cm0ar7;
#[doc = "channel x memory address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm1ar7](cm1ar7) module"]
pub type CM1AR7 = crate::Reg<u32, _CM1AR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM1AR7;
#[doc = "`read()` method returns [cm1ar7::R](cm1ar7::R) reader structure"]
impl crate::Readable for CM1AR7 {}
#[doc = "`write(|w| ..)` method takes [cm1ar7::W](cm1ar7::W) writer structure"]
impl crate::Writable for CM1AR7 {}
#[doc = "channel x memory address register"]
pub mod cm1ar7;
#[doc = "channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr8](ccr8) module"]
pub type CCR8 = crate::Reg<u32, _CCR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR8;
#[doc = "`read()` method returns [ccr8::R](ccr8::R) reader structure"]
impl crate::Readable for CCR8 {}
#[doc = "`write(|w| ..)` method takes [ccr8::W](ccr8::W) writer structure"]
impl crate::Writable for CCR8 {}
#[doc = "channel x configuration register"]
pub mod ccr8;
#[doc = "channel x number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndtr8](cndtr8) module"]
pub type CNDTR8 = crate::Reg<u32, _CNDTR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDTR8;
#[doc = "`read()` method returns [cndtr8::R](cndtr8::R) reader structure"]
impl crate::Readable for CNDTR8 {}
#[doc = "`write(|w| ..)` method takes [cndtr8::W](cndtr8::W) writer structure"]
impl crate::Writable for CNDTR8 {}
#[doc = "channel x number of data register"]
pub mod cndtr8;
#[doc = "channel x peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpar8](cpar8) module"]
pub type CPAR8 = crate::Reg<u32, _CPAR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPAR8;
#[doc = "`read()` method returns [cpar8::R](cpar8::R) reader structure"]
impl crate::Readable for CPAR8 {}
#[doc = "`write(|w| ..)` method takes [cpar8::W](cpar8::W) writer structure"]
impl crate::Writable for CPAR8 {}
#[doc = "channel x peripheral address register"]
pub mod cpar8;
#[doc = "channel x peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0ar8](cm0ar8) module"]
pub type CM0AR8 = crate::Reg<u32, _CM0AR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0AR8;
#[doc = "`read()` method returns [cm0ar8::R](cm0ar8::R) reader structure"]
impl crate::Readable for CM0AR8 {}
#[doc = "`write(|w| ..)` method takes [cm0ar8::W](cm0ar8::W) writer structure"]
impl crate::Writable for CM0AR8 {}
#[doc = "channel x peripheral address register"]
pub mod cm0ar8;
#[doc = "channel x peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm1ar8](cm1ar8) module"]
pub type CM1AR8 = crate::Reg<u32, _CM1AR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM1AR8;
#[doc = "`read()` method returns [cm1ar8::R](cm1ar8::R) reader structure"]
impl crate::Readable for CM1AR8 {}
#[doc = "`write(|w| ..)` method takes [cm1ar8::W](cm1ar8::W) writer structure"]
impl crate::Writable for CM1AR8 {}
#[doc = "channel x peripheral address register"]
pub mod cm1ar8;
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
