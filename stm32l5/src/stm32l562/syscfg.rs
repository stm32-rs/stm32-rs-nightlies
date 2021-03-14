#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SYSCFG secure configuration register"]
    pub seccfgr: SECCFGR,
    #[doc = "0x04 - configuration register 1"]
    pub cfgr1: CFGR1,
    #[doc = "0x08 - FPU interrupt mask register"]
    pub fpuimr: FPUIMR,
    #[doc = "0x0c - SYSCFG CPU non-secure lock register"]
    pub cnslckr: CNSLCKR,
    #[doc = "0x10 - SYSCFG CPU secure lock register"]
    pub cslockr: CSLOCKR,
    #[doc = "0x14 - CFGR2"]
    pub cfgr2: CFGR2,
    #[doc = "0x18 - SCSR"]
    pub scsr: SCSR,
    #[doc = "0x1c - SKR"]
    pub skr: SKR,
    #[doc = "0x20 - SWPR"]
    pub swpr: SWPR,
    #[doc = "0x24 - SWPR2"]
    pub swpr2: SWPR2,
    _reserved10: [u8; 4usize],
    #[doc = "0x2c - RSSCMDR"]
    pub rsscmdr: RSSCMDR,
}
#[doc = "SYSCFG secure configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seccfgr](seccfgr) module"]
pub type SECCFGR = crate::Reg<u32, _SECCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECCFGR;
#[doc = "`read()` method returns [seccfgr::R](seccfgr::R) reader structure"]
impl crate::Readable for SECCFGR {}
#[doc = "`write(|w| ..)` method takes [seccfgr::W](seccfgr::W) writer structure"]
impl crate::Writable for SECCFGR {}
#[doc = "SYSCFG secure configuration register"]
pub mod seccfgr;
#[doc = "configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr1](cfgr1) module"]
pub type CFGR1 = crate::Reg<u32, _CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR1;
#[doc = "`read()` method returns [cfgr1::R](cfgr1::R) reader structure"]
impl crate::Readable for CFGR1 {}
#[doc = "`write(|w| ..)` method takes [cfgr1::W](cfgr1::W) writer structure"]
impl crate::Writable for CFGR1 {}
#[doc = "configuration register 1"]
pub mod cfgr1;
#[doc = "FPU interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpuimr](fpuimr) module"]
pub type FPUIMR = crate::Reg<u32, _FPUIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPUIMR;
#[doc = "`read()` method returns [fpuimr::R](fpuimr::R) reader structure"]
impl crate::Readable for FPUIMR {}
#[doc = "`write(|w| ..)` method takes [fpuimr::W](fpuimr::W) writer structure"]
impl crate::Writable for FPUIMR {}
#[doc = "FPU interrupt mask register"]
pub mod fpuimr;
#[doc = "SYSCFG CPU non-secure lock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnslckr](cnslckr) module"]
pub type CNSLCKR = crate::Reg<u32, _CNSLCKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNSLCKR;
#[doc = "`read()` method returns [cnslckr::R](cnslckr::R) reader structure"]
impl crate::Readable for CNSLCKR {}
#[doc = "`write(|w| ..)` method takes [cnslckr::W](cnslckr::W) writer structure"]
impl crate::Writable for CNSLCKR {}
#[doc = "SYSCFG CPU non-secure lock register"]
pub mod cnslckr;
#[doc = "SYSCFG CPU secure lock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cslockr](cslockr) module"]
pub type CSLOCKR = crate::Reg<u32, _CSLOCKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSLOCKR;
#[doc = "`read()` method returns [cslockr::R](cslockr::R) reader structure"]
impl crate::Readable for CSLOCKR {}
#[doc = "`write(|w| ..)` method takes [cslockr::W](cslockr::W) writer structure"]
impl crate::Writable for CSLOCKR {}
#[doc = "SYSCFG CPU secure lock register"]
pub mod cslockr;
#[doc = "SCSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scsr](scsr) module"]
pub type SCSR = crate::Reg<u32, _SCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCSR;
#[doc = "`read()` method returns [scsr::R](scsr::R) reader structure"]
impl crate::Readable for SCSR {}
#[doc = "`write(|w| ..)` method takes [scsr::W](scsr::W) writer structure"]
impl crate::Writable for SCSR {}
#[doc = "SCSR"]
pub mod scsr;
#[doc = "CFGR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr2](cfgr2) module"]
pub type CFGR2 = crate::Reg<u32, _CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR2;
#[doc = "`read()` method returns [cfgr2::R](cfgr2::R) reader structure"]
impl crate::Readable for CFGR2 {}
#[doc = "`write(|w| ..)` method takes [cfgr2::W](cfgr2::W) writer structure"]
impl crate::Writable for CFGR2 {}
#[doc = "CFGR2"]
pub mod cfgr2;
#[doc = "SWPR\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swpr](swpr) module"]
pub type SWPR = crate::Reg<u32, _SWPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWPR;
#[doc = "`write(|w| ..)` method takes [swpr::W](swpr::W) writer structure"]
impl crate::Writable for SWPR {}
#[doc = "SWPR"]
pub mod swpr;
#[doc = "SKR\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [skr](skr) module"]
pub type SKR = crate::Reg<u32, _SKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SKR;
#[doc = "`write(|w| ..)` method takes [skr::W](skr::W) writer structure"]
impl crate::Writable for SKR {}
#[doc = "SKR"]
pub mod skr;
#[doc = "SWPR2\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swpr2](swpr2) module"]
pub type SWPR2 = crate::Reg<u32, _SWPR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWPR2;
#[doc = "`write(|w| ..)` method takes [swpr2::W](swpr2::W) writer structure"]
impl crate::Writable for SWPR2 {}
#[doc = "SWPR2"]
pub mod swpr2;
#[doc = "RSSCMDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsscmdr](rsscmdr) module"]
pub type RSSCMDR = crate::Reg<u32, _RSSCMDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSSCMDR;
#[doc = "`read()` method returns [rsscmdr::R](rsscmdr::R) reader structure"]
impl crate::Readable for RSSCMDR {}
#[doc = "`write(|w| ..)` method takes [rsscmdr::W](rsscmdr::W) writer structure"]
impl crate::Writable for RSSCMDR {}
#[doc = "RSSCMDR"]
pub mod rsscmdr;
