#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock control register"]
    pub cr: CR,
    #[doc = "0x04 - Internal clock sources calibration register"]
    pub icscr: ICSCR,
    #[doc = "0x08 - Clock configuration register"]
    pub cfgr: CFGR,
    #[doc = "0x0c - PLL configuration register"]
    pub pllsyscfgr: PLLSYSCFGR,
    _reserved4: [u8; 8usize],
    #[doc = "0x18 - Clock interrupt enable register"]
    pub cier: CIER,
    #[doc = "0x1c - Clock interrupt flag register"]
    pub cifr: CIFR,
    #[doc = "0x20 - Clock interrupt clear register"]
    pub cicr: CICR,
    #[doc = "0x24 - GPIO reset register"]
    pub ioprstr: IOPRSTR,
    #[doc = "0x28 - AHB peripheral reset register"]
    pub ahbrstr: AHBRSTR,
    #[doc = "0x2c - APB peripheral reset register 1"]
    pub apbrstr1: APBRSTR1,
    #[doc = "0x30 - APB peripheral reset register 2"]
    pub apbrstr2: APBRSTR2,
    #[doc = "0x34 - GPIO clock enable register"]
    pub iopenr: IOPENR,
    #[doc = "0x38 - AHB peripheral clock enable register"]
    pub ahbenr: AHBENR,
    #[doc = "0x3c - APB peripheral clock enable register 1"]
    pub apbenr1: APBENR1,
    #[doc = "0x40 - APB peripheral clock enable register 2"]
    pub apbenr2: APBENR2,
    #[doc = "0x44 - GPIO in Sleep mode clock enable register"]
    pub iopsmenr: IOPSMENR,
    #[doc = "0x48 - AHB peripheral clock enable in Sleep mode register"]
    pub ahbsmenr: AHBSMENR,
    #[doc = "0x4c - APB peripheral clock enable in Sleep mode register 1"]
    pub apbsmenr1: APBSMENR1,
    #[doc = "0x50 - APB peripheral clock enable in Sleep mode register 2"]
    pub apbsmenr2: APBSMENR2,
    #[doc = "0x54 - Peripherals independent clock configuration register"]
    pub ccipr: CCIPR,
    _reserved20: [u8; 4usize],
    #[doc = "0x5c - RTC domain control register"]
    pub bdcr: BDCR,
    #[doc = "0x60 - Control/status register"]
    pub csr: CSR,
}
#[doc = "Clock control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "Clock control register"]
pub mod cr;
#[doc = "Internal clock sources calibration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icscr](icscr) module"]
pub type ICSCR = crate::Reg<u32, _ICSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICSCR;
#[doc = "`read()` method returns [icscr::R](icscr::R) reader structure"]
impl crate::Readable for ICSCR {}
#[doc = "`write(|w| ..)` method takes [icscr::W](icscr::W) writer structure"]
impl crate::Writable for ICSCR {}
#[doc = "Internal clock sources calibration register"]
pub mod icscr;
#[doc = "Clock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr](cfgr) module"]
pub type CFGR = crate::Reg<u32, _CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR;
#[doc = "`read()` method returns [cfgr::R](cfgr::R) reader structure"]
impl crate::Readable for CFGR {}
#[doc = "`write(|w| ..)` method takes [cfgr::W](cfgr::W) writer structure"]
impl crate::Writable for CFGR {}
#[doc = "Clock configuration register"]
pub mod cfgr;
#[doc = "PLL configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllsyscfgr](pllsyscfgr) module"]
pub type PLLSYSCFGR = crate::Reg<u32, _PLLSYSCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLLSYSCFGR;
#[doc = "`read()` method returns [pllsyscfgr::R](pllsyscfgr::R) reader structure"]
impl crate::Readable for PLLSYSCFGR {}
#[doc = "`write(|w| ..)` method takes [pllsyscfgr::W](pllsyscfgr::W) writer structure"]
impl crate::Writable for PLLSYSCFGR {}
#[doc = "PLL configuration register"]
pub mod pllsyscfgr;
#[doc = "Clock interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cier](cier) module"]
pub type CIER = crate::Reg<u32, _CIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIER;
#[doc = "`read()` method returns [cier::R](cier::R) reader structure"]
impl crate::Readable for CIER {}
#[doc = "`write(|w| ..)` method takes [cier::W](cier::W) writer structure"]
impl crate::Writable for CIER {}
#[doc = "Clock interrupt enable register"]
pub mod cier;
#[doc = "Clock interrupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cifr](cifr) module"]
pub type CIFR = crate::Reg<u32, _CIFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIFR;
#[doc = "`read()` method returns [cifr::R](cifr::R) reader structure"]
impl crate::Readable for CIFR {}
#[doc = "Clock interrupt flag register"]
pub mod cifr;
#[doc = "Clock interrupt clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cicr](cicr) module"]
pub type CICR = crate::Reg<u32, _CICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CICR;
#[doc = "`write(|w| ..)` method takes [cicr::W](cicr::W) writer structure"]
impl crate::Writable for CICR {}
#[doc = "Clock interrupt clear register"]
pub mod cicr;
#[doc = "AHB peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbrstr](ahbrstr) module"]
pub type AHBRSTR = crate::Reg<u32, _AHBRSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBRSTR;
#[doc = "`read()` method returns [ahbrstr::R](ahbrstr::R) reader structure"]
impl crate::Readable for AHBRSTR {}
#[doc = "`write(|w| ..)` method takes [ahbrstr::W](ahbrstr::W) writer structure"]
impl crate::Writable for AHBRSTR {}
#[doc = "AHB peripheral reset register"]
pub mod ahbrstr;
#[doc = "GPIO reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ioprstr](ioprstr) module"]
pub type IOPRSTR = crate::Reg<u32, _IOPRSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOPRSTR;
#[doc = "`read()` method returns [ioprstr::R](ioprstr::R) reader structure"]
impl crate::Readable for IOPRSTR {}
#[doc = "`write(|w| ..)` method takes [ioprstr::W](ioprstr::W) writer structure"]
impl crate::Writable for IOPRSTR {}
#[doc = "GPIO reset register"]
pub mod ioprstr;
#[doc = "APB peripheral reset register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbrstr1](apbrstr1) module"]
pub type APBRSTR1 = crate::Reg<u32, _APBRSTR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APBRSTR1;
#[doc = "`read()` method returns [apbrstr1::R](apbrstr1::R) reader structure"]
impl crate::Readable for APBRSTR1 {}
#[doc = "`write(|w| ..)` method takes [apbrstr1::W](apbrstr1::W) writer structure"]
impl crate::Writable for APBRSTR1 {}
#[doc = "APB peripheral reset register 1"]
pub mod apbrstr1;
#[doc = "APB peripheral reset register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbrstr2](apbrstr2) module"]
pub type APBRSTR2 = crate::Reg<u32, _APBRSTR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APBRSTR2;
#[doc = "`read()` method returns [apbrstr2::R](apbrstr2::R) reader structure"]
impl crate::Readable for APBRSTR2 {}
#[doc = "`write(|w| ..)` method takes [apbrstr2::W](apbrstr2::W) writer structure"]
impl crate::Writable for APBRSTR2 {}
#[doc = "APB peripheral reset register 2"]
pub mod apbrstr2;
#[doc = "GPIO clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iopenr](iopenr) module"]
pub type IOPENR = crate::Reg<u32, _IOPENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOPENR;
#[doc = "`read()` method returns [iopenr::R](iopenr::R) reader structure"]
impl crate::Readable for IOPENR {}
#[doc = "`write(|w| ..)` method takes [iopenr::W](iopenr::W) writer structure"]
impl crate::Writable for IOPENR {}
#[doc = "GPIO clock enable register"]
pub mod iopenr;
#[doc = "AHB peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbenr](ahbenr) module"]
pub type AHBENR = crate::Reg<u32, _AHBENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBENR;
#[doc = "`read()` method returns [ahbenr::R](ahbenr::R) reader structure"]
impl crate::Readable for AHBENR {}
#[doc = "`write(|w| ..)` method takes [ahbenr::W](ahbenr::W) writer structure"]
impl crate::Writable for AHBENR {}
#[doc = "AHB peripheral clock enable register"]
pub mod ahbenr;
#[doc = "APB peripheral clock enable register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbenr1](apbenr1) module"]
pub type APBENR1 = crate::Reg<u32, _APBENR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APBENR1;
#[doc = "`read()` method returns [apbenr1::R](apbenr1::R) reader structure"]
impl crate::Readable for APBENR1 {}
#[doc = "`write(|w| ..)` method takes [apbenr1::W](apbenr1::W) writer structure"]
impl crate::Writable for APBENR1 {}
#[doc = "APB peripheral clock enable register 1"]
pub mod apbenr1;
#[doc = "APB peripheral clock enable register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbenr2](apbenr2) module"]
pub type APBENR2 = crate::Reg<u32, _APBENR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APBENR2;
#[doc = "`read()` method returns [apbenr2::R](apbenr2::R) reader structure"]
impl crate::Readable for APBENR2 {}
#[doc = "`write(|w| ..)` method takes [apbenr2::W](apbenr2::W) writer structure"]
impl crate::Writable for APBENR2 {}
#[doc = "APB peripheral clock enable register 2"]
pub mod apbenr2;
#[doc = "GPIO in Sleep mode clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iopsmenr](iopsmenr) module"]
pub type IOPSMENR = crate::Reg<u32, _IOPSMENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOPSMENR;
#[doc = "`read()` method returns [iopsmenr::R](iopsmenr::R) reader structure"]
impl crate::Readable for IOPSMENR {}
#[doc = "`write(|w| ..)` method takes [iopsmenr::W](iopsmenr::W) writer structure"]
impl crate::Writable for IOPSMENR {}
#[doc = "GPIO in Sleep mode clock enable register"]
pub mod iopsmenr;
#[doc = "AHB peripheral clock enable in Sleep mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbsmenr](ahbsmenr) module"]
pub type AHBSMENR = crate::Reg<u32, _AHBSMENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBSMENR;
#[doc = "`read()` method returns [ahbsmenr::R](ahbsmenr::R) reader structure"]
impl crate::Readable for AHBSMENR {}
#[doc = "`write(|w| ..)` method takes [ahbsmenr::W](ahbsmenr::W) writer structure"]
impl crate::Writable for AHBSMENR {}
#[doc = "AHB peripheral clock enable in Sleep mode register"]
pub mod ahbsmenr;
#[doc = "APB peripheral clock enable in Sleep mode register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbsmenr1](apbsmenr1) module"]
pub type APBSMENR1 = crate::Reg<u32, _APBSMENR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APBSMENR1;
#[doc = "`read()` method returns [apbsmenr1::R](apbsmenr1::R) reader structure"]
impl crate::Readable for APBSMENR1 {}
#[doc = "`write(|w| ..)` method takes [apbsmenr1::W](apbsmenr1::W) writer structure"]
impl crate::Writable for APBSMENR1 {}
#[doc = "APB peripheral clock enable in Sleep mode register 1"]
pub mod apbsmenr1;
#[doc = "APB peripheral clock enable in Sleep mode register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbsmenr2](apbsmenr2) module"]
pub type APBSMENR2 = crate::Reg<u32, _APBSMENR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APBSMENR2;
#[doc = "`read()` method returns [apbsmenr2::R](apbsmenr2::R) reader structure"]
impl crate::Readable for APBSMENR2 {}
#[doc = "`write(|w| ..)` method takes [apbsmenr2::W](apbsmenr2::W) writer structure"]
impl crate::Writable for APBSMENR2 {}
#[doc = "APB peripheral clock enable in Sleep mode register 2"]
pub mod apbsmenr2;
#[doc = "Peripherals independent clock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccipr](ccipr) module"]
pub type CCIPR = crate::Reg<u32, _CCIPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCIPR;
#[doc = "`read()` method returns [ccipr::R](ccipr::R) reader structure"]
impl crate::Readable for CCIPR {}
#[doc = "`write(|w| ..)` method takes [ccipr::W](ccipr::W) writer structure"]
impl crate::Writable for CCIPR {}
#[doc = "Peripherals independent clock configuration register"]
pub mod ccipr;
#[doc = "RTC domain control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdcr](bdcr) module"]
pub type BDCR = crate::Reg<u32, _BDCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDCR;
#[doc = "`read()` method returns [bdcr::R](bdcr::R) reader structure"]
impl crate::Readable for BDCR {}
#[doc = "`write(|w| ..)` method takes [bdcr::W](bdcr::W) writer structure"]
impl crate::Writable for BDCR {}
#[doc = "RTC domain control register"]
pub mod bdcr;
#[doc = "Control/status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](csr) module"]
pub type CSR = crate::Reg<u32, _CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR;
#[doc = "`read()` method returns [csr::R](csr::R) reader structure"]
impl crate::Readable for CSR {}
#[doc = "`write(|w| ..)` method takes [csr::W](csr::W) writer structure"]
impl crate::Writable for CSR {}
#[doc = "Control/status register"]
pub mod csr;
