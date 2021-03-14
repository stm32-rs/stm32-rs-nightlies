#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub cr: CR,
    #[doc = "0x04 - software trigger register"]
    pub swtrigr: SWTRIGR,
    #[doc = "0x08 - channel1 12-bit right-aligned data holding register"]
    pub dhr12r1: DHR12R1,
    #[doc = "0x0c - DAC channel1 12-bit left aligned data holding register"]
    pub dhr12l1: DHR12L1,
    #[doc = "0x10 - DAC channel1 8-bit right aligned data holding register"]
    pub dhr8r1: DHR8R1,
    _reserved5: [u8; 24usize],
    #[doc = "0x2c - DAC channel1 data output register"]
    pub dor1: DOR1,
    _reserved6: [u8; 4usize],
    #[doc = "0x34 - DAC status register"]
    pub sr: SR,
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
#[doc = "software trigger register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swtrigr](swtrigr) module"]
pub type SWTRIGR = crate::Reg<u32, _SWTRIGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWTRIGR;
#[doc = "`write(|w| ..)` method takes [swtrigr::W](swtrigr::W) writer structure"]
impl crate::Writable for SWTRIGR {}
#[doc = "software trigger register"]
pub mod swtrigr;
#[doc = "channel1 12-bit right-aligned data holding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhr12r1](dhr12r1) module"]
pub type DHR12R1 = crate::Reg<u32, _DHR12R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHR12R1;
#[doc = "`read()` method returns [dhr12r1::R](dhr12r1::R) reader structure"]
impl crate::Readable for DHR12R1 {}
#[doc = "`write(|w| ..)` method takes [dhr12r1::W](dhr12r1::W) writer structure"]
impl crate::Writable for DHR12R1 {}
#[doc = "channel1 12-bit right-aligned data holding register"]
pub mod dhr12r1;
#[doc = "DAC channel1 12-bit left aligned data holding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhr12l1](dhr12l1) module"]
pub type DHR12L1 = crate::Reg<u32, _DHR12L1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHR12L1;
#[doc = "`read()` method returns [dhr12l1::R](dhr12l1::R) reader structure"]
impl crate::Readable for DHR12L1 {}
#[doc = "`write(|w| ..)` method takes [dhr12l1::W](dhr12l1::W) writer structure"]
impl crate::Writable for DHR12L1 {}
#[doc = "DAC channel1 12-bit left aligned data holding register"]
pub mod dhr12l1;
#[doc = "DAC channel1 8-bit right aligned data holding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhr8r1](dhr8r1) module"]
pub type DHR8R1 = crate::Reg<u32, _DHR8R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHR8R1;
#[doc = "`read()` method returns [dhr8r1::R](dhr8r1::R) reader structure"]
impl crate::Readable for DHR8R1 {}
#[doc = "`write(|w| ..)` method takes [dhr8r1::W](dhr8r1::W) writer structure"]
impl crate::Writable for DHR8R1 {}
#[doc = "DAC channel1 8-bit right aligned data holding register"]
pub mod dhr8r1;
#[doc = "DAC channel1 data output register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dor1](dor1) module"]
pub type DOR1 = crate::Reg<u32, _DOR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOR1;
#[doc = "`read()` method returns [dor1::R](dor1::R) reader structure"]
impl crate::Readable for DOR1 {}
#[doc = "DAC channel1 data output register"]
pub mod dor1;
#[doc = "DAC status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "`write(|w| ..)` method takes [sr::W](sr::W) writer structure"]
impl crate::Writable for SR {}
#[doc = "DAC status register"]
pub mod sr;
