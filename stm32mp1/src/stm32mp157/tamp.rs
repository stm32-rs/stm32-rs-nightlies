#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
    pub tamp_cr1: TAMP_CR1,
    #[doc = "0x04 - This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
    pub tamp_cr2: TAMP_CR2,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
    pub tamp_fltcr: TAMP_FLTCR,
    #[doc = "0x10 - This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
    pub tamp_atcr1: TAMP_ATCR1,
    #[doc = "0x14 - This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
    pub tamp_atseedr: TAMP_ATSEEDR,
    #[doc = "0x18 - This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
    pub tamp_ator: TAMP_ATOR,
    _reserved6: [u8; 4usize],
    #[doc = "0x20 - This register can be written only when the APB access is secure."]
    pub tamp_smcr: TAMP_SMCR,
    _reserved7: [u8; 8usize],
    #[doc = "0x2c - This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
    pub tamp_ier: TAMP_IER,
    #[doc = "0x30 - This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
    pub tamp_sr: TAMP_SR,
    #[doc = "0x34 - TAMP non-secure masked interrupt status register"]
    pub tamp_misr: TAMP_MISR,
    #[doc = "0x38 - TAMP secure masked interrupt status register"]
    pub tamp_smisr: TAMP_SMISR,
    #[doc = "0x3c - TAMP status clear register"]
    pub tamp_scr: TAMP_SCR,
    #[doc = "0x40 - TAMP monotonic counter register"]
    pub tamp_countr: TAMP_COUNTR,
    _reserved13: [u8; 12usize],
    #[doc = "0x50 - TAMP configuration register"]
    pub tamp_cfgr: TAMP_CFGR,
    _reserved14: [u8; 172usize],
    #[doc = "0x100 - TAMP backup 0 register"]
    pub tamp_bkp0r: TAMP_BKP0R,
    #[doc = "0x104 - TAMP backup 1 register"]
    pub tamp_bkp1r: TAMP_BKP1R,
    #[doc = "0x108 - TAMP backup 2 register"]
    pub tamp_bkp2r: TAMP_BKP2R,
    #[doc = "0x10c - TAMP backup 3 register"]
    pub tamp_bkp3r: TAMP_BKP3R,
    #[doc = "0x110 - TAMP backup 4 register"]
    pub tamp_bkp4r: TAMP_BKP4R,
    #[doc = "0x114 - TAMP backup 5 register"]
    pub tamp_bkp5r: TAMP_BKP5R,
    #[doc = "0x118 - TAMP backup 6 register"]
    pub tamp_bkp6r: TAMP_BKP6R,
    #[doc = "0x11c - TAMP backup 7 register"]
    pub tamp_bkp7r: TAMP_BKP7R,
    #[doc = "0x120 - TAMP backup 8 register"]
    pub tamp_bkp8r: TAMP_BKP8R,
    #[doc = "0x124 - TAMP backup 9 register"]
    pub tamp_bkp9r: TAMP_BKP9R,
    #[doc = "0x128 - TAMP backup 10 register"]
    pub tamp_bkp10r: TAMP_BKP10R,
    #[doc = "0x12c - TAMP backup 11 register"]
    pub tamp_bkp11r: TAMP_BKP11R,
    #[doc = "0x130 - TAMP backup 12 register"]
    pub tamp_bkp12r: TAMP_BKP12R,
    #[doc = "0x134 - TAMP backup 13 register"]
    pub tamp_bkp13r: TAMP_BKP13R,
    #[doc = "0x138 - TAMP backup 14 register"]
    pub tamp_bkp14r: TAMP_BKP14R,
    #[doc = "0x13c - TAMP backup 15 register"]
    pub tamp_bkp15r: TAMP_BKP15R,
    #[doc = "0x140 - TAMP backup 16 register"]
    pub tamp_bkp16r: TAMP_BKP16R,
    #[doc = "0x144 - TAMP backup 17 register"]
    pub tamp_bkp17r: TAMP_BKP17R,
    #[doc = "0x148 - TAMP backup 18 register"]
    pub tamp_bkp18r: TAMP_BKP18R,
    #[doc = "0x14c - TAMP backup 19 register"]
    pub tamp_bkp19r: TAMP_BKP19R,
    #[doc = "0x150 - TAMP backup 20 register"]
    pub tamp_bkp20r: TAMP_BKP20R,
    #[doc = "0x154 - TAMP backup 21 register"]
    pub tamp_bkp21r: TAMP_BKP21R,
    #[doc = "0x158 - TAMP backup 22 register"]
    pub tamp_bkp22r: TAMP_BKP22R,
    #[doc = "0x15c - TAMP backup 23 register"]
    pub tamp_bkp23r: TAMP_BKP23R,
    #[doc = "0x160 - TAMP backup 24 register"]
    pub tamp_bkp24r: TAMP_BKP24R,
    #[doc = "0x164 - TAMP backup 25 register"]
    pub tamp_bkp25r: TAMP_BKP25R,
    #[doc = "0x168 - TAMP backup 26 register"]
    pub tamp_bkp26r: TAMP_BKP26R,
    #[doc = "0x16c - TAMP backup 27 register"]
    pub tamp_bkp27r: TAMP_BKP27R,
    #[doc = "0x170 - TAMP backup 28 register"]
    pub tamp_bkp28r: TAMP_BKP28R,
    #[doc = "0x174 - TAMP backup 29 register"]
    pub tamp_bkp29r: TAMP_BKP29R,
    #[doc = "0x178 - TAMP backup 30 register"]
    pub tamp_bkp30r: TAMP_BKP30R,
    #[doc = "0x17c - TAMP backup 31 register"]
    pub tamp_bkp31r: TAMP_BKP31R,
    _reserved46: [u8; 620usize],
    #[doc = "0x3ec - TAMP hardware configuration register 2"]
    pub tamp_hwcfgr2: TAMP_HWCFGR2,
    #[doc = "0x3f0 - TAMP hardware configuration register 1"]
    pub tamp_hwcfgr1: TAMP_HWCFGR1,
    #[doc = "0x3f4 - TAMP version register"]
    pub tamp_verr: TAMP_VERR,
    #[doc = "0x3f8 - TAMP identification register"]
    pub tamp_ipidr: TAMP_IPIDR,
    #[doc = "0x3fc - TAMP size identification register"]
    pub tamp_sidr: TAMP_SIDR,
}
#[doc = "This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_cr1](tamp_cr1) module"]
pub type TAMP_CR1 = crate::Reg<u32, _TAMP_CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_CR1;
#[doc = "`read()` method returns [tamp_cr1::R](tamp_cr1::R) reader structure"]
impl crate::Readable for TAMP_CR1 {}
#[doc = "`write(|w| ..)` method takes [tamp_cr1::W](tamp_cr1::W) writer structure"]
impl crate::Writable for TAMP_CR1 {}
#[doc = "This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
pub mod tamp_cr1;
#[doc = "This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_cr2](tamp_cr2) module"]
pub type TAMP_CR2 = crate::Reg<u32, _TAMP_CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_CR2;
#[doc = "`read()` method returns [tamp_cr2::R](tamp_cr2::R) reader structure"]
impl crate::Readable for TAMP_CR2 {}
#[doc = "`write(|w| ..)` method takes [tamp_cr2::W](tamp_cr2::W) writer structure"]
impl crate::Writable for TAMP_CR2 {}
#[doc = "This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
pub mod tamp_cr2;
#[doc = "This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_fltcr](tamp_fltcr) module"]
pub type TAMP_FLTCR = crate::Reg<u32, _TAMP_FLTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_FLTCR;
#[doc = "`read()` method returns [tamp_fltcr::R](tamp_fltcr::R) reader structure"]
impl crate::Readable for TAMP_FLTCR {}
#[doc = "`write(|w| ..)` method takes [tamp_fltcr::W](tamp_fltcr::W) writer structure"]
impl crate::Writable for TAMP_FLTCR {}
#[doc = "This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
pub mod tamp_fltcr;
#[doc = "This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_atcr1](tamp_atcr1) module"]
pub type TAMP_ATCR1 = crate::Reg<u32, _TAMP_ATCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_ATCR1;
#[doc = "`read()` method returns [tamp_atcr1::R](tamp_atcr1::R) reader structure"]
impl crate::Readable for TAMP_ATCR1 {}
#[doc = "`write(|w| ..)` method takes [tamp_atcr1::W](tamp_atcr1::W) writer structure"]
impl crate::Writable for TAMP_ATCR1 {}
#[doc = "This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
pub mod tamp_atcr1;
#[doc = "This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_atseedr](tamp_atseedr) module"]
pub type TAMP_ATSEEDR = crate::Reg<u32, _TAMP_ATSEEDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_ATSEEDR;
#[doc = "`write(|w| ..)` method takes [tamp_atseedr::W](tamp_atseedr::W) writer structure"]
impl crate::Writable for TAMP_ATSEEDR {}
#[doc = "This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
pub mod tamp_atseedr;
#[doc = "This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_ator](tamp_ator) module"]
pub type TAMP_ATOR = crate::Reg<u32, _TAMP_ATOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_ATOR;
#[doc = "`read()` method returns [tamp_ator::R](tamp_ator::R) reader structure"]
impl crate::Readable for TAMP_ATOR {}
#[doc = "This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
pub mod tamp_ator;
#[doc = "This register can be written only when the APB access is secure.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_smcr](tamp_smcr) module"]
pub type TAMP_SMCR = crate::Reg<u32, _TAMP_SMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_SMCR;
#[doc = "`read()` method returns [tamp_smcr::R](tamp_smcr::R) reader structure"]
impl crate::Readable for TAMP_SMCR {}
#[doc = "`write(|w| ..)` method takes [tamp_smcr::W](tamp_smcr::W) writer structure"]
impl crate::Writable for TAMP_SMCR {}
#[doc = "This register can be written only when the APB access is secure."]
pub mod tamp_smcr;
#[doc = "This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_ier](tamp_ier) module"]
pub type TAMP_IER = crate::Reg<u32, _TAMP_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_IER;
#[doc = "`read()` method returns [tamp_ier::R](tamp_ier::R) reader structure"]
impl crate::Readable for TAMP_IER {}
#[doc = "`write(|w| ..)` method takes [tamp_ier::W](tamp_ier::W) writer structure"]
impl crate::Writable for TAMP_IER {}
#[doc = "This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
pub mod tamp_ier;
#[doc = "This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_sr](tamp_sr) module"]
pub type TAMP_SR = crate::Reg<u32, _TAMP_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_SR;
#[doc = "`read()` method returns [tamp_sr::R](tamp_sr::R) reader structure"]
impl crate::Readable for TAMP_SR {}
#[doc = "This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
pub mod tamp_sr;
#[doc = "TAMP non-secure masked interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_misr](tamp_misr) module"]
pub type TAMP_MISR = crate::Reg<u32, _TAMP_MISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_MISR;
#[doc = "`read()` method returns [tamp_misr::R](tamp_misr::R) reader structure"]
impl crate::Readable for TAMP_MISR {}
#[doc = "TAMP non-secure masked interrupt status register"]
pub mod tamp_misr;
#[doc = "TAMP secure masked interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_smisr](tamp_smisr) module"]
pub type TAMP_SMISR = crate::Reg<u32, _TAMP_SMISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_SMISR;
#[doc = "`read()` method returns [tamp_smisr::R](tamp_smisr::R) reader structure"]
impl crate::Readable for TAMP_SMISR {}
#[doc = "TAMP secure masked interrupt status register"]
pub mod tamp_smisr;
#[doc = "TAMP status clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_scr](tamp_scr) module"]
pub type TAMP_SCR = crate::Reg<u32, _TAMP_SCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_SCR;
#[doc = "`write(|w| ..)` method takes [tamp_scr::W](tamp_scr::W) writer structure"]
impl crate::Writable for TAMP_SCR {}
#[doc = "TAMP status clear register"]
pub mod tamp_scr;
#[doc = "TAMP monotonic counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_countr](tamp_countr) module"]
pub type TAMP_COUNTR = crate::Reg<u32, _TAMP_COUNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_COUNTR;
#[doc = "`read()` method returns [tamp_countr::R](tamp_countr::R) reader structure"]
impl crate::Readable for TAMP_COUNTR {}
#[doc = "TAMP monotonic counter register"]
pub mod tamp_countr;
#[doc = "TAMP configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_cfgr](tamp_cfgr) module"]
pub type TAMP_CFGR = crate::Reg<u32, _TAMP_CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_CFGR;
#[doc = "`read()` method returns [tamp_cfgr::R](tamp_cfgr::R) reader structure"]
impl crate::Readable for TAMP_CFGR {}
#[doc = "`write(|w| ..)` method takes [tamp_cfgr::W](tamp_cfgr::W) writer structure"]
impl crate::Writable for TAMP_CFGR {}
#[doc = "TAMP configuration register"]
pub mod tamp_cfgr;
#[doc = "TAMP backup 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_bkp0r](tamp_bkp0r) module"]
pub type TAMP_BKP0R = crate::Reg<u32, _TAMP_BKP0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_BKP0R;
#[doc = "`read()` method returns [tamp_bkp0r::R](tamp_bkp0r::R) reader structure"]
impl crate::Readable for TAMP_BKP0R {}
#[doc = "`write(|w| ..)` method takes [tamp_bkp0r::W](tamp_bkp0r::W) writer structure"]
impl crate::Writable for TAMP_BKP0R {}
#[doc = "TAMP backup 0 register"]
pub mod tamp_bkp0r;
#[doc = "TAMP backup 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_bkp1r](tamp_bkp1r) module"]
pub type TAMP_BKP1R = crate::Reg<u32, _TAMP_BKP1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_BKP1R;
#[doc = "`read()` method returns [tamp_bkp1r::R](tamp_bkp1r::R) reader structure"]
impl crate::Readable for TAMP_BKP1R {}
#[doc = "`write(|w| ..)` method takes [tamp_bkp1r::W](tamp_bkp1r::W) writer structure"]
impl crate::Writable for TAMP_BKP1R {}
#[doc = "TAMP backup 1 register"]
pub mod tamp_bkp1r;
#[doc = "TAMP backup 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_bkp2r](tamp_bkp2r) module"]
pub type TAMP_BKP2R = crate::Reg<u32, _TAMP_BKP2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_BKP2R;
#[doc = "`read()` method returns [tamp_bkp2r::R](tamp_bkp2r::R) reader structure"]
impl crate::Readable for TAMP_BKP2R {}
#[doc = "`write(|w| ..)` method takes [tamp_bkp2r::W](tamp_bkp2r::W) writer structure"]
impl crate::Writable for TAMP_BKP2R {}
#[doc = "TAMP backup 2 register"]
pub mod tamp_bkp2r;
#[doc = "TAMP backup 3 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_bkp3r](tamp_bkp3r) module"]
pub type TAMP_BKP3R = crate::Reg<u32, _TAMP_BKP3R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_BKP3R;
#[doc = "`read()` method returns [tamp_bkp3r::R](tamp_bkp3r::R) reader structure"]
impl crate::Readable for TAMP_BKP3R {}
#[doc = "`write(|w| ..)` method takes [tamp_bkp3r::W](tamp_bkp3r::W) writer structure"]
impl crate::Writable for TAMP_BKP3R {}
#[doc = "TAMP backup 3 register"]
pub mod tamp_bkp3r;
#[doc = "TAMP backup 4 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_bkp4r](tamp_bkp4r) module"]
pub type TAMP_BKP4R = crate::Reg<u32, _TAMP_BKP4R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_BKP4R;
#[doc = "`read()` method returns [tamp_bkp4r::R](tamp_bkp4r::R) reader structure"]
impl crate::Readable for TAMP_BKP4R {}
#[doc = "`write(|w| ..)` method takes [tamp_bkp4r::W](tamp_bkp4r::W) writer structure"]
impl crate::Writable for TAMP_BKP4R {}
#[doc = "TAMP backup 4 register"]
pub mod tamp_bkp4r;
#[doc = "TAMP backup 5 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_bkp5r](tamp_bkp5r) module"]
pub type TAMP_BKP5R = crate::Reg<u32, _TAMP_BKP5R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_BKP5R;
#[doc = "`read()` method returns [tamp_bkp5r::R](tamp_bkp5r::R) reader structure"]
impl crate::Readable for TAMP_BKP5R {}
#[doc = "`write(|w| ..)` method takes [tamp_bkp5r::W](tamp_bkp5r::W) writer structure"]
impl crate::Writable for TAMP_BKP5R {}
#[doc = "TAMP backup 5 register"]
pub mod tamp_bkp5r;
#[doc = "TAMP backup 6 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_bkp6r](tamp_bkp6r) module"]
pub type TAMP_BKP6R = crate::Reg<u32, _TAMP_BKP6R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_BKP6R;
#[doc = "`read()` method returns [tamp_bkp6r::R](tamp_bkp6r::R) reader structure"]
impl crate::Readable for TAMP_BKP6R {}
#[doc = "`write(|w| ..)` method takes [tamp_bkp6r::W](tamp_bkp6r::W) writer structure"]
impl crate::Writable for TAMP_BKP6R {}
#[doc = "TAMP backup 6 register"]
pub mod tamp_bkp6r;
#[doc = "TAMP backup 7 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_bkp7r](tamp_bkp7r) module"]
pub type TAMP_BKP7R = crate::Reg<u32, _TAMP_BKP7R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_BKP7R;
#[doc = "`read()` method returns [tamp_bkp7r::R](tamp_bkp7r::R) reader structure"]
impl crate::Readable for TAMP_BKP7R {}
#[doc = "`write(|w| ..)` method takes [tamp_bkp7r::W](tamp_bkp7r::W) writer structure"]
impl crate::Writable for TAMP_BKP7R {}
#[doc = "TAMP backup 7 register"]
pub mod tamp_bkp7r;
#[doc = "TAMP backup 8 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_bkp8r](tamp_bkp8r) module"]
pub type TAMP_BKP8R = crate::Reg<u32, _TAMP_BKP8R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_BKP8R;
#[doc = "`read()` method returns [tamp_bkp8r::R](tamp_bkp8r::R) reader structure"]
impl crate::Readable for TAMP_BKP8R {}
#[doc = "`write(|w| ..)` method takes [tamp_bkp8r::W](tamp_bkp8r::W) writer structure"]
impl crate::Writable for TAMP_BKP8R {}
#[doc = "TAMP backup 8 register"]
pub mod tamp_bkp8r;
#[doc = "TAMP backup 9 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_bkp9r](tamp_bkp9r) module"]
pub type TAMP_BKP9R = crate::Reg<u32, _TAMP_BKP9R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_BKP9R;
#[doc = "`read()` method returns [tamp_bkp9r::R](tamp_bkp9r::R) reader structure"]
impl crate::Readable for TAMP_BKP9R {}
#[doc = "`write(|w| ..)` method takes [tamp_bkp9r::W](tamp_bkp9r::W) writer structure"]
impl crate::Writable for TAMP_BKP9R {}
#[doc = "TAMP backup 9 register"]
pub mod tamp_bkp9r;
#[doc = "TAMP backup 10 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_bkp10r](tamp_bkp10r) module"]
pub type TAMP_BKP10R = crate::Reg<u32, _TAMP_BKP10R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_BKP10R;
#[doc = "`read()` method returns [tamp_bkp10r::R](tamp_bkp10r::R) reader structure"]
impl crate::Readable for TAMP_BKP10R {}
#[doc = "`write(|w| ..)` method takes [tamp_bkp10r::W](tamp_bkp10r::W) writer structure"]
impl crate::Writable for TAMP_BKP10R {}
#[doc = "TAMP backup 10 register"]
pub mod tamp_bkp10r;
#[doc = "TAMP backup 11 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_bkp11r](tamp_bkp11r) module"]
pub type TAMP_BKP11R = crate::Reg<u32, _TAMP_BKP11R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_BKP11R;
#[doc = "`read()` method returns [tamp_bkp11r::R](tamp_bkp11r::R) reader structure"]
impl crate::Readable for TAMP_BKP11R {}
#[doc = "`write(|w| ..)` method takes [tamp_bkp11r::W](tamp_bkp11r::W) writer structure"]
impl crate::Writable for TAMP_BKP11R {}
#[doc = "TAMP backup 11 register"]
pub mod tamp_bkp11r;
#[doc = "TAMP backup 12 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_bkp12r](tamp_bkp12r) module"]
pub type TAMP_BKP12R = crate::Reg<u32, _TAMP_BKP12R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_BKP12R;
#[doc = "`read()` method returns [tamp_bkp12r::R](tamp_bkp12r::R) reader structure"]
impl crate::Readable for TAMP_BKP12R {}
#[doc = "`write(|w| ..)` method takes [tamp_bkp12r::W](tamp_bkp12r::W) writer structure"]
impl crate::Writable for TAMP_BKP12R {}
#[doc = "TAMP backup 12 register"]
pub mod tamp_bkp12r;
#[doc = "TAMP backup 13 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_bkp13r](tamp_bkp13r) module"]
pub type TAMP_BKP13R = crate::Reg<u32, _TAMP_BKP13R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_BKP13R;
#[doc = "`read()` method returns [tamp_bkp13r::R](tamp_bkp13r::R) reader structure"]
impl crate::Readable for TAMP_BKP13R {}
#[doc = "`write(|w| ..)` method takes [tamp_bkp13r::W](tamp_bkp13r::W) writer structure"]
impl crate::Writable for TAMP_BKP13R {}
#[doc = "TAMP backup 13 register"]
pub mod tamp_bkp13r;
#[doc = "TAMP backup 14 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_bkp14r](tamp_bkp14r) module"]
pub type TAMP_BKP14R = crate::Reg<u32, _TAMP_BKP14R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_BKP14R;
#[doc = "`read()` method returns [tamp_bkp14r::R](tamp_bkp14r::R) reader structure"]
impl crate::Readable for TAMP_BKP14R {}
#[doc = "`write(|w| ..)` method takes [tamp_bkp14r::W](tamp_bkp14r::W) writer structure"]
impl crate::Writable for TAMP_BKP14R {}
#[doc = "TAMP backup 14 register"]
pub mod tamp_bkp14r;
#[doc = "TAMP backup 15 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_bkp15r](tamp_bkp15r) module"]
pub type TAMP_BKP15R = crate::Reg<u32, _TAMP_BKP15R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_BKP15R;
#[doc = "`read()` method returns [tamp_bkp15r::R](tamp_bkp15r::R) reader structure"]
impl crate::Readable for TAMP_BKP15R {}
#[doc = "`write(|w| ..)` method takes [tamp_bkp15r::W](tamp_bkp15r::W) writer structure"]
impl crate::Writable for TAMP_BKP15R {}
#[doc = "TAMP backup 15 register"]
pub mod tamp_bkp15r;
#[doc = "TAMP backup 16 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_bkp16r](tamp_bkp16r) module"]
pub type TAMP_BKP16R = crate::Reg<u32, _TAMP_BKP16R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_BKP16R;
#[doc = "`read()` method returns [tamp_bkp16r::R](tamp_bkp16r::R) reader structure"]
impl crate::Readable for TAMP_BKP16R {}
#[doc = "`write(|w| ..)` method takes [tamp_bkp16r::W](tamp_bkp16r::W) writer structure"]
impl crate::Writable for TAMP_BKP16R {}
#[doc = "TAMP backup 16 register"]
pub mod tamp_bkp16r;
#[doc = "TAMP backup 17 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_bkp17r](tamp_bkp17r) module"]
pub type TAMP_BKP17R = crate::Reg<u32, _TAMP_BKP17R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_BKP17R;
#[doc = "`read()` method returns [tamp_bkp17r::R](tamp_bkp17r::R) reader structure"]
impl crate::Readable for TAMP_BKP17R {}
#[doc = "`write(|w| ..)` method takes [tamp_bkp17r::W](tamp_bkp17r::W) writer structure"]
impl crate::Writable for TAMP_BKP17R {}
#[doc = "TAMP backup 17 register"]
pub mod tamp_bkp17r;
#[doc = "TAMP backup 18 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_bkp18r](tamp_bkp18r) module"]
pub type TAMP_BKP18R = crate::Reg<u32, _TAMP_BKP18R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_BKP18R;
#[doc = "`read()` method returns [tamp_bkp18r::R](tamp_bkp18r::R) reader structure"]
impl crate::Readable for TAMP_BKP18R {}
#[doc = "`write(|w| ..)` method takes [tamp_bkp18r::W](tamp_bkp18r::W) writer structure"]
impl crate::Writable for TAMP_BKP18R {}
#[doc = "TAMP backup 18 register"]
pub mod tamp_bkp18r;
#[doc = "TAMP backup 19 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_bkp19r](tamp_bkp19r) module"]
pub type TAMP_BKP19R = crate::Reg<u32, _TAMP_BKP19R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_BKP19R;
#[doc = "`read()` method returns [tamp_bkp19r::R](tamp_bkp19r::R) reader structure"]
impl crate::Readable for TAMP_BKP19R {}
#[doc = "`write(|w| ..)` method takes [tamp_bkp19r::W](tamp_bkp19r::W) writer structure"]
impl crate::Writable for TAMP_BKP19R {}
#[doc = "TAMP backup 19 register"]
pub mod tamp_bkp19r;
#[doc = "TAMP backup 20 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_bkp20r](tamp_bkp20r) module"]
pub type TAMP_BKP20R = crate::Reg<u32, _TAMP_BKP20R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_BKP20R;
#[doc = "`read()` method returns [tamp_bkp20r::R](tamp_bkp20r::R) reader structure"]
impl crate::Readable for TAMP_BKP20R {}
#[doc = "`write(|w| ..)` method takes [tamp_bkp20r::W](tamp_bkp20r::W) writer structure"]
impl crate::Writable for TAMP_BKP20R {}
#[doc = "TAMP backup 20 register"]
pub mod tamp_bkp20r;
#[doc = "TAMP backup 21 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_bkp21r](tamp_bkp21r) module"]
pub type TAMP_BKP21R = crate::Reg<u32, _TAMP_BKP21R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_BKP21R;
#[doc = "`read()` method returns [tamp_bkp21r::R](tamp_bkp21r::R) reader structure"]
impl crate::Readable for TAMP_BKP21R {}
#[doc = "`write(|w| ..)` method takes [tamp_bkp21r::W](tamp_bkp21r::W) writer structure"]
impl crate::Writable for TAMP_BKP21R {}
#[doc = "TAMP backup 21 register"]
pub mod tamp_bkp21r;
#[doc = "TAMP backup 22 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_bkp22r](tamp_bkp22r) module"]
pub type TAMP_BKP22R = crate::Reg<u32, _TAMP_BKP22R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_BKP22R;
#[doc = "`read()` method returns [tamp_bkp22r::R](tamp_bkp22r::R) reader structure"]
impl crate::Readable for TAMP_BKP22R {}
#[doc = "`write(|w| ..)` method takes [tamp_bkp22r::W](tamp_bkp22r::W) writer structure"]
impl crate::Writable for TAMP_BKP22R {}
#[doc = "TAMP backup 22 register"]
pub mod tamp_bkp22r;
#[doc = "TAMP backup 23 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_bkp23r](tamp_bkp23r) module"]
pub type TAMP_BKP23R = crate::Reg<u32, _TAMP_BKP23R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_BKP23R;
#[doc = "`read()` method returns [tamp_bkp23r::R](tamp_bkp23r::R) reader structure"]
impl crate::Readable for TAMP_BKP23R {}
#[doc = "`write(|w| ..)` method takes [tamp_bkp23r::W](tamp_bkp23r::W) writer structure"]
impl crate::Writable for TAMP_BKP23R {}
#[doc = "TAMP backup 23 register"]
pub mod tamp_bkp23r;
#[doc = "TAMP backup 24 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_bkp24r](tamp_bkp24r) module"]
pub type TAMP_BKP24R = crate::Reg<u32, _TAMP_BKP24R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_BKP24R;
#[doc = "`read()` method returns [tamp_bkp24r::R](tamp_bkp24r::R) reader structure"]
impl crate::Readable for TAMP_BKP24R {}
#[doc = "`write(|w| ..)` method takes [tamp_bkp24r::W](tamp_bkp24r::W) writer structure"]
impl crate::Writable for TAMP_BKP24R {}
#[doc = "TAMP backup 24 register"]
pub mod tamp_bkp24r;
#[doc = "TAMP backup 25 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_bkp25r](tamp_bkp25r) module"]
pub type TAMP_BKP25R = crate::Reg<u32, _TAMP_BKP25R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_BKP25R;
#[doc = "`read()` method returns [tamp_bkp25r::R](tamp_bkp25r::R) reader structure"]
impl crate::Readable for TAMP_BKP25R {}
#[doc = "`write(|w| ..)` method takes [tamp_bkp25r::W](tamp_bkp25r::W) writer structure"]
impl crate::Writable for TAMP_BKP25R {}
#[doc = "TAMP backup 25 register"]
pub mod tamp_bkp25r;
#[doc = "TAMP backup 26 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_bkp26r](tamp_bkp26r) module"]
pub type TAMP_BKP26R = crate::Reg<u32, _TAMP_BKP26R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_BKP26R;
#[doc = "`read()` method returns [tamp_bkp26r::R](tamp_bkp26r::R) reader structure"]
impl crate::Readable for TAMP_BKP26R {}
#[doc = "`write(|w| ..)` method takes [tamp_bkp26r::W](tamp_bkp26r::W) writer structure"]
impl crate::Writable for TAMP_BKP26R {}
#[doc = "TAMP backup 26 register"]
pub mod tamp_bkp26r;
#[doc = "TAMP backup 27 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_bkp27r](tamp_bkp27r) module"]
pub type TAMP_BKP27R = crate::Reg<u32, _TAMP_BKP27R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_BKP27R;
#[doc = "`read()` method returns [tamp_bkp27r::R](tamp_bkp27r::R) reader structure"]
impl crate::Readable for TAMP_BKP27R {}
#[doc = "`write(|w| ..)` method takes [tamp_bkp27r::W](tamp_bkp27r::W) writer structure"]
impl crate::Writable for TAMP_BKP27R {}
#[doc = "TAMP backup 27 register"]
pub mod tamp_bkp27r;
#[doc = "TAMP backup 28 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_bkp28r](tamp_bkp28r) module"]
pub type TAMP_BKP28R = crate::Reg<u32, _TAMP_BKP28R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_BKP28R;
#[doc = "`read()` method returns [tamp_bkp28r::R](tamp_bkp28r::R) reader structure"]
impl crate::Readable for TAMP_BKP28R {}
#[doc = "`write(|w| ..)` method takes [tamp_bkp28r::W](tamp_bkp28r::W) writer structure"]
impl crate::Writable for TAMP_BKP28R {}
#[doc = "TAMP backup 28 register"]
pub mod tamp_bkp28r;
#[doc = "TAMP backup 29 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_bkp29r](tamp_bkp29r) module"]
pub type TAMP_BKP29R = crate::Reg<u32, _TAMP_BKP29R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_BKP29R;
#[doc = "`read()` method returns [tamp_bkp29r::R](tamp_bkp29r::R) reader structure"]
impl crate::Readable for TAMP_BKP29R {}
#[doc = "`write(|w| ..)` method takes [tamp_bkp29r::W](tamp_bkp29r::W) writer structure"]
impl crate::Writable for TAMP_BKP29R {}
#[doc = "TAMP backup 29 register"]
pub mod tamp_bkp29r;
#[doc = "TAMP backup 30 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_bkp30r](tamp_bkp30r) module"]
pub type TAMP_BKP30R = crate::Reg<u32, _TAMP_BKP30R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_BKP30R;
#[doc = "`read()` method returns [tamp_bkp30r::R](tamp_bkp30r::R) reader structure"]
impl crate::Readable for TAMP_BKP30R {}
#[doc = "`write(|w| ..)` method takes [tamp_bkp30r::W](tamp_bkp30r::W) writer structure"]
impl crate::Writable for TAMP_BKP30R {}
#[doc = "TAMP backup 30 register"]
pub mod tamp_bkp30r;
#[doc = "TAMP backup 31 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_bkp31r](tamp_bkp31r) module"]
pub type TAMP_BKP31R = crate::Reg<u32, _TAMP_BKP31R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_BKP31R;
#[doc = "`read()` method returns [tamp_bkp31r::R](tamp_bkp31r::R) reader structure"]
impl crate::Readable for TAMP_BKP31R {}
#[doc = "`write(|w| ..)` method takes [tamp_bkp31r::W](tamp_bkp31r::W) writer structure"]
impl crate::Writable for TAMP_BKP31R {}
#[doc = "TAMP backup 31 register"]
pub mod tamp_bkp31r;
#[doc = "TAMP hardware configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_hwcfgr2](tamp_hwcfgr2) module"]
pub type TAMP_HWCFGR2 = crate::Reg<u32, _TAMP_HWCFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_HWCFGR2;
#[doc = "`read()` method returns [tamp_hwcfgr2::R](tamp_hwcfgr2::R) reader structure"]
impl crate::Readable for TAMP_HWCFGR2 {}
#[doc = "TAMP hardware configuration register 2"]
pub mod tamp_hwcfgr2;
#[doc = "TAMP hardware configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_hwcfgr1](tamp_hwcfgr1) module"]
pub type TAMP_HWCFGR1 = crate::Reg<u32, _TAMP_HWCFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_HWCFGR1;
#[doc = "`read()` method returns [tamp_hwcfgr1::R](tamp_hwcfgr1::R) reader structure"]
impl crate::Readable for TAMP_HWCFGR1 {}
#[doc = "TAMP hardware configuration register 1"]
pub mod tamp_hwcfgr1;
#[doc = "TAMP version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_verr](tamp_verr) module"]
pub type TAMP_VERR = crate::Reg<u32, _TAMP_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_VERR;
#[doc = "`read()` method returns [tamp_verr::R](tamp_verr::R) reader structure"]
impl crate::Readable for TAMP_VERR {}
#[doc = "TAMP version register"]
pub mod tamp_verr;
#[doc = "TAMP identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_ipidr](tamp_ipidr) module"]
pub type TAMP_IPIDR = crate::Reg<u32, _TAMP_IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_IPIDR;
#[doc = "`read()` method returns [tamp_ipidr::R](tamp_ipidr::R) reader structure"]
impl crate::Readable for TAMP_IPIDR {}
#[doc = "TAMP identification register"]
pub mod tamp_ipidr;
#[doc = "TAMP size identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_sidr](tamp_sidr) module"]
pub type TAMP_SIDR = crate::Reg<u32, _TAMP_SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP_SIDR;
#[doc = "`read()` method returns [tamp_sidr::R](tamp_sidr::R) reader structure"]
impl crate::Readable for TAMP_SIDR {}
#[doc = "TAMP size identification register"]
pub mod tamp_sidr;
