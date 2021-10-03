#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - The RTC_TR is the calendar time shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page1830 and Reading the calendar on page1831. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub rtc_tr: crate::Reg<rtc_tr::RTC_TR_SPEC>,
    #[doc = "0x04 - The RTC_DR is the calendar date shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page1830 and Reading the calendar on page1831. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub rtc_dr: crate::Reg<rtc_dr::RTC_DR_SPEC>,
    #[doc = "0x08 - RTC sub second register"]
    pub rtc_ssr: crate::Reg<rtc_ssr::RTC_SSR_SPEC>,
    #[doc = "0x0c - This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be globally protected, or each bit of this register can be individually protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub rtc_icsr: crate::Reg<rtc_icsr::RTC_ICSR_SPEC>,
    #[doc = "0x10 - This register must be written in initialization mode only. The initialization must be performed in two separate write accesses. Refer to Calendar initialization and configuration on page1830. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub rtc_prer: crate::Reg<rtc_prer::RTC_PRER_SPEC>,
    #[doc = "0x14 - This register can be written only when WUTWF is set to 1 in RTC_ICSR. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub rtc_wutr: crate::Reg<rtc_wutr::RTC_WUTR_SPEC>,
    #[doc = "0x18 - This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be globally protected, or each bit of this register can be individually protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub rtc_cr: crate::Reg<rtc_cr::RTC_CR_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - This register can be written only when the APB access is secure."]
    pub rtc_smcr: crate::Reg<rtc_smcr::RTC_SMCR_SPEC>,
    #[doc = "0x24 - RTC write protection register"]
    pub rtc_wpr: crate::Reg<rtc_wpr::RTC_WPR_SPEC>,
    #[doc = "0x28 - This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub rtc_calr: crate::Reg<rtc_calr::RTC_CALR_SPEC>,
    #[doc = "0x2c - This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub rtc_shiftr: crate::Reg<rtc_shiftr::RTC_SHIFTR_SPEC>,
    #[doc = "0x30 - The content of this register is valid only when TSF is set to 1 in RTC_SR. It is cleared when TSF bit is reset. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub rtc_tstr: crate::Reg<rtc_tstr::RTC_TSTR_SPEC>,
    #[doc = "0x34 - The content of this register is valid only when TSF is set to 1 in RTC_SR. It is cleared when TSF bit is reset. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub rtc_tsdr: crate::Reg<rtc_tsdr::RTC_TSDR_SPEC>,
    #[doc = "0x38 - The content of this register is valid only when TSF is set to 1 in RTC_SR. It is cleared when the TSF bit is reset. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub rtc_tsssr: crate::Reg<rtc_tsssr::RTC_TSSSR_SPEC>,
    _reserved14: [u8; 0x04],
    #[doc = "0x40 - This register can be written only when ALRAWF is set to 1 in RTC_ICSR, or in initialization mode. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub rtc_alrmar: crate::Reg<rtc_alrmar::RTC_ALRMAR_SPEC>,
    #[doc = "0x44 - This register can be written only when ALRAWF is set to 1 in RTC_ICSR, or in initialization mode. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub rtc_alrmassr: crate::Reg<rtc_alrmassr::RTC_ALRMASSR_SPEC>,
    #[doc = "0x48 - This register can be written only when ALRBWF is set to 1 in RTC_ICSR, or in initialization mode. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub rtc_alrmbr: crate::Reg<rtc_alrmbr::RTC_ALRMBR_SPEC>,
    #[doc = "0x4c - This register can be written only when ALRBE is reset in RTC_CR register, or in initialization mode. This register is write protected.The write access procedure is described in Section: RTC register write protection. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub rtc_alrmbssr: crate::Reg<rtc_alrmbssr::RTC_ALRMBSSR_SPEC>,
    #[doc = "0x50 - This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub rtc_sr: crate::Reg<rtc_sr::RTC_SR_SPEC>,
    #[doc = "0x54 - RTC non-secure masked interrupt status register"]
    pub rtc_misr: crate::Reg<rtc_misr::RTC_MISR_SPEC>,
    #[doc = "0x58 - This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub rtc_smisr: crate::Reg<rtc_smisr::RTC_SMISR_SPEC>,
    #[doc = "0x5c - This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    pub rtc_scr: crate::Reg<rtc_scr::RTC_SCR_SPEC>,
    #[doc = "0x60 - RTC configuration register"]
    pub rtc_cfgr: crate::Reg<rtc_cfgr::RTC_CFGR_SPEC>,
    _reserved23: [u8; 0x038c],
    #[doc = "0x3f0 - RTC hardware configuration register"]
    pub rtc_hwcfgr: crate::Reg<rtc_hwcfgr::RTC_HWCFGR_SPEC>,
    #[doc = "0x3f4 - RTC version register"]
    pub rtc_verr: crate::Reg<rtc_verr::RTC_VERR_SPEC>,
    #[doc = "0x3f8 - RTC identification register"]
    pub rtc_ipidr: crate::Reg<rtc_ipidr::RTC_IPIDR_SPEC>,
    #[doc = "0x3fc - RTC size identification register"]
    pub rtc_sidr: crate::Reg<rtc_sidr::RTC_SIDR_SPEC>,
}
#[doc = "RTC_TR register accessor: an alias for `Reg<RTC_TR_SPEC>`"]
pub type RTC_TR = crate::Reg<rtc_tr::RTC_TR_SPEC>;
#[doc = "The RTC_TR is the calendar time shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page1830 and Reading the calendar on page1831. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod rtc_tr;
#[doc = "RTC_DR register accessor: an alias for `Reg<RTC_DR_SPEC>`"]
pub type RTC_DR = crate::Reg<rtc_dr::RTC_DR_SPEC>;
#[doc = "The RTC_DR is the calendar date shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page1830 and Reading the calendar on page1831. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod rtc_dr;
#[doc = "RTC_SSR register accessor: an alias for `Reg<RTC_SSR_SPEC>`"]
pub type RTC_SSR = crate::Reg<rtc_ssr::RTC_SSR_SPEC>;
#[doc = "RTC sub second register"]
pub mod rtc_ssr;
#[doc = "RTC_ICSR register accessor: an alias for `Reg<RTC_ICSR_SPEC>`"]
pub type RTC_ICSR = crate::Reg<rtc_icsr::RTC_ICSR_SPEC>;
#[doc = "This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be globally protected, or each bit of this register can be individually protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod rtc_icsr;
#[doc = "RTC_PRER register accessor: an alias for `Reg<RTC_PRER_SPEC>`"]
pub type RTC_PRER = crate::Reg<rtc_prer::RTC_PRER_SPEC>;
#[doc = "This register must be written in initialization mode only. The initialization must be performed in two separate write accesses. Refer to Calendar initialization and configuration on page1830. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod rtc_prer;
#[doc = "RTC_WUTR register accessor: an alias for `Reg<RTC_WUTR_SPEC>`"]
pub type RTC_WUTR = crate::Reg<rtc_wutr::RTC_WUTR_SPEC>;
#[doc = "This register can be written only when WUTWF is set to 1 in RTC_ICSR. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod rtc_wutr;
#[doc = "RTC_CR register accessor: an alias for `Reg<RTC_CR_SPEC>`"]
pub type RTC_CR = crate::Reg<rtc_cr::RTC_CR_SPEC>;
#[doc = "This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be globally protected, or each bit of this register can be individually protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod rtc_cr;
#[doc = "RTC_SMCR register accessor: an alias for `Reg<RTC_SMCR_SPEC>`"]
pub type RTC_SMCR = crate::Reg<rtc_smcr::RTC_SMCR_SPEC>;
#[doc = "This register can be written only when the APB access is secure."]
pub mod rtc_smcr;
#[doc = "RTC_WPR register accessor: an alias for `Reg<RTC_WPR_SPEC>`"]
pub type RTC_WPR = crate::Reg<rtc_wpr::RTC_WPR_SPEC>;
#[doc = "RTC write protection register"]
pub mod rtc_wpr;
#[doc = "RTC_CALR register accessor: an alias for `Reg<RTC_CALR_SPEC>`"]
pub type RTC_CALR = crate::Reg<rtc_calr::RTC_CALR_SPEC>;
#[doc = "This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod rtc_calr;
#[doc = "RTC_SHIFTR register accessor: an alias for `Reg<RTC_SHIFTR_SPEC>`"]
pub type RTC_SHIFTR = crate::Reg<rtc_shiftr::RTC_SHIFTR_SPEC>;
#[doc = "This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod rtc_shiftr;
#[doc = "RTC_TSTR register accessor: an alias for `Reg<RTC_TSTR_SPEC>`"]
pub type RTC_TSTR = crate::Reg<rtc_tstr::RTC_TSTR_SPEC>;
#[doc = "The content of this register is valid only when TSF is set to 1 in RTC_SR. It is cleared when TSF bit is reset. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod rtc_tstr;
#[doc = "RTC_TSDR register accessor: an alias for `Reg<RTC_TSDR_SPEC>`"]
pub type RTC_TSDR = crate::Reg<rtc_tsdr::RTC_TSDR_SPEC>;
#[doc = "The content of this register is valid only when TSF is set to 1 in RTC_SR. It is cleared when TSF bit is reset. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod rtc_tsdr;
#[doc = "RTC_TSSSR register accessor: an alias for `Reg<RTC_TSSSR_SPEC>`"]
pub type RTC_TSSSR = crate::Reg<rtc_tsssr::RTC_TSSSR_SPEC>;
#[doc = "The content of this register is valid only when TSF is set to 1 in RTC_SR. It is cleared when the TSF bit is reset. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod rtc_tsssr;
#[doc = "RTC_ALRMAR register accessor: an alias for `Reg<RTC_ALRMAR_SPEC>`"]
pub type RTC_ALRMAR = crate::Reg<rtc_alrmar::RTC_ALRMAR_SPEC>;
#[doc = "This register can be written only when ALRAWF is set to 1 in RTC_ICSR, or in initialization mode. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod rtc_alrmar;
#[doc = "RTC_ALRMASSR register accessor: an alias for `Reg<RTC_ALRMASSR_SPEC>`"]
pub type RTC_ALRMASSR = crate::Reg<rtc_alrmassr::RTC_ALRMASSR_SPEC>;
#[doc = "This register can be written only when ALRAWF is set to 1 in RTC_ICSR, or in initialization mode. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod rtc_alrmassr;
#[doc = "RTC_ALRMBR register accessor: an alias for `Reg<RTC_ALRMBR_SPEC>`"]
pub type RTC_ALRMBR = crate::Reg<rtc_alrmbr::RTC_ALRMBR_SPEC>;
#[doc = "This register can be written only when ALRBWF is set to 1 in RTC_ICSR, or in initialization mode. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod rtc_alrmbr;
#[doc = "RTC_ALRMBSSR register accessor: an alias for `Reg<RTC_ALRMBSSR_SPEC>`"]
pub type RTC_ALRMBSSR = crate::Reg<rtc_alrmbssr::RTC_ALRMBSSR_SPEC>;
#[doc = "This register can be written only when ALRBE is reset in RTC_CR register, or in initialization mode. This register is write protected.The write access procedure is described in Section: RTC register write protection. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod rtc_alrmbssr;
#[doc = "RTC_SR register accessor: an alias for `Reg<RTC_SR_SPEC>`"]
pub type RTC_SR = crate::Reg<rtc_sr::RTC_SR_SPEC>;
#[doc = "This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod rtc_sr;
#[doc = "RTC_MISR register accessor: an alias for `Reg<RTC_MISR_SPEC>`"]
pub type RTC_MISR = crate::Reg<rtc_misr::RTC_MISR_SPEC>;
#[doc = "RTC non-secure masked interrupt status register"]
pub mod rtc_misr;
#[doc = "RTC_SMISR register accessor: an alias for `Reg<RTC_SMISR_SPEC>`"]
pub type RTC_SMISR = crate::Reg<rtc_smisr::RTC_SMISR_SPEC>;
#[doc = "This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod rtc_smisr;
#[doc = "RTC_SCR register accessor: an alias for `Reg<RTC_SCR_SPEC>`"]
pub type RTC_SCR = crate::Reg<rtc_scr::RTC_SCR_SPEC>;
#[doc = "This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod rtc_scr;
#[doc = "RTC_CFGR register accessor: an alias for `Reg<RTC_CFGR_SPEC>`"]
pub type RTC_CFGR = crate::Reg<rtc_cfgr::RTC_CFGR_SPEC>;
#[doc = "RTC configuration register"]
pub mod rtc_cfgr;
#[doc = "RTC_HWCFGR register accessor: an alias for `Reg<RTC_HWCFGR_SPEC>`"]
pub type RTC_HWCFGR = crate::Reg<rtc_hwcfgr::RTC_HWCFGR_SPEC>;
#[doc = "RTC hardware configuration register"]
pub mod rtc_hwcfgr;
#[doc = "RTC_VERR register accessor: an alias for `Reg<RTC_VERR_SPEC>`"]
pub type RTC_VERR = crate::Reg<rtc_verr::RTC_VERR_SPEC>;
#[doc = "RTC version register"]
pub mod rtc_verr;
#[doc = "RTC_IPIDR register accessor: an alias for `Reg<RTC_IPIDR_SPEC>`"]
pub type RTC_IPIDR = crate::Reg<rtc_ipidr::RTC_IPIDR_SPEC>;
#[doc = "RTC identification register"]
pub mod rtc_ipidr;
#[doc = "RTC_SIDR register accessor: an alias for `Reg<RTC_SIDR_SPEC>`"]
pub type RTC_SIDR = crate::Reg<rtc_sidr::RTC_SIDR_SPEC>;
#[doc = "RTC size identification register"]
pub mod rtc_sidr;
