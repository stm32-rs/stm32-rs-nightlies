#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MDIOS configuration register"]
    pub cr: CR,
    #[doc = "0x04 - MDIOS write flag register"]
    pub wrfr: WRFR,
    #[doc = "0x08 - MDIOS clear write flag register"]
    pub cwrfr: CWRFR,
    #[doc = "0x0c - MDIOS read flag register"]
    pub rdfr: RDFR,
    #[doc = "0x10 - MDIOS clear read flag register"]
    pub crdfr: CRDFR,
    #[doc = "0x14 - MDIOS status register"]
    pub sr: SR,
    #[doc = "0x18 - MDIOS clear flag register"]
    pub clrfr: CLRFR,
    #[doc = "0x1c - MDIOS input data register 0"]
    pub dinr0: DINR0,
    #[doc = "0x20 - MDIOS input data register 1"]
    pub dinr1: DINR1,
    #[doc = "0x24 - MDIOS input data register 2"]
    pub dinr2: DINR2,
    #[doc = "0x28 - MDIOS input data register 3"]
    pub dinr3: DINR3,
    #[doc = "0x2c - MDIOS input data register 4"]
    pub dinr4: DINR4,
    #[doc = "0x30 - MDIOS input data register 5"]
    pub dinr5: DINR5,
    #[doc = "0x34 - MDIOS input data register 6"]
    pub dinr6: DINR6,
    #[doc = "0x38 - MDIOS input data register 7"]
    pub dinr7: DINR7,
    #[doc = "0x3c - MDIOS input data register 8"]
    pub dinr8: DINR8,
    #[doc = "0x40 - MDIOS input data register 9"]
    pub dinr9: DINR9,
    #[doc = "0x44 - MDIOS input data register 10"]
    pub dinr10: DINR10,
    #[doc = "0x48 - MDIOS input data register 11"]
    pub dinr11: DINR11,
    #[doc = "0x4c - MDIOS input data register 12"]
    pub dinr12: DINR12,
    #[doc = "0x50 - MDIOS input data register 13"]
    pub dinr13: DINR13,
    #[doc = "0x54 - MDIOS input data register 14"]
    pub dinr14: DINR14,
    #[doc = "0x58 - MDIOS input data register 15"]
    pub dinr15: DINR15,
    #[doc = "0x5c - MDIOS input data register 16"]
    pub dinr16: DINR16,
    #[doc = "0x60 - MDIOS input data register 17"]
    pub dinr17: DINR17,
    #[doc = "0x64 - MDIOS input data register 18"]
    pub dinr18: DINR18,
    #[doc = "0x68 - MDIOS input data register 19"]
    pub dinr19: DINR19,
    #[doc = "0x6c - MDIOS input data register 20"]
    pub dinr20: DINR20,
    #[doc = "0x70 - MDIOS input data register 21"]
    pub dinr21: DINR21,
    #[doc = "0x74 - MDIOS input data register 22"]
    pub dinr22: DINR22,
    #[doc = "0x78 - MDIOS input data register 23"]
    pub dinr23: DINR23,
    #[doc = "0x7c - MDIOS input data register 24"]
    pub dinr24: DINR24,
    #[doc = "0x80 - MDIOS input data register 25"]
    pub dinr25: DINR25,
    #[doc = "0x84 - MDIOS input data register 26"]
    pub dinr26: DINR26,
    #[doc = "0x88 - MDIOS input data register 27"]
    pub dinr27: DINR27,
    #[doc = "0x8c - MDIOS input data register 28"]
    pub dinr28: DINR28,
    #[doc = "0x90 - MDIOS input data register 29"]
    pub dinr29: DINR29,
    #[doc = "0x94 - MDIOS input data register 30"]
    pub dinr30: DINR30,
    #[doc = "0x98 - MDIOS input data register 31"]
    pub dinr31: DINR31,
    #[doc = "0x9c - MDIOS output data register 0"]
    pub doutr0: DOUTR0,
    #[doc = "0xa0 - MDIOS output data register 1"]
    pub doutr1: DOUTR1,
    #[doc = "0xa4 - MDIOS output data register 2"]
    pub doutr2: DOUTR2,
    #[doc = "0xa8 - MDIOS output data register 3"]
    pub doutr3: DOUTR3,
    #[doc = "0xac - MDIOS output data register 4"]
    pub doutr4: DOUTR4,
    #[doc = "0xb0 - MDIOS output data register 5"]
    pub doutr5: DOUTR5,
    #[doc = "0xb4 - MDIOS output data register 6"]
    pub doutr6: DOUTR6,
    #[doc = "0xb8 - MDIOS output data register 7"]
    pub doutr7: DOUTR7,
    #[doc = "0xbc - MDIOS output data register 8"]
    pub doutr8: DOUTR8,
    #[doc = "0xc0 - MDIOS output data register 9"]
    pub doutr9: DOUTR9,
    #[doc = "0xc4 - MDIOS output data register 10"]
    pub doutr10: DOUTR10,
    #[doc = "0xc8 - MDIOS output data register 11"]
    pub doutr11: DOUTR11,
    #[doc = "0xcc - MDIOS output data register 12"]
    pub doutr12: DOUTR12,
    #[doc = "0xd0 - MDIOS output data register 13"]
    pub doutr13: DOUTR13,
    #[doc = "0xd4 - MDIOS output data register 14"]
    pub doutr14: DOUTR14,
    #[doc = "0xd8 - MDIOS output data register 15"]
    pub doutr15: DOUTR15,
    #[doc = "0xdc - MDIOS output data register 16"]
    pub doutr16: DOUTR16,
    #[doc = "0xe0 - MDIOS output data register 17"]
    pub doutr17: DOUTR17,
    #[doc = "0xe4 - MDIOS output data register 18"]
    pub doutr18: DOUTR18,
    #[doc = "0xe8 - MDIOS output data register 19"]
    pub doutr19: DOUTR19,
    #[doc = "0xec - MDIOS output data register 20"]
    pub doutr20: DOUTR20,
    #[doc = "0xf0 - MDIOS output data register 21"]
    pub doutr21: DOUTR21,
    #[doc = "0xf4 - MDIOS output data register 22"]
    pub doutr22: DOUTR22,
    #[doc = "0xf8 - MDIOS output data register 23"]
    pub doutr23: DOUTR23,
    #[doc = "0xfc - MDIOS output data register 24"]
    pub doutr24: DOUTR24,
    #[doc = "0x100 - MDIOS output data register 25"]
    pub doutr25: DOUTR25,
    #[doc = "0x104 - MDIOS output data register 26"]
    pub doutr26: DOUTR26,
    #[doc = "0x108 - MDIOS output data register 27"]
    pub doutr27: DOUTR27,
    #[doc = "0x10c - MDIOS output data register 28"]
    pub doutr28: DOUTR28,
    #[doc = "0x110 - MDIOS output data register 29"]
    pub doutr29: DOUTR29,
    #[doc = "0x114 - MDIOS output data register 30"]
    pub doutr30: DOUTR30,
    #[doc = "0x118 - MDIOS output data register 31"]
    pub doutr31: DOUTR31,
}
#[doc = "MDIOS configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "MDIOS configuration register"]
pub mod cr;
#[doc = "MDIOS write flag register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrfr](wrfr) module"]
pub type WRFR = crate::Reg<u32, _WRFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WRFR;
#[doc = "`read()` method returns [wrfr::R](wrfr::R) reader structure"]
impl crate::Readable for WRFR {}
#[doc = "MDIOS write flag register"]
pub mod wrfr;
#[doc = "MDIOS clear write flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cwrfr](cwrfr) module"]
pub type CWRFR = crate::Reg<u32, _CWRFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CWRFR;
#[doc = "`read()` method returns [cwrfr::R](cwrfr::R) reader structure"]
impl crate::Readable for CWRFR {}
#[doc = "`write(|w| ..)` method takes [cwrfr::W](cwrfr::W) writer structure"]
impl crate::Writable for CWRFR {}
#[doc = "MDIOS clear write flag register"]
pub mod cwrfr;
#[doc = "MDIOS read flag register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdfr](rdfr) module"]
pub type RDFR = crate::Reg<u32, _RDFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDFR;
#[doc = "`read()` method returns [rdfr::R](rdfr::R) reader structure"]
impl crate::Readable for RDFR {}
#[doc = "MDIOS read flag register"]
pub mod rdfr;
#[doc = "MDIOS clear read flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crdfr](crdfr) module"]
pub type CRDFR = crate::Reg<u32, _CRDFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRDFR;
#[doc = "`read()` method returns [crdfr::R](crdfr::R) reader structure"]
impl crate::Readable for CRDFR {}
#[doc = "`write(|w| ..)` method takes [crdfr::W](crdfr::W) writer structure"]
impl crate::Writable for CRDFR {}
#[doc = "MDIOS clear read flag register"]
pub mod crdfr;
#[doc = "MDIOS status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "MDIOS status register"]
pub mod sr;
#[doc = "MDIOS clear flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clrfr](clrfr) module"]
pub type CLRFR = crate::Reg<u32, _CLRFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLRFR;
#[doc = "`read()` method returns [clrfr::R](clrfr::R) reader structure"]
impl crate::Readable for CLRFR {}
#[doc = "`write(|w| ..)` method takes [clrfr::W](clrfr::W) writer structure"]
impl crate::Writable for CLRFR {}
#[doc = "MDIOS clear flag register"]
pub mod clrfr;
#[doc = "MDIOS input data register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr0](dinr0) module"]
pub type DINR0 = crate::Reg<u32, _DINR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DINR0;
#[doc = "`read()` method returns [dinr0::R](dinr0::R) reader structure"]
impl crate::Readable for DINR0 {}
#[doc = "MDIOS input data register 0"]
pub mod dinr0;
#[doc = "MDIOS input data register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr1](dinr1) module"]
pub type DINR1 = crate::Reg<u32, _DINR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DINR1;
#[doc = "`read()` method returns [dinr1::R](dinr1::R) reader structure"]
impl crate::Readable for DINR1 {}
#[doc = "MDIOS input data register 1"]
pub mod dinr1;
#[doc = "MDIOS input data register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr2](dinr2) module"]
pub type DINR2 = crate::Reg<u32, _DINR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DINR2;
#[doc = "`read()` method returns [dinr2::R](dinr2::R) reader structure"]
impl crate::Readable for DINR2 {}
#[doc = "MDIOS input data register 2"]
pub mod dinr2;
#[doc = "MDIOS input data register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr3](dinr3) module"]
pub type DINR3 = crate::Reg<u32, _DINR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DINR3;
#[doc = "`read()` method returns [dinr3::R](dinr3::R) reader structure"]
impl crate::Readable for DINR3 {}
#[doc = "MDIOS input data register 3"]
pub mod dinr3;
#[doc = "MDIOS input data register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr4](dinr4) module"]
pub type DINR4 = crate::Reg<u32, _DINR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DINR4;
#[doc = "`read()` method returns [dinr4::R](dinr4::R) reader structure"]
impl crate::Readable for DINR4 {}
#[doc = "MDIOS input data register 4"]
pub mod dinr4;
#[doc = "MDIOS input data register 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr5](dinr5) module"]
pub type DINR5 = crate::Reg<u32, _DINR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DINR5;
#[doc = "`read()` method returns [dinr5::R](dinr5::R) reader structure"]
impl crate::Readable for DINR5 {}
#[doc = "MDIOS input data register 5"]
pub mod dinr5;
#[doc = "MDIOS input data register 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr6](dinr6) module"]
pub type DINR6 = crate::Reg<u32, _DINR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DINR6;
#[doc = "`read()` method returns [dinr6::R](dinr6::R) reader structure"]
impl crate::Readable for DINR6 {}
#[doc = "MDIOS input data register 6"]
pub mod dinr6;
#[doc = "MDIOS input data register 7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr7](dinr7) module"]
pub type DINR7 = crate::Reg<u32, _DINR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DINR7;
#[doc = "`read()` method returns [dinr7::R](dinr7::R) reader structure"]
impl crate::Readable for DINR7 {}
#[doc = "MDIOS input data register 7"]
pub mod dinr7;
#[doc = "MDIOS input data register 8\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr8](dinr8) module"]
pub type DINR8 = crate::Reg<u32, _DINR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DINR8;
#[doc = "`read()` method returns [dinr8::R](dinr8::R) reader structure"]
impl crate::Readable for DINR8 {}
#[doc = "MDIOS input data register 8"]
pub mod dinr8;
#[doc = "MDIOS input data register 9\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr9](dinr9) module"]
pub type DINR9 = crate::Reg<u32, _DINR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DINR9;
#[doc = "`read()` method returns [dinr9::R](dinr9::R) reader structure"]
impl crate::Readable for DINR9 {}
#[doc = "MDIOS input data register 9"]
pub mod dinr9;
#[doc = "MDIOS input data register 10\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr10](dinr10) module"]
pub type DINR10 = crate::Reg<u32, _DINR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DINR10;
#[doc = "`read()` method returns [dinr10::R](dinr10::R) reader structure"]
impl crate::Readable for DINR10 {}
#[doc = "MDIOS input data register 10"]
pub mod dinr10;
#[doc = "MDIOS input data register 11\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr11](dinr11) module"]
pub type DINR11 = crate::Reg<u32, _DINR11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DINR11;
#[doc = "`read()` method returns [dinr11::R](dinr11::R) reader structure"]
impl crate::Readable for DINR11 {}
#[doc = "MDIOS input data register 11"]
pub mod dinr11;
#[doc = "MDIOS input data register 12\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr12](dinr12) module"]
pub type DINR12 = crate::Reg<u32, _DINR12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DINR12;
#[doc = "`read()` method returns [dinr12::R](dinr12::R) reader structure"]
impl crate::Readable for DINR12 {}
#[doc = "MDIOS input data register 12"]
pub mod dinr12;
#[doc = "MDIOS input data register 13\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr13](dinr13) module"]
pub type DINR13 = crate::Reg<u32, _DINR13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DINR13;
#[doc = "`read()` method returns [dinr13::R](dinr13::R) reader structure"]
impl crate::Readable for DINR13 {}
#[doc = "MDIOS input data register 13"]
pub mod dinr13;
#[doc = "MDIOS input data register 14\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr14](dinr14) module"]
pub type DINR14 = crate::Reg<u32, _DINR14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DINR14;
#[doc = "`read()` method returns [dinr14::R](dinr14::R) reader structure"]
impl crate::Readable for DINR14 {}
#[doc = "MDIOS input data register 14"]
pub mod dinr14;
#[doc = "MDIOS input data register 15\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr15](dinr15) module"]
pub type DINR15 = crate::Reg<u32, _DINR15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DINR15;
#[doc = "`read()` method returns [dinr15::R](dinr15::R) reader structure"]
impl crate::Readable for DINR15 {}
#[doc = "MDIOS input data register 15"]
pub mod dinr15;
#[doc = "MDIOS input data register 16\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr16](dinr16) module"]
pub type DINR16 = crate::Reg<u32, _DINR16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DINR16;
#[doc = "`read()` method returns [dinr16::R](dinr16::R) reader structure"]
impl crate::Readable for DINR16 {}
#[doc = "MDIOS input data register 16"]
pub mod dinr16;
#[doc = "MDIOS input data register 17\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr17](dinr17) module"]
pub type DINR17 = crate::Reg<u32, _DINR17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DINR17;
#[doc = "`read()` method returns [dinr17::R](dinr17::R) reader structure"]
impl crate::Readable for DINR17 {}
#[doc = "MDIOS input data register 17"]
pub mod dinr17;
#[doc = "MDIOS input data register 18\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr18](dinr18) module"]
pub type DINR18 = crate::Reg<u32, _DINR18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DINR18;
#[doc = "`read()` method returns [dinr18::R](dinr18::R) reader structure"]
impl crate::Readable for DINR18 {}
#[doc = "MDIOS input data register 18"]
pub mod dinr18;
#[doc = "MDIOS input data register 19\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr19](dinr19) module"]
pub type DINR19 = crate::Reg<u32, _DINR19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DINR19;
#[doc = "`read()` method returns [dinr19::R](dinr19::R) reader structure"]
impl crate::Readable for DINR19 {}
#[doc = "MDIOS input data register 19"]
pub mod dinr19;
#[doc = "MDIOS input data register 20\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr20](dinr20) module"]
pub type DINR20 = crate::Reg<u32, _DINR20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DINR20;
#[doc = "`read()` method returns [dinr20::R](dinr20::R) reader structure"]
impl crate::Readable for DINR20 {}
#[doc = "MDIOS input data register 20"]
pub mod dinr20;
#[doc = "MDIOS input data register 21\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr21](dinr21) module"]
pub type DINR21 = crate::Reg<u32, _DINR21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DINR21;
#[doc = "`read()` method returns [dinr21::R](dinr21::R) reader structure"]
impl crate::Readable for DINR21 {}
#[doc = "MDIOS input data register 21"]
pub mod dinr21;
#[doc = "MDIOS input data register 22\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr22](dinr22) module"]
pub type DINR22 = crate::Reg<u32, _DINR22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DINR22;
#[doc = "`read()` method returns [dinr22::R](dinr22::R) reader structure"]
impl crate::Readable for DINR22 {}
#[doc = "MDIOS input data register 22"]
pub mod dinr22;
#[doc = "MDIOS input data register 23\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr23](dinr23) module"]
pub type DINR23 = crate::Reg<u32, _DINR23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DINR23;
#[doc = "`read()` method returns [dinr23::R](dinr23::R) reader structure"]
impl crate::Readable for DINR23 {}
#[doc = "MDIOS input data register 23"]
pub mod dinr23;
#[doc = "MDIOS input data register 24\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr24](dinr24) module"]
pub type DINR24 = crate::Reg<u32, _DINR24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DINR24;
#[doc = "`read()` method returns [dinr24::R](dinr24::R) reader structure"]
impl crate::Readable for DINR24 {}
#[doc = "MDIOS input data register 24"]
pub mod dinr24;
#[doc = "MDIOS input data register 25\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr25](dinr25) module"]
pub type DINR25 = crate::Reg<u32, _DINR25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DINR25;
#[doc = "`read()` method returns [dinr25::R](dinr25::R) reader structure"]
impl crate::Readable for DINR25 {}
#[doc = "MDIOS input data register 25"]
pub mod dinr25;
#[doc = "MDIOS input data register 26\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr26](dinr26) module"]
pub type DINR26 = crate::Reg<u32, _DINR26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DINR26;
#[doc = "`read()` method returns [dinr26::R](dinr26::R) reader structure"]
impl crate::Readable for DINR26 {}
#[doc = "MDIOS input data register 26"]
pub mod dinr26;
#[doc = "MDIOS input data register 27\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr27](dinr27) module"]
pub type DINR27 = crate::Reg<u32, _DINR27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DINR27;
#[doc = "`read()` method returns [dinr27::R](dinr27::R) reader structure"]
impl crate::Readable for DINR27 {}
#[doc = "MDIOS input data register 27"]
pub mod dinr27;
#[doc = "MDIOS input data register 28\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr28](dinr28) module"]
pub type DINR28 = crate::Reg<u32, _DINR28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DINR28;
#[doc = "`read()` method returns [dinr28::R](dinr28::R) reader structure"]
impl crate::Readable for DINR28 {}
#[doc = "MDIOS input data register 28"]
pub mod dinr28;
#[doc = "MDIOS input data register 29\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr29](dinr29) module"]
pub type DINR29 = crate::Reg<u32, _DINR29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DINR29;
#[doc = "`read()` method returns [dinr29::R](dinr29::R) reader structure"]
impl crate::Readable for DINR29 {}
#[doc = "MDIOS input data register 29"]
pub mod dinr29;
#[doc = "MDIOS input data register 30\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr30](dinr30) module"]
pub type DINR30 = crate::Reg<u32, _DINR30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DINR30;
#[doc = "`read()` method returns [dinr30::R](dinr30::R) reader structure"]
impl crate::Readable for DINR30 {}
#[doc = "MDIOS input data register 30"]
pub mod dinr30;
#[doc = "MDIOS input data register 31\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr31](dinr31) module"]
pub type DINR31 = crate::Reg<u32, _DINR31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DINR31;
#[doc = "`read()` method returns [dinr31::R](dinr31::R) reader structure"]
impl crate::Readable for DINR31 {}
#[doc = "MDIOS input data register 31"]
pub mod dinr31;
#[doc = "MDIOS output data register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr0](doutr0) module"]
pub type DOUTR0 = crate::Reg<u32, _DOUTR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUTR0;
#[doc = "`read()` method returns [doutr0::R](doutr0::R) reader structure"]
impl crate::Readable for DOUTR0 {}
#[doc = "`write(|w| ..)` method takes [doutr0::W](doutr0::W) writer structure"]
impl crate::Writable for DOUTR0 {}
#[doc = "MDIOS output data register 0"]
pub mod doutr0;
#[doc = "MDIOS output data register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr1](doutr1) module"]
pub type DOUTR1 = crate::Reg<u32, _DOUTR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUTR1;
#[doc = "`read()` method returns [doutr1::R](doutr1::R) reader structure"]
impl crate::Readable for DOUTR1 {}
#[doc = "`write(|w| ..)` method takes [doutr1::W](doutr1::W) writer structure"]
impl crate::Writable for DOUTR1 {}
#[doc = "MDIOS output data register 1"]
pub mod doutr1;
#[doc = "MDIOS output data register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr2](doutr2) module"]
pub type DOUTR2 = crate::Reg<u32, _DOUTR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUTR2;
#[doc = "`read()` method returns [doutr2::R](doutr2::R) reader structure"]
impl crate::Readable for DOUTR2 {}
#[doc = "`write(|w| ..)` method takes [doutr2::W](doutr2::W) writer structure"]
impl crate::Writable for DOUTR2 {}
#[doc = "MDIOS output data register 2"]
pub mod doutr2;
#[doc = "MDIOS output data register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr3](doutr3) module"]
pub type DOUTR3 = crate::Reg<u32, _DOUTR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUTR3;
#[doc = "`read()` method returns [doutr3::R](doutr3::R) reader structure"]
impl crate::Readable for DOUTR3 {}
#[doc = "`write(|w| ..)` method takes [doutr3::W](doutr3::W) writer structure"]
impl crate::Writable for DOUTR3 {}
#[doc = "MDIOS output data register 3"]
pub mod doutr3;
#[doc = "MDIOS output data register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr4](doutr4) module"]
pub type DOUTR4 = crate::Reg<u32, _DOUTR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUTR4;
#[doc = "`read()` method returns [doutr4::R](doutr4::R) reader structure"]
impl crate::Readable for DOUTR4 {}
#[doc = "`write(|w| ..)` method takes [doutr4::W](doutr4::W) writer structure"]
impl crate::Writable for DOUTR4 {}
#[doc = "MDIOS output data register 4"]
pub mod doutr4;
#[doc = "MDIOS output data register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr5](doutr5) module"]
pub type DOUTR5 = crate::Reg<u32, _DOUTR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUTR5;
#[doc = "`read()` method returns [doutr5::R](doutr5::R) reader structure"]
impl crate::Readable for DOUTR5 {}
#[doc = "`write(|w| ..)` method takes [doutr5::W](doutr5::W) writer structure"]
impl crate::Writable for DOUTR5 {}
#[doc = "MDIOS output data register 5"]
pub mod doutr5;
#[doc = "MDIOS output data register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr6](doutr6) module"]
pub type DOUTR6 = crate::Reg<u32, _DOUTR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUTR6;
#[doc = "`read()` method returns [doutr6::R](doutr6::R) reader structure"]
impl crate::Readable for DOUTR6 {}
#[doc = "`write(|w| ..)` method takes [doutr6::W](doutr6::W) writer structure"]
impl crate::Writable for DOUTR6 {}
#[doc = "MDIOS output data register 6"]
pub mod doutr6;
#[doc = "MDIOS output data register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr7](doutr7) module"]
pub type DOUTR7 = crate::Reg<u32, _DOUTR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUTR7;
#[doc = "`read()` method returns [doutr7::R](doutr7::R) reader structure"]
impl crate::Readable for DOUTR7 {}
#[doc = "`write(|w| ..)` method takes [doutr7::W](doutr7::W) writer structure"]
impl crate::Writable for DOUTR7 {}
#[doc = "MDIOS output data register 7"]
pub mod doutr7;
#[doc = "MDIOS output data register 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr8](doutr8) module"]
pub type DOUTR8 = crate::Reg<u32, _DOUTR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUTR8;
#[doc = "`read()` method returns [doutr8::R](doutr8::R) reader structure"]
impl crate::Readable for DOUTR8 {}
#[doc = "`write(|w| ..)` method takes [doutr8::W](doutr8::W) writer structure"]
impl crate::Writable for DOUTR8 {}
#[doc = "MDIOS output data register 8"]
pub mod doutr8;
#[doc = "MDIOS output data register 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr9](doutr9) module"]
pub type DOUTR9 = crate::Reg<u32, _DOUTR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUTR9;
#[doc = "`read()` method returns [doutr9::R](doutr9::R) reader structure"]
impl crate::Readable for DOUTR9 {}
#[doc = "`write(|w| ..)` method takes [doutr9::W](doutr9::W) writer structure"]
impl crate::Writable for DOUTR9 {}
#[doc = "MDIOS output data register 9"]
pub mod doutr9;
#[doc = "MDIOS output data register 10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr10](doutr10) module"]
pub type DOUTR10 = crate::Reg<u32, _DOUTR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUTR10;
#[doc = "`read()` method returns [doutr10::R](doutr10::R) reader structure"]
impl crate::Readable for DOUTR10 {}
#[doc = "`write(|w| ..)` method takes [doutr10::W](doutr10::W) writer structure"]
impl crate::Writable for DOUTR10 {}
#[doc = "MDIOS output data register 10"]
pub mod doutr10;
#[doc = "MDIOS output data register 11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr11](doutr11) module"]
pub type DOUTR11 = crate::Reg<u32, _DOUTR11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUTR11;
#[doc = "`read()` method returns [doutr11::R](doutr11::R) reader structure"]
impl crate::Readable for DOUTR11 {}
#[doc = "`write(|w| ..)` method takes [doutr11::W](doutr11::W) writer structure"]
impl crate::Writable for DOUTR11 {}
#[doc = "MDIOS output data register 11"]
pub mod doutr11;
#[doc = "MDIOS output data register 12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr12](doutr12) module"]
pub type DOUTR12 = crate::Reg<u32, _DOUTR12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUTR12;
#[doc = "`read()` method returns [doutr12::R](doutr12::R) reader structure"]
impl crate::Readable for DOUTR12 {}
#[doc = "`write(|w| ..)` method takes [doutr12::W](doutr12::W) writer structure"]
impl crate::Writable for DOUTR12 {}
#[doc = "MDIOS output data register 12"]
pub mod doutr12;
#[doc = "MDIOS output data register 13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr13](doutr13) module"]
pub type DOUTR13 = crate::Reg<u32, _DOUTR13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUTR13;
#[doc = "`read()` method returns [doutr13::R](doutr13::R) reader structure"]
impl crate::Readable for DOUTR13 {}
#[doc = "`write(|w| ..)` method takes [doutr13::W](doutr13::W) writer structure"]
impl crate::Writable for DOUTR13 {}
#[doc = "MDIOS output data register 13"]
pub mod doutr13;
#[doc = "MDIOS output data register 14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr14](doutr14) module"]
pub type DOUTR14 = crate::Reg<u32, _DOUTR14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUTR14;
#[doc = "`read()` method returns [doutr14::R](doutr14::R) reader structure"]
impl crate::Readable for DOUTR14 {}
#[doc = "`write(|w| ..)` method takes [doutr14::W](doutr14::W) writer structure"]
impl crate::Writable for DOUTR14 {}
#[doc = "MDIOS output data register 14"]
pub mod doutr14;
#[doc = "MDIOS output data register 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr15](doutr15) module"]
pub type DOUTR15 = crate::Reg<u32, _DOUTR15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUTR15;
#[doc = "`read()` method returns [doutr15::R](doutr15::R) reader structure"]
impl crate::Readable for DOUTR15 {}
#[doc = "`write(|w| ..)` method takes [doutr15::W](doutr15::W) writer structure"]
impl crate::Writable for DOUTR15 {}
#[doc = "MDIOS output data register 15"]
pub mod doutr15;
#[doc = "MDIOS output data register 16\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr16](doutr16) module"]
pub type DOUTR16 = crate::Reg<u32, _DOUTR16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUTR16;
#[doc = "`read()` method returns [doutr16::R](doutr16::R) reader structure"]
impl crate::Readable for DOUTR16 {}
#[doc = "`write(|w| ..)` method takes [doutr16::W](doutr16::W) writer structure"]
impl crate::Writable for DOUTR16 {}
#[doc = "MDIOS output data register 16"]
pub mod doutr16;
#[doc = "MDIOS output data register 17\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr17](doutr17) module"]
pub type DOUTR17 = crate::Reg<u32, _DOUTR17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUTR17;
#[doc = "`read()` method returns [doutr17::R](doutr17::R) reader structure"]
impl crate::Readable for DOUTR17 {}
#[doc = "`write(|w| ..)` method takes [doutr17::W](doutr17::W) writer structure"]
impl crate::Writable for DOUTR17 {}
#[doc = "MDIOS output data register 17"]
pub mod doutr17;
#[doc = "MDIOS output data register 18\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr18](doutr18) module"]
pub type DOUTR18 = crate::Reg<u32, _DOUTR18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUTR18;
#[doc = "`read()` method returns [doutr18::R](doutr18::R) reader structure"]
impl crate::Readable for DOUTR18 {}
#[doc = "`write(|w| ..)` method takes [doutr18::W](doutr18::W) writer structure"]
impl crate::Writable for DOUTR18 {}
#[doc = "MDIOS output data register 18"]
pub mod doutr18;
#[doc = "MDIOS output data register 19\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr19](doutr19) module"]
pub type DOUTR19 = crate::Reg<u32, _DOUTR19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUTR19;
#[doc = "`read()` method returns [doutr19::R](doutr19::R) reader structure"]
impl crate::Readable for DOUTR19 {}
#[doc = "`write(|w| ..)` method takes [doutr19::W](doutr19::W) writer structure"]
impl crate::Writable for DOUTR19 {}
#[doc = "MDIOS output data register 19"]
pub mod doutr19;
#[doc = "MDIOS output data register 20\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr20](doutr20) module"]
pub type DOUTR20 = crate::Reg<u32, _DOUTR20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUTR20;
#[doc = "`read()` method returns [doutr20::R](doutr20::R) reader structure"]
impl crate::Readable for DOUTR20 {}
#[doc = "`write(|w| ..)` method takes [doutr20::W](doutr20::W) writer structure"]
impl crate::Writable for DOUTR20 {}
#[doc = "MDIOS output data register 20"]
pub mod doutr20;
#[doc = "MDIOS output data register 21\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr21](doutr21) module"]
pub type DOUTR21 = crate::Reg<u32, _DOUTR21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUTR21;
#[doc = "`read()` method returns [doutr21::R](doutr21::R) reader structure"]
impl crate::Readable for DOUTR21 {}
#[doc = "`write(|w| ..)` method takes [doutr21::W](doutr21::W) writer structure"]
impl crate::Writable for DOUTR21 {}
#[doc = "MDIOS output data register 21"]
pub mod doutr21;
#[doc = "MDIOS output data register 22\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr22](doutr22) module"]
pub type DOUTR22 = crate::Reg<u32, _DOUTR22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUTR22;
#[doc = "`read()` method returns [doutr22::R](doutr22::R) reader structure"]
impl crate::Readable for DOUTR22 {}
#[doc = "`write(|w| ..)` method takes [doutr22::W](doutr22::W) writer structure"]
impl crate::Writable for DOUTR22 {}
#[doc = "MDIOS output data register 22"]
pub mod doutr22;
#[doc = "MDIOS output data register 23\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr23](doutr23) module"]
pub type DOUTR23 = crate::Reg<u32, _DOUTR23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUTR23;
#[doc = "`read()` method returns [doutr23::R](doutr23::R) reader structure"]
impl crate::Readable for DOUTR23 {}
#[doc = "`write(|w| ..)` method takes [doutr23::W](doutr23::W) writer structure"]
impl crate::Writable for DOUTR23 {}
#[doc = "MDIOS output data register 23"]
pub mod doutr23;
#[doc = "MDIOS output data register 24\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr24](doutr24) module"]
pub type DOUTR24 = crate::Reg<u32, _DOUTR24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUTR24;
#[doc = "`read()` method returns [doutr24::R](doutr24::R) reader structure"]
impl crate::Readable for DOUTR24 {}
#[doc = "`write(|w| ..)` method takes [doutr24::W](doutr24::W) writer structure"]
impl crate::Writable for DOUTR24 {}
#[doc = "MDIOS output data register 24"]
pub mod doutr24;
#[doc = "MDIOS output data register 25\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr25](doutr25) module"]
pub type DOUTR25 = crate::Reg<u32, _DOUTR25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUTR25;
#[doc = "`read()` method returns [doutr25::R](doutr25::R) reader structure"]
impl crate::Readable for DOUTR25 {}
#[doc = "`write(|w| ..)` method takes [doutr25::W](doutr25::W) writer structure"]
impl crate::Writable for DOUTR25 {}
#[doc = "MDIOS output data register 25"]
pub mod doutr25;
#[doc = "MDIOS output data register 26\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr26](doutr26) module"]
pub type DOUTR26 = crate::Reg<u32, _DOUTR26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUTR26;
#[doc = "`read()` method returns [doutr26::R](doutr26::R) reader structure"]
impl crate::Readable for DOUTR26 {}
#[doc = "`write(|w| ..)` method takes [doutr26::W](doutr26::W) writer structure"]
impl crate::Writable for DOUTR26 {}
#[doc = "MDIOS output data register 26"]
pub mod doutr26;
#[doc = "MDIOS output data register 27\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr27](doutr27) module"]
pub type DOUTR27 = crate::Reg<u32, _DOUTR27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUTR27;
#[doc = "`read()` method returns [doutr27::R](doutr27::R) reader structure"]
impl crate::Readable for DOUTR27 {}
#[doc = "`write(|w| ..)` method takes [doutr27::W](doutr27::W) writer structure"]
impl crate::Writable for DOUTR27 {}
#[doc = "MDIOS output data register 27"]
pub mod doutr27;
#[doc = "MDIOS output data register 28\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr28](doutr28) module"]
pub type DOUTR28 = crate::Reg<u32, _DOUTR28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUTR28;
#[doc = "`read()` method returns [doutr28::R](doutr28::R) reader structure"]
impl crate::Readable for DOUTR28 {}
#[doc = "`write(|w| ..)` method takes [doutr28::W](doutr28::W) writer structure"]
impl crate::Writable for DOUTR28 {}
#[doc = "MDIOS output data register 28"]
pub mod doutr28;
#[doc = "MDIOS output data register 29\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr29](doutr29) module"]
pub type DOUTR29 = crate::Reg<u32, _DOUTR29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUTR29;
#[doc = "`read()` method returns [doutr29::R](doutr29::R) reader structure"]
impl crate::Readable for DOUTR29 {}
#[doc = "`write(|w| ..)` method takes [doutr29::W](doutr29::W) writer structure"]
impl crate::Writable for DOUTR29 {}
#[doc = "MDIOS output data register 29"]
pub mod doutr29;
#[doc = "MDIOS output data register 30\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr30](doutr30) module"]
pub type DOUTR30 = crate::Reg<u32, _DOUTR30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUTR30;
#[doc = "`read()` method returns [doutr30::R](doutr30::R) reader structure"]
impl crate::Readable for DOUTR30 {}
#[doc = "`write(|w| ..)` method takes [doutr30::W](doutr30::W) writer structure"]
impl crate::Writable for DOUTR30 {}
#[doc = "MDIOS output data register 30"]
pub mod doutr30;
#[doc = "MDIOS output data register 31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr31](doutr31) module"]
pub type DOUTR31 = crate::Reg<u32, _DOUTR31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUTR31;
#[doc = "`read()` method returns [doutr31::R](doutr31::R) reader structure"]
impl crate::Readable for DOUTR31 {}
#[doc = "`write(|w| ..)` method takes [doutr31::W](doutr31::W) writer structure"]
impl crate::Writable for DOUTR31 {}
#[doc = "MDIOS output data register 31"]
pub mod doutr31;
