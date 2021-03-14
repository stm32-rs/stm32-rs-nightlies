#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub cr: CR,
    #[doc = "0x04 - status register"]
    pub sr: SR,
    #[doc = "0x08 - data input register"]
    pub din: DIN,
    #[doc = "0x0c - data output register"]
    pub dout: DOUT,
    #[doc = "0x10 - DMA control register"]
    pub dmacr: DMACR,
    #[doc = "0x14 - interrupt mask set/clear register"]
    pub imscr: IMSCR,
    #[doc = "0x18 - raw interrupt status register"]
    pub risr: RISR,
    #[doc = "0x1c - masked interrupt status register"]
    pub misr: MISR,
    #[doc = "0x20 - key registers"]
    pub k0lr: K0LR,
    #[doc = "0x24 - key registers"]
    pub k0rr: K0RR,
    #[doc = "0x28 - key registers"]
    pub k1lr: K1LR,
    #[doc = "0x2c - key registers"]
    pub k1rr: K1RR,
    #[doc = "0x30 - key registers"]
    pub k2lr: K2LR,
    #[doc = "0x34 - key registers"]
    pub k2rr: K2RR,
    #[doc = "0x38 - key registers"]
    pub k3lr: K3LR,
    #[doc = "0x3c - key registers"]
    pub k3rr: K3RR,
    #[doc = "0x40 - initialization vector registers"]
    pub iv0lr: IV0LR,
    #[doc = "0x44 - initialization vector registers"]
    pub iv0rr: IV0RR,
    #[doc = "0x48 - initialization vector registers"]
    pub iv1lr: IV1LR,
    #[doc = "0x4c - initialization vector registers"]
    pub iv1rr: IV1RR,
    #[doc = "0x50 - context swap register"]
    pub csgcmccm0r: CSGCMCCM0R,
    #[doc = "0x54 - context swap register"]
    pub csgcmccm1r: CSGCMCCM1R,
    #[doc = "0x58 - context swap register"]
    pub csgcmccm2r: CSGCMCCM2R,
    #[doc = "0x5c - context swap register"]
    pub csgcmccm3r: CSGCMCCM3R,
    #[doc = "0x60 - context swap register"]
    pub csgcmccm4r: CSGCMCCM4R,
    #[doc = "0x64 - context swap register"]
    pub csgcmccm5r: CSGCMCCM5R,
    #[doc = "0x68 - context swap register"]
    pub csgcmccm6r: CSGCMCCM6R,
    #[doc = "0x6c - context swap register"]
    pub csgcmccm7r: CSGCMCCM7R,
    #[doc = "0x70 - context swap register"]
    pub csgcm0r: CSGCM0R,
    #[doc = "0x74 - context swap register"]
    pub csgcm1r: CSGCM1R,
    #[doc = "0x78 - context swap register"]
    pub csgcm2r: CSGCM2R,
    #[doc = "0x7c - context swap register"]
    pub csgcm3r: CSGCM3R,
    #[doc = "0x80 - context swap register"]
    pub csgcm4r: CSGCM4R,
    #[doc = "0x84 - context swap register"]
    pub csgcm5r: CSGCM5R,
    #[doc = "0x88 - context swap register"]
    pub csgcm6r: CSGCM6R,
    #[doc = "0x8c - context swap register"]
    pub csgcm7r: CSGCM7R,
}
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
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "status register"]
pub mod sr;
#[doc = "data input register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [din](din) module"]
pub type DIN = crate::Reg<u32, _DIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIN;
#[doc = "`read()` method returns [din::R](din::R) reader structure"]
impl crate::Readable for DIN {}
#[doc = "`write(|w| ..)` method takes [din::W](din::W) writer structure"]
impl crate::Writable for DIN {}
#[doc = "data input register"]
pub mod din;
#[doc = "data output register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dout](dout) module"]
pub type DOUT = crate::Reg<u32, _DOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUT;
#[doc = "`read()` method returns [dout::R](dout::R) reader structure"]
impl crate::Readable for DOUT {}
#[doc = "data output register"]
pub mod dout;
#[doc = "DMA control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacr](dmacr) module"]
pub type DMACR = crate::Reg<u32, _DMACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACR;
#[doc = "`read()` method returns [dmacr::R](dmacr::R) reader structure"]
impl crate::Readable for DMACR {}
#[doc = "`write(|w| ..)` method takes [dmacr::W](dmacr::W) writer structure"]
impl crate::Writable for DMACR {}
#[doc = "DMA control register"]
pub mod dmacr;
#[doc = "interrupt mask set/clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imscr](imscr) module"]
pub type IMSCR = crate::Reg<u32, _IMSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMSCR;
#[doc = "`read()` method returns [imscr::R](imscr::R) reader structure"]
impl crate::Readable for IMSCR {}
#[doc = "`write(|w| ..)` method takes [imscr::W](imscr::W) writer structure"]
impl crate::Writable for IMSCR {}
#[doc = "interrupt mask set/clear register"]
pub mod imscr;
#[doc = "raw interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [risr](risr) module"]
pub type RISR = crate::Reg<u32, _RISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RISR;
#[doc = "`read()` method returns [risr::R](risr::R) reader structure"]
impl crate::Readable for RISR {}
#[doc = "raw interrupt status register"]
pub mod risr;
#[doc = "masked interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misr](misr) module"]
pub type MISR = crate::Reg<u32, _MISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISR;
#[doc = "`read()` method returns [misr::R](misr::R) reader structure"]
impl crate::Readable for MISR {}
#[doc = "masked interrupt status register"]
pub mod misr;
#[doc = "key registers\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [k0lr](k0lr) module"]
pub type K0LR = crate::Reg<u32, _K0LR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _K0LR;
#[doc = "`write(|w| ..)` method takes [k0lr::W](k0lr::W) writer structure"]
impl crate::Writable for K0LR {}
#[doc = "key registers"]
pub mod k0lr;
#[doc = "key registers\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [k0rr](k0rr) module"]
pub type K0RR = crate::Reg<u32, _K0RR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _K0RR;
#[doc = "`write(|w| ..)` method takes [k0rr::W](k0rr::W) writer structure"]
impl crate::Writable for K0RR {}
#[doc = "key registers"]
pub mod k0rr;
#[doc = "key registers\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [k1lr](k1lr) module"]
pub type K1LR = crate::Reg<u32, _K1LR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _K1LR;
#[doc = "`write(|w| ..)` method takes [k1lr::W](k1lr::W) writer structure"]
impl crate::Writable for K1LR {}
#[doc = "key registers"]
pub mod k1lr;
#[doc = "key registers\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [k1rr](k1rr) module"]
pub type K1RR = crate::Reg<u32, _K1RR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _K1RR;
#[doc = "`write(|w| ..)` method takes [k1rr::W](k1rr::W) writer structure"]
impl crate::Writable for K1RR {}
#[doc = "key registers"]
pub mod k1rr;
#[doc = "key registers\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [k2lr](k2lr) module"]
pub type K2LR = crate::Reg<u32, _K2LR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _K2LR;
#[doc = "`write(|w| ..)` method takes [k2lr::W](k2lr::W) writer structure"]
impl crate::Writable for K2LR {}
#[doc = "key registers"]
pub mod k2lr;
#[doc = "key registers\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [k2rr](k2rr) module"]
pub type K2RR = crate::Reg<u32, _K2RR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _K2RR;
#[doc = "`write(|w| ..)` method takes [k2rr::W](k2rr::W) writer structure"]
impl crate::Writable for K2RR {}
#[doc = "key registers"]
pub mod k2rr;
#[doc = "key registers\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [k3lr](k3lr) module"]
pub type K3LR = crate::Reg<u32, _K3LR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _K3LR;
#[doc = "`write(|w| ..)` method takes [k3lr::W](k3lr::W) writer structure"]
impl crate::Writable for K3LR {}
#[doc = "key registers"]
pub mod k3lr;
#[doc = "key registers\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [k3rr](k3rr) module"]
pub type K3RR = crate::Reg<u32, _K3RR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _K3RR;
#[doc = "`write(|w| ..)` method takes [k3rr::W](k3rr::W) writer structure"]
impl crate::Writable for K3RR {}
#[doc = "key registers"]
pub mod k3rr;
#[doc = "initialization vector registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iv0lr](iv0lr) module"]
pub type IV0LR = crate::Reg<u32, _IV0LR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IV0LR;
#[doc = "`read()` method returns [iv0lr::R](iv0lr::R) reader structure"]
impl crate::Readable for IV0LR {}
#[doc = "`write(|w| ..)` method takes [iv0lr::W](iv0lr::W) writer structure"]
impl crate::Writable for IV0LR {}
#[doc = "initialization vector registers"]
pub mod iv0lr;
#[doc = "initialization vector registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iv0rr](iv0rr) module"]
pub type IV0RR = crate::Reg<u32, _IV0RR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IV0RR;
#[doc = "`read()` method returns [iv0rr::R](iv0rr::R) reader structure"]
impl crate::Readable for IV0RR {}
#[doc = "`write(|w| ..)` method takes [iv0rr::W](iv0rr::W) writer structure"]
impl crate::Writable for IV0RR {}
#[doc = "initialization vector registers"]
pub mod iv0rr;
#[doc = "initialization vector registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iv1lr](iv1lr) module"]
pub type IV1LR = crate::Reg<u32, _IV1LR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IV1LR;
#[doc = "`read()` method returns [iv1lr::R](iv1lr::R) reader structure"]
impl crate::Readable for IV1LR {}
#[doc = "`write(|w| ..)` method takes [iv1lr::W](iv1lr::W) writer structure"]
impl crate::Writable for IV1LR {}
#[doc = "initialization vector registers"]
pub mod iv1lr;
#[doc = "initialization vector registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iv1rr](iv1rr) module"]
pub type IV1RR = crate::Reg<u32, _IV1RR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IV1RR;
#[doc = "`read()` method returns [iv1rr::R](iv1rr::R) reader structure"]
impl crate::Readable for IV1RR {}
#[doc = "`write(|w| ..)` method takes [iv1rr::W](iv1rr::W) writer structure"]
impl crate::Writable for IV1RR {}
#[doc = "initialization vector registers"]
pub mod iv1rr;
#[doc = "context swap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgcmccm0r](csgcmccm0r) module"]
pub type CSGCMCCM0R = crate::Reg<u32, _CSGCMCCM0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSGCMCCM0R;
#[doc = "`read()` method returns [csgcmccm0r::R](csgcmccm0r::R) reader structure"]
impl crate::Readable for CSGCMCCM0R {}
#[doc = "`write(|w| ..)` method takes [csgcmccm0r::W](csgcmccm0r::W) writer structure"]
impl crate::Writable for CSGCMCCM0R {}
#[doc = "context swap register"]
pub mod csgcmccm0r;
#[doc = "context swap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgcmccm1r](csgcmccm1r) module"]
pub type CSGCMCCM1R = crate::Reg<u32, _CSGCMCCM1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSGCMCCM1R;
#[doc = "`read()` method returns [csgcmccm1r::R](csgcmccm1r::R) reader structure"]
impl crate::Readable for CSGCMCCM1R {}
#[doc = "`write(|w| ..)` method takes [csgcmccm1r::W](csgcmccm1r::W) writer structure"]
impl crate::Writable for CSGCMCCM1R {}
#[doc = "context swap register"]
pub mod csgcmccm1r;
#[doc = "context swap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgcmccm2r](csgcmccm2r) module"]
pub type CSGCMCCM2R = crate::Reg<u32, _CSGCMCCM2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSGCMCCM2R;
#[doc = "`read()` method returns [csgcmccm2r::R](csgcmccm2r::R) reader structure"]
impl crate::Readable for CSGCMCCM2R {}
#[doc = "`write(|w| ..)` method takes [csgcmccm2r::W](csgcmccm2r::W) writer structure"]
impl crate::Writable for CSGCMCCM2R {}
#[doc = "context swap register"]
pub mod csgcmccm2r;
#[doc = "context swap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgcmccm3r](csgcmccm3r) module"]
pub type CSGCMCCM3R = crate::Reg<u32, _CSGCMCCM3R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSGCMCCM3R;
#[doc = "`read()` method returns [csgcmccm3r::R](csgcmccm3r::R) reader structure"]
impl crate::Readable for CSGCMCCM3R {}
#[doc = "`write(|w| ..)` method takes [csgcmccm3r::W](csgcmccm3r::W) writer structure"]
impl crate::Writable for CSGCMCCM3R {}
#[doc = "context swap register"]
pub mod csgcmccm3r;
#[doc = "context swap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgcmccm4r](csgcmccm4r) module"]
pub type CSGCMCCM4R = crate::Reg<u32, _CSGCMCCM4R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSGCMCCM4R;
#[doc = "`read()` method returns [csgcmccm4r::R](csgcmccm4r::R) reader structure"]
impl crate::Readable for CSGCMCCM4R {}
#[doc = "`write(|w| ..)` method takes [csgcmccm4r::W](csgcmccm4r::W) writer structure"]
impl crate::Writable for CSGCMCCM4R {}
#[doc = "context swap register"]
pub mod csgcmccm4r;
#[doc = "context swap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgcmccm5r](csgcmccm5r) module"]
pub type CSGCMCCM5R = crate::Reg<u32, _CSGCMCCM5R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSGCMCCM5R;
#[doc = "`read()` method returns [csgcmccm5r::R](csgcmccm5r::R) reader structure"]
impl crate::Readable for CSGCMCCM5R {}
#[doc = "`write(|w| ..)` method takes [csgcmccm5r::W](csgcmccm5r::W) writer structure"]
impl crate::Writable for CSGCMCCM5R {}
#[doc = "context swap register"]
pub mod csgcmccm5r;
#[doc = "context swap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgcmccm6r](csgcmccm6r) module"]
pub type CSGCMCCM6R = crate::Reg<u32, _CSGCMCCM6R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSGCMCCM6R;
#[doc = "`read()` method returns [csgcmccm6r::R](csgcmccm6r::R) reader structure"]
impl crate::Readable for CSGCMCCM6R {}
#[doc = "`write(|w| ..)` method takes [csgcmccm6r::W](csgcmccm6r::W) writer structure"]
impl crate::Writable for CSGCMCCM6R {}
#[doc = "context swap register"]
pub mod csgcmccm6r;
#[doc = "context swap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgcmccm7r](csgcmccm7r) module"]
pub type CSGCMCCM7R = crate::Reg<u32, _CSGCMCCM7R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSGCMCCM7R;
#[doc = "`read()` method returns [csgcmccm7r::R](csgcmccm7r::R) reader structure"]
impl crate::Readable for CSGCMCCM7R {}
#[doc = "`write(|w| ..)` method takes [csgcmccm7r::W](csgcmccm7r::W) writer structure"]
impl crate::Writable for CSGCMCCM7R {}
#[doc = "context swap register"]
pub mod csgcmccm7r;
#[doc = "context swap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgcm0r](csgcm0r) module"]
pub type CSGCM0R = crate::Reg<u32, _CSGCM0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSGCM0R;
#[doc = "`read()` method returns [csgcm0r::R](csgcm0r::R) reader structure"]
impl crate::Readable for CSGCM0R {}
#[doc = "`write(|w| ..)` method takes [csgcm0r::W](csgcm0r::W) writer structure"]
impl crate::Writable for CSGCM0R {}
#[doc = "context swap register"]
pub mod csgcm0r;
#[doc = "context swap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgcm1r](csgcm1r) module"]
pub type CSGCM1R = crate::Reg<u32, _CSGCM1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSGCM1R;
#[doc = "`read()` method returns [csgcm1r::R](csgcm1r::R) reader structure"]
impl crate::Readable for CSGCM1R {}
#[doc = "`write(|w| ..)` method takes [csgcm1r::W](csgcm1r::W) writer structure"]
impl crate::Writable for CSGCM1R {}
#[doc = "context swap register"]
pub mod csgcm1r;
#[doc = "context swap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgcm2r](csgcm2r) module"]
pub type CSGCM2R = crate::Reg<u32, _CSGCM2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSGCM2R;
#[doc = "`read()` method returns [csgcm2r::R](csgcm2r::R) reader structure"]
impl crate::Readable for CSGCM2R {}
#[doc = "`write(|w| ..)` method takes [csgcm2r::W](csgcm2r::W) writer structure"]
impl crate::Writable for CSGCM2R {}
#[doc = "context swap register"]
pub mod csgcm2r;
#[doc = "context swap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgcm3r](csgcm3r) module"]
pub type CSGCM3R = crate::Reg<u32, _CSGCM3R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSGCM3R;
#[doc = "`read()` method returns [csgcm3r::R](csgcm3r::R) reader structure"]
impl crate::Readable for CSGCM3R {}
#[doc = "`write(|w| ..)` method takes [csgcm3r::W](csgcm3r::W) writer structure"]
impl crate::Writable for CSGCM3R {}
#[doc = "context swap register"]
pub mod csgcm3r;
#[doc = "context swap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgcm4r](csgcm4r) module"]
pub type CSGCM4R = crate::Reg<u32, _CSGCM4R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSGCM4R;
#[doc = "`read()` method returns [csgcm4r::R](csgcm4r::R) reader structure"]
impl crate::Readable for CSGCM4R {}
#[doc = "`write(|w| ..)` method takes [csgcm4r::W](csgcm4r::W) writer structure"]
impl crate::Writable for CSGCM4R {}
#[doc = "context swap register"]
pub mod csgcm4r;
#[doc = "context swap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgcm5r](csgcm5r) module"]
pub type CSGCM5R = crate::Reg<u32, _CSGCM5R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSGCM5R;
#[doc = "`read()` method returns [csgcm5r::R](csgcm5r::R) reader structure"]
impl crate::Readable for CSGCM5R {}
#[doc = "`write(|w| ..)` method takes [csgcm5r::W](csgcm5r::W) writer structure"]
impl crate::Writable for CSGCM5R {}
#[doc = "context swap register"]
pub mod csgcm5r;
#[doc = "context swap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgcm6r](csgcm6r) module"]
pub type CSGCM6R = crate::Reg<u32, _CSGCM6R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSGCM6R;
#[doc = "`read()` method returns [csgcm6r::R](csgcm6r::R) reader structure"]
impl crate::Readable for CSGCM6R {}
#[doc = "`write(|w| ..)` method takes [csgcm6r::W](csgcm6r::W) writer structure"]
impl crate::Writable for CSGCM6R {}
#[doc = "context swap register"]
pub mod csgcm6r;
#[doc = "context swap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgcm7r](csgcm7r) module"]
pub type CSGCM7R = crate::Reg<u32, _CSGCM7R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSGCM7R;
#[doc = "`read()` method returns [csgcm7r::R](csgcm7r::R) reader structure"]
impl crate::Readable for CSGCM7R {}
#[doc = "`write(|w| ..)` method takes [csgcm7r::W](csgcm7r::W) writer structure"]
impl crate::Writable for CSGCM7R {}
#[doc = "context swap register"]
pub mod csgcm7r;
