#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock control register"]
    pub cr: CR,
    #[doc = "0x04 - Internal clock sources calibration register"]
    pub icscr: ICSCR,
    #[doc = "0x08 - Clock configuration register"]
    pub cfgr: CFGR,
    #[doc = "0x0c - Clock interrupt register"]
    pub cir: CIR,
    #[doc = "0x10 - AHB peripheral reset register"]
    pub ahbrstr: AHBRSTR,
    #[doc = "0x14 - APB2 peripheral reset register"]
    pub apb2rstr: APB2RSTR,
    #[doc = "0x18 - APB1 peripheral reset register"]
    pub apb1rstr: APB1RSTR,
    #[doc = "0x1c - AHB peripheral clock enable register"]
    pub ahbenr: AHBENR,
    #[doc = "0x20 - APB2 peripheral clock enable register"]
    pub apb2enr: APB2ENR,
    #[doc = "0x24 - APB1 peripheral clock enable register"]
    pub apb1enr: APB1ENR,
    #[doc = "0x28 - AHB peripheral clock enable in low power mode register"]
    pub ahblpenr: AHBLPENR,
    #[doc = "0x2c - APB2 peripheral clock enable in low power mode register"]
    pub apb2lpenr: APB2LPENR,
    #[doc = "0x30 - APB1 peripheral clock enable in low power mode register"]
    pub apb1lpenr: APB1LPENR,
    #[doc = "0x34 - Control/status register"]
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
#[doc = "Clock interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cir](cir) module"]
pub type CIR = crate::Reg<u32, _CIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIR;
#[doc = "`read()` method returns [cir::R](cir::R) reader structure"]
impl crate::Readable for CIR {}
#[doc = "`write(|w| ..)` method takes [cir::W](cir::W) writer structure"]
impl crate::Writable for CIR {}
#[doc = "Clock interrupt register"]
pub mod cir;
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
#[doc = "AHB peripheral clock enable in low power mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahblpenr](ahblpenr) module"]
pub type AHBLPENR = crate::Reg<u32, _AHBLPENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBLPENR;
#[doc = "`read()` method returns [ahblpenr::R](ahblpenr::R) reader structure"]
impl crate::Readable for AHBLPENR {}
#[doc = "`write(|w| ..)` method takes [ahblpenr::W](ahblpenr::W) writer structure"]
impl crate::Writable for AHBLPENR {}
#[doc = "AHB peripheral clock enable in low power mode register"]
pub mod ahblpenr;
#[doc = "APB2 peripheral clock enable in low power mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2lpenr](apb2lpenr) module"]
pub type APB2LPENR = crate::Reg<u32, _APB2LPENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB2LPENR;
#[doc = "`read()` method returns [apb2lpenr::R](apb2lpenr::R) reader structure"]
impl crate::Readable for APB2LPENR {}
#[doc = "`write(|w| ..)` method takes [apb2lpenr::W](apb2lpenr::W) writer structure"]
impl crate::Writable for APB2LPENR {}
#[doc = "APB2 peripheral clock enable in low power mode register"]
pub mod apb2lpenr;
#[doc = "APB1 peripheral clock enable in low power mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1lpenr](apb1lpenr) module"]
pub type APB1LPENR = crate::Reg<u32, _APB1LPENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1LPENR;
#[doc = "`read()` method returns [apb1lpenr::R](apb1lpenr::R) reader structure"]
impl crate::Readable for APB1LPENR {}
#[doc = "`write(|w| ..)` method takes [apb1lpenr::W](apb1lpenr::W) writer structure"]
impl crate::Writable for APB1LPENR {}
#[doc = "APB1 peripheral clock enable in low power mode register"]
pub mod apb1lpenr;
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
