#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - The RTC_TR is the calendar time shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page9 and Reading the calendar on page10.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    pub tr: TR,
    #[doc = "0x04 - The RTC_DR is the calendar date shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page9 and Reading the calendar on page10.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    pub dr: DR,
    #[doc = "0x08 - RTC control register"]
    pub cr: CR,
    #[doc = "0x0c - This register is write protected (except for RTC_ISR\\[13:8\\]
bits). The write access procedure is described in RTC register write protection on page9."]
    pub isr: ISR,
    #[doc = "0x10 - This register must be written in initialization mode only. The initialization must be performed in two separate write accesses. Refer to Calendar initialization and configuration on page9.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    pub prer: PRER,
    #[doc = "0x14 - This register can be written only when WUTWF is set to 1 in RTC_ISR.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    pub wutr: WUTR,
    _reserved6: [u8; 4usize],
    #[doc = "0x1c - This register can be written only when ALRAWF is set to 1 in RTC_ISR, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    pub alrmar: ALRMAR,
    #[doc = "0x20 - This register can be written only when ALRBWF is set to 1 in RTC_ISR, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    pub alrmbr: ALRMBR,
    #[doc = "0x24 - RTC write protection register"]
    pub wpr: WPR,
    #[doc = "0x28 - RTC sub second register"]
    pub ssr: SSR,
    #[doc = "0x2c - This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    pub shiftr: SHIFTR,
    #[doc = "0x30 - The content of this register is valid only when TSF is set to 1 in RTC_ISR. It is cleared when TSF bit is reset."]
    pub tstr: TSTR,
    #[doc = "0x34 - The content of this register is valid only when TSF is set to 1 in RTC_ISR. It is cleared when TSF bit is reset."]
    pub tsdr: TSDR,
    #[doc = "0x38 - The content of this register is valid only when RTC_ISR/TSF is set. It is cleared when the RTC_ISR/TSF bit is reset."]
    pub tsssr: TSSSR,
    #[doc = "0x3c - This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    pub calr: CALR,
    #[doc = "0x40 - RTC tamper and alternate function configuration register"]
    pub tampcr: TAMPCR,
    #[doc = "0x44 - This register can be written only when ALRAE is reset in RTC_CR register, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9"]
    pub alrmassr: ALRMASSR,
    #[doc = "0x48 - This register can be written only when ALRBE is reset in RTC_CR register, or in initialization mode.This register is write protected.The write access procedure is described in Section: RTC register write protection."]
    pub alrmbssr: ALRMBSSR,
    #[doc = "0x4c - RTC option register"]
    pub or: OR,
    #[doc = "0x50 - RTC backup registers"]
    pub bkp0r: BKP0R,
    #[doc = "0x54 - RTC backup registers"]
    pub bkp1r: BKP1R,
    #[doc = "0x58 - RTC backup registers"]
    pub bkp2r: BKP2R,
    #[doc = "0x5c - RTC backup registers"]
    pub bkp3r: BKP3R,
    #[doc = "0x60 - RTC backup registers"]
    pub bkp4r: BKP4R,
    #[doc = "0x64 - RTC backup registers"]
    pub bkp5r: BKP5R,
    #[doc = "0x68 - RTC backup registers"]
    pub bkp6r: BKP6R,
    #[doc = "0x6c - RTC backup registers"]
    pub bkp7r: BKP7R,
    #[doc = "0x70 - RTC backup registers"]
    pub bkp8r: BKP8R,
    #[doc = "0x74 - RTC backup registers"]
    pub bkp9r: BKP9R,
    #[doc = "0x78 - RTC backup registers"]
    pub bkp10r: BKP10R,
    #[doc = "0x7c - RTC backup registers"]
    pub bkp11r: BKP11R,
    #[doc = "0x80 - RTC backup registers"]
    pub bkp12r: BKP12R,
    #[doc = "0x84 - RTC backup registers"]
    pub bkp13r: BKP13R,
    #[doc = "0x88 - RTC backup registers"]
    pub bkp14r: BKP14R,
    #[doc = "0x8c - RTC backup registers"]
    pub bkp15r: BKP15R,
    #[doc = "0x90 - RTC backup registers"]
    pub bkp16r: BKP16R,
    #[doc = "0x94 - RTC backup registers"]
    pub bkp17r: BKP17R,
    #[doc = "0x98 - RTC backup registers"]
    pub bkp18r: BKP18R,
    #[doc = "0x9c - RTC backup registers"]
    pub bkp19r: BKP19R,
    #[doc = "0xa0 - RTC backup registers"]
    pub bkp20r: BKP20R,
    #[doc = "0xa4 - RTC backup registers"]
    pub bkp21r: BKP21R,
    #[doc = "0xa8 - RTC backup registers"]
    pub bkp22r: BKP22R,
    #[doc = "0xac - RTC backup registers"]
    pub bkp23r: BKP23R,
    #[doc = "0xb0 - RTC backup registers"]
    pub bkp24r: BKP24R,
    #[doc = "0xb4 - RTC backup registers"]
    pub bkp25r: BKP25R,
    #[doc = "0xb8 - RTC backup registers"]
    pub bkp26r: BKP26R,
    #[doc = "0xbc - RTC backup registers"]
    pub bkp27r: BKP27R,
    #[doc = "0xc0 - RTC backup registers"]
    pub bkp28r: BKP28R,
    #[doc = "0xc4 - RTC backup registers"]
    pub bkp29r: BKP29R,
    #[doc = "0xc8 - RTC backup registers"]
    pub bkp30r: BKP30R,
    #[doc = "0xcc - RTC backup registers"]
    pub bkp31r: BKP31R,
}
#[doc = "The RTC_TR is the calendar time shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page9 and Reading the calendar on page10.This register is write protected. The write access procedure is described in RTC register write protection on page9.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr](tr) module"]
pub type TR = crate::Reg<u32, _TR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TR;
#[doc = "`read()` method returns [tr::R](tr::R) reader structure"]
impl crate::Readable for TR {}
#[doc = "`write(|w| ..)` method takes [tr::W](tr::W) writer structure"]
impl crate::Writable for TR {}
#[doc = "The RTC_TR is the calendar time shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page9 and Reading the calendar on page10.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod tr;
#[doc = "The RTC_DR is the calendar date shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page9 and Reading the calendar on page10.This register is write protected. The write access procedure is described in RTC register write protection on page9.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr](dr) module"]
pub type DR = crate::Reg<u32, _DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR;
#[doc = "`read()` method returns [dr::R](dr::R) reader structure"]
impl crate::Readable for DR {}
#[doc = "`write(|w| ..)` method takes [dr::W](dr::W) writer structure"]
impl crate::Writable for DR {}
#[doc = "The RTC_DR is the calendar date shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page9 and Reading the calendar on page10.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod dr;
#[doc = "RTC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "RTC control register"]
pub mod cr;
#[doc = "This register is write protected (except for RTC_ISR\\[13:8\\]
bits). The write access procedure is described in RTC register write protection on page9.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "`write(|w| ..)` method takes [isr::W](isr::W) writer structure"]
impl crate::Writable for ISR {}
#[doc = "This register is write protected (except for RTC_ISR\\[13:8\\]
bits). The write access procedure is described in RTC register write protection on page9."]
pub mod isr;
#[doc = "This register must be written in initialization mode only. The initialization must be performed in two separate write accesses. Refer to Calendar initialization and configuration on page9.This register is write protected. The write access procedure is described in RTC register write protection on page9.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prer](prer) module"]
pub type PRER = crate::Reg<u32, _PRER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRER;
#[doc = "`read()` method returns [prer::R](prer::R) reader structure"]
impl crate::Readable for PRER {}
#[doc = "`write(|w| ..)` method takes [prer::W](prer::W) writer structure"]
impl crate::Writable for PRER {}
#[doc = "This register must be written in initialization mode only. The initialization must be performed in two separate write accesses. Refer to Calendar initialization and configuration on page9.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod prer;
#[doc = "This register can be written only when WUTWF is set to 1 in RTC_ISR.This register is write protected. The write access procedure is described in RTC register write protection on page9.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wutr](wutr) module"]
pub type WUTR = crate::Reg<u32, _WUTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WUTR;
#[doc = "`read()` method returns [wutr::R](wutr::R) reader structure"]
impl crate::Readable for WUTR {}
#[doc = "`write(|w| ..)` method takes [wutr::W](wutr::W) writer structure"]
impl crate::Writable for WUTR {}
#[doc = "This register can be written only when WUTWF is set to 1 in RTC_ISR.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod wutr;
#[doc = "This register can be written only when ALRAWF is set to 1 in RTC_ISR, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrmar](alrmar) module"]
pub type ALRMAR = crate::Reg<u32, _ALRMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALRMAR;
#[doc = "`read()` method returns [alrmar::R](alrmar::R) reader structure"]
impl crate::Readable for ALRMAR {}
#[doc = "`write(|w| ..)` method takes [alrmar::W](alrmar::W) writer structure"]
impl crate::Writable for ALRMAR {}
#[doc = "This register can be written only when ALRAWF is set to 1 in RTC_ISR, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod alrmar;
#[doc = "This register can be written only when ALRBWF is set to 1 in RTC_ISR, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrmbr](alrmbr) module"]
pub type ALRMBR = crate::Reg<u32, _ALRMBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALRMBR;
#[doc = "`read()` method returns [alrmbr::R](alrmbr::R) reader structure"]
impl crate::Readable for ALRMBR {}
#[doc = "`write(|w| ..)` method takes [alrmbr::W](alrmbr::W) writer structure"]
impl crate::Writable for ALRMBR {}
#[doc = "This register can be written only when ALRBWF is set to 1 in RTC_ISR, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod alrmbr;
#[doc = "RTC write protection register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpr](wpr) module"]
pub type WPR = crate::Reg<u32, _WPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPR;
#[doc = "`write(|w| ..)` method takes [wpr::W](wpr::W) writer structure"]
impl crate::Writable for WPR {}
#[doc = "RTC write protection register"]
pub mod wpr;
#[doc = "RTC sub second register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssr](ssr) module"]
pub type SSR = crate::Reg<u32, _SSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSR;
#[doc = "`read()` method returns [ssr::R](ssr::R) reader structure"]
impl crate::Readable for SSR {}
#[doc = "RTC sub second register"]
pub mod ssr;
#[doc = "This register is write protected. The write access procedure is described in RTC register write protection on page9.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftr](shiftr) module"]
pub type SHIFTR = crate::Reg<u32, _SHIFTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTR;
#[doc = "`write(|w| ..)` method takes [shiftr::W](shiftr::W) writer structure"]
impl crate::Writable for SHIFTR {}
#[doc = "This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod shiftr;
#[doc = "The content of this register is valid only when TSF is set to 1 in RTC_ISR. It is cleared when TSF bit is reset.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tstr](tstr) module"]
pub type TSTR = crate::Reg<u32, _TSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSTR;
#[doc = "`read()` method returns [tstr::R](tstr::R) reader structure"]
impl crate::Readable for TSTR {}
#[doc = "The content of this register is valid only when TSF is set to 1 in RTC_ISR. It is cleared when TSF bit is reset."]
pub mod tstr;
#[doc = "The content of this register is valid only when TSF is set to 1 in RTC_ISR. It is cleared when TSF bit is reset.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsdr](tsdr) module"]
pub type TSDR = crate::Reg<u32, _TSDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSDR;
#[doc = "`read()` method returns [tsdr::R](tsdr::R) reader structure"]
impl crate::Readable for TSDR {}
#[doc = "The content of this register is valid only when TSF is set to 1 in RTC_ISR. It is cleared when TSF bit is reset."]
pub mod tsdr;
#[doc = "The content of this register is valid only when RTC_ISR/TSF is set. It is cleared when the RTC_ISR/TSF bit is reset.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsssr](tsssr) module"]
pub type TSSSR = crate::Reg<u32, _TSSSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSSSR;
#[doc = "`read()` method returns [tsssr::R](tsssr::R) reader structure"]
impl crate::Readable for TSSSR {}
#[doc = "The content of this register is valid only when RTC_ISR/TSF is set. It is cleared when the RTC_ISR/TSF bit is reset."]
pub mod tsssr;
#[doc = "This register is write protected. The write access procedure is described in RTC register write protection on page9.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calr](calr) module"]
pub type CALR = crate::Reg<u32, _CALR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALR;
#[doc = "`read()` method returns [calr::R](calr::R) reader structure"]
impl crate::Readable for CALR {}
#[doc = "`write(|w| ..)` method takes [calr::W](calr::W) writer structure"]
impl crate::Writable for CALR {}
#[doc = "This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod calr;
#[doc = "RTC tamper and alternate function configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tampcr](tampcr) module"]
pub type TAMPCR = crate::Reg<u32, _TAMPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMPCR;
#[doc = "`read()` method returns [tampcr::R](tampcr::R) reader structure"]
impl crate::Readable for TAMPCR {}
#[doc = "`write(|w| ..)` method takes [tampcr::W](tampcr::W) writer structure"]
impl crate::Writable for TAMPCR {}
#[doc = "RTC tamper and alternate function configuration register"]
pub mod tampcr;
#[doc = "This register can be written only when ALRAE is reset in RTC_CR register, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrmassr](alrmassr) module"]
pub type ALRMASSR = crate::Reg<u32, _ALRMASSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALRMASSR;
#[doc = "`read()` method returns [alrmassr::R](alrmassr::R) reader structure"]
impl crate::Readable for ALRMASSR {}
#[doc = "`write(|w| ..)` method takes [alrmassr::W](alrmassr::W) writer structure"]
impl crate::Writable for ALRMASSR {}
#[doc = "This register can be written only when ALRAE is reset in RTC_CR register, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9"]
pub mod alrmassr;
#[doc = "This register can be written only when ALRBE is reset in RTC_CR register, or in initialization mode.This register is write protected.The write access procedure is described in Section: RTC register write protection.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrmbssr](alrmbssr) module"]
pub type ALRMBSSR = crate::Reg<u32, _ALRMBSSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALRMBSSR;
#[doc = "`read()` method returns [alrmbssr::R](alrmbssr::R) reader structure"]
impl crate::Readable for ALRMBSSR {}
#[doc = "`write(|w| ..)` method takes [alrmbssr::W](alrmbssr::W) writer structure"]
impl crate::Writable for ALRMBSSR {}
#[doc = "This register can be written only when ALRBE is reset in RTC_CR register, or in initialization mode.This register is write protected.The write access procedure is described in Section: RTC register write protection."]
pub mod alrmbssr;
#[doc = "RTC backup registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp0r](bkp0r) module"]
pub type BKP0R = crate::Reg<u32, _BKP0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP0R;
#[doc = "`read()` method returns [bkp0r::R](bkp0r::R) reader structure"]
impl crate::Readable for BKP0R {}
#[doc = "`write(|w| ..)` method takes [bkp0r::W](bkp0r::W) writer structure"]
impl crate::Writable for BKP0R {}
#[doc = "RTC backup registers"]
pub mod bkp0r;
#[doc = "RTC backup registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp1r](bkp1r) module"]
pub type BKP1R = crate::Reg<u32, _BKP1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP1R;
#[doc = "`read()` method returns [bkp1r::R](bkp1r::R) reader structure"]
impl crate::Readable for BKP1R {}
#[doc = "`write(|w| ..)` method takes [bkp1r::W](bkp1r::W) writer structure"]
impl crate::Writable for BKP1R {}
#[doc = "RTC backup registers"]
pub mod bkp1r;
#[doc = "RTC backup registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp2r](bkp2r) module"]
pub type BKP2R = crate::Reg<u32, _BKP2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP2R;
#[doc = "`read()` method returns [bkp2r::R](bkp2r::R) reader structure"]
impl crate::Readable for BKP2R {}
#[doc = "`write(|w| ..)` method takes [bkp2r::W](bkp2r::W) writer structure"]
impl crate::Writable for BKP2R {}
#[doc = "RTC backup registers"]
pub mod bkp2r;
#[doc = "RTC backup registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp3r](bkp3r) module"]
pub type BKP3R = crate::Reg<u32, _BKP3R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP3R;
#[doc = "`read()` method returns [bkp3r::R](bkp3r::R) reader structure"]
impl crate::Readable for BKP3R {}
#[doc = "`write(|w| ..)` method takes [bkp3r::W](bkp3r::W) writer structure"]
impl crate::Writable for BKP3R {}
#[doc = "RTC backup registers"]
pub mod bkp3r;
#[doc = "RTC backup registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp4r](bkp4r) module"]
pub type BKP4R = crate::Reg<u32, _BKP4R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP4R;
#[doc = "`read()` method returns [bkp4r::R](bkp4r::R) reader structure"]
impl crate::Readable for BKP4R {}
#[doc = "`write(|w| ..)` method takes [bkp4r::W](bkp4r::W) writer structure"]
impl crate::Writable for BKP4R {}
#[doc = "RTC backup registers"]
pub mod bkp4r;
#[doc = "RTC backup registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp5r](bkp5r) module"]
pub type BKP5R = crate::Reg<u32, _BKP5R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP5R;
#[doc = "`read()` method returns [bkp5r::R](bkp5r::R) reader structure"]
impl crate::Readable for BKP5R {}
#[doc = "`write(|w| ..)` method takes [bkp5r::W](bkp5r::W) writer structure"]
impl crate::Writable for BKP5R {}
#[doc = "RTC backup registers"]
pub mod bkp5r;
#[doc = "RTC backup registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp6r](bkp6r) module"]
pub type BKP6R = crate::Reg<u32, _BKP6R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP6R;
#[doc = "`read()` method returns [bkp6r::R](bkp6r::R) reader structure"]
impl crate::Readable for BKP6R {}
#[doc = "`write(|w| ..)` method takes [bkp6r::W](bkp6r::W) writer structure"]
impl crate::Writable for BKP6R {}
#[doc = "RTC backup registers"]
pub mod bkp6r;
#[doc = "RTC backup registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp7r](bkp7r) module"]
pub type BKP7R = crate::Reg<u32, _BKP7R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP7R;
#[doc = "`read()` method returns [bkp7r::R](bkp7r::R) reader structure"]
impl crate::Readable for BKP7R {}
#[doc = "`write(|w| ..)` method takes [bkp7r::W](bkp7r::W) writer structure"]
impl crate::Writable for BKP7R {}
#[doc = "RTC backup registers"]
pub mod bkp7r;
#[doc = "RTC backup registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp8r](bkp8r) module"]
pub type BKP8R = crate::Reg<u32, _BKP8R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP8R;
#[doc = "`read()` method returns [bkp8r::R](bkp8r::R) reader structure"]
impl crate::Readable for BKP8R {}
#[doc = "`write(|w| ..)` method takes [bkp8r::W](bkp8r::W) writer structure"]
impl crate::Writable for BKP8R {}
#[doc = "RTC backup registers"]
pub mod bkp8r;
#[doc = "RTC backup registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp9r](bkp9r) module"]
pub type BKP9R = crate::Reg<u32, _BKP9R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP9R;
#[doc = "`read()` method returns [bkp9r::R](bkp9r::R) reader structure"]
impl crate::Readable for BKP9R {}
#[doc = "`write(|w| ..)` method takes [bkp9r::W](bkp9r::W) writer structure"]
impl crate::Writable for BKP9R {}
#[doc = "RTC backup registers"]
pub mod bkp9r;
#[doc = "RTC backup registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp10r](bkp10r) module"]
pub type BKP10R = crate::Reg<u32, _BKP10R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP10R;
#[doc = "`read()` method returns [bkp10r::R](bkp10r::R) reader structure"]
impl crate::Readable for BKP10R {}
#[doc = "`write(|w| ..)` method takes [bkp10r::W](bkp10r::W) writer structure"]
impl crate::Writable for BKP10R {}
#[doc = "RTC backup registers"]
pub mod bkp10r;
#[doc = "RTC backup registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp11r](bkp11r) module"]
pub type BKP11R = crate::Reg<u32, _BKP11R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP11R;
#[doc = "`read()` method returns [bkp11r::R](bkp11r::R) reader structure"]
impl crate::Readable for BKP11R {}
#[doc = "`write(|w| ..)` method takes [bkp11r::W](bkp11r::W) writer structure"]
impl crate::Writable for BKP11R {}
#[doc = "RTC backup registers"]
pub mod bkp11r;
#[doc = "RTC backup registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp12r](bkp12r) module"]
pub type BKP12R = crate::Reg<u32, _BKP12R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP12R;
#[doc = "`read()` method returns [bkp12r::R](bkp12r::R) reader structure"]
impl crate::Readable for BKP12R {}
#[doc = "`write(|w| ..)` method takes [bkp12r::W](bkp12r::W) writer structure"]
impl crate::Writable for BKP12R {}
#[doc = "RTC backup registers"]
pub mod bkp12r;
#[doc = "RTC backup registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp13r](bkp13r) module"]
pub type BKP13R = crate::Reg<u32, _BKP13R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP13R;
#[doc = "`read()` method returns [bkp13r::R](bkp13r::R) reader structure"]
impl crate::Readable for BKP13R {}
#[doc = "`write(|w| ..)` method takes [bkp13r::W](bkp13r::W) writer structure"]
impl crate::Writable for BKP13R {}
#[doc = "RTC backup registers"]
pub mod bkp13r;
#[doc = "RTC backup registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp14r](bkp14r) module"]
pub type BKP14R = crate::Reg<u32, _BKP14R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP14R;
#[doc = "`read()` method returns [bkp14r::R](bkp14r::R) reader structure"]
impl crate::Readable for BKP14R {}
#[doc = "`write(|w| ..)` method takes [bkp14r::W](bkp14r::W) writer structure"]
impl crate::Writable for BKP14R {}
#[doc = "RTC backup registers"]
pub mod bkp14r;
#[doc = "RTC backup registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp15r](bkp15r) module"]
pub type BKP15R = crate::Reg<u32, _BKP15R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP15R;
#[doc = "`read()` method returns [bkp15r::R](bkp15r::R) reader structure"]
impl crate::Readable for BKP15R {}
#[doc = "`write(|w| ..)` method takes [bkp15r::W](bkp15r::W) writer structure"]
impl crate::Writable for BKP15R {}
#[doc = "RTC backup registers"]
pub mod bkp15r;
#[doc = "RTC option register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [or](or) module"]
pub type OR = crate::Reg<u32, _OR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OR;
#[doc = "`read()` method returns [or::R](or::R) reader structure"]
impl crate::Readable for OR {}
#[doc = "`write(|w| ..)` method takes [or::W](or::W) writer structure"]
impl crate::Writable for OR {}
#[doc = "RTC option register"]
pub mod or;
#[doc = "RTC backup registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp16r](bkp16r) module"]
pub type BKP16R = crate::Reg<u32, _BKP16R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP16R;
#[doc = "`read()` method returns [bkp16r::R](bkp16r::R) reader structure"]
impl crate::Readable for BKP16R {}
#[doc = "`write(|w| ..)` method takes [bkp16r::W](bkp16r::W) writer structure"]
impl crate::Writable for BKP16R {}
#[doc = "RTC backup registers"]
pub mod bkp16r;
#[doc = "RTC backup registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp17r](bkp17r) module"]
pub type BKP17R = crate::Reg<u32, _BKP17R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP17R;
#[doc = "`read()` method returns [bkp17r::R](bkp17r::R) reader structure"]
impl crate::Readable for BKP17R {}
#[doc = "`write(|w| ..)` method takes [bkp17r::W](bkp17r::W) writer structure"]
impl crate::Writable for BKP17R {}
#[doc = "RTC backup registers"]
pub mod bkp17r;
#[doc = "RTC backup registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp18r](bkp18r) module"]
pub type BKP18R = crate::Reg<u32, _BKP18R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP18R;
#[doc = "`read()` method returns [bkp18r::R](bkp18r::R) reader structure"]
impl crate::Readable for BKP18R {}
#[doc = "`write(|w| ..)` method takes [bkp18r::W](bkp18r::W) writer structure"]
impl crate::Writable for BKP18R {}
#[doc = "RTC backup registers"]
pub mod bkp18r;
#[doc = "RTC backup registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp19r](bkp19r) module"]
pub type BKP19R = crate::Reg<u32, _BKP19R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP19R;
#[doc = "`read()` method returns [bkp19r::R](bkp19r::R) reader structure"]
impl crate::Readable for BKP19R {}
#[doc = "`write(|w| ..)` method takes [bkp19r::W](bkp19r::W) writer structure"]
impl crate::Writable for BKP19R {}
#[doc = "RTC backup registers"]
pub mod bkp19r;
#[doc = "RTC backup registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp20r](bkp20r) module"]
pub type BKP20R = crate::Reg<u32, _BKP20R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP20R;
#[doc = "`read()` method returns [bkp20r::R](bkp20r::R) reader structure"]
impl crate::Readable for BKP20R {}
#[doc = "`write(|w| ..)` method takes [bkp20r::W](bkp20r::W) writer structure"]
impl crate::Writable for BKP20R {}
#[doc = "RTC backup registers"]
pub mod bkp20r;
#[doc = "RTC backup registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp21r](bkp21r) module"]
pub type BKP21R = crate::Reg<u32, _BKP21R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP21R;
#[doc = "`read()` method returns [bkp21r::R](bkp21r::R) reader structure"]
impl crate::Readable for BKP21R {}
#[doc = "`write(|w| ..)` method takes [bkp21r::W](bkp21r::W) writer structure"]
impl crate::Writable for BKP21R {}
#[doc = "RTC backup registers"]
pub mod bkp21r;
#[doc = "RTC backup registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp22r](bkp22r) module"]
pub type BKP22R = crate::Reg<u32, _BKP22R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP22R;
#[doc = "`read()` method returns [bkp22r::R](bkp22r::R) reader structure"]
impl crate::Readable for BKP22R {}
#[doc = "`write(|w| ..)` method takes [bkp22r::W](bkp22r::W) writer structure"]
impl crate::Writable for BKP22R {}
#[doc = "RTC backup registers"]
pub mod bkp22r;
#[doc = "RTC backup registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp23r](bkp23r) module"]
pub type BKP23R = crate::Reg<u32, _BKP23R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP23R;
#[doc = "`read()` method returns [bkp23r::R](bkp23r::R) reader structure"]
impl crate::Readable for BKP23R {}
#[doc = "`write(|w| ..)` method takes [bkp23r::W](bkp23r::W) writer structure"]
impl crate::Writable for BKP23R {}
#[doc = "RTC backup registers"]
pub mod bkp23r;
#[doc = "RTC backup registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp24r](bkp24r) module"]
pub type BKP24R = crate::Reg<u32, _BKP24R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP24R;
#[doc = "`read()` method returns [bkp24r::R](bkp24r::R) reader structure"]
impl crate::Readable for BKP24R {}
#[doc = "`write(|w| ..)` method takes [bkp24r::W](bkp24r::W) writer structure"]
impl crate::Writable for BKP24R {}
#[doc = "RTC backup registers"]
pub mod bkp24r;
#[doc = "RTC backup registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp25r](bkp25r) module"]
pub type BKP25R = crate::Reg<u32, _BKP25R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP25R;
#[doc = "`read()` method returns [bkp25r::R](bkp25r::R) reader structure"]
impl crate::Readable for BKP25R {}
#[doc = "`write(|w| ..)` method takes [bkp25r::W](bkp25r::W) writer structure"]
impl crate::Writable for BKP25R {}
#[doc = "RTC backup registers"]
pub mod bkp25r;
#[doc = "RTC backup registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp26r](bkp26r) module"]
pub type BKP26R = crate::Reg<u32, _BKP26R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP26R;
#[doc = "`read()` method returns [bkp26r::R](bkp26r::R) reader structure"]
impl crate::Readable for BKP26R {}
#[doc = "`write(|w| ..)` method takes [bkp26r::W](bkp26r::W) writer structure"]
impl crate::Writable for BKP26R {}
#[doc = "RTC backup registers"]
pub mod bkp26r;
#[doc = "RTC backup registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp27r](bkp27r) module"]
pub type BKP27R = crate::Reg<u32, _BKP27R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP27R;
#[doc = "`read()` method returns [bkp27r::R](bkp27r::R) reader structure"]
impl crate::Readable for BKP27R {}
#[doc = "`write(|w| ..)` method takes [bkp27r::W](bkp27r::W) writer structure"]
impl crate::Writable for BKP27R {}
#[doc = "RTC backup registers"]
pub mod bkp27r;
#[doc = "RTC backup registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp28r](bkp28r) module"]
pub type BKP28R = crate::Reg<u32, _BKP28R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP28R;
#[doc = "`read()` method returns [bkp28r::R](bkp28r::R) reader structure"]
impl crate::Readable for BKP28R {}
#[doc = "`write(|w| ..)` method takes [bkp28r::W](bkp28r::W) writer structure"]
impl crate::Writable for BKP28R {}
#[doc = "RTC backup registers"]
pub mod bkp28r;
#[doc = "RTC backup registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp29r](bkp29r) module"]
pub type BKP29R = crate::Reg<u32, _BKP29R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP29R;
#[doc = "`read()` method returns [bkp29r::R](bkp29r::R) reader structure"]
impl crate::Readable for BKP29R {}
#[doc = "`write(|w| ..)` method takes [bkp29r::W](bkp29r::W) writer structure"]
impl crate::Writable for BKP29R {}
#[doc = "RTC backup registers"]
pub mod bkp29r;
#[doc = "RTC backup registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp30r](bkp30r) module"]
pub type BKP30R = crate::Reg<u32, _BKP30R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP30R;
#[doc = "`read()` method returns [bkp30r::R](bkp30r::R) reader structure"]
impl crate::Readable for BKP30R {}
#[doc = "`write(|w| ..)` method takes [bkp30r::W](bkp30r::W) writer structure"]
impl crate::Writable for BKP30R {}
#[doc = "RTC backup registers"]
pub mod bkp30r;
#[doc = "RTC backup registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp31r](bkp31r) module"]
pub type BKP31R = crate::Reg<u32, _BKP31R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP31R;
#[doc = "`read()` method returns [bkp31r::R](bkp31r::R) reader structure"]
impl crate::Readable for BKP31R {}
#[doc = "`write(|w| ..)` method takes [bkp31r::W](bkp31r::W) writer structure"]
impl crate::Writable for BKP31R {}
#[doc = "RTC backup registers"]
pub mod bkp31r;
