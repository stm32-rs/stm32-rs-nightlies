#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock control register"]
    pub cr: CR,
    #[doc = "0x04 - Internal clock sources calibration register"]
    pub icscr: ICSCR,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - Clock configuration register"]
    pub cfgr: CFGR,
    #[doc = "0x10 - Clock interrupt enable register"]
    pub cier: CIER,
    #[doc = "0x14 - Clock interrupt flag register"]
    pub cifr: CIFR,
    #[doc = "0x18 - Clock interrupt clear register"]
    pub cicr: CICR,
    #[doc = "0x1c - GPIO reset register"]
    pub ioprstr: IOPRSTR,
    #[doc = "0x20 - AHB peripheral reset register"]
    pub ahbrstr: AHBRSTR,
    #[doc = "0x24 - APB2 peripheral reset register"]
    pub apb2rstr: APB2RSTR,
    #[doc = "0x28 - APB1 peripheral reset register"]
    pub apb1rstr: APB1RSTR,
    #[doc = "0x2c - GPIO clock enable register"]
    pub iopenr: IOPENR,
    #[doc = "0x30 - AHB peripheral clock enable register"]
    pub ahbenr: AHBENR,
    #[doc = "0x34 - APB2 peripheral clock enable register"]
    pub apb2enr: APB2ENR,
    #[doc = "0x38 - APB1 peripheral clock enable register"]
    pub apb1enr: APB1ENR,
    #[doc = "0x3c - GPIO clock enable in sleep mode register"]
    pub iopsmen: IOPSMEN,
    #[doc = "0x40 - AHB peripheral clock enable in sleep mode register"]
    pub ahbsmenr: AHBSMENR,
    #[doc = "0x44 - APB2 peripheral clock enable in sleep mode register"]
    pub apb2smenr: APB2SMENR,
    #[doc = "0x48 - APB1 peripheral clock enable in sleep mode register"]
    pub apb1smenr: APB1SMENR,
    #[doc = "0x4c - Clock configuration register"]
    pub ccipr: CCIPR,
    #[doc = "0x50 - Control and status register"]
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
#[doc = "Clock interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cier](cier) module"]
pub type CIER = crate::Reg<u32, _CIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIER;
#[doc = "`read()` method returns [cier::R](cier::R) reader structure"]
impl crate::Readable for CIER {}
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
#[doc = "Clock interrupt clear register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cicr](cicr) module"]
pub type CICR = crate::Reg<u32, _CICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CICR;
#[doc = "`read()` method returns [cicr::R](cicr::R) reader structure"]
impl crate::Readable for CICR {}
#[doc = "Clock interrupt clear register"]
pub mod cicr;
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
#[doc = "APB2 peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2rstr](apb2rstr) module"]
pub type APB2RSTR = crate::Reg<u32, _APB2RSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB2RSTR;
#[doc = "`read()` method returns [apb2rstr::R](apb2rstr::R) reader structure"]
impl crate::Readable for APB2RSTR {}
#[doc = "`write(|w| ..)` method takes [apb2rstr::W](apb2rstr::W) writer structure"]
impl crate::Writable for APB2RSTR {}
#[doc = "APB2 peripheral reset register"]
pub mod apb2rstr;
#[doc = "APB1 peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1rstr](apb1rstr) module"]
pub type APB1RSTR = crate::Reg<u32, _APB1RSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1RSTR;
#[doc = "`read()` method returns [apb1rstr::R](apb1rstr::R) reader structure"]
impl crate::Readable for APB1RSTR {}
#[doc = "`write(|w| ..)` method takes [apb1rstr::W](apb1rstr::W) writer structure"]
impl crate::Writable for APB1RSTR {}
#[doc = "APB1 peripheral reset register"]
pub mod apb1rstr;
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
#[doc = "APB2 peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2enr](apb2enr) module"]
pub type APB2ENR = crate::Reg<u32, _APB2ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB2ENR;
#[doc = "`read()` method returns [apb2enr::R](apb2enr::R) reader structure"]
impl crate::Readable for APB2ENR {}
#[doc = "`write(|w| ..)` method takes [apb2enr::W](apb2enr::W) writer structure"]
impl crate::Writable for APB2ENR {}
#[doc = "APB2 peripheral clock enable register"]
pub mod apb2enr;
#[doc = "APB1 peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1enr](apb1enr) module"]
pub type APB1ENR = crate::Reg<u32, _APB1ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1ENR;
#[doc = "`read()` method returns [apb1enr::R](apb1enr::R) reader structure"]
impl crate::Readable for APB1ENR {}
#[doc = "`write(|w| ..)` method takes [apb1enr::W](apb1enr::W) writer structure"]
impl crate::Writable for APB1ENR {}
#[doc = "APB1 peripheral clock enable register"]
pub mod apb1enr;
#[doc = "GPIO clock enable in sleep mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iopsmen](iopsmen) module"]
pub type IOPSMEN = crate::Reg<u32, _IOPSMEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOPSMEN;
#[doc = "`read()` method returns [iopsmen::R](iopsmen::R) reader structure"]
impl crate::Readable for IOPSMEN {}
#[doc = "`write(|w| ..)` method takes [iopsmen::W](iopsmen::W) writer structure"]
impl crate::Writable for IOPSMEN {}
#[doc = "GPIO clock enable in sleep mode register"]
pub mod iopsmen;
#[doc = "AHB peripheral clock enable in sleep mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbsmenr](ahbsmenr) module"]
pub type AHBSMENR = crate::Reg<u32, _AHBSMENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBSMENR;
#[doc = "`read()` method returns [ahbsmenr::R](ahbsmenr::R) reader structure"]
impl crate::Readable for AHBSMENR {}
#[doc = "`write(|w| ..)` method takes [ahbsmenr::W](ahbsmenr::W) writer structure"]
impl crate::Writable for AHBSMENR {}
#[doc = "AHB peripheral clock enable in sleep mode register"]
pub mod ahbsmenr;
#[doc = "APB2 peripheral clock enable in sleep mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2smenr](apb2smenr) module"]
pub type APB2SMENR = crate::Reg<u32, _APB2SMENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB2SMENR;
#[doc = "`read()` method returns [apb2smenr::R](apb2smenr::R) reader structure"]
impl crate::Readable for APB2SMENR {}
#[doc = "`write(|w| ..)` method takes [apb2smenr::W](apb2smenr::W) writer structure"]
impl crate::Writable for APB2SMENR {}
#[doc = "APB2 peripheral clock enable in sleep mode register"]
pub mod apb2smenr;
#[doc = "APB1 peripheral clock enable in sleep mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1smenr](apb1smenr) module"]
pub type APB1SMENR = crate::Reg<u32, _APB1SMENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1SMENR;
#[doc = "`read()` method returns [apb1smenr::R](apb1smenr::R) reader structure"]
impl crate::Readable for APB1SMENR {}
#[doc = "`write(|w| ..)` method takes [apb1smenr::W](apb1smenr::W) writer structure"]
impl crate::Writable for APB1SMENR {}
#[doc = "APB1 peripheral clock enable in sleep mode register"]
pub mod apb1smenr;
#[doc = "Clock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccipr](ccipr) module"]
pub type CCIPR = crate::Reg<u32, _CCIPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCIPR;
#[doc = "`read()` method returns [ccipr::R](ccipr::R) reader structure"]
impl crate::Readable for CCIPR {}
#[doc = "`write(|w| ..)` method takes [ccipr::W](ccipr::W) writer structure"]
impl crate::Writable for CCIPR {}
#[doc = "Clock configuration register"]
pub mod ccipr;
#[doc = "Control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](csr) module"]
pub type CSR = crate::Reg<u32, _CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR;
#[doc = "`read()` method returns [csr::R](csr::R) reader structure"]
impl crate::Readable for CSR {}
#[doc = "`write(|w| ..)` method takes [csr::W](csr::W) writer structure"]
impl crate::Writable for CSR {}
#[doc = "Control and status register"]
pub mod csr;
