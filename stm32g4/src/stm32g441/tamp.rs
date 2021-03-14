#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 1"]
    pub cr1: CR1,
    #[doc = "0x04 - control register 2"]
    pub cr2: CR2,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - TAMP filter control register"]
    pub fltcr: FLTCR,
    _reserved3: [u8; 28usize],
    #[doc = "0x2c - TAMP interrupt enable register"]
    pub ier: IER,
    #[doc = "0x30 - TAMP status register"]
    pub sr: SR,
    #[doc = "0x34 - TAMP masked interrupt status register"]
    pub misr: MISR,
    _reserved6: [u8; 4usize],
    #[doc = "0x3c - TAMP status clear register"]
    pub scr: SCR,
    _reserved7: [u8; 192usize],
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
    #[doc = "0x128 - TAMP backup register"]
    pub bkp10r: BKP10R,
    #[doc = "0x12c - TAMP backup register"]
    pub bkp11r: BKP11R,
    #[doc = "0x130 - TAMP backup register"]
    pub bkp12r: BKP12R,
    #[doc = "0x134 - TAMP backup register"]
    pub bkp13r: BKP13R,
    #[doc = "0x138 - TAMP backup register"]
    pub bkp14r: BKP14R,
    #[doc = "0x13c - TAMP backup register"]
    pub bkp15r: BKP15R,
    #[doc = "0x140 - TAMP backup register"]
    pub bkp16r: BKP16R,
    #[doc = "0x144 - TAMP backup register"]
    pub bkp17r: BKP17R,
    #[doc = "0x148 - TAMP backup register"]
    pub bkp18r: BKP18R,
    #[doc = "0x14c - TAMP backup register"]
    pub bkp19r: BKP19R,
    #[doc = "0x150 - TAMP backup register"]
    pub bkp20r: BKP20R,
    #[doc = "0x154 - TAMP backup register"]
    pub bkp21r: BKP21R,
    #[doc = "0x158 - TAMP backup register"]
    pub bkp22r: BKP22R,
    #[doc = "0x15c - TAMP backup register"]
    pub bkp23r: BKP23R,
    #[doc = "0x160 - TAMP backup register"]
    pub bkp24r: BKP24R,
    #[doc = "0x164 - TAMP backup register"]
    pub bkp25r: BKP25R,
    #[doc = "0x168 - TAMP backup register"]
    pub bkp26r: BKP26R,
    #[doc = "0x16c - TAMP backup register"]
    pub bkp27r: BKP27R,
    #[doc = "0x170 - TAMP backup register"]
    pub bkp28r: BKP28R,
    #[doc = "0x174 - TAMP backup register"]
    pub bkp29r: BKP29R,
    #[doc = "0x178 - TAMP backup register"]
    pub bkp30r: BKP30R,
    #[doc = "0x17c - TAMP backup register"]
    pub bkp31r: BKP31R,
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
#[doc = "TAMP status clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](scr) module"]
pub type SCR = crate::Reg<u32, _SCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR;
#[doc = "`read()` method returns [scr::R](scr::R) reader structure"]
impl crate::Readable for SCR {}
#[doc = "`write(|w| ..)` method takes [scr::W](scr::W) writer structure"]
impl crate::Writable for SCR {}
#[doc = "TAMP status clear register"]
pub mod scr;
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
#[doc = "TAMP backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp20r](bkp20r) module"]
pub type BKP20R = crate::Reg<u32, _BKP20R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP20R;
#[doc = "`read()` method returns [bkp20r::R](bkp20r::R) reader structure"]
impl crate::Readable for BKP20R {}
#[doc = "`write(|w| ..)` method takes [bkp20r::W](bkp20r::W) writer structure"]
impl crate::Writable for BKP20R {}
#[doc = "TAMP backup register"]
pub mod bkp20r;
#[doc = "TAMP backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp21r](bkp21r) module"]
pub type BKP21R = crate::Reg<u32, _BKP21R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP21R;
#[doc = "`read()` method returns [bkp21r::R](bkp21r::R) reader structure"]
impl crate::Readable for BKP21R {}
#[doc = "`write(|w| ..)` method takes [bkp21r::W](bkp21r::W) writer structure"]
impl crate::Writable for BKP21R {}
#[doc = "TAMP backup register"]
pub mod bkp21r;
#[doc = "TAMP backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp22r](bkp22r) module"]
pub type BKP22R = crate::Reg<u32, _BKP22R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP22R;
#[doc = "`read()` method returns [bkp22r::R](bkp22r::R) reader structure"]
impl crate::Readable for BKP22R {}
#[doc = "`write(|w| ..)` method takes [bkp22r::W](bkp22r::W) writer structure"]
impl crate::Writable for BKP22R {}
#[doc = "TAMP backup register"]
pub mod bkp22r;
#[doc = "TAMP backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp23r](bkp23r) module"]
pub type BKP23R = crate::Reg<u32, _BKP23R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP23R;
#[doc = "`read()` method returns [bkp23r::R](bkp23r::R) reader structure"]
impl crate::Readable for BKP23R {}
#[doc = "`write(|w| ..)` method takes [bkp23r::W](bkp23r::W) writer structure"]
impl crate::Writable for BKP23R {}
#[doc = "TAMP backup register"]
pub mod bkp23r;
#[doc = "TAMP backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp24r](bkp24r) module"]
pub type BKP24R = crate::Reg<u32, _BKP24R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP24R;
#[doc = "`read()` method returns [bkp24r::R](bkp24r::R) reader structure"]
impl crate::Readable for BKP24R {}
#[doc = "`write(|w| ..)` method takes [bkp24r::W](bkp24r::W) writer structure"]
impl crate::Writable for BKP24R {}
#[doc = "TAMP backup register"]
pub mod bkp24r;
#[doc = "TAMP backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp25r](bkp25r) module"]
pub type BKP25R = crate::Reg<u32, _BKP25R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP25R;
#[doc = "`read()` method returns [bkp25r::R](bkp25r::R) reader structure"]
impl crate::Readable for BKP25R {}
#[doc = "`write(|w| ..)` method takes [bkp25r::W](bkp25r::W) writer structure"]
impl crate::Writable for BKP25R {}
#[doc = "TAMP backup register"]
pub mod bkp25r;
#[doc = "TAMP backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp26r](bkp26r) module"]
pub type BKP26R = crate::Reg<u32, _BKP26R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP26R;
#[doc = "`read()` method returns [bkp26r::R](bkp26r::R) reader structure"]
impl crate::Readable for BKP26R {}
#[doc = "`write(|w| ..)` method takes [bkp26r::W](bkp26r::W) writer structure"]
impl crate::Writable for BKP26R {}
#[doc = "TAMP backup register"]
pub mod bkp26r;
#[doc = "TAMP backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp27r](bkp27r) module"]
pub type BKP27R = crate::Reg<u32, _BKP27R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP27R;
#[doc = "`read()` method returns [bkp27r::R](bkp27r::R) reader structure"]
impl crate::Readable for BKP27R {}
#[doc = "`write(|w| ..)` method takes [bkp27r::W](bkp27r::W) writer structure"]
impl crate::Writable for BKP27R {}
#[doc = "TAMP backup register"]
pub mod bkp27r;
#[doc = "TAMP backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp28r](bkp28r) module"]
pub type BKP28R = crate::Reg<u32, _BKP28R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP28R;
#[doc = "`read()` method returns [bkp28r::R](bkp28r::R) reader structure"]
impl crate::Readable for BKP28R {}
#[doc = "`write(|w| ..)` method takes [bkp28r::W](bkp28r::W) writer structure"]
impl crate::Writable for BKP28R {}
#[doc = "TAMP backup register"]
pub mod bkp28r;
#[doc = "TAMP backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp29r](bkp29r) module"]
pub type BKP29R = crate::Reg<u32, _BKP29R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP29R;
#[doc = "`read()` method returns [bkp29r::R](bkp29r::R) reader structure"]
impl crate::Readable for BKP29R {}
#[doc = "`write(|w| ..)` method takes [bkp29r::W](bkp29r::W) writer structure"]
impl crate::Writable for BKP29R {}
#[doc = "TAMP backup register"]
pub mod bkp29r;
#[doc = "TAMP backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp30r](bkp30r) module"]
pub type BKP30R = crate::Reg<u32, _BKP30R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP30R;
#[doc = "`read()` method returns [bkp30r::R](bkp30r::R) reader structure"]
impl crate::Readable for BKP30R {}
#[doc = "`write(|w| ..)` method takes [bkp30r::W](bkp30r::W) writer structure"]
impl crate::Writable for BKP30R {}
#[doc = "TAMP backup register"]
pub mod bkp30r;
#[doc = "TAMP backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp31r](bkp31r) module"]
pub type BKP31R = crate::Reg<u32, _BKP31R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP31R;
#[doc = "`read()` method returns [bkp31r::R](bkp31r::R) reader structure"]
impl crate::Readable for BKP31R {}
#[doc = "`write(|w| ..)` method takes [bkp31r::W](bkp31r::W) writer structure"]
impl crate::Writable for BKP31R {}
#[doc = "TAMP backup register"]
pub mod bkp31r;
