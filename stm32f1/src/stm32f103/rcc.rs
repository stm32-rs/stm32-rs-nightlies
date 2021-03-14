#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock control register"]
    pub cr: CR,
    #[doc = "0x04 - Clock configuration register (RCC_CFGR)"]
    pub cfgr: CFGR,
    #[doc = "0x08 - Clock interrupt register (RCC_CIR)"]
    pub cir: CIR,
    #[doc = "0x0c - APB2 peripheral reset register (RCC_APB2RSTR)"]
    pub apb2rstr: APB2RSTR,
    #[doc = "0x10 - APB1 peripheral reset register (RCC_APB1RSTR)"]
    pub apb1rstr: APB1RSTR,
    #[doc = "0x14 - AHB Peripheral Clock enable register (RCC_AHBENR)"]
    pub ahbenr: AHBENR,
    #[doc = "0x18 - APB2 peripheral clock enable register (RCC_APB2ENR)"]
    pub apb2enr: APB2ENR,
    #[doc = "0x1c - APB1 peripheral clock enable register (RCC_APB1ENR)"]
    pub apb1enr: APB1ENR,
    #[doc = "0x20 - Backup domain control register (RCC_BDCR)"]
    pub bdcr: BDCR,
    #[doc = "0x24 - Control/status register (RCC_CSR)"]
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
#[doc = "Clock configuration register (RCC_CFGR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr](cfgr) module"]
pub type CFGR = crate::Reg<u32, _CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR;
#[doc = "`read()` method returns [cfgr::R](cfgr::R) reader structure"]
impl crate::Readable for CFGR {}
#[doc = "`write(|w| ..)` method takes [cfgr::W](cfgr::W) writer structure"]
impl crate::Writable for CFGR {}
#[doc = "Clock configuration register (RCC_CFGR)"]
pub mod cfgr;
#[doc = "Clock interrupt register (RCC_CIR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cir](cir) module"]
pub type CIR = crate::Reg<u32, _CIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIR;
#[doc = "`read()` method returns [cir::R](cir::R) reader structure"]
impl crate::Readable for CIR {}
#[doc = "`write(|w| ..)` method takes [cir::W](cir::W) writer structure"]
impl crate::Writable for CIR {}
#[doc = "Clock interrupt register (RCC_CIR)"]
pub mod cir;
#[doc = "APB2 peripheral reset register (RCC_APB2RSTR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2rstr](apb2rstr) module"]
pub type APB2RSTR = crate::Reg<u32, _APB2RSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB2RSTR;
#[doc = "`read()` method returns [apb2rstr::R](apb2rstr::R) reader structure"]
impl crate::Readable for APB2RSTR {}
#[doc = "`write(|w| ..)` method takes [apb2rstr::W](apb2rstr::W) writer structure"]
impl crate::Writable for APB2RSTR {}
#[doc = "APB2 peripheral reset register (RCC_APB2RSTR)"]
pub mod apb2rstr;
#[doc = "APB1 peripheral reset register (RCC_APB1RSTR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1rstr](apb1rstr) module"]
pub type APB1RSTR = crate::Reg<u32, _APB1RSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1RSTR;
#[doc = "`read()` method returns [apb1rstr::R](apb1rstr::R) reader structure"]
impl crate::Readable for APB1RSTR {}
#[doc = "`write(|w| ..)` method takes [apb1rstr::W](apb1rstr::W) writer structure"]
impl crate::Writable for APB1RSTR {}
#[doc = "APB1 peripheral reset register (RCC_APB1RSTR)"]
pub mod apb1rstr;
#[doc = "AHB Peripheral Clock enable register (RCC_AHBENR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbenr](ahbenr) module"]
pub type AHBENR = crate::Reg<u32, _AHBENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBENR;
#[doc = "`read()` method returns [ahbenr::R](ahbenr::R) reader structure"]
impl crate::Readable for AHBENR {}
#[doc = "`write(|w| ..)` method takes [ahbenr::W](ahbenr::W) writer structure"]
impl crate::Writable for AHBENR {}
#[doc = "AHB Peripheral Clock enable register (RCC_AHBENR)"]
pub mod ahbenr;
#[doc = "APB2 peripheral clock enable register (RCC_APB2ENR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2enr](apb2enr) module"]
pub type APB2ENR = crate::Reg<u32, _APB2ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB2ENR;
#[doc = "`read()` method returns [apb2enr::R](apb2enr::R) reader structure"]
impl crate::Readable for APB2ENR {}
#[doc = "`write(|w| ..)` method takes [apb2enr::W](apb2enr::W) writer structure"]
impl crate::Writable for APB2ENR {}
#[doc = "APB2 peripheral clock enable register (RCC_APB2ENR)"]
pub mod apb2enr;
#[doc = "APB1 peripheral clock enable register (RCC_APB1ENR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1enr](apb1enr) module"]
pub type APB1ENR = crate::Reg<u32, _APB1ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1ENR;
#[doc = "`read()` method returns [apb1enr::R](apb1enr::R) reader structure"]
impl crate::Readable for APB1ENR {}
#[doc = "`write(|w| ..)` method takes [apb1enr::W](apb1enr::W) writer structure"]
impl crate::Writable for APB1ENR {}
#[doc = "APB1 peripheral clock enable register (RCC_APB1ENR)"]
pub mod apb1enr;
#[doc = "Backup domain control register (RCC_BDCR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdcr](bdcr) module"]
pub type BDCR = crate::Reg<u32, _BDCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDCR;
#[doc = "`read()` method returns [bdcr::R](bdcr::R) reader structure"]
impl crate::Readable for BDCR {}
#[doc = "`write(|w| ..)` method takes [bdcr::W](bdcr::W) writer structure"]
impl crate::Writable for BDCR {}
#[doc = "Backup domain control register (RCC_BDCR)"]
pub mod bdcr;
#[doc = "Control/status register (RCC_CSR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](csr) module"]
pub type CSR = crate::Reg<u32, _CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR;
#[doc = "`read()` method returns [csr::R](csr::R) reader structure"]
impl crate::Readable for CSR {}
#[doc = "`write(|w| ..)` method takes [csr::W](csr::W) writer structure"]
impl crate::Writable for CSR {}
#[doc = "Control/status register (RCC_CSR)"]
pub mod csr;
