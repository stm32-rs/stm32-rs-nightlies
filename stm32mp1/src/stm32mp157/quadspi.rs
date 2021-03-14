#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - QUADSPI control register"]
    pub quadspi_cr: QUADSPI_CR,
    #[doc = "0x04 - QUADSPI device configuration register"]
    pub quadspi_dcr: QUADSPI_DCR,
    #[doc = "0x08 - QUADSPI status register"]
    pub quadspi_sr: QUADSPI_SR,
    #[doc = "0x0c - QUADSPI flag clear register"]
    pub quadspi_fcr: QUADSPI_FCR,
    #[doc = "0x10 - QUADSPI data length register"]
    pub quadspi_dlr: QUADSPI_DLR,
    #[doc = "0x14 - QUADSPI communication configuration register"]
    pub quadspi_ccr: QUADSPI_CCR,
    #[doc = "0x18 - QUADSPI address register"]
    pub quadspi_ar: QUADSPI_AR,
    #[doc = "0x1c - QUADSPI alternate bytes registers"]
    pub quadspi_abr: QUADSPI_ABR,
    #[doc = "0x20 - QUADSPI data register"]
    pub quadspi_dr: QUADSPI_DR,
    #[doc = "0x24 - QUADSPI polling status mask register"]
    pub quadspi_psmkr: QUADSPI_PSMKR,
    #[doc = "0x28 - QUADSPI polling status match register"]
    pub quadspi_psmar: QUADSPI_PSMAR,
    #[doc = "0x2c - QUADSPI polling interval register"]
    pub quadspi_pir: QUADSPI_PIR,
    #[doc = "0x30 - QUADSPI low-power timeout register"]
    pub quadspi_lptr: QUADSPI_LPTR,
    _reserved13: [u8; 956usize],
    #[doc = "0x3f0 - QUADSPI HW configuration register"]
    pub quadspi_hwcfgr: QUADSPI_HWCFGR,
    #[doc = "0x3f4 - QUADSPI version register"]
    pub quadspi_verr: QUADSPI_VERR,
    #[doc = "0x3f8 - QUADSPI identification register"]
    pub quadspi_ipidr: QUADSPI_IPIDR,
    #[doc = "0x3fc - QUADSPI size identification register"]
    pub quadspi_sidr: QUADSPI_SIDR,
}
#[doc = "QUADSPI control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [quadspi_cr](quadspi_cr) module"]
pub type QUADSPI_CR = crate::Reg<u32, _QUADSPI_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QUADSPI_CR;
#[doc = "`read()` method returns [quadspi_cr::R](quadspi_cr::R) reader structure"]
impl crate::Readable for QUADSPI_CR {}
#[doc = "`write(|w| ..)` method takes [quadspi_cr::W](quadspi_cr::W) writer structure"]
impl crate::Writable for QUADSPI_CR {}
#[doc = "QUADSPI control register"]
pub mod quadspi_cr;
#[doc = "QUADSPI device configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [quadspi_dcr](quadspi_dcr) module"]
pub type QUADSPI_DCR = crate::Reg<u32, _QUADSPI_DCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QUADSPI_DCR;
#[doc = "`read()` method returns [quadspi_dcr::R](quadspi_dcr::R) reader structure"]
impl crate::Readable for QUADSPI_DCR {}
#[doc = "`write(|w| ..)` method takes [quadspi_dcr::W](quadspi_dcr::W) writer structure"]
impl crate::Writable for QUADSPI_DCR {}
#[doc = "QUADSPI device configuration register"]
pub mod quadspi_dcr;
#[doc = "QUADSPI status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [quadspi_sr](quadspi_sr) module"]
pub type QUADSPI_SR = crate::Reg<u32, _QUADSPI_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QUADSPI_SR;
#[doc = "`read()` method returns [quadspi_sr::R](quadspi_sr::R) reader structure"]
impl crate::Readable for QUADSPI_SR {}
#[doc = "QUADSPI status register"]
pub mod quadspi_sr;
#[doc = "QUADSPI flag clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [quadspi_fcr](quadspi_fcr) module"]
pub type QUADSPI_FCR = crate::Reg<u32, _QUADSPI_FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QUADSPI_FCR;
#[doc = "`write(|w| ..)` method takes [quadspi_fcr::W](quadspi_fcr::W) writer structure"]
impl crate::Writable for QUADSPI_FCR {}
#[doc = "QUADSPI flag clear register"]
pub mod quadspi_fcr;
#[doc = "QUADSPI data length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [quadspi_dlr](quadspi_dlr) module"]
pub type QUADSPI_DLR = crate::Reg<u32, _QUADSPI_DLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QUADSPI_DLR;
#[doc = "`read()` method returns [quadspi_dlr::R](quadspi_dlr::R) reader structure"]
impl crate::Readable for QUADSPI_DLR {}
#[doc = "`write(|w| ..)` method takes [quadspi_dlr::W](quadspi_dlr::W) writer structure"]
impl crate::Writable for QUADSPI_DLR {}
#[doc = "QUADSPI data length register"]
pub mod quadspi_dlr;
#[doc = "QUADSPI communication configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [quadspi_ccr](quadspi_ccr) module"]
pub type QUADSPI_CCR = crate::Reg<u32, _QUADSPI_CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QUADSPI_CCR;
#[doc = "`read()` method returns [quadspi_ccr::R](quadspi_ccr::R) reader structure"]
impl crate::Readable for QUADSPI_CCR {}
#[doc = "`write(|w| ..)` method takes [quadspi_ccr::W](quadspi_ccr::W) writer structure"]
impl crate::Writable for QUADSPI_CCR {}
#[doc = "QUADSPI communication configuration register"]
pub mod quadspi_ccr;
#[doc = "QUADSPI address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [quadspi_ar](quadspi_ar) module"]
pub type QUADSPI_AR = crate::Reg<u32, _QUADSPI_AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QUADSPI_AR;
#[doc = "`read()` method returns [quadspi_ar::R](quadspi_ar::R) reader structure"]
impl crate::Readable for QUADSPI_AR {}
#[doc = "`write(|w| ..)` method takes [quadspi_ar::W](quadspi_ar::W) writer structure"]
impl crate::Writable for QUADSPI_AR {}
#[doc = "QUADSPI address register"]
pub mod quadspi_ar;
#[doc = "QUADSPI alternate bytes registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [quadspi_abr](quadspi_abr) module"]
pub type QUADSPI_ABR = crate::Reg<u32, _QUADSPI_ABR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QUADSPI_ABR;
#[doc = "`read()` method returns [quadspi_abr::R](quadspi_abr::R) reader structure"]
impl crate::Readable for QUADSPI_ABR {}
#[doc = "`write(|w| ..)` method takes [quadspi_abr::W](quadspi_abr::W) writer structure"]
impl crate::Writable for QUADSPI_ABR {}
#[doc = "QUADSPI alternate bytes registers"]
pub mod quadspi_abr;
#[doc = "QUADSPI data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [quadspi_dr](quadspi_dr) module"]
pub type QUADSPI_DR = crate::Reg<u32, _QUADSPI_DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QUADSPI_DR;
#[doc = "`read()` method returns [quadspi_dr::R](quadspi_dr::R) reader structure"]
impl crate::Readable for QUADSPI_DR {}
#[doc = "`write(|w| ..)` method takes [quadspi_dr::W](quadspi_dr::W) writer structure"]
impl crate::Writable for QUADSPI_DR {}
#[doc = "QUADSPI data register"]
pub mod quadspi_dr;
#[doc = "QUADSPI polling status mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [quadspi_psmkr](quadspi_psmkr) module"]
pub type QUADSPI_PSMKR = crate::Reg<u32, _QUADSPI_PSMKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QUADSPI_PSMKR;
#[doc = "`read()` method returns [quadspi_psmkr::R](quadspi_psmkr::R) reader structure"]
impl crate::Readable for QUADSPI_PSMKR {}
#[doc = "`write(|w| ..)` method takes [quadspi_psmkr::W](quadspi_psmkr::W) writer structure"]
impl crate::Writable for QUADSPI_PSMKR {}
#[doc = "QUADSPI polling status mask register"]
pub mod quadspi_psmkr;
#[doc = "QUADSPI polling status match register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [quadspi_psmar](quadspi_psmar) module"]
pub type QUADSPI_PSMAR = crate::Reg<u32, _QUADSPI_PSMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QUADSPI_PSMAR;
#[doc = "`read()` method returns [quadspi_psmar::R](quadspi_psmar::R) reader structure"]
impl crate::Readable for QUADSPI_PSMAR {}
#[doc = "`write(|w| ..)` method takes [quadspi_psmar::W](quadspi_psmar::W) writer structure"]
impl crate::Writable for QUADSPI_PSMAR {}
#[doc = "QUADSPI polling status match register"]
pub mod quadspi_psmar;
#[doc = "QUADSPI polling interval register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [quadspi_pir](quadspi_pir) module"]
pub type QUADSPI_PIR = crate::Reg<u32, _QUADSPI_PIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QUADSPI_PIR;
#[doc = "`read()` method returns [quadspi_pir::R](quadspi_pir::R) reader structure"]
impl crate::Readable for QUADSPI_PIR {}
#[doc = "`write(|w| ..)` method takes [quadspi_pir::W](quadspi_pir::W) writer structure"]
impl crate::Writable for QUADSPI_PIR {}
#[doc = "QUADSPI polling interval register"]
pub mod quadspi_pir;
#[doc = "QUADSPI low-power timeout register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [quadspi_lptr](quadspi_lptr) module"]
pub type QUADSPI_LPTR = crate::Reg<u32, _QUADSPI_LPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QUADSPI_LPTR;
#[doc = "`read()` method returns [quadspi_lptr::R](quadspi_lptr::R) reader structure"]
impl crate::Readable for QUADSPI_LPTR {}
#[doc = "`write(|w| ..)` method takes [quadspi_lptr::W](quadspi_lptr::W) writer structure"]
impl crate::Writable for QUADSPI_LPTR {}
#[doc = "QUADSPI low-power timeout register"]
pub mod quadspi_lptr;
#[doc = "QUADSPI HW configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [quadspi_hwcfgr](quadspi_hwcfgr) module"]
pub type QUADSPI_HWCFGR = crate::Reg<u32, _QUADSPI_HWCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QUADSPI_HWCFGR;
#[doc = "`read()` method returns [quadspi_hwcfgr::R](quadspi_hwcfgr::R) reader structure"]
impl crate::Readable for QUADSPI_HWCFGR {}
#[doc = "QUADSPI HW configuration register"]
pub mod quadspi_hwcfgr;
#[doc = "QUADSPI version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [quadspi_verr](quadspi_verr) module"]
pub type QUADSPI_VERR = crate::Reg<u32, _QUADSPI_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QUADSPI_VERR;
#[doc = "`read()` method returns [quadspi_verr::R](quadspi_verr::R) reader structure"]
impl crate::Readable for QUADSPI_VERR {}
#[doc = "QUADSPI version register"]
pub mod quadspi_verr;
#[doc = "QUADSPI identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [quadspi_ipidr](quadspi_ipidr) module"]
pub type QUADSPI_IPIDR = crate::Reg<u32, _QUADSPI_IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QUADSPI_IPIDR;
#[doc = "`read()` method returns [quadspi_ipidr::R](quadspi_ipidr::R) reader structure"]
impl crate::Readable for QUADSPI_IPIDR {}
#[doc = "QUADSPI identification register"]
pub mod quadspi_ipidr;
#[doc = "QUADSPI size identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [quadspi_sidr](quadspi_sidr) module"]
pub type QUADSPI_SIDR = crate::Reg<u32, _QUADSPI_SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QUADSPI_SIDR;
#[doc = "`read()` method returns [quadspi_sidr::R](quadspi_sidr::R) reader structure"]
impl crate::Readable for QUADSPI_SIDR {}
#[doc = "QUADSPI size identification register"]
pub mod quadspi_sidr;
