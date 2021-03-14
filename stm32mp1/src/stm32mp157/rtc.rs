#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - The RTC_TR is the calendar time shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page1830 and Reading the calendar on page1831. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub rtc_tr: RTC_TR,
    #[doc = "0x04 - The RTC_DR is the calendar date shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page1830 and Reading the calendar on page1831. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub rtc_dr: RTC_DR,
    #[doc = "0x08 - RTC sub second register"]
    pub rtc_ssr: RTC_SSR,
    #[doc = "0x0c - This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be globally protected, or each bit of this register can be individually protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub rtc_icsr: RTC_ICSR,
    #[doc = "0x10 - This register must be written in initialization mode only. The initialization must be performed in two separate write accesses. Refer to Calendar initialization and configuration on page1830. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub rtc_prer: RTC_PRER,
    #[doc = "0x14 - This register can be written only when WUTWF is set to 1 in RTC_ICSR. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub rtc_wutr: RTC_WUTR,
    #[doc = "0x18 - This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be globally protected, or each bit of this register can be individually protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub rtc_cr: RTC_CR,
    _reserved7: [u8; 4usize],
    #[doc = "0x20 - This register can be written only when the APB access is secure."]
    pub rtc_smcr: RTC_SMCR,
    #[doc = "0x24 - RTC write protection register"]
    pub rtc_wpr: RTC_WPR,
    #[doc = "0x28 - This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub rtc_calr: RTC_CALR,
    #[doc = "0x2c - This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub rtc_shiftr: RTC_SHIFTR,
    #[doc = "0x30 - The content of this register is valid only when TSF is set to 1 in RTC_SR. It is cleared when TSF bit is reset. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub rtc_tstr: RTC_TSTR,
    #[doc = "0x34 - The content of this register is valid only when TSF is set to 1 in RTC_SR. It is cleared when TSF bit is reset. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub rtc_tsdr: RTC_TSDR,
    #[doc = "0x38 - The content of this register is valid only when TSF is set to 1 in RTC_SR. It is cleared when the TSF bit is reset. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub rtc_tsssr: RTC_TSSSR,
    _reserved14: [u8; 4usize],
    #[doc = "0x40 - This register can be written only when ALRAWF is set to 1 in RTC_ICSR, or in initialization mode. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub rtc_alrmar: RTC_ALRMAR,
    #[doc = "0x44 - This register can be written only when ALRAWF is set to 1 in RTC_ICSR, or in initialization mode. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub rtc_alrmassr: RTC_ALRMASSR,
    #[doc = "0x48 - This register can be written only when ALRBWF is set to 1 in RTC_ICSR, or in initialization mode. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub rtc_alrmbr: RTC_ALRMBR,
    #[doc = "0x4c - This register can be written only when ALRBE is reset in RTC_CR register, or in initialization mode. This register is write protected.The write access procedure is described in Section: RTC register write protection. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub rtc_alrmbssr: RTC_ALRMBSSR,
    #[doc = "0x50 - This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub rtc_sr: RTC_SR,
    #[doc = "0x54 - RTC non-secure masked interrupt status register"]
    pub rtc_misr: RTC_MISR,
    #[doc = "0x58 - This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub rtc_smisr: RTC_SMISR,
    #[doc = "0x5c - This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub rtc_scr: RTC_SCR,
    #[doc = "0x60 - RTC configuration register"]
    pub rtc_cfgr: RTC_CFGR,
    _reserved23: [u8; 908usize],
    #[doc = "0x3f0 - RTC hardware configuration register"]
    pub rtc_hwcfgr: RTC_HWCFGR,
    #[doc = "0x3f4 - RTC version register"]
    pub rtc_verr: RTC_VERR,
    #[doc = "0x3f8 - RTC identification register"]
    pub rtc_ipidr: RTC_IPIDR,
    #[doc = "0x3fc - RTC size identification register"]
    pub rtc_sidr: RTC_SIDR,
}
#[doc = "The RTC_TR is the calendar time shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page1830 and Reading the calendar on page1831. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_tr](rtc_tr) module"]
pub type RTC_TR = crate::Reg<u32, _RTC_TR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_TR;
#[doc = "`read()` method returns [rtc_tr::R](rtc_tr::R) reader structure"]
impl crate::Readable for RTC_TR {}
#[doc = "`write(|w| ..)` method takes [rtc_tr::W](rtc_tr::W) writer structure"]
impl crate::Writable for RTC_TR {}
#[doc = "The RTC_TR is the calendar time shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page1830 and Reading the calendar on page1831. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod rtc_tr;
#[doc = "The RTC_DR is the calendar date shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page1830 and Reading the calendar on page1831. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_dr](rtc_dr) module"]
pub type RTC_DR = crate::Reg<u32, _RTC_DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_DR;
#[doc = "`read()` method returns [rtc_dr::R](rtc_dr::R) reader structure"]
impl crate::Readable for RTC_DR {}
#[doc = "`write(|w| ..)` method takes [rtc_dr::W](rtc_dr::W) writer structure"]
impl crate::Writable for RTC_DR {}
#[doc = "The RTC_DR is the calendar date shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page1830 and Reading the calendar on page1831. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod rtc_dr;
#[doc = "RTC sub second register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_ssr](rtc_ssr) module"]
pub type RTC_SSR = crate::Reg<u32, _RTC_SSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_SSR;
#[doc = "`read()` method returns [rtc_ssr::R](rtc_ssr::R) reader structure"]
impl crate::Readable for RTC_SSR {}
#[doc = "RTC sub second register"]
pub mod rtc_ssr;
#[doc = "This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be globally protected, or each bit of this register can be individually protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_icsr](rtc_icsr) module"]
pub type RTC_ICSR = crate::Reg<u32, _RTC_ICSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_ICSR;
#[doc = "`read()` method returns [rtc_icsr::R](rtc_icsr::R) reader structure"]
impl crate::Readable for RTC_ICSR {}
#[doc = "`write(|w| ..)` method takes [rtc_icsr::W](rtc_icsr::W) writer structure"]
impl crate::Writable for RTC_ICSR {}
#[doc = "This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be globally protected, or each bit of this register can be individually protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod rtc_icsr;
#[doc = "This register must be written in initialization mode only. The initialization must be performed in two separate write accesses. Refer to Calendar initialization and configuration on page1830. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_prer](rtc_prer) module"]
pub type RTC_PRER = crate::Reg<u32, _RTC_PRER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_PRER;
#[doc = "`read()` method returns [rtc_prer::R](rtc_prer::R) reader structure"]
impl crate::Readable for RTC_PRER {}
#[doc = "`write(|w| ..)` method takes [rtc_prer::W](rtc_prer::W) writer structure"]
impl crate::Writable for RTC_PRER {}
#[doc = "This register must be written in initialization mode only. The initialization must be performed in two separate write accesses. Refer to Calendar initialization and configuration on page1830. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod rtc_prer;
#[doc = "This register can be written only when WUTWF is set to 1 in RTC_ICSR. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_wutr](rtc_wutr) module"]
pub type RTC_WUTR = crate::Reg<u32, _RTC_WUTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_WUTR;
#[doc = "`read()` method returns [rtc_wutr::R](rtc_wutr::R) reader structure"]
impl crate::Readable for RTC_WUTR {}
#[doc = "`write(|w| ..)` method takes [rtc_wutr::W](rtc_wutr::W) writer structure"]
impl crate::Writable for RTC_WUTR {}
#[doc = "This register can be written only when WUTWF is set to 1 in RTC_ICSR. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod rtc_wutr;
#[doc = "This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be globally protected, or each bit of this register can be individually protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cr](rtc_cr) module"]
pub type RTC_CR = crate::Reg<u32, _RTC_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CR;
#[doc = "`read()` method returns [rtc_cr::R](rtc_cr::R) reader structure"]
impl crate::Readable for RTC_CR {}
#[doc = "`write(|w| ..)` method takes [rtc_cr::W](rtc_cr::W) writer structure"]
impl crate::Writable for RTC_CR {}
#[doc = "This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be globally protected, or each bit of this register can be individually protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod rtc_cr;
#[doc = "This register can be written only when the APB access is secure.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_smcr](rtc_smcr) module"]
pub type RTC_SMCR = crate::Reg<u32, _RTC_SMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_SMCR;
#[doc = "`read()` method returns [rtc_smcr::R](rtc_smcr::R) reader structure"]
impl crate::Readable for RTC_SMCR {}
#[doc = "`write(|w| ..)` method takes [rtc_smcr::W](rtc_smcr::W) writer structure"]
impl crate::Writable for RTC_SMCR {}
#[doc = "This register can be written only when the APB access is secure."]
pub mod rtc_smcr;
#[doc = "RTC write protection register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_wpr](rtc_wpr) module"]
pub type RTC_WPR = crate::Reg<u32, _RTC_WPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_WPR;
#[doc = "`write(|w| ..)` method takes [rtc_wpr::W](rtc_wpr::W) writer structure"]
impl crate::Writable for RTC_WPR {}
#[doc = "RTC write protection register"]
pub mod rtc_wpr;
#[doc = "This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_calr](rtc_calr) module"]
pub type RTC_CALR = crate::Reg<u32, _RTC_CALR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CALR;
#[doc = "`read()` method returns [rtc_calr::R](rtc_calr::R) reader structure"]
impl crate::Readable for RTC_CALR {}
#[doc = "`write(|w| ..)` method takes [rtc_calr::W](rtc_calr::W) writer structure"]
impl crate::Writable for RTC_CALR {}
#[doc = "This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod rtc_calr;
#[doc = "This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_shiftr](rtc_shiftr) module"]
pub type RTC_SHIFTR = crate::Reg<u32, _RTC_SHIFTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_SHIFTR;
#[doc = "`write(|w| ..)` method takes [rtc_shiftr::W](rtc_shiftr::W) writer structure"]
impl crate::Writable for RTC_SHIFTR {}
#[doc = "This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod rtc_shiftr;
#[doc = "The content of this register is valid only when TSF is set to 1 in RTC_SR. It is cleared when TSF bit is reset. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_tstr](rtc_tstr) module"]
pub type RTC_TSTR = crate::Reg<u32, _RTC_TSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_TSTR;
#[doc = "`read()` method returns [rtc_tstr::R](rtc_tstr::R) reader structure"]
impl crate::Readable for RTC_TSTR {}
#[doc = "The content of this register is valid only when TSF is set to 1 in RTC_SR. It is cleared when TSF bit is reset. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod rtc_tstr;
#[doc = "The content of this register is valid only when TSF is set to 1 in RTC_SR. It is cleared when TSF bit is reset. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_tsdr](rtc_tsdr) module"]
pub type RTC_TSDR = crate::Reg<u32, _RTC_TSDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_TSDR;
#[doc = "`read()` method returns [rtc_tsdr::R](rtc_tsdr::R) reader structure"]
impl crate::Readable for RTC_TSDR {}
#[doc = "The content of this register is valid only when TSF is set to 1 in RTC_SR. It is cleared when TSF bit is reset. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod rtc_tsdr;
#[doc = "The content of this register is valid only when TSF is set to 1 in RTC_SR. It is cleared when the TSF bit is reset. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_tsssr](rtc_tsssr) module"]
pub type RTC_TSSSR = crate::Reg<u32, _RTC_TSSSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_TSSSR;
#[doc = "`read()` method returns [rtc_tsssr::R](rtc_tsssr::R) reader structure"]
impl crate::Readable for RTC_TSSSR {}
#[doc = "The content of this register is valid only when TSF is set to 1 in RTC_SR. It is cleared when the TSF bit is reset. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod rtc_tsssr;
#[doc = "This register can be written only when ALRAWF is set to 1 in RTC_ICSR, or in initialization mode. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_alrmar](rtc_alrmar) module"]
pub type RTC_ALRMAR = crate::Reg<u32, _RTC_ALRMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_ALRMAR;
#[doc = "`read()` method returns [rtc_alrmar::R](rtc_alrmar::R) reader structure"]
impl crate::Readable for RTC_ALRMAR {}
#[doc = "`write(|w| ..)` method takes [rtc_alrmar::W](rtc_alrmar::W) writer structure"]
impl crate::Writable for RTC_ALRMAR {}
#[doc = "This register can be written only when ALRAWF is set to 1 in RTC_ICSR, or in initialization mode. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod rtc_alrmar;
#[doc = "This register can be written only when ALRAWF is set to 1 in RTC_ICSR, or in initialization mode. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_alrmassr](rtc_alrmassr) module"]
pub type RTC_ALRMASSR = crate::Reg<u32, _RTC_ALRMASSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_ALRMASSR;
#[doc = "`read()` method returns [rtc_alrmassr::R](rtc_alrmassr::R) reader structure"]
impl crate::Readable for RTC_ALRMASSR {}
#[doc = "`write(|w| ..)` method takes [rtc_alrmassr::W](rtc_alrmassr::W) writer structure"]
impl crate::Writable for RTC_ALRMASSR {}
#[doc = "This register can be written only when ALRAWF is set to 1 in RTC_ICSR, or in initialization mode. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod rtc_alrmassr;
#[doc = "This register can be written only when ALRBWF is set to 1 in RTC_ICSR, or in initialization mode. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_alrmbr](rtc_alrmbr) module"]
pub type RTC_ALRMBR = crate::Reg<u32, _RTC_ALRMBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_ALRMBR;
#[doc = "`read()` method returns [rtc_alrmbr::R](rtc_alrmbr::R) reader structure"]
impl crate::Readable for RTC_ALRMBR {}
#[doc = "`write(|w| ..)` method takes [rtc_alrmbr::W](rtc_alrmbr::W) writer structure"]
impl crate::Writable for RTC_ALRMBR {}
#[doc = "This register can be written only when ALRBWF is set to 1 in RTC_ICSR, or in initialization mode. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod rtc_alrmbr;
#[doc = "This register can be written only when ALRBE is reset in RTC_CR register, or in initialization mode. This register is write protected.The write access procedure is described in Section: RTC register write protection. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_alrmbssr](rtc_alrmbssr) module"]
pub type RTC_ALRMBSSR = crate::Reg<u32, _RTC_ALRMBSSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_ALRMBSSR;
#[doc = "`read()` method returns [rtc_alrmbssr::R](rtc_alrmbssr::R) reader structure"]
impl crate::Readable for RTC_ALRMBSSR {}
#[doc = "`write(|w| ..)` method takes [rtc_alrmbssr::W](rtc_alrmbssr::W) writer structure"]
impl crate::Writable for RTC_ALRMBSSR {}
#[doc = "This register can be written only when ALRBE is reset in RTC_CR register, or in initialization mode. This register is write protected.The write access procedure is described in Section: RTC register write protection. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod rtc_alrmbssr;
#[doc = "This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_sr](rtc_sr) module"]
pub type RTC_SR = crate::Reg<u32, _RTC_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_SR;
#[doc = "`read()` method returns [rtc_sr::R](rtc_sr::R) reader structure"]
impl crate::Readable for RTC_SR {}
#[doc = "This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod rtc_sr;
#[doc = "RTC non-secure masked interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_misr](rtc_misr) module"]
pub type RTC_MISR = crate::Reg<u32, _RTC_MISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_MISR;
#[doc = "`read()` method returns [rtc_misr::R](rtc_misr::R) reader structure"]
impl crate::Readable for RTC_MISR {}
#[doc = "RTC non-secure masked interrupt status register"]
pub mod rtc_misr;
#[doc = "This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_smisr](rtc_smisr) module"]
pub type RTC_SMISR = crate::Reg<u32, _RTC_SMISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_SMISR;
#[doc = "`read()` method returns [rtc_smisr::R](rtc_smisr::R) reader structure"]
impl crate::Readable for RTC_SMISR {}
#[doc = "This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod rtc_smisr;
#[doc = "This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_scr](rtc_scr) module"]
pub type RTC_SCR = crate::Reg<u32, _RTC_SCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_SCR;
#[doc = "`write(|w| ..)` method takes [rtc_scr::W](rtc_scr::W) writer structure"]
impl crate::Writable for RTC_SCR {}
#[doc = "This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod rtc_scr;
#[doc = "RTC configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cfgr](rtc_cfgr) module"]
pub type RTC_CFGR = crate::Reg<u32, _RTC_CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CFGR;
#[doc = "`read()` method returns [rtc_cfgr::R](rtc_cfgr::R) reader structure"]
impl crate::Readable for RTC_CFGR {}
#[doc = "`write(|w| ..)` method takes [rtc_cfgr::W](rtc_cfgr::W) writer structure"]
impl crate::Writable for RTC_CFGR {}
#[doc = "RTC configuration register"]
pub mod rtc_cfgr;
#[doc = "RTC hardware configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_hwcfgr](rtc_hwcfgr) module"]
pub type RTC_HWCFGR = crate::Reg<u32, _RTC_HWCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_HWCFGR;
#[doc = "`read()` method returns [rtc_hwcfgr::R](rtc_hwcfgr::R) reader structure"]
impl crate::Readable for RTC_HWCFGR {}
#[doc = "RTC hardware configuration register"]
pub mod rtc_hwcfgr;
#[doc = "RTC version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_verr](rtc_verr) module"]
pub type RTC_VERR = crate::Reg<u32, _RTC_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_VERR;
#[doc = "`read()` method returns [rtc_verr::R](rtc_verr::R) reader structure"]
impl crate::Readable for RTC_VERR {}
#[doc = "RTC version register"]
pub mod rtc_verr;
#[doc = "RTC identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_ipidr](rtc_ipidr) module"]
pub type RTC_IPIDR = crate::Reg<u32, _RTC_IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IPIDR;
#[doc = "`read()` method returns [rtc_ipidr::R](rtc_ipidr::R) reader structure"]
impl crate::Readable for RTC_IPIDR {}
#[doc = "RTC identification register"]
pub mod rtc_ipidr;
#[doc = "RTC size identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_sidr](rtc_sidr) module"]
pub type RTC_SIDR = crate::Reg<u32, _RTC_SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_SIDR;
#[doc = "`read()` method returns [rtc_sidr::R](rtc_sidr::R) reader structure"]
impl crate::Readable for RTC_SIDR {}
#[doc = "RTC size identification register"]
pub mod rtc_sidr;
