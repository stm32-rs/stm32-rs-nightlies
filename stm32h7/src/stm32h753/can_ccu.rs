#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock Calibration Unit Core Release Register"]
    pub crel: CREL,
    #[doc = "0x04 - Calibration Configuration Register"]
    pub ccfg: CCFG,
    #[doc = "0x08 - Calibration Status Register"]
    pub cstat: CSTAT,
    #[doc = "0x0c - Calibration Watchdog Register"]
    pub cwd: CWD,
    #[doc = "0x10 - Clock Calibration Unit Interrupt Register"]
    pub ir: IR,
    #[doc = "0x14 - Clock Calibration Unit Interrupt Enable Register"]
    pub ie: IE,
}
#[doc = "Clock Calibration Unit Core Release Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crel](crel) module"]
pub type CREL = crate::Reg<u32, _CREL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CREL;
#[doc = "`read()` method returns [crel::R](crel::R) reader structure"]
impl crate::Readable for CREL {}
#[doc = "`write(|w| ..)` method takes [crel::W](crel::W) writer structure"]
impl crate::Writable for CREL {}
#[doc = "Clock Calibration Unit Core Release Register"]
pub mod crel;
#[doc = "Calibration Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg](ccfg) module"]
pub type CCFG = crate::Reg<u32, _CCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCFG;
#[doc = "`read()` method returns [ccfg::R](ccfg::R) reader structure"]
impl crate::Readable for CCFG {}
#[doc = "`write(|w| ..)` method takes [ccfg::W](ccfg::W) writer structure"]
impl crate::Writable for CCFG {}
#[doc = "Calibration Configuration Register"]
pub mod ccfg;
#[doc = "Calibration Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cstat](cstat) module"]
pub type CSTAT = crate::Reg<u32, _CSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSTAT;
#[doc = "`read()` method returns [cstat::R](cstat::R) reader structure"]
impl crate::Readable for CSTAT {}
#[doc = "`write(|w| ..)` method takes [cstat::W](cstat::W) writer structure"]
impl crate::Writable for CSTAT {}
#[doc = "Calibration Status Register"]
pub mod cstat;
#[doc = "Calibration Watchdog Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cwd](cwd) module"]
pub type CWD = crate::Reg<u32, _CWD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CWD;
#[doc = "`read()` method returns [cwd::R](cwd::R) reader structure"]
impl crate::Readable for CWD {}
#[doc = "`write(|w| ..)` method takes [cwd::W](cwd::W) writer structure"]
impl crate::Writable for CWD {}
#[doc = "Calibration Watchdog Register"]
pub mod cwd;
#[doc = "Clock Calibration Unit Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ir](ir) module"]
pub type IR = crate::Reg<u32, _IR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IR;
#[doc = "`read()` method returns [ir::R](ir::R) reader structure"]
impl crate::Readable for IR {}
#[doc = "`write(|w| ..)` method takes [ir::W](ir::W) writer structure"]
impl crate::Writable for IR {}
#[doc = "Clock Calibration Unit Interrupt Register"]
pub mod ir;
#[doc = "Clock Calibration Unit Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ie](ie) module"]
pub type IE = crate::Reg<u32, _IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IE;
#[doc = "`read()` method returns [ie::R](ie::R) reader structure"]
impl crate::Readable for IE {}
#[doc = "`write(|w| ..)` method takes [ie::W](ie::W) writer structure"]
impl crate::Writable for IE {}
#[doc = "Clock Calibration Unit Interrupt Enable Register"]
pub mod ie;
