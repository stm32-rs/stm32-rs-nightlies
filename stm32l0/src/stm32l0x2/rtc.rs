#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC time register"]
    pub tr: TR,
    #[doc = "0x04 - RTC date register"]
    pub dr: DR,
    #[doc = "0x08 - RTC control register"]
    pub cr: CR,
    #[doc = "0x0c - RTC initialization and status register"]
    pub isr: ISR,
    #[doc = "0x10 - RTC prescaler register"]
    pub prer: PRER,
    #[doc = "0x14 - RTC wakeup timer register"]
    pub wutr: WUTR,
    _reserved6: [u8; 4usize],
    #[doc = "0x1c - RTC alarm A register"]
    pub alrmar: ALRMAR,
    #[doc = "0x20 - RTC alarm B register"]
    pub alrmbr: ALRMBR,
    #[doc = "0x24 - write protection register"]
    pub wpr: WPR,
    #[doc = "0x28 - RTC sub second register"]
    pub ssr: SSR,
    #[doc = "0x2c - RTC shift control register"]
    pub shiftr: SHIFTR,
    #[doc = "0x30 - RTC timestamp time register"]
    pub tstr: TSTR,
    #[doc = "0x34 - RTC timestamp date register"]
    pub tsdr: TSDR,
    #[doc = "0x38 - RTC time-stamp sub second register"]
    pub tsssr: TSSSR,
    #[doc = "0x3c - RTC calibration register"]
    pub calr: CALR,
    #[doc = "0x40 - RTC tamper configuration register"]
    pub tampcr: TAMPCR,
    #[doc = "0x44 - RTC alarm A sub second register"]
    pub alrmassr: ALRMASSR,
    #[doc = "0x48 - RTC alarm B sub second register"]
    pub alrmbssr: ALRMBSSR,
    #[doc = "0x4c - option register"]
    pub or: OR,
    #[doc = "0x50 - RTC backup registers"]
    pub bkpr: [BKPR; 5],
}
#[doc = "RTC time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr](tr) module"]
pub type TR = crate::Reg<u32, _TR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TR;
#[doc = "`read()` method returns [tr::R](tr::R) reader structure"]
impl crate::Readable for TR {}
#[doc = "`write(|w| ..)` method takes [tr::W](tr::W) writer structure"]
impl crate::Writable for TR {}
#[doc = "RTC time register"]
pub mod tr;
#[doc = "RTC date register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr](dr) module"]
pub type DR = crate::Reg<u32, _DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR;
#[doc = "`read()` method returns [dr::R](dr::R) reader structure"]
impl crate::Readable for DR {}
#[doc = "`write(|w| ..)` method takes [dr::W](dr::W) writer structure"]
impl crate::Writable for DR {}
#[doc = "RTC date register"]
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
#[doc = "RTC initialization and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "`write(|w| ..)` method takes [isr::W](isr::W) writer structure"]
impl crate::Writable for ISR {}
#[doc = "RTC initialization and status register"]
pub mod isr;
#[doc = "RTC prescaler register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prer](prer) module"]
pub type PRER = crate::Reg<u32, _PRER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRER;
#[doc = "`read()` method returns [prer::R](prer::R) reader structure"]
impl crate::Readable for PRER {}
#[doc = "`write(|w| ..)` method takes [prer::W](prer::W) writer structure"]
impl crate::Writable for PRER {}
#[doc = "RTC prescaler register"]
pub mod prer;
#[doc = "RTC wakeup timer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wutr](wutr) module"]
pub type WUTR = crate::Reg<u32, _WUTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WUTR;
#[doc = "`read()` method returns [wutr::R](wutr::R) reader structure"]
impl crate::Readable for WUTR {}
#[doc = "`write(|w| ..)` method takes [wutr::W](wutr::W) writer structure"]
impl crate::Writable for WUTR {}
#[doc = "RTC wakeup timer register"]
pub mod wutr;
#[doc = "RTC alarm A register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrmar](alrmar) module"]
pub type ALRMAR = crate::Reg<u32, _ALRMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALRMAR;
#[doc = "`read()` method returns [alrmar::R](alrmar::R) reader structure"]
impl crate::Readable for ALRMAR {}
#[doc = "`write(|w| ..)` method takes [alrmar::W](alrmar::W) writer structure"]
impl crate::Writable for ALRMAR {}
#[doc = "RTC alarm A register"]
pub mod alrmar;
#[doc = "RTC alarm B register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrmbr](alrmbr) module"]
pub type ALRMBR = crate::Reg<u32, _ALRMBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALRMBR;
#[doc = "`read()` method returns [alrmbr::R](alrmbr::R) reader structure"]
impl crate::Readable for ALRMBR {}
#[doc = "`write(|w| ..)` method takes [alrmbr::W](alrmbr::W) writer structure"]
impl crate::Writable for ALRMBR {}
#[doc = "RTC alarm B register"]
pub mod alrmbr;
#[doc = "write protection register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpr](wpr) module"]
pub type WPR = crate::Reg<u32, _WPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPR;
#[doc = "`write(|w| ..)` method takes [wpr::W](wpr::W) writer structure"]
impl crate::Writable for WPR {}
#[doc = "write protection register"]
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
#[doc = "RTC shift control register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftr](shiftr) module"]
pub type SHIFTR = crate::Reg<u32, _SHIFTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTR;
#[doc = "`write(|w| ..)` method takes [shiftr::W](shiftr::W) writer structure"]
impl crate::Writable for SHIFTR {}
#[doc = "RTC shift control register"]
pub mod shiftr;
#[doc = "RTC timestamp time register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tstr](tstr) module"]
pub type TSTR = crate::Reg<u32, _TSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSTR;
#[doc = "`read()` method returns [tstr::R](tstr::R) reader structure"]
impl crate::Readable for TSTR {}
#[doc = "RTC timestamp time register"]
pub mod tstr;
#[doc = "RTC timestamp date register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsdr](tsdr) module"]
pub type TSDR = crate::Reg<u32, _TSDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSDR;
#[doc = "`read()` method returns [tsdr::R](tsdr::R) reader structure"]
impl crate::Readable for TSDR {}
#[doc = "RTC timestamp date register"]
pub mod tsdr;
#[doc = "RTC time-stamp sub second register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsssr](tsssr) module"]
pub type TSSSR = crate::Reg<u32, _TSSSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSSSR;
#[doc = "`read()` method returns [tsssr::R](tsssr::R) reader structure"]
impl crate::Readable for TSSSR {}
#[doc = "RTC time-stamp sub second register"]
pub mod tsssr;
#[doc = "RTC calibration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calr](calr) module"]
pub type CALR = crate::Reg<u32, _CALR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALR;
#[doc = "`read()` method returns [calr::R](calr::R) reader structure"]
impl crate::Readable for CALR {}
#[doc = "`write(|w| ..)` method takes [calr::W](calr::W) writer structure"]
impl crate::Writable for CALR {}
#[doc = "RTC calibration register"]
pub mod calr;
#[doc = "RTC tamper configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tampcr](tampcr) module"]
pub type TAMPCR = crate::Reg<u32, _TAMPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMPCR;
#[doc = "`read()` method returns [tampcr::R](tampcr::R) reader structure"]
impl crate::Readable for TAMPCR {}
#[doc = "`write(|w| ..)` method takes [tampcr::W](tampcr::W) writer structure"]
impl crate::Writable for TAMPCR {}
#[doc = "RTC tamper configuration register"]
pub mod tampcr;
#[doc = "RTC alarm A sub second register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrmassr](alrmassr) module"]
pub type ALRMASSR = crate::Reg<u32, _ALRMASSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALRMASSR;
#[doc = "`read()` method returns [alrmassr::R](alrmassr::R) reader structure"]
impl crate::Readable for ALRMASSR {}
#[doc = "`write(|w| ..)` method takes [alrmassr::W](alrmassr::W) writer structure"]
impl crate::Writable for ALRMASSR {}
#[doc = "RTC alarm A sub second register"]
pub mod alrmassr;
#[doc = "RTC alarm B sub second register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrmbssr](alrmbssr) module"]
pub type ALRMBSSR = crate::Reg<u32, _ALRMBSSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALRMBSSR;
#[doc = "`read()` method returns [alrmbssr::R](alrmbssr::R) reader structure"]
impl crate::Readable for ALRMBSSR {}
#[doc = "`write(|w| ..)` method takes [alrmbssr::W](alrmbssr::W) writer structure"]
impl crate::Writable for ALRMBSSR {}
#[doc = "RTC alarm B sub second register"]
pub mod alrmbssr;
#[doc = "option register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [or](or) module"]
pub type OR = crate::Reg<u32, _OR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OR;
#[doc = "`read()` method returns [or::R](or::R) reader structure"]
impl crate::Readable for OR {}
#[doc = "`write(|w| ..)` method takes [or::W](or::W) writer structure"]
impl crate::Writable for OR {}
#[doc = "option register"]
pub mod or;
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
