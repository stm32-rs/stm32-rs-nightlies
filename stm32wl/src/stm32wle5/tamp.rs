#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 1"]
    pub cr1: CR1,
    #[doc = "0x04 - control register 2"]
    pub cr2: CR2,
    #[doc = "0x08 - TAMP control register 3"]
    pub cr3: CR3,
    #[doc = "0x0c - TAMP filter control register"]
    pub fltcr: FLTCR,
    _reserved4: [u8; 28usize],
    #[doc = "0x2c - TAMP interrupt enable register"]
    pub ier: IER,
    #[doc = "0x30 - TAMP status register"]
    pub sr: SR,
    #[doc = "0x34 - TAMP masked interrupt status register"]
    pub misr: MISR,
    _reserved7: [u8; 4usize],
    #[doc = "0x3c - TAMP status clear register"]
    pub scr: SCR,
    #[doc = "0x40 - monotonic counter register"]
    pub countr: COUNTR,
    _reserved9: [u8; 188usize],
    #[doc = "0x100 - TAMP backup register"]
    pub bkp0r: BKP0R,
    #[doc = "0x104 - TAMP backup register"]
    pub bkp1r: BKP1R,
    #[doc = "0x108 - TAMP backup register"]
    pub bkp2r: BKP2R,
    #[doc = "0x10c - TAMP backup register"]
    pub bkp3r: BKP3R,
    #[doc = "0x110 - TAMP backup register"]
    pub bkp4r: BKP4R,
    #[doc = "0x114 - TAMP backup register"]
    pub bkp5r: BKP5R,
    #[doc = "0x118 - TAMP backup register"]
    pub bkp6r: BKP6R,
    #[doc = "0x11c - TAMP backup register"]
    pub bkp7r: BKP7R,
    #[doc = "0x120 - TAMP backup register"]
    pub bkp8r: BKP8R,
    #[doc = "0x124 - TAMP backup register"]
    pub bkp9r: BKP9R,
    _reserved19: [u8; 24usize],
    #[doc = "0x140 - TAMP backup register"]
    pub bkp10r: BKP10R,
    #[doc = "0x144 - TAMP backup register"]
    pub bkp11r: BKP11R,
    #[doc = "0x148 - TAMP backup register"]
    pub bkp12r: BKP12R,
    #[doc = "0x14c - TAMP backup register"]
    pub bkp13r: BKP13R,
    #[doc = "0x150 - TAMP backup register"]
    pub bkp14r: BKP14R,
    #[doc = "0x154 - TAMP backup register"]
    pub bkp15r: BKP15R,
    #[doc = "0x158 - TAMP backup register"]
    pub bkp16r: BKP16R,
    #[doc = "0x15c - TAMP backup register"]
    pub bkp17r: BKP17R,
    #[doc = "0x160 - TAMP backup register"]
    pub bkp18r: BKP18R,
    #[doc = "0x164 - TAMP backup register"]
    pub bkp19r: BKP19R,
}
#[doc = "control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](cr1) module"]
pub type CR1 = crate::Reg<u32, _CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR1;
#[doc = "`read()` method returns [cr1::R](cr1::R) reader structure"]
impl crate::Readable for CR1 {}
#[doc = "`write(|w| ..)` method takes [cr1::W](cr1::W) writer structure"]
impl crate::Writable for CR1 {}
#[doc = "control register 1"]
pub mod cr1;
#[doc = "control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](cr2) module"]
pub type CR2 = crate::Reg<u32, _CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR2;
#[doc = "`read()` method returns [cr2::R](cr2::R) reader structure"]
impl crate::Readable for CR2 {}
#[doc = "`write(|w| ..)` method takes [cr2::W](cr2::W) writer structure"]
impl crate::Writable for CR2 {}
#[doc = "control register 2"]
pub mod cr2;
#[doc = "TAMP control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr3](cr3) module"]
pub type CR3 = crate::Reg<u32, _CR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR3;
#[doc = "`read()` method returns [cr3::R](cr3::R) reader structure"]
impl crate::Readable for CR3 {}
#[doc = "`write(|w| ..)` method takes [cr3::W](cr3::W) writer structure"]
impl crate::Writable for CR3 {}
#[doc = "TAMP control register 3"]
pub mod cr3;
#[doc = "TAMP filter control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fltcr](fltcr) module"]
pub type FLTCR = crate::Reg<u32, _FLTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLTCR;
#[doc = "`read()` method returns [fltcr::R](fltcr::R) reader structure"]
impl crate::Readable for FLTCR {}
#[doc = "`write(|w| ..)` method takes [fltcr::W](fltcr::W) writer structure"]
impl crate::Writable for FLTCR {}
#[doc = "TAMP filter control register"]
pub mod fltcr;
#[doc = "TAMP interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`read()` method returns [ier::R](ier::R) reader structure"]
impl crate::Readable for IER {}
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "TAMP interrupt enable register"]
pub mod ier;
#[doc = "TAMP status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "TAMP status register"]
pub mod sr;
#[doc = "TAMP masked interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misr](misr) module"]
pub type MISR = crate::Reg<u32, _MISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISR;
#[doc = "`read()` method returns [misr::R](misr::R) reader structure"]
impl crate::Readable for MISR {}
#[doc = "TAMP masked interrupt status register"]
pub mod misr;
#[doc = "TAMP status clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](scr) module"]
pub type SCR = crate::Reg<u32, _SCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR;
#[doc = "`write(|w| ..)` method takes [scr::W](scr::W) writer structure"]
impl crate::Writable for SCR {}
#[doc = "TAMP status clear register"]
pub mod scr;
#[doc = "monotonic counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [countr](countr) module"]
pub type COUNTR = crate::Reg<u32, _COUNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNTR;
#[doc = "`read()` method returns [countr::R](countr::R) reader structure"]
impl crate::Readable for COUNTR {}
#[doc = "monotonic counter register"]
pub mod countr;
#[doc = "TAMP backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp0r](bkp0r) module"]
pub type BKP0R = crate::Reg<u32, _BKP0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP0R;
#[doc = "`read()` method returns [bkp0r::R](bkp0r::R) reader structure"]
impl crate::Readable for BKP0R {}
#[doc = "`write(|w| ..)` method takes [bkp0r::W](bkp0r::W) writer structure"]
impl crate::Writable for BKP0R {}
#[doc = "TAMP backup register"]
pub mod bkp0r;
#[doc = "TAMP backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp1r](bkp1r) module"]
pub type BKP1R = crate::Reg<u32, _BKP1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP1R;
#[doc = "`read()` method returns [bkp1r::R](bkp1r::R) reader structure"]
impl crate::Readable for BKP1R {}
#[doc = "`write(|w| ..)` method takes [bkp1r::W](bkp1r::W) writer structure"]
impl crate::Writable for BKP1R {}
#[doc = "TAMP backup register"]
pub mod bkp1r;
#[doc = "TAMP backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp2r](bkp2r) module"]
pub type BKP2R = crate::Reg<u32, _BKP2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP2R;
#[doc = "`read()` method returns [bkp2r::R](bkp2r::R) reader structure"]
impl crate::Readable for BKP2R {}
#[doc = "`write(|w| ..)` method takes [bkp2r::W](bkp2r::W) writer structure"]
impl crate::Writable for BKP2R {}
#[doc = "TAMP backup register"]
pub mod bkp2r;
#[doc = "TAMP backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp3r](bkp3r) module"]
pub type BKP3R = crate::Reg<u32, _BKP3R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP3R;
#[doc = "`read()` method returns [bkp3r::R](bkp3r::R) reader structure"]
impl crate::Readable for BKP3R {}
#[doc = "`write(|w| ..)` method takes [bkp3r::W](bkp3r::W) writer structure"]
impl crate::Writable for BKP3R {}
#[doc = "TAMP backup register"]
pub mod bkp3r;
#[doc = "TAMP backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp4r](bkp4r) module"]
pub type BKP4R = crate::Reg<u32, _BKP4R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP4R;
#[doc = "`read()` method returns [bkp4r::R](bkp4r::R) reader structure"]
impl crate::Readable for BKP4R {}
#[doc = "`write(|w| ..)` method takes [bkp4r::W](bkp4r::W) writer structure"]
impl crate::Writable for BKP4R {}
#[doc = "TAMP backup register"]
pub mod bkp4r;
#[doc = "TAMP backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp5r](bkp5r) module"]
pub type BKP5R = crate::Reg<u32, _BKP5R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP5R;
#[doc = "`read()` method returns [bkp5r::R](bkp5r::R) reader structure"]
impl crate::Readable for BKP5R {}
#[doc = "`write(|w| ..)` method takes [bkp5r::W](bkp5r::W) writer structure"]
impl crate::Writable for BKP5R {}
#[doc = "TAMP backup register"]
pub mod bkp5r;
#[doc = "TAMP backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp6r](bkp6r) module"]
pub type BKP6R = crate::Reg<u32, _BKP6R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP6R;
#[doc = "`read()` method returns [bkp6r::R](bkp6r::R) reader structure"]
impl crate::Readable for BKP6R {}
#[doc = "`write(|w| ..)` method takes [bkp6r::W](bkp6r::W) writer structure"]
impl crate::Writable for BKP6R {}
#[doc = "TAMP backup register"]
pub mod bkp6r;
#[doc = "TAMP backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp7r](bkp7r) module"]
pub type BKP7R = crate::Reg<u32, _BKP7R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP7R;
#[doc = "`read()` method returns [bkp7r::R](bkp7r::R) reader structure"]
impl crate::Readable for BKP7R {}
#[doc = "`write(|w| ..)` method takes [bkp7r::W](bkp7r::W) writer structure"]
impl crate::Writable for BKP7R {}
#[doc = "TAMP backup register"]
pub mod bkp7r;
#[doc = "TAMP backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp8r](bkp8r) module"]
pub type BKP8R = crate::Reg<u32, _BKP8R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP8R;
#[doc = "`read()` method returns [bkp8r::R](bkp8r::R) reader structure"]
impl crate::Readable for BKP8R {}
#[doc = "`write(|w| ..)` method takes [bkp8r::W](bkp8r::W) writer structure"]
impl crate::Writable for BKP8R {}
#[doc = "TAMP backup register"]
pub mod bkp8r;
#[doc = "TAMP backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp9r](bkp9r) module"]
pub type BKP9R = crate::Reg<u32, _BKP9R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP9R;
#[doc = "`read()` method returns [bkp9r::R](bkp9r::R) reader structure"]
impl crate::Readable for BKP9R {}
#[doc = "`write(|w| ..)` method takes [bkp9r::W](bkp9r::W) writer structure"]
impl crate::Writable for BKP9R {}
#[doc = "TAMP backup register"]
pub mod bkp9r;
#[doc = "TAMP backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp10r](bkp10r) module"]
pub type BKP10R = crate::Reg<u32, _BKP10R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP10R;
#[doc = "`read()` method returns [bkp10r::R](bkp10r::R) reader structure"]
impl crate::Readable for BKP10R {}
#[doc = "`write(|w| ..)` method takes [bkp10r::W](bkp10r::W) writer structure"]
impl crate::Writable for BKP10R {}
#[doc = "TAMP backup register"]
pub mod bkp10r;
#[doc = "TAMP backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp11r](bkp11r) module"]
pub type BKP11R = crate::Reg<u32, _BKP11R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP11R;
#[doc = "`read()` method returns [bkp11r::R](bkp11r::R) reader structure"]
impl crate::Readable for BKP11R {}
#[doc = "`write(|w| ..)` method takes [bkp11r::W](bkp11r::W) writer structure"]
impl crate::Writable for BKP11R {}
#[doc = "TAMP backup register"]
pub mod bkp11r;
#[doc = "TAMP backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp12r](bkp12r) module"]
pub type BKP12R = crate::Reg<u32, _BKP12R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP12R;
#[doc = "`read()` method returns [bkp12r::R](bkp12r::R) reader structure"]
impl crate::Readable for BKP12R {}
#[doc = "`write(|w| ..)` method takes [bkp12r::W](bkp12r::W) writer structure"]
impl crate::Writable for BKP12R {}
#[doc = "TAMP backup register"]
pub mod bkp12r;
#[doc = "TAMP backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp13r](bkp13r) module"]
pub type BKP13R = crate::Reg<u32, _BKP13R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP13R;
#[doc = "`read()` method returns [bkp13r::R](bkp13r::R) reader structure"]
impl crate::Readable for BKP13R {}
#[doc = "`write(|w| ..)` method takes [bkp13r::W](bkp13r::W) writer structure"]
impl crate::Writable for BKP13R {}
#[doc = "TAMP backup register"]
pub mod bkp13r;
#[doc = "TAMP backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp14r](bkp14r) module"]
pub type BKP14R = crate::Reg<u32, _BKP14R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP14R;
#[doc = "`read()` method returns [bkp14r::R](bkp14r::R) reader structure"]
impl crate::Readable for BKP14R {}
#[doc = "`write(|w| ..)` method takes [bkp14r::W](bkp14r::W) writer structure"]
impl crate::Writable for BKP14R {}
#[doc = "TAMP backup register"]
pub mod bkp14r;
#[doc = "TAMP backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp15r](bkp15r) module"]
pub type BKP15R = crate::Reg<u32, _BKP15R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP15R;
#[doc = "`read()` method returns [bkp15r::R](bkp15r::R) reader structure"]
impl crate::Readable for BKP15R {}
#[doc = "`write(|w| ..)` method takes [bkp15r::W](bkp15r::W) writer structure"]
impl crate::Writable for BKP15R {}
#[doc = "TAMP backup register"]
pub mod bkp15r;
#[doc = "TAMP backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp16r](bkp16r) module"]
pub type BKP16R = crate::Reg<u32, _BKP16R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP16R;
#[doc = "`read()` method returns [bkp16r::R](bkp16r::R) reader structure"]
impl crate::Readable for BKP16R {}
#[doc = "`write(|w| ..)` method takes [bkp16r::W](bkp16r::W) writer structure"]
impl crate::Writable for BKP16R {}
#[doc = "TAMP backup register"]
pub mod bkp16r;
#[doc = "TAMP backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp17r](bkp17r) module"]
pub type BKP17R = crate::Reg<u32, _BKP17R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP17R;
#[doc = "`read()` method returns [bkp17r::R](bkp17r::R) reader structure"]
impl crate::Readable for BKP17R {}
#[doc = "`write(|w| ..)` method takes [bkp17r::W](bkp17r::W) writer structure"]
impl crate::Writable for BKP17R {}
#[doc = "TAMP backup register"]
pub mod bkp17r;
#[doc = "TAMP backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp18r](bkp18r) module"]
pub type BKP18R = crate::Reg<u32, _BKP18R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP18R;
#[doc = "`read()` method returns [bkp18r::R](bkp18r::R) reader structure"]
impl crate::Readable for BKP18R {}
#[doc = "`write(|w| ..)` method takes [bkp18r::W](bkp18r::W) writer structure"]
impl crate::Writable for BKP18R {}
#[doc = "TAMP backup register"]
pub mod bkp18r;
#[doc = "TAMP backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp19r](bkp19r) module"]
pub type BKP19R = crate::Reg<u32, _BKP19R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP19R;
#[doc = "`read()` method returns [bkp19r::R](bkp19r::R) reader structure"]
impl crate::Readable for BKP19R {}
#[doc = "`write(|w| ..)` method takes [bkp19r::W](bkp19r::W) writer structure"]
impl crate::Writable for BKP19R {}
#[doc = "TAMP backup register"]
pub mod bkp19r;
