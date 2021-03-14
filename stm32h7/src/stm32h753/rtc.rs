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
    pub bkpr: [BKPR; 32],
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
#[doc = "RTC backup registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkpr](bkpr) module"]
pub type BKPR = crate::Reg<u32, _BKPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKPR;
#[doc = "`read()` method returns [bkpr::R](bkpr::R) reader structure"]
impl crate::Readable for BKPR {}
#[doc = "`write(|w| ..)` method takes [bkpr::W](bkpr::W) writer structure"]
impl crate::Writable for BKPR {}
#[doc = "RTC backup registers"]
pub mod bkpr;
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
