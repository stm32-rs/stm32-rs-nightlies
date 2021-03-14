#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - time register"]
    pub tr: TR,
    #[doc = "0x04 - date register"]
    pub dr: DR,
    #[doc = "0x08 - control register"]
    pub cr: CR,
    #[doc = "0x0c - initialization and status register"]
    pub isr: ISR,
    #[doc = "0x10 - prescaler register"]
    pub prer: PRER,
    #[doc = "0x14 - wakeup timer register"]
    pub wutr: WUTR,
    _reserved6: [u8; 4usize],
    #[doc = "0x1c - alarm A register"]
    pub alrmar: ALRMAR,
    #[doc = "0x20 - alarm B register"]
    pub alrmbr: ALRMBR,
    #[doc = "0x24 - write protection register"]
    pub wpr: WPR,
    #[doc = "0x28 - sub second register"]
    pub ssr: SSR,
    #[doc = "0x2c - shift control register"]
    pub shiftr: SHIFTR,
    #[doc = "0x30 - time stamp time register"]
    pub tstr: TSTR,
    #[doc = "0x34 - time stamp date register"]
    pub tsdr: TSDR,
    #[doc = "0x38 - timestamp sub second register"]
    pub tsssr: TSSSR,
    #[doc = "0x3c - calibration register"]
    pub calr: CALR,
    #[doc = "0x40 - tamper configuration register"]
    pub tampcr: TAMPCR,
    #[doc = "0x44 - alarm A sub second register"]
    pub alrmassr: ALRMASSR,
    #[doc = "0x48 - alarm B sub second register"]
    pub alrmbssr: ALRMBSSR,
    #[doc = "0x4c - option register"]
    pub or: OR,
    #[doc = "0x50 - backup register"]
    pub bkp0r: BKP0R,
    #[doc = "0x54 - backup register"]
    pub bkp1r: BKP1R,
    #[doc = "0x58 - backup register"]
    pub bkp2r: BKP2R,
    #[doc = "0x5c - backup register"]
    pub bkp3r: BKP3R,
    #[doc = "0x60 - backup register"]
    pub bkp4r: BKP4R,
    #[doc = "0x64 - backup register"]
    pub bkp5r: BKP5R,
    #[doc = "0x68 - backup register"]
    pub bkp6r: BKP6R,
    #[doc = "0x6c - backup register"]
    pub bkp7r: BKP7R,
    #[doc = "0x70 - backup register"]
    pub bkp8r: BKP8R,
    #[doc = "0x74 - backup register"]
    pub bkp9r: BKP9R,
    #[doc = "0x78 - backup register"]
    pub bkp10r: BKP10R,
    #[doc = "0x7c - backup register"]
    pub bkp11r: BKP11R,
    #[doc = "0x80 - backup register"]
    pub bkp12r: BKP12R,
    #[doc = "0x84 - backup register"]
    pub bkp13r: BKP13R,
    #[doc = "0x88 - backup register"]
    pub bkp14r: BKP14R,
    #[doc = "0x8c - backup register"]
    pub bkp15r: BKP15R,
    #[doc = "0x90 - backup register"]
    pub bkp16r: BKP16R,
    #[doc = "0x94 - backup register"]
    pub bkp17r: BKP17R,
    #[doc = "0x98 - backup register"]
    pub bkp18r: BKP18R,
    #[doc = "0x9c - backup register"]
    pub bkp19r: BKP19R,
}
#[doc = "time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr](tr) module"]
pub type TR = crate::Reg<u32, _TR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TR;
#[doc = "`read()` method returns [tr::R](tr::R) reader structure"]
impl crate::Readable for TR {}
#[doc = "`write(|w| ..)` method takes [tr::W](tr::W) writer structure"]
impl crate::Writable for TR {}
#[doc = "time register"]
pub mod tr;
#[doc = "date register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr](dr) module"]
pub type DR = crate::Reg<u32, _DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR;
#[doc = "`read()` method returns [dr::R](dr::R) reader structure"]
impl crate::Readable for DR {}
#[doc = "`write(|w| ..)` method takes [dr::W](dr::W) writer structure"]
impl crate::Writable for DR {}
#[doc = "date register"]
pub mod dr;
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "control register"]
pub mod cr;
#[doc = "initialization and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "`write(|w| ..)` method takes [isr::W](isr::W) writer structure"]
impl crate::Writable for ISR {}
#[doc = "initialization and status register"]
pub mod isr;
#[doc = "prescaler register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prer](prer) module"]
pub type PRER = crate::Reg<u32, _PRER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRER;
#[doc = "`read()` method returns [prer::R](prer::R) reader structure"]
impl crate::Readable for PRER {}
#[doc = "`write(|w| ..)` method takes [prer::W](prer::W) writer structure"]
impl crate::Writable for PRER {}
#[doc = "prescaler register"]
pub mod prer;
#[doc = "wakeup timer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wutr](wutr) module"]
pub type WUTR = crate::Reg<u32, _WUTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WUTR;
#[doc = "`read()` method returns [wutr::R](wutr::R) reader structure"]
impl crate::Readable for WUTR {}
#[doc = "`write(|w| ..)` method takes [wutr::W](wutr::W) writer structure"]
impl crate::Writable for WUTR {}
#[doc = "wakeup timer register"]
pub mod wutr;
#[doc = "alarm A register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrmar](alrmar) module"]
pub type ALRMAR = crate::Reg<u32, _ALRMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALRMAR;
#[doc = "`read()` method returns [alrmar::R](alrmar::R) reader structure"]
impl crate::Readable for ALRMAR {}
#[doc = "`write(|w| ..)` method takes [alrmar::W](alrmar::W) writer structure"]
impl crate::Writable for ALRMAR {}
#[doc = "alarm A register"]
pub mod alrmar;
#[doc = "alarm B register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrmbr](alrmbr) module"]
pub type ALRMBR = crate::Reg<u32, _ALRMBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALRMBR;
#[doc = "`read()` method returns [alrmbr::R](alrmbr::R) reader structure"]
impl crate::Readable for ALRMBR {}
#[doc = "`write(|w| ..)` method takes [alrmbr::W](alrmbr::W) writer structure"]
impl crate::Writable for ALRMBR {}
#[doc = "alarm B register"]
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
#[doc = "sub second register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssr](ssr) module"]
pub type SSR = crate::Reg<u32, _SSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSR;
#[doc = "`read()` method returns [ssr::R](ssr::R) reader structure"]
impl crate::Readable for SSR {}
#[doc = "sub second register"]
pub mod ssr;
#[doc = "shift control register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftr](shiftr) module"]
pub type SHIFTR = crate::Reg<u32, _SHIFTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTR;
#[doc = "`write(|w| ..)` method takes [shiftr::W](shiftr::W) writer structure"]
impl crate::Writable for SHIFTR {}
#[doc = "shift control register"]
pub mod shiftr;
#[doc = "time stamp time register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tstr](tstr) module"]
pub type TSTR = crate::Reg<u32, _TSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSTR;
#[doc = "`read()` method returns [tstr::R](tstr::R) reader structure"]
impl crate::Readable for TSTR {}
#[doc = "time stamp time register"]
pub mod tstr;
#[doc = "time stamp date register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsdr](tsdr) module"]
pub type TSDR = crate::Reg<u32, _TSDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSDR;
#[doc = "`read()` method returns [tsdr::R](tsdr::R) reader structure"]
impl crate::Readable for TSDR {}
#[doc = "time stamp date register"]
pub mod tsdr;
#[doc = "timestamp sub second register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsssr](tsssr) module"]
pub type TSSSR = crate::Reg<u32, _TSSSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSSSR;
#[doc = "`read()` method returns [tsssr::R](tsssr::R) reader structure"]
impl crate::Readable for TSSSR {}
#[doc = "timestamp sub second register"]
pub mod tsssr;
#[doc = "calibration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calr](calr) module"]
pub type CALR = crate::Reg<u32, _CALR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALR;
#[doc = "`read()` method returns [calr::R](calr::R) reader structure"]
impl crate::Readable for CALR {}
#[doc = "`write(|w| ..)` method takes [calr::W](calr::W) writer structure"]
impl crate::Writable for CALR {}
#[doc = "calibration register"]
pub mod calr;
#[doc = "tamper configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tampcr](tampcr) module"]
pub type TAMPCR = crate::Reg<u32, _TAMPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMPCR;
#[doc = "`read()` method returns [tampcr::R](tampcr::R) reader structure"]
impl crate::Readable for TAMPCR {}
#[doc = "`write(|w| ..)` method takes [tampcr::W](tampcr::W) writer structure"]
impl crate::Writable for TAMPCR {}
#[doc = "tamper configuration register"]
pub mod tampcr;
#[doc = "alarm A sub second register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrmassr](alrmassr) module"]
pub type ALRMASSR = crate::Reg<u32, _ALRMASSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALRMASSR;
#[doc = "`read()` method returns [alrmassr::R](alrmassr::R) reader structure"]
impl crate::Readable for ALRMASSR {}
#[doc = "`write(|w| ..)` method takes [alrmassr::W](alrmassr::W) writer structure"]
impl crate::Writable for ALRMASSR {}
#[doc = "alarm A sub second register"]
pub mod alrmassr;
#[doc = "alarm B sub second register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrmbssr](alrmbssr) module"]
pub type ALRMBSSR = crate::Reg<u32, _ALRMBSSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALRMBSSR;
#[doc = "`read()` method returns [alrmbssr::R](alrmbssr::R) reader structure"]
impl crate::Readable for ALRMBSSR {}
#[doc = "`write(|w| ..)` method takes [alrmbssr::W](alrmbssr::W) writer structure"]
impl crate::Writable for ALRMBSSR {}
#[doc = "alarm B sub second register"]
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
#[doc = "backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp0r](bkp0r) module"]
pub type BKP0R = crate::Reg<u32, _BKP0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP0R;
#[doc = "`read()` method returns [bkp0r::R](bkp0r::R) reader structure"]
impl crate::Readable for BKP0R {}
#[doc = "`write(|w| ..)` method takes [bkp0r::W](bkp0r::W) writer structure"]
impl crate::Writable for BKP0R {}
#[doc = "backup register"]
pub mod bkp0r;
#[doc = "backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp1r](bkp1r) module"]
pub type BKP1R = crate::Reg<u32, _BKP1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP1R;
#[doc = "`read()` method returns [bkp1r::R](bkp1r::R) reader structure"]
impl crate::Readable for BKP1R {}
#[doc = "`write(|w| ..)` method takes [bkp1r::W](bkp1r::W) writer structure"]
impl crate::Writable for BKP1R {}
#[doc = "backup register"]
pub mod bkp1r;
#[doc = "backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp2r](bkp2r) module"]
pub type BKP2R = crate::Reg<u32, _BKP2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP2R;
#[doc = "`read()` method returns [bkp2r::R](bkp2r::R) reader structure"]
impl crate::Readable for BKP2R {}
#[doc = "`write(|w| ..)` method takes [bkp2r::W](bkp2r::W) writer structure"]
impl crate::Writable for BKP2R {}
#[doc = "backup register"]
pub mod bkp2r;
#[doc = "backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp3r](bkp3r) module"]
pub type BKP3R = crate::Reg<u32, _BKP3R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP3R;
#[doc = "`read()` method returns [bkp3r::R](bkp3r::R) reader structure"]
impl crate::Readable for BKP3R {}
#[doc = "`write(|w| ..)` method takes [bkp3r::W](bkp3r::W) writer structure"]
impl crate::Writable for BKP3R {}
#[doc = "backup register"]
pub mod bkp3r;
#[doc = "backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp4r](bkp4r) module"]
pub type BKP4R = crate::Reg<u32, _BKP4R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP4R;
#[doc = "`read()` method returns [bkp4r::R](bkp4r::R) reader structure"]
impl crate::Readable for BKP4R {}
#[doc = "`write(|w| ..)` method takes [bkp4r::W](bkp4r::W) writer structure"]
impl crate::Writable for BKP4R {}
#[doc = "backup register"]
pub mod bkp4r;
#[doc = "backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp5r](bkp5r) module"]
pub type BKP5R = crate::Reg<u32, _BKP5R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP5R;
#[doc = "`read()` method returns [bkp5r::R](bkp5r::R) reader structure"]
impl crate::Readable for BKP5R {}
#[doc = "`write(|w| ..)` method takes [bkp5r::W](bkp5r::W) writer structure"]
impl crate::Writable for BKP5R {}
#[doc = "backup register"]
pub mod bkp5r;
#[doc = "backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp6r](bkp6r) module"]
pub type BKP6R = crate::Reg<u32, _BKP6R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP6R;
#[doc = "`read()` method returns [bkp6r::R](bkp6r::R) reader structure"]
impl crate::Readable for BKP6R {}
#[doc = "`write(|w| ..)` method takes [bkp6r::W](bkp6r::W) writer structure"]
impl crate::Writable for BKP6R {}
#[doc = "backup register"]
pub mod bkp6r;
#[doc = "backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp7r](bkp7r) module"]
pub type BKP7R = crate::Reg<u32, _BKP7R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP7R;
#[doc = "`read()` method returns [bkp7r::R](bkp7r::R) reader structure"]
impl crate::Readable for BKP7R {}
#[doc = "`write(|w| ..)` method takes [bkp7r::W](bkp7r::W) writer structure"]
impl crate::Writable for BKP7R {}
#[doc = "backup register"]
pub mod bkp7r;
#[doc = "backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp8r](bkp8r) module"]
pub type BKP8R = crate::Reg<u32, _BKP8R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP8R;
#[doc = "`read()` method returns [bkp8r::R](bkp8r::R) reader structure"]
impl crate::Readable for BKP8R {}
#[doc = "`write(|w| ..)` method takes [bkp8r::W](bkp8r::W) writer structure"]
impl crate::Writable for BKP8R {}
#[doc = "backup register"]
pub mod bkp8r;
#[doc = "backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp9r](bkp9r) module"]
pub type BKP9R = crate::Reg<u32, _BKP9R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP9R;
#[doc = "`read()` method returns [bkp9r::R](bkp9r::R) reader structure"]
impl crate::Readable for BKP9R {}
#[doc = "`write(|w| ..)` method takes [bkp9r::W](bkp9r::W) writer structure"]
impl crate::Writable for BKP9R {}
#[doc = "backup register"]
pub mod bkp9r;
#[doc = "backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp10r](bkp10r) module"]
pub type BKP10R = crate::Reg<u32, _BKP10R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP10R;
#[doc = "`read()` method returns [bkp10r::R](bkp10r::R) reader structure"]
impl crate::Readable for BKP10R {}
#[doc = "`write(|w| ..)` method takes [bkp10r::W](bkp10r::W) writer structure"]
impl crate::Writable for BKP10R {}
#[doc = "backup register"]
pub mod bkp10r;
#[doc = "backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp11r](bkp11r) module"]
pub type BKP11R = crate::Reg<u32, _BKP11R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP11R;
#[doc = "`read()` method returns [bkp11r::R](bkp11r::R) reader structure"]
impl crate::Readable for BKP11R {}
#[doc = "`write(|w| ..)` method takes [bkp11r::W](bkp11r::W) writer structure"]
impl crate::Writable for BKP11R {}
#[doc = "backup register"]
pub mod bkp11r;
#[doc = "backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp12r](bkp12r) module"]
pub type BKP12R = crate::Reg<u32, _BKP12R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP12R;
#[doc = "`read()` method returns [bkp12r::R](bkp12r::R) reader structure"]
impl crate::Readable for BKP12R {}
#[doc = "`write(|w| ..)` method takes [bkp12r::W](bkp12r::W) writer structure"]
impl crate::Writable for BKP12R {}
#[doc = "backup register"]
pub mod bkp12r;
#[doc = "backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp13r](bkp13r) module"]
pub type BKP13R = crate::Reg<u32, _BKP13R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP13R;
#[doc = "`read()` method returns [bkp13r::R](bkp13r::R) reader structure"]
impl crate::Readable for BKP13R {}
#[doc = "`write(|w| ..)` method takes [bkp13r::W](bkp13r::W) writer structure"]
impl crate::Writable for BKP13R {}
#[doc = "backup register"]
pub mod bkp13r;
#[doc = "backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp14r](bkp14r) module"]
pub type BKP14R = crate::Reg<u32, _BKP14R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP14R;
#[doc = "`read()` method returns [bkp14r::R](bkp14r::R) reader structure"]
impl crate::Readable for BKP14R {}
#[doc = "`write(|w| ..)` method takes [bkp14r::W](bkp14r::W) writer structure"]
impl crate::Writable for BKP14R {}
#[doc = "backup register"]
pub mod bkp14r;
#[doc = "backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp15r](bkp15r) module"]
pub type BKP15R = crate::Reg<u32, _BKP15R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP15R;
#[doc = "`read()` method returns [bkp15r::R](bkp15r::R) reader structure"]
impl crate::Readable for BKP15R {}
#[doc = "`write(|w| ..)` method takes [bkp15r::W](bkp15r::W) writer structure"]
impl crate::Writable for BKP15R {}
#[doc = "backup register"]
pub mod bkp15r;
#[doc = "backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp16r](bkp16r) module"]
pub type BKP16R = crate::Reg<u32, _BKP16R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP16R;
#[doc = "`read()` method returns [bkp16r::R](bkp16r::R) reader structure"]
impl crate::Readable for BKP16R {}
#[doc = "`write(|w| ..)` method takes [bkp16r::W](bkp16r::W) writer structure"]
impl crate::Writable for BKP16R {}
#[doc = "backup register"]
pub mod bkp16r;
#[doc = "backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp17r](bkp17r) module"]
pub type BKP17R = crate::Reg<u32, _BKP17R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP17R;
#[doc = "`read()` method returns [bkp17r::R](bkp17r::R) reader structure"]
impl crate::Readable for BKP17R {}
#[doc = "`write(|w| ..)` method takes [bkp17r::W](bkp17r::W) writer structure"]
impl crate::Writable for BKP17R {}
#[doc = "backup register"]
pub mod bkp17r;
#[doc = "backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp18r](bkp18r) module"]
pub type BKP18R = crate::Reg<u32, _BKP18R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP18R;
#[doc = "`read()` method returns [bkp18r::R](bkp18r::R) reader structure"]
impl crate::Readable for BKP18R {}
#[doc = "`write(|w| ..)` method takes [bkp18r::W](bkp18r::W) writer structure"]
impl crate::Writable for BKP18R {}
#[doc = "backup register"]
pub mod bkp18r;
#[doc = "backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp19r](bkp19r) module"]
pub type BKP19R = crate::Reg<u32, _BKP19R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP19R;
#[doc = "`read()` method returns [bkp19r::R](bkp19r::R) reader structure"]
impl crate::Readable for BKP19R {}
#[doc = "`write(|w| ..)` method takes [bkp19r::W](bkp19r::W) writer structure"]
impl crate::Writable for BKP19R {}
#[doc = "backup register"]
pub mod bkp19r;
